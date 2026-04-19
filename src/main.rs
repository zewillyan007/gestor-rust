use gestor_rust::infrastructure::config;
use gestor_rust::infrastructure::db;
use gestor_rust::infrastructure::server;

use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Carrega .env e inicializa tracing
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("gestor_rust=debug".parse().unwrap()),
        )
        .init();

    // Carrega configurações
    let cfg = config::Config::from_env();
    tracing::info!("Iniciando Gestor de Loja na porta {}", cfg.server_port);

    // Inicializa o banco de dados
    let pool = db::init_pool(&cfg.database_url)
        .await
        .expect("Falha ao inicializar o banco de dados");

    tracing::info!("Banco de dados conectado com sucesso");

    // Configura CORS
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    // Cria o router com injeção de dependências
    let app = server::create_router(pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Bind do servidor
    let addr = format!("0.0.0.0:{}", cfg.server_port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Falha ao bindar na porta {}", cfg.server_port));

    tracing::info!("Servidor rodando em http://{}", addr);
    tracing::info!("Swagger UI em http://{}/swagger-ui", addr);

    axum::serve(listener, app)
        .await
        .expect("Falha ao iniciar o servidor");
}
