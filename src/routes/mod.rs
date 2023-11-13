mod miscellaneous;
mod quotes;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use miscellaneous::{miscellaneous_one, miscellaneous_three, miscellaneous_two};
use quotes::{create_quote, delete_quote, get_all_quotes, get_one_quote, update_quote};

pub fn create_router() -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes", get(get_all_quotes))
        .route("/quotes/:id", get(get_one_quote))
        .route("/quotes/:id", put(update_quote))
        .route("/quotes/:id", delete(delete_quote))
        .route("/miscellaneous/one", get(miscellaneous_one))
        .route("/miscellaneous/two", get(miscellaneous_two))
        .route("/miscellaneous/three", get(miscellaneous_three))
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
