use std::{env, sync::{Arc, Weak}};

use axum::{Router, extract::{Path, State, WebSocketUpgrade, ws::WebSocket}, response::IntoResponse, routing::get};
use dashmap::DashMap;
use parking_lot::RwLock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::sync::Mutex;
use uuid::Uuid;
use y_sync::{awareness::Awareness, net::BroadcastGroup};
use futures_util::StreamExt;
use yrs_axum::ws::{AxumSink, AxumStream};


pub struct Room {
    pub tx: tokio::sync::broadcast::Sender<Vec<u8>>,
    pub doc: RwLock<yrs::Doc>
}
pub struct AppState {
    _db: Pool<Postgres>,
    rooms: DashMap<Uuid, Weak<BroadcastGroup>>
}

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.expect("Error connecting to the database");
    
    let state = Arc::new(AppState {
        _db: pool,
        rooms: DashMap::new(),
    });
    // build our application with a route
    let app = Router::new()
        .route("/ws/note/{note_id}", get(ws_handler)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(State(state): State<Arc<AppState>>,Path(note_id): Path<Uuid>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, note_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, note_id: Uuid) {

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