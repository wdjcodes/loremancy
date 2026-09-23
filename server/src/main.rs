use std::{
    env,
    sync::{Arc, Weak},
};

use axum::{
    Router,
    routing::{get, post},
};
use axum_login::{
    AuthManagerLayerBuilder, login_required,
    tower_sessions::{Expiry, SessionManagerLayer},
};
use dashmap::DashMap;
use parking_lot::RwLock;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;
use y_sync::net::BroadcastGroup;

mod handlers;
use handlers::collaboration;

use crate::handlers::{
    auth::{self, AppBackend},
    campaign,
};

mod stores;
pub struct Room {
    pub tx: tokio::sync::broadcast::Sender<Vec<u8>>,
    pub doc: RwLock<yrs::Doc>,
}
pub struct AppState {
    pool: Pool<Postgres>,
    rooms: DashMap<Uuid, Weak<BroadcastGroup>>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to the database");

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to migrate the db to support the session store");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production (requires HTTPS)
        .with_expiry(Expiry::OnSessionEnd);
    let auth_layer =
        AuthManagerLayerBuilder::new(AppBackend { pool: pool.clone() }, session_layer).build();

    let state = Arc::new(AppState {
        pool,
        rooms: DashMap::new(),
    });

    let protected_routes = Router::new()
        .route("/api/campaigns", get(campaign::get_users_campaigns))
        .route("/api/campaign", post(campaign::create_campaign))
        // The login_required macro blocks requests without a valid session cookie
        .route_layer(login_required!(AppBackend, login_url = "/api/auth/login"));

    // build our application with a route
    let app = Router::new()
        .route("/api/register", post(auth::register))
        .route("/api/login", post(auth::login))
        .route("/api/logout", post(auth::logout))
        .route("/ws/note/{note_id}", get(collaboration::ws_handler))
        .merge(protected_routes)
        .layer(auth_layer)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
