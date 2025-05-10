use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}