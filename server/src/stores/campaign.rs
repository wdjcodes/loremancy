use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub async fn get_users_campaigns(
    pool: &Pool<Postgres>,
    id: Uuid,
) -> Result<Vec<Campaign>, sqlx::Error> {
    sqlx::query_as!(
        Campaign,
        r#"
            SELECT 
                id, 
                name,
                owner_id, 
                created_at as "created_at: chrono::DateTime<chrono::Utc>"
            FROM campaigns c
            WHERE EXISTS(
                SELECT 1 
                FROM campaign_members cm
                WHERE cm.user_id = $1 AND cm.campaign_id = c.id
            );"#,
        id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_campaign(
    pool: &Pool<Postgres>,
    name: &str,
    owner_id: Uuid,
    member_ids: &[Uuid],
) -> Result<Campaign, sqlx::Error> {
    sqlx::query_as!(
        Campaign,
        r#"
            WITH new_campaign (id, name, owner_id, created_at) AS (
                INSERT INTO campaigns (
                    name,
                    owner_id
                )
                VALUES ( $1, $2 )
                RETURNING 
                    id,
                    name,
                    owner_id,
                    created_at
            ),
            owner_member AS (
                INSERT INTO campaign_members (
                    campaign_id,
                    user_id
                )
                SELECT new_campaign.id, $2
                FROM new_campaign
            ),
            inserted_members AS (
                INSERT INTO campaign_members (
                    campaign_id,
                    user_id
                )
                SELECT new_campaign.id, unnest($3::Uuid[])
                FROM new_campaign
            )
            SELECT id, name, owner_id, created_at as "created_at: chrono::DateTime<chrono::Utc>"
            FROM new_campaign;"#,
        name,
        owner_id,
        member_ids
    )
    .fetch_one(pool)
    .await
}
