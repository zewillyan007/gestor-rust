use axum::Router;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use gestor_rust::routes::create_router;

/// Cria um pool SQLite em memória para testes e executa as migrações.
/// Usa max_connections=1 para garantir que todas as operações compartilhem
/// o mesmo banco em memória.
pub async fn create_test_app() -> Router {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .unwrap();

    let migrations = [
        include_str!("../../migrations/001_create_categories.sql"),
        include_str!("../../migrations/002_create_products.sql"),
        include_str!("../../migrations/003_create_product_categories.sql"),
        include_str!("../../migrations/004_create_prices.sql"),
        include_str!("../../migrations/005_create_stocks.sql"),
        include_str!("../../migrations/006_create_stock_movements.sql"),
        include_str!("../../migrations/007_create_warranties.sql"),
        include_str!("../../migrations/008_create_returns.sql"),
        include_str!("../../migrations/009_create_sales.sql"),
    ];

    for migration in &migrations {
        for statement in migration.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(&pool).await.unwrap();
            }
        }
    }

    create_router(pool)
}
