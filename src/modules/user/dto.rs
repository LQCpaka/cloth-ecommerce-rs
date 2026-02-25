use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::user::model::UserRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub role: UserRole,
}
