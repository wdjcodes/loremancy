use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    AppState,
    handlers::auth::AuthSession,
    stores::{self, campaign::Campaign},
};

pub async fn get_users_campaigns(
    State(state): State<Arc<AppState>>,
    auth_session: AuthSession,
) -> Result<Json<Vec<Campaign>>, StatusCode> {
    if let Some(user) = auth_session.user {
        let campaigns = stores::campaign::get_users_campaigns(&state.pool, user.id)
            .await
            .map_err(|err| {
                println!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        Ok(Json(campaigns))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Deserialize)]
pub struct CreateCampaign {
    name: String,
    members: Vec<Uuid>,
}

pub async fn create_campaign(
    State(state): State<Arc<AppState>>,
    auth_session: AuthSession,
    Json(body): Json<CreateCampaign>,
) -> Result<Json<Campaign>, StatusCode> {
    if let Some(user) = auth_session.user {
        let campaign =
            stores::campaign::create_campaign(&state.pool, &body.name, user.id, &body.members)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(campaign))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
