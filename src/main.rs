use axum_multiple_routers::App;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT")
        .parse()
        .expect("PORT environment variable must be a valid port number");
    let app = App::new(port);

    app.run().await.expect("Server exited with error");
}
