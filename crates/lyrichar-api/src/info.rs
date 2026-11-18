use axum::Json;
use bon::Builder;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

pub const VALID: &str = "base url must be valid";

pub const BASE: &str = "https://api.lyrichar.app/";

pub fn base() -> Url {
    BASE.parse().unwrap()
}

pub const LATEST: &str = "v1";

pub const TAG: &str = "info";

/// Represents API information.
#[derive(Builder, Deserialize, Serialize, ToSchema)]
pub struct Info {
    /// The API base URL.
    #[builder(into)]
    pub base: Url,
    /// The latest API version.
    #[builder(into)]
    pub latest: String,
}

#[utoipa::path(
    get,
    path = "/",
    tag = TAG,
    summary = "Returns API information",
    responses(
        (status = OK, description = "API information", body = Info)
    )
)]
pub async fn info() -> Json<Info> {
    let info = Info::builder().base(base()).latest(LATEST).build();

    Json(info)
}
