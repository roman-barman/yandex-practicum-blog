use crate::api::grpc_handlers::blog::blog_service_server::BlogServiceServer;
use crate::api::grpc_handlers::blog_service::GrpcBlogService;
use crate::api::http::http_handlers::{auth, posts};
use crate::api::http::middleware;
use crate::application::contracts::{PostRepository, UserRepository};
use crate::configuration::Configuration;
use crate::infrastructure::{JwtService, PostgresPostRepository, PostgresUserRepository};
use actix_web::middleware::from_fn;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tower_http::trace::TraceLayer;
use tracing_actix_web::TracingLogger;

pub(crate) struct Server {
    http_server: JoinHandle<std::io::Result<()>>,
    grpc_server: JoinHandle<Result<(), tonic::transport::Error>>,
}

impl Server {
    pub(crate) async fn start(config: Configuration) -> anyhow::Result<Self> {
        let pg_pool = Arc::new(
            PgPoolOptions::new()
                .connect_lazy_with(config.get_database_configuration().get_connection_options()),
        );
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(PostgresUserRepository::new(Arc::clone(&pg_pool)));
        let post_repository: Arc<dyn PostRepository> =
            Arc::new(PostgresPostRepository::new(Arc::clone(&pg_pool)));
        let jwt_service = Arc::new(JwtService::new(
            config.get_jwt_configuration().get_secret().clone(),
        ));

        let blog_service = GrpcBlogService::new(
            Arc::clone(&user_repository),
            Arc::clone(&post_repository),
            Arc::clone(&jwt_service),
        );

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
            .build_v1()?;
        let grpc_address = config.get_server_configuration().get_grpc_address()?;
        let grpc_server = tokio::spawn(async move {
            tonic::transport::Server::builder()
                .layer(TraceLayer::new_for_grpc())
                .add_service(BlogServiceServer::new(blog_service))
                .add_service(reflection_service)
                .serve(grpc_address)
                .await
        });

        let http_server =
            run_http_server(&config, &user_repository, &post_repository, &jwt_service)?;
        Ok(Self {
            http_server,
            grpc_server,
        })
    }

    pub(crate) async fn run_until_shutdown(self) -> anyhow::Result<()> {
        tokio::select! {
            o = self.grpc_server => match o {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
            o = self.http_server => match o {
                Ok(_) => Ok(()),
                Err(e) => Err(e.into()),
            },
        }
    }
}

fn run_http_server(
    config: &Configuration,
    user_repository: &Arc<dyn UserRepository>,
    post_repository: &Arc<dyn PostRepository>,
    jwt_service: &Arc<JwtService>,
) -> anyhow::Result<JoinHandle<std::io::Result<()>>> {
    let user_repository_data: web::Data<Arc<dyn UserRepository>> =
        web::Data::new(Arc::clone(&user_repository));
    let post_repository_data: web::Data<Arc<dyn PostRepository>> =
        web::Data::new(Arc::clone(&post_repository));
    let jwt_service_data = web::Data::new(Arc::clone(&jwt_service));

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
                    .service(posts::get_post)
                    .service(posts::get_post_list)
                    .service(
                        web::scope("/posts")
                            .wrap(from_fn(middleware::auth::auth_middleware))
                            .service(posts::create_post)
                            .service(posts::update_post)
                            .service(posts::delete_post),
                    ),
            )
            .app_data(user_repository_data.clone())
            .app_data(post_repository_data.clone())
            .app_data(jwt_service_data.clone())
    })
    .bind(config.get_server_configuration().get_http_address())?
    .run();

    Ok(tokio::spawn(server))
}

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("blog_descriptor");
}
