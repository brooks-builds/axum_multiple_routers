mod routes;

use eyre::Result;
use routes::create_router;
use std::net::SocketAddr;

pub struct App {
    address: [u8; 4],
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = [127, 0, 0, 1];

        tracing_subscriber::fmt::init();

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        let router = create_router();
        let address = SocketAddr::from((self.address, self.port));

        tracing::info!("Server running on port: {}", self.port);

        axum::Server::bind(&address)
            .serve(router.into_make_service())
            .await?;

        Ok(())
    }
}
