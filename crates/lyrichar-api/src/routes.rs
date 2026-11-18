use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::{docs::Docs, info, v1};

pub fn router() -> Router {
    let (router, api) = OpenApiRouter::with_openapi(Docs::openapi())
        .routes(routes!(info::info))
        .nest("/v1", v1::routes::router())
        .split_for_parts();

    let swagger_ui = SwaggerUi::new("/docs").url("/docs/openapi.json", api);

    router.merge(swagger_ui)
}
