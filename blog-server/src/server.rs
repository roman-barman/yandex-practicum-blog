use crate::configuration::Configuration;
use actix_web::{App, HttpServer};

pub(crate) struct Server {
    server: actix_web::dev::Server,
}

impl Server {
    pub(crate) async fn start(config: Configuration) -> anyhow::Result<Self> {
        let server = HttpServer::new(App::new)
            .bind(config.get_server_configuration().get_address())?
            .run();
        Ok(Self { server })
    }

    pub(crate) async fn run_until_shutdown(self) -> std::io::Result<()> {
        self.server.await
    }
}
