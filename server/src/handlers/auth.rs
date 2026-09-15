use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_login::{AuthUser, AuthnBackend};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::AppState;

/// Rust representation of the database User object. This should 
/// NEVER be sent to the front end, use [`User`] instead for sending
/// user data for rendering on the frontend
#[derive(Debug, FromRow, Clone)]
pub struct BackendUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    #[expect(unused)]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl From<BackendUser> for User {
    fn from(value: BackendUser) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreation {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl AuthUser for BackendUser {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct AppBackend {
    pub pool: PgPool,
}

pub type AuthSession = axum_login::AuthSession<AppBackend>;

impl AuthnBackend for AppBackend {
    type User = BackendUser;

    type Credentials = Credentials;

    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as::<_, BackendUser>("SELECT * FROM users WHERE username = $1")
            .bind(&creds.username)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(u) = user {
            let parsed_hash = match PasswordHash::new(&u.password_hash) {
                Ok(hash) => hash,
                Err(_) => return Ok(None), // Invalid hash format
            };
            match Argon2::default().verify_password(creds.password.as_bytes(), &parsed_hash) {
                Ok(_) => Ok(Some(u)),
                Err(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as::<_, BackendUser>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    mut auth_session: AuthSession,
    Json(user_creation): Json<UserCreation>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let hashed = Argon2::default()
        .hash_password(user_creation.password.as_bytes())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query_as::<_, BackendUser>(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user_creation.username) 
    .bind(&user_creation.email)
    .bind(hashed.to_string())
    .fetch_one(&state.pool)
    .await
    .map_err(|_| StatusCode::CONFLICT)?; // 409 if username or email already exists

    // Auto-login after registration
    auth_session
        .login(&user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> Result<Json<User>, StatusCode> {
    let user = match auth_session.authenticate(creds).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Creates the DB session and sets the HTTP-only cookie
    auth_session.login(&user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user.into()))
}

pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
    // Deletes the session from the DB and clears the cookie
    let _ = auth_session.logout().await;
    StatusCode::OK
}
