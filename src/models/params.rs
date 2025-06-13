use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct AidParams {
    pub aid: String,
}

#[derive(Deserialize, IntoParams)]
pub struct CidParams {
    pub cid: String,
}