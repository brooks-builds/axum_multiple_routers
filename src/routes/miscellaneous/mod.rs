mod one;
mod three;
mod two;

use axum::{routing::get, Router};
use one::miscellaneous_one;
use three::miscellaneous_three;
use two::miscellaneous_two;

pub fn create_miscellaneous_router() -> Router {
    Router::new()
        .route("/do/something/fun", get(miscellaneous_one))
        .route("/completely/different/path", get(miscellaneous_two))
        .route("/one/more", get(miscellaneous_three))
}
