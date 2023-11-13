mod miscellaneous;
mod quotes;

use axum::Router;

use self::{miscellaneous::create_miscellaneous_router, quotes::create_quote_router};

pub fn create_router() -> Router {
    Router::new()
        .nest("/quotes", create_quote_router())
        .merge(create_miscellaneous_router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
