// use serde::{Deserialize, Serialize};
// use sqlx::types::Json;
// use uuid::Uuid;

// #[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "order_status_type", rename_all = "lowercase")]
// #[serde(rename_all = "snake_case")]
// pub enum OrderStatusType {
//     Pending,
//     Confirmed,
//     Processing,
//     Shipped,
//     Delivered,
//     Cancelled,
//     Returned,
// }

// #[derive(PartialEq, Debug, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "payment_status_type", rename_all = "lowercase")]
// #[serde(rename_all = "snake_case")]
// pub enum PaymentStatusType {
//     Pending,
//     Paid,
//     Failed,
//     Refunded,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserAddresses {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub recipient_name: String,
//     pub recipient_phone: String,
//     pub address_line: String,
//     pub ward: String,
//     pub district: String,
//     pub city: String,

//     pub is_default: bool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Orders {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub address_id: Uuid,
//     pub order_number: i32,
//     pub status: OrderStatusType,
//     pub payment_status: PaymentStatusType,
//     pub shipping_address_snapshot: Json<String>,
//     pub total_amount: i32,
//     pub shipping_fee: i64,
// }
