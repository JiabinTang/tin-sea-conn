#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use crate::ConnectionError;
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
#[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SslMode {
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    #[cfg(feature = "postgres")]
    fn as_postgres_param(self) -> &'static str {
        match self {
            SslMode::Disable => "disable",
            SslMode::Prefer => "prefer",
            SslMode::Require => "require",
            SslMode::VerifyCa => "verify-ca",
            SslMode::VerifyFull => "verify-full",
        }
    }

    #[cfg(feature = "mysql")]
    fn as_mysql_param(self) -> &'static str {
        match self {
            SslMode::Disable => "DISABLED",
            SslMode::Prefer => "PREFERRED",
            SslMode::Require => "REQUIRED",
            SslMode::VerifyCa => "VERIFY_CA",
            SslMode::VerifyFull => "VERIFY_IDENTITY",
        }
    }
}

#[derive(Debug, Clone)]
pub enum DatabaseType {
    #[cfg(feature = "postgres")]
    PostgreSQL,
    #[cfg(feature = "mysql")]
    MySQL,
    #[cfg(feature = "sqlite")]
    SQLite,
}

#[derive(Debug, Clone)]
pub struct DbConnector {
    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    db_type: Option<DatabaseType>,
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
    database: Option<String>,
    ssl_mode: Option<SslMode>,
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    connect_timeout: Option<u64>,
    idle_timeout: Option<u64>,
    test_before_acquire: Option<bool>,
    sqlx_logging: Option<bool>,
}

impl Default for DbConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl DbConnector {
    pub fn new() -> Self {
        Self {
            #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
            db_type: None,
            host: None,
            port: None,
            username: None,
            password: None,
            database: None,
            ssl_mode: None,
            max_connections: Some(10),
            min_connections: Some(1),
            connect_timeout: Some(30),
            idle_timeout: Some(60),
            test_before_acquire: Some(true),
            sqlx_logging: Self::default_sqlx_logging(),
        }
    }

    #[cfg(feature = "postgres")]
    pub fn postgres(mut self) -> Self {
        self.db_type = Some(DatabaseType::PostgreSQL);
        self.port = Some(5432);
        self
    }

    #[cfg(feature = "mysql")]
    pub fn mysql(mut self) -> Self {
        self.db_type = Some(DatabaseType::MySQL);
        self.port = Some(3306);
        self
    }

    #[cfg(feature = "sqlite")]
    pub fn sqlite(mut self) -> Self {
        self.db_type = Some(DatabaseType::SQLite);
        self
    }

    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn username<S: Into<String>>(mut self, username: S) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn database<S: Into<String>>(mut self, database: S) -> Self {
        self.database = Some(database.into());
        self
    }

    /// Configure SSL/TLS mode for Postgres/MySQL connections.
    ///
    /// - Postgres uses `sslmode` (e.g. `require`, `verify-full`)
    /// - MySQL uses `ssl-mode` (e.g. `REQUIRED`, `VERIFY_IDENTITY`)
    /// - SQLite ignores this setting
    pub fn ssl_mode(mut self, mode: SslMode) -> Self {
        self.ssl_mode = Some(mode);
        self
    }

    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    pub fn min_connections(mut self, min: u32) -> Self {
        self.min_connections = Some(min);
        self
    }

    pub fn connect_timeout(mut self, timeout: u64) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    pub fn idle_timeout(mut self, timeout: u64) -> Self {
        self.idle_timeout = Some(timeout);
        self
    }

    pub fn test_before_acquire(mut self, test: bool) -> Self {
        self.test_before_acquire = Some(test);
        self
    }

    pub fn sqlx_logging(mut self, logging: bool) -> Self {
        self.sqlx_logging = Some(logging);
        self
    }

    #[cfg(any(feature = "postgres", feature = "mysql"))]
    fn append_query_param(mut url: String, key: &str, value: &str) -> String {
        if url.contains('?') {
            url.push('&');
        } else {
            url.push('?');
        }
        url.push_str(key);
        url.push('=');
        url.push_str(value);
        url
    }

    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite"))]
    fn build_database_url(&self) -> Result<String, &'static str> {
        match &self.db_type {
            #[cfg(feature = "postgres")]
            Some(DatabaseType::PostgreSQL) => {
                let host = self.host.as_ref().ok_or("Host is required")?;
                let port = self.port.ok_or("Port is required")?;
                let username = self.username.as_ref().ok_or("Username is required")?;
                let password = self.password.as_ref().ok_or("Password is required")?;
                let database = self.database.as_ref().ok_or("Database name is required")?;

                let url = format!("postgres://{username}:{password}@{host}:{port}/{database}");
                Ok(match self.ssl_mode {
                    Some(mode) => Self::append_query_param(url, "sslmode", mode.as_postgres_param()),
                    None => url,
                })
            }
            #[cfg(feature = "mysql")]
            Some(DatabaseType::MySQL) => {
                let host = self.host.as_ref().ok_or("Host is required")?;
                let port = self.port.ok_or("Port is required")?;
                let username = self.username.as_ref().ok_or("Username is required")?;
                let password = self.password.as_ref().ok_or("Password is required")?;
                let database = self.database.as_ref().ok_or("Database name is required")?;

                let url = format!("mysql://{username}:{password}@{host}:{port}/{database}");
                Ok(match self.ssl_mode {
                    Some(mode) => Self::append_query_param(url, "ssl-mode", mode.as_mysql_param()),
                    None => url,
                })
            }
            #[cfg(feature = "sqlite")]
            Some(DatabaseType::SQLite) => {
                let database = self
                    .database
                    .as_ref()
                    .ok_or("Database file path is required")?;
                Ok(format!("sqlite://{database}?mode=rwc"))
            }

            _ => Err("Database type is required"),
        }
    }

    fn default_sqlx_logging() -> Option<bool> {
        if log::max_level() >= log::LevelFilter::Debug {
            log::debug!("SQLx logging is enabled based on current log level");
            Some(true)
        } else {
            Some(false)
        }
    }

    #[cfg(any(feature = "postgres", feature = "mysql", feature = "sqlite",))]
    pub async fn connect(self) -> Result<DatabaseConnection, ConnectionError> {
        let database_url = self
            .build_database_url()
            .map_err(|e| ConnectionError::InvalidConfig(e.to_string()))?;

        log::debug!("Database URL: {database_url}");

        let mut opt = ConnectOptions::new(database_url);

        // 设置连接池参数
        if let Some(max_conn) = self.max_connections {
            opt.max_connections(max_conn);
        }

        if let Some(min_conn) = self.min_connections {
            opt.min_connections(min_conn);
        }

        if let Some(timeout) = self.connect_timeout {
            opt.connect_timeout(Duration::from_secs(timeout));
        }

        if let Some(timeout) = self.idle_timeout {
            opt.idle_timeout(Duration::from_secs(timeout));
        }

        if let Some(test) = self.test_before_acquire {
            opt.test_before_acquire(test);
        }

        if let Some(logging) = self.sqlx_logging {
            opt.sqlx_logging(logging);
        }

        let conn = Database::connect(opt)
            .await
            .map_err(|e| ConnectionError::ConnectionFailed(e.to_string()))?;

        Ok(conn)
    }
}
