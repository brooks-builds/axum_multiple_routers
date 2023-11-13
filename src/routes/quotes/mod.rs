pub mod create;
pub mod delete;
pub mod get_all;
pub mod get_one;
pub mod update;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use create::create_quote;
use delete::delete_quote;
use get_all::get_all_quotes;
use get_one::get_one_quote;
use update::update_quote;

pub fn create_quote_router() -> Router {
    Router::new()
        .route("/", post(create_quote))
        .route("/", get(get_all_quotes))
        .nest(
            "/:id",
            Router::new()
                .route("/", get(get_one_quote))
                .route("/", put(update_quote))
                .route("/", delete(delete_quote)),
        )
}
