use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserCreateRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ArticleCreateRequest {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub summary: String,
    pub content: String,
    pub secret: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ArticleModifyRequest {
    pub aid: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub secret: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CommentCreateRequest {
    pub aid: String,
    pub uid: String,
    pub content: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CommentModifyRequest {
    pub cid: String,
    pub content: String,
}