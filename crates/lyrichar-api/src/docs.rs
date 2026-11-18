use utoipa::OpenApi;

use crate::info;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = info::TAG, description = "API information")
    )
)]
pub struct Docs;
