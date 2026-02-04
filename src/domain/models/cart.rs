use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// DB type: order_status_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status_type", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Returned,
}

// DB type: payment_status_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status_type", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Paid,
    Failed,
    Refunded,
}

//==================| ORDER STRUTCT |=====================
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,

    pub user_id: Option<Uuid>,
    pub address_id: Option<Uuid>,
    pub shipping_address_snapshot: serde_json::Value,
    pub order_number: i32,

    pub status: OrderStatus,
    pub payment_status: PaymentStatus,

    pub total_amount: BigDecimal,
    pub shipping_fee: BigDecimal,

    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

//==================| ORDER ITEM STRUTCT |=====================
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub variant_id: Option<Uuid>,

    pub quantity: i32,
    pub price_at_purchase: BigDecimal,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserAddress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub recipient_name: String,
    pub recipient_phone: String,

    pub address_line: String,
    pub ward: Option<String>,
    pub district: String,
    pub city: String,

    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
