use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//==================| ENUM |=====================

// DB type: user_role_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user-role_type", rename_all = "lowercase")]
pub enum UserRole {
    User,
    Seller,
    Moderator,
    Admin,
}

// DB type: user_status_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user-role_type", rename_all = "lowercase")]
pub enum UserStatus {
    Unverified,
    Ative,
    Banned,
}

// DB type: auth_provider_type
#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user-role_type", rename_all = "lowercase")]
pub enum AuthProvider {
    Local,
    Google,
}

//==================| USER STRUCT |=====================

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,

    pub email: String,

    // When return JSON to FE, auto hide this field
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,

    pub name: String,
    pub avatar_url: Option<String>,
    pub description: String,

    pub role: UserRole,
    pub status: UserStatus,

    pub provider: AuthProvider,
    pub provider_id: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
