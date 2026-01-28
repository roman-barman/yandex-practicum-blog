use crate::api;
use crate::application::contracts::UserRepository;
use crate::configuration::Configuration;
use crate::infrastructure::PostgresUserRepository;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub(crate) struct Server {
    server: actix_web::dev::Server,
}

impl Server {
    pub(crate) async fn start(config: Configuration) -> anyhow::Result<Self> {
        let pg_pool = PgPoolOptions::new()
            .connect_lazy_with(config.get_database_configuration().get_connection_options());
        let user_repository: web::Data<Arc<dyn UserRepository>> =
            web::Data::new(Arc::new(PostgresUserRepository::new(pg_pool)));

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .service(
                    web::scope("/api")
                        .service(api::auth::register_user)
                        .service(api::auth::login),
                )
                .app_data(user_repository.clone())
        })
        .bind(config.get_server_configuration().get_address())?
        .run();
        Ok(Self { server })
    }

    pub(crate) async fn run_until_shutdown(self) -> std::io::Result<()> {
        self.server.await
    }
}
