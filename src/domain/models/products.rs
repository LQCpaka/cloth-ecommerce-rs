use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Products {
    pub id: Uuid,
    category_id: i32,
    name: String,
    slug: String,
    description: Option<String>,
    base_price: BigDecimal,
    status_text: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
