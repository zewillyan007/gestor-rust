use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

/// Cria o pool de conexão com o banco SQLite e executa as migrações.
pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    run_migrations(&pool).await?;

    Ok(pool)
}

/// Executa os scripts SQL de migração na ordem correta.
async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let migrations = [
        include_str!("../migrations/001_create_categories.sql"),
        include_str!("../migrations/002_create_products.sql"),
        include_str!("../migrations/003_create_product_categories.sql"),
        include_str!("../migrations/004_create_prices.sql"),
        include_str!("../migrations/005_create_stocks.sql"),
        include_str!("../migrations/006_create_stock_movements.sql"),
        include_str!("../migrations/007_create_warranties.sql"),
        include_str!("../migrations/008_create_returns.sql"),
        include_str!("../migrations/009_create_sales.sql"),
    ];

    for migration in &migrations {
        // Divide para evitar executar múltiplos statements de uma vez (SQLite limitação)
        for statement in migration.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(pool).await?;
            }
        }
    }

    tracing::info!("Migrações executadas com sucesso");
    Ok(())
}
