# tin-sea-conn

A simple and flexible database connection library for Rust based on SeaORM, supporting PostgreSQL, MySQL, and SQLite with configurable connection pooling.

## Features

- 🚀 **Easy to use**: Simple builder pattern for configuring database connections
- 🔧 **Flexible**: Support for PostgreSQL, MySQL, and SQLite databases
- ⚡ **Connection pooling**: Built-in connection pool configuration
- 🛠️ **Feature-based**: Enable only the database drivers you need
- 📝 **Type-safe**: Full Rust type safety with comprehensive error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tin-sea-conn = { version = "0.1.0", features = ["postgres"] }
```

## Quick Start

### PostgreSQL

```rust
use tin_sea_conn::DbConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = DbConnector::new()
        .postgres()
        .host("localhost")
        .port(5432)
        .username("user")
        .password("password")
        .database("mydb")
        .max_connections(20)
        .min_connections(5);

    let db = connector.connect().await?;
    
    // Use the database connection...
    
    Ok(())
}
```

### MySQL

```rust
use tin_sea_conn::DbConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = DbConnector::new()
        .mysql()
        .host("localhost")
        .port(3306)
        .username("user")
        .password("password")
        .database("mydb");

    let db = connector.connect().await?;
    
    // Use the database connection...
    
    Ok(())
}
```

### SQLite

```rust
use tin_sea_conn::DbConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connector = DbConnector::new()
        .sqlite()
        .database("./mydb.sqlite");

    let db = connector.connect().await?;
    
    // Use the database connection...
    
    Ok(())
}
```

## Configuration Options

The `DbConnector` supports the following configuration options:

| Method | Description | Default |
|--------|-------------|---------|
| `host(host)` | Database host | Required for PostgreSQL/MySQL |
| `port(port)` | Database port | 5432 (PostgreSQL), 3306 (MySQL) |
| `username(user)` | Database username | Required for PostgreSQL/MySQL |
| `password(pass)` | Database password | Required for PostgreSQL/MySQL |
| `database(db)` | Database name or file path | Required |
| `max_connections(max)` | Maximum connections in pool | 10 |
| `min_connections(min)` | Minimum connections in pool | 1 |
| `connect_timeout(seconds)` | Connection timeout in seconds | 30 |
| `idle_timeout(seconds)` | Idle connection timeout in seconds | 60 |
| `test_before_acquire(bool)` | Test connections before use | true |
| `sqlx_logging(bool)` | Enable SQLx logging | Auto-detected from log level |

## Features

This crate uses Cargo features to enable database drivers:

- `postgres` - Enable PostgreSQL support
- `mysql` - Enable MySQL support  
- `sqlite` - Enable SQLite support

You can enable multiple features to support multiple database types in the same application.

## Building

To build with a specific database feature:

```bash
# PostgreSQL
cargo build --features postgres

# MySQL
cargo build --features mysql

# SQLite  
cargo build --features sqlite

# Multiple databases
cargo build --features "postgres,mysql,sqlite"
```

## Error Handling

The library provides a comprehensive `ConnectionError` enum for error handling:

```rust
use tin_sea_conn::{DbConnector, ConnectionError};

match connector.connect().await {
    Ok(db) => {
        // Use database connection
    }
    Err(ConnectionError::InvalidConfig(msg)) => {
        eprintln!("Configuration error: {}", msg);
    }
    Err(ConnectionError::ConnectionFailed(msg)) => {
        eprintln!("Failed to connect: {}", msg);
    }
    Err(ConnectionError::DatabaseError(msg)) => {
        eprintln!("Database error: {}", msg);
    }
}
```

## Dependencies

This library is built on top of:

- [SeaORM](https://github.com/SeaQL/sea-orm) - The main ORM and database abstraction
- [SQLx](https://github.com/launchbadge/sqlx) - The underlying database drivers
- [Tokio](https://github.com/tokio-rs/tokio) - Async runtime

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### 0.1.0

- Initial release
- Support for PostgreSQL, MySQL, and SQLite
- Configurable connection pooling
- Builder pattern API
- Comprehensive error handling
