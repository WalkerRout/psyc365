use axum::Router;
use axum::extract::OriginalUri;
use axum::response::IntoResponse;
use axum::routing::get;

use tower_http::compression::CompressionLayer;

use tracing::warn;

use crate::template::HtmlTemplate;
use crate::template::error::Error404;
use crate::template::presentation;

pub fn router() -> Router {
  Router::new()
    .route("/", get(home))
    .fallback(error_404)
    .layer(CompressionLayer::new().br(true).gzip(true))
}

async fn home() -> impl IntoResponse {
  HtmlTemplate::from(presentation::Presentation)
}

async fn error_404(OriginalUri(uri): OriginalUri) -> impl IntoResponse {
  warn!("unable to find resource: {uri}");
  HtmlTemplate::from(Error404)
}
