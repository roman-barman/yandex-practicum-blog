use crate::api;
use crate::configuration::Configuration;
use actix_web::{App, HttpServer, web};
use tracing_actix_web::TracingLogger;

pub(crate) struct Server {
    server: actix_web::dev::Server,
}

impl Server {
    pub(crate) async fn start(config: Configuration) -> anyhow::Result<Self> {
        let server = HttpServer::new(|| {
            App::new()
                .wrap(TracingLogger::default())
                .service(web::scope("/api").service(api::auth::register_user))
        })
        .bind(config.get_server_configuration().get_address())?
        .run();
        Ok(Self { server })
    }

    pub(crate) async fn run_until_shutdown(self) -> std::io::Result<()> {
        self.server.await
    }
}
