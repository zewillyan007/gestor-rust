use std::env;

/// Configurações da aplicação carregadas do ambiente.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    /// Carrega configurações a partir de variáveis de ambiente ou arquivo .env.
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:gestor.db".to_string());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "10020".to_string())
            .parse()
            .expect("SERVER_PORT deve ser um número válido");

        Self {
            database_url,
            server_port,
        }
    }
}
