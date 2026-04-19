use axum::routing::{get, patch, post, put};
use axum::Router;
use sqlx::SqlitePool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers;

/// Documentação OpenAPI gerada automaticamente.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Gestor de Loja de Acessórios",
        description = "API de gerenciamento de loja de acessórios femininos e infantis.\n\nFuncionalidades: cadastro de produtos, controle de estoque, categorização, controle de disponibilidade, garantias, devoluções, vendas e relatórios.",
        version = "0.1.0",
    ),
    tags(
        (name = "Produtos", description = "Cadastro e gerenciamento de produtos"),
        (name = "Categorias", description = "Categorização de produtos"),
        (name = "Preços", description = "Histórico e gestão de preços"),
        (name = "Estoque", description = "Controle de estoque e movimentações"),
        (name = "Garantias", description = "Controle de garantias de produtos vendidos"),
        (name = "Devoluções", description = "Gestão de devoluções e reembolsos"),
        (name = "Vendas", description = "Registro de vendas"),
        (name = "Relatórios", description = "Relatórios de vendas, estoque e devoluções"),
    ),
    paths(
        handlers::product::list_products,
        handlers::product::get_product,
        handlers::product::create_product,
        handlers::product::update_product,
        handlers::product::update_product_status,
        handlers::product::delete_product,
        handlers::category::list_categories,
        handlers::category::get_category,
        handlers::category::create_category,
        handlers::category::update_category,
        handlers::category::delete_category,
        handlers::category::link_product_category,
        handlers::category::unlink_product_category,
        handlers::price::list_prices,
        handlers::price::create_price,
        handlers::price::update_price,
        handlers::price::delete_price,
        handlers::stock::get_stock,
        handlers::stock::create_stock,
        handlers::stock::create_stock_movement,
        handlers::stock::list_stock_movements,
        handlers::stock::list_low_stock,
        handlers::warranty::list_warranties,
        handlers::warranty::get_warranty,
        handlers::warranty::create_warranty,
        handlers::warranty::update_warranty_status,
        handlers::return_handler::list_returns,
        handlers::return_handler::get_return,
        handlers::return_handler::create_return,
        handlers::return_handler::update_return_status,
        handlers::sale::create_sale,
        handlers::report::sales_report,
        handlers::report::stock_report,
        handlers::report::returns_report,
    ),
    components(
        schemas(
            crate::models::product::Product,
            crate::models::product::CreateProduct,
            crate::models::product::UpdateProduct,
            crate::models::product::UpdateProductStatus,
            crate::models::category::Category,
            crate::models::category::CreateCategory,
            crate::models::category::UpdateCategory,
            crate::models::price::Price,
            crate::models::price::CreatePrice,
            crate::models::price::UpdatePrice,
            crate::models::stock::Stock,
            crate::models::stock::StockMovement,
            crate::models::stock::CreateStockMovement,
            crate::models::stock::CreateStock,
            crate::models::stock::LowStockProduct,
            crate::models::warranty::Warranty,
            crate::models::warranty::CreateWarranty,
            crate::models::warranty::UpdateWarrantyStatus,
            crate::models::return_model::Return,
            crate::models::return_model::CreateReturn,
            crate::models::return_model::UpdateReturnStatus,
            crate::models::sale::Sale,
            crate::models::sale::CreateSale,
            crate::models::report::SalesReportItem,
            crate::models::report::SalesReport,
            crate::models::report::StockReportItem,
            crate::models::report::ReturnReportItem,
            crate::models::report::ReportFilter,
        )
    )
)]
struct ApiDoc;

/// Constroi todas as rotas da aplicação com o pool de banco de dados.
pub fn create_router(pool: SqlitePool) -> Router {
    let api_routes = Router::new()
        // Produtos
        .route("/products", get(handlers::product::list_products).post(handlers::product::create_product))
        .route("/products/{id}", get(handlers::product::get_product).put(handlers::product::update_product).delete(handlers::product::delete_product))
        .route("/products/{id}/status", patch(handlers::product::update_product_status))
        // Categorias
        .route("/categories", get(handlers::category::list_categories).post(handlers::category::create_category))
        .route("/categories/{id}", get(handlers::category::get_category).put(handlers::category::update_category).delete(handlers::category::delete_category))
        .route("/products/{product_id}/categories/{category_id}", post(handlers::category::link_product_category).delete(handlers::category::unlink_product_category))
        // Preços
        .route("/products/{id}/prices", get(handlers::price::list_prices).post(handlers::price::create_price))
        .route("/prices/{id}", put(handlers::price::update_price).delete(handlers::price::delete_price))
        // Estoque
        .route("/stocks", post(handlers::stock::create_stock))
        .route("/products/{id}/stock", get(handlers::stock::get_stock))
        .route("/stock/movements", get(handlers::stock::list_stock_movements).post(handlers::stock::create_stock_movement))
        .route("/stock/low", get(handlers::stock::list_low_stock))
        // Garantias
        .route("/warranties", get(handlers::warranty::list_warranties).post(handlers::warranty::create_warranty))
        .route("/warranties/{id}", get(handlers::warranty::get_warranty))
        .route("/warranties/{id}/status", patch(handlers::warranty::update_warranty_status))
        // Devoluções
        .route("/returns", get(handlers::return_handler::list_returns).post(handlers::return_handler::create_return))
        .route("/returns/{id}", get(handlers::return_handler::get_return))
        .route("/returns/{id}/status", patch(handlers::return_handler::update_return_status))
        // Vendas
        .route("/sales", post(handlers::sale::create_sale))
        // Relatórios
        .route("/reports/sales", get(handlers::report::sales_report))
        .route("/reports/stock", get(handlers::report::stock_report))
        .route("/reports/returns", get(handlers::report::returns_report))
        .with_state(pool);

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
