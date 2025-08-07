use std::fmt;

// 定义自己的错误类型
#[derive(Debug)]
pub enum ConnectionError {
    InvalidConfig(String),
    ConnectionFailed(String),
    DatabaseError(String),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::InvalidConfig(msg) => write!(f, "Configuration error: {msg}"),
            ConnectionError::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
            ConnectionError::DatabaseError(msg) => write!(f, "Database error: {msg}"),
        }
    }
}

impl std::error::Error for ConnectionError {}
