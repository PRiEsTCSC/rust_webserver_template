{% if database == "postgres" %}
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Postgres>,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = load_database_url();
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}
{% elsif database == "mysql" %}
use sqlx::{mysql::MySqlPoolOptions, Pool, MySql};

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<MySql>,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = load_database_url();
        let pool = MySqlPoolOptions::new()
            .max_connections(100)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}
{% elsif database == "sqlite" %}
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: Pool<Sqlite>,
}

impl DatabaseConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = load_database_url();
        let pool = SqlitePoolOptions::new()
            .max_connections(100)
            .connect(&database_url)
            .await?;
        Ok(Self { pool })
    }
}
{% endif %}

use crate::configs::env_load::load_database_url;