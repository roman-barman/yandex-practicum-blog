use crate::api::http_handlers::{auth, posts};
use crate::api::middleware;
use crate::application::contracts::{PostRepository, UserRepository};
use crate::configuration::Configuration;
use crate::infrastructure::{JwtService, PostgresPostRepository, PostgresUserRepository};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

pub(crate) struct Server {
    server: actix_web::dev::Server,
}

impl Server {
    pub(crate) async fn start(config: Configuration) -> anyhow::Result<Self> {
        let pg_pool = Arc::new(
            PgPoolOptions::new()
                .connect_lazy_with(config.get_database_configuration().get_connection_options()),
        );
        let user_repository: web::Data<Arc<dyn UserRepository>> =
            web::Data::new(Arc::new(PostgresUserRepository::new(Arc::clone(&pg_pool))));
        let post_repository: web::Data<Arc<dyn PostRepository>> =
            web::Data::new(Arc::new(PostgresPostRepository::new(Arc::clone(&pg_pool))));
        let jwt_service = web::Data::new(JwtService::new(
            config.get_jwt_configuration().get_secret().clone(),
        ));

        let server = HttpServer::new(move || {
            App::new()
                .wrap(TracingLogger::default())
                .service(
                    web::scope("/api")
                        .service(
                            web::scope("/auth")
                                .service(auth::register_user)
                                .service(auth::login),
                        )
                        .service(
                            web::scope("/posts")
                                .wrap(from_fn(middleware::auth::auth_middleware))
                                .service(posts::create_post)
                                .service(posts::update_post)
                                .service(posts::delete_post),
                        ),
                )
                .app_data(user_repository.clone())
                .app_data(post_repository.clone())
                .app_data(jwt_service.clone())
        })
        .bind(config.get_server_configuration().get_address())?
        .run();
        Ok(Self { server })
    }

    pub(crate) async fn run_until_shutdown(self) -> std::io::Result<()> {
        self.server.await
    }
}
