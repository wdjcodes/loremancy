use std::sync::Arc;

use axum::{extract::{Path, State, WebSocketUpgrade, ws::WebSocket}, http::StatusCode, response::IntoResponse};
use futures_util::StreamExt;
use tokio::sync::Mutex;
use uuid::Uuid;
use y_sync::{awareness::Awareness, net::BroadcastGroup};
use yrs_axum::ws::{AxumSink, AxumStream};

use crate::{AppState, handlers::auth::{AuthSession, BackendUser}};



pub async fn ws_handler(State(state): State<Arc<AppState>>,Path(note_id): Path<Uuid>, auth_session: AuthSession, ws: WebSocketUpgrade) -> Result<impl IntoResponse, StatusCode> {
    if let Some(user) = auth_session.user {
        Ok(ws.on_upgrade(move |socket| handle_socket(socket, state, note_id, user)))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, note_id: Uuid, _user: BackendUser) {

    let room = if let Some(refer) = state.rooms.get(&note_id) && let Some(room) = refer.upgrade() {
        room
    } else {
        let doc = yrs::Doc::new();
        let awareness = Arc::new(tokio::sync::RwLock::new(Awareness::new(doc)));
        let room = Arc::new(BroadcastGroup::new(awareness, 32).await);
        // let doc = Arc::new(RwLock::new(yrs::Doc::new()));
        state.rooms.insert(note_id, Arc::downgrade(&room));
        room
    };

    let (sink, stream) = socket.split();
    let sub = room.subscribe(Arc::new(Mutex::new(AxumSink::from(sink))), AxumStream::from(stream));

    match sub.completed().await {
        Ok(_) => println!("User hangup"),
        Err(_) => println!("Error in y-sync"),
    }

    // TODO: Currently a memory leak with the room not being freed. Need to also determine
    // the strategy for persisting document to the database when the room becomes vacant.
    // Persistance might end up necessatating a refactor away from using Weak both issues
    // should be handled together.
}