use secrecy::{ExposeSecret, SecretString};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::net::{SocketAddr, SocketAddrV4};

#[derive(serde::Deserialize, Clone)]
pub(crate) struct Configuration {
    server: ServerConfiguration,
    database: DatabaseConfiguration,
    jwt: JwtConfiguration,
}

impl Configuration {
    pub(crate) fn read_configuration() -> anyhow::Result<Self> {
        let env = std::env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let base_path = std::env::current_dir()?;
        let config_dir = base_path.join("config");
        let env_file = format!("{}.yaml", env);
        let configuration = config::Config::builder()
            .add_source(config::File::from(config_dir.join("base.yaml")))
            .add_source(config::File::from(config_dir.join(env_file)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        let result = configuration.try_deserialize::<Configuration>()?;
        Ok(result)
    }

    pub(crate) fn get_server_configuration(&self) -> &ServerConfiguration {
        &self.server
    }
    pub(crate) fn get_database_configuration(&self) -> &DatabaseConfiguration {
        &self.database
    }

    pub(crate) fn get_jwt_configuration(&self) -> &JwtConfiguration {
        &self.jwt
    }
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct ServerConfiguration {
    host: String,
    http_port: u16,
    grpc_port: u16,
    log_level: String,
    white_list: Vec<String>,
}

impl ServerConfiguration {
    pub(crate) fn get_http_address(&self) -> (&str, u16) {
        (self.host.as_str(), self.http_port)
    }

    pub(crate) fn get_grpc_address(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        Ok(SocketAddr::V4(SocketAddrV4::new(
            self.host.parse()?,
            self.grpc_port,
        )))
    }

    pub(crate) fn get_log_level(&self) -> &str {
        self.log_level.as_str()
    }

    pub(crate) fn get_white_list(&self) -> &[String] {
        &self.white_list
    }
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct DatabaseConfiguration {
    username: String,
    password: SecretString,
    port: u16,
    host: String,
    database_name: String,
    require_ssl: bool,
}

impl DatabaseConfiguration {
    pub(crate) fn get_connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
    }
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct JwtConfiguration {
    secret: SecretString,
}

impl JwtConfiguration {
    pub(crate) fn get_secret(&self) -> &SecretString {
        &self.secret
    }
}
