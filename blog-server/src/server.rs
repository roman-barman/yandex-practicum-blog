use actix_web::{App, HttpServer};

pub(crate) struct Server {
    server: actix_web::dev::Server,
}

impl Server {
    pub(crate) async fn start() -> anyhow::Result<Self> {
        let server = HttpServer::new(App::new).bind(("127.0.0.1", 8080))?.run();
        Ok(Self { server })
    }

    pub(crate) async fn run_until_shutdown(self) -> std::io::Result<()> {
        self.server.await
    }
}
