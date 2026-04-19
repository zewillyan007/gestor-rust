use std::sync::Arc;

use axum::routing::{get, patch, post, put};
use axum::Router;
use sqlx::SqlitePool;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::adapter::inbound::http;
use crate::adapter::outbound::sqlite::{
    product_repo::SqliteProductRepository,
    category_repo::SqliteCategoryRepository,
    price_repo::SqlitePriceRepository,
    stock_repo::SqliteStockRepository,
    warranty_repo::SqliteWarrantyRepository,
    return_repo::SqliteReturnRepository,
    sale_repo::SqliteSaleRepository,
    report_repo::SqliteReportRepository,
    unit_of_work_impl::SqliteUnitOfWorkFactory,
};
use crate::domain::usecase::product_usecase::ProductUseCase;
use crate::domain::usecase::category_usecase::CategoryUseCase;
use crate::domain::usecase::price_usecase::PriceUseCase;
use crate::domain::usecase::stock_usecase::StockUseCase;
use crate::domain::usecase::warranty_usecase::WarrantyUseCase;
use crate::domain::usecase::return_usecase::ReturnUseCase;
use crate::domain::usecase::sale_usecase::SaleUseCase;
use crate::domain::usecase::report_usecase::ReportUseCase;

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
        http::product_handler::list_products,
        http::product_handler::get_product,
        http::product_handler::create_product,
        http::product_handler::update_product,
        http::product_handler::update_product_status,
        http::product_handler::delete_product,
        http::category_handler::list_categories,
        http::category_handler::get_category,
        http::category_handler::create_category,
        http::category_handler::update_category,
        http::category_handler::delete_category,
        http::category_handler::link_product_category,
        http::category_handler::unlink_product_category,
        http::price_handler::list_prices,
        http::price_handler::create_price,
        http::price_handler::update_price,
        http::price_handler::delete_price,
        http::stock_handler::get_stock,
        http::stock_handler::create_stock,
        http::stock_handler::create_stock_movement,
        http::stock_handler::list_stock_movements,
        http::stock_handler::list_low_stock,
        http::warranty_handler::list_warranties,
        http::warranty_handler::get_warranty,
        http::warranty_handler::create_warranty,
        http::warranty_handler::update_warranty_status,
        http::return_handler::list_returns,
        http::return_handler::get_return,
        http::return_handler::create_return,
        http::return_handler::update_return_status,
        http::sale_handler::create_sale,
        http::report_handler::sales_report,
        http::report_handler::stock_report,
        http::report_handler::returns_report,
    ),
    components(
        schemas(
            crate::domain::entity::product::Product,
            crate::adapter::inbound::http::dto::CreateProductDto,
            crate::adapter::inbound::http::dto::UpdateProductDto,
            crate::adapter::inbound::http::dto::UpdateProductStatusDto,
            crate::domain::entity::category::Category,
            crate::adapter::inbound::http::dto::CreateCategoryDto,
            crate::adapter::inbound::http::dto::UpdateCategoryDto,
            crate::domain::entity::price::Price,
            crate::adapter::inbound::http::dto::CreatePriceDto,
            crate::adapter::inbound::http::dto::UpdatePriceDto,
            crate::domain::entity::stock::Stock,
            crate::domain::entity::stock::StockMovement,
            crate::adapter::inbound::http::dto::CreateStockMovementDto,
            crate::adapter::inbound::http::dto::CreateStockDto,
            crate::domain::entity::stock::LowStockProduct,
            crate::domain::entity::warranty::Warranty,
            crate::adapter::inbound::http::dto::CreateWarrantyDto,
            crate::adapter::inbound::http::dto::UpdateWarrantyStatusDto,
            crate::domain::entity::return_model::Return,
            crate::adapter::inbound::http::dto::CreateReturnDto,
            crate::adapter::inbound::http::dto::UpdateReturnStatusDto,
            crate::domain::entity::sale::Sale,
            crate::adapter::inbound::http::dto::CreateSaleDto,
            crate::domain::entity::report::SalesReportItem,
            crate::domain::entity::report::SalesReport,
            crate::domain::entity::report::StockReportItem,
            crate::domain::entity::report::ReturnReportItem,
            crate::adapter::inbound::http::dto::ReportFilterDto,
        )
    )
)]
struct ApiDoc;

/// Estado compartilhado da aplicação — injetado nos handlers via Axum State.
#[derive(Clone)]
pub struct AppState {
    pub product_uc: Arc<ProductUseCase>,
    pub category_uc: Arc<CategoryUseCase>,
    pub price_uc: Arc<PriceUseCase>,
    pub stock_uc: Arc<StockUseCase>,
    pub warranty_uc: Arc<WarrantyUseCase>,
    pub return_uc: Arc<ReturnUseCase>,
    pub sale_uc: Arc<SaleUseCase>,
    pub report_uc: Arc<ReportUseCase>,
}

/// Constroi todas as rotas da aplicação com injeção de dependências.
pub fn create_router(pool: SqlitePool) -> Router {
    let pool = Arc::new(pool);

    // Repositórios SQLite (outbound adapters)
    let product_repo = Arc::new(SqliteProductRepository::new(pool.clone()));
    let category_repo = Arc::new(SqliteCategoryRepository::new(pool.clone()));
    let price_repo = Arc::new(SqlitePriceRepository::new(pool.clone()));
    let stock_repo = Arc::new(SqliteStockRepository::new(pool.clone()));
    let warranty_repo = Arc::new(SqliteWarrantyRepository::new(pool.clone()));
    let return_repo = Arc::new(SqliteReturnRepository::new(pool.clone()));
    let _sale_repo = Arc::new(SqliteSaleRepository::new(pool.clone()));
    let report_repo = Arc::new(SqliteReportRepository::new(pool.clone()));
    let uow_factory = Arc::new(SqliteUnitOfWorkFactory::new(pool.clone()));

    // Use cases (domain)
    let product_uc = Arc::new(ProductUseCase::new(product_repo.clone()));
    let category_uc = Arc::new(CategoryUseCase::new(category_repo, product_repo.clone()));
    let price_uc = Arc::new(PriceUseCase::new(price_repo, product_repo.clone()));
    let stock_uc = Arc::new(StockUseCase::new(stock_repo, product_repo.clone(), uow_factory.clone()));
    let warranty_uc = Arc::new(WarrantyUseCase::new(warranty_repo, product_repo.clone()));
    let return_uc = Arc::new(ReturnUseCase::new(return_repo, product_repo.clone()));
    let sale_uc = Arc::new(SaleUseCase::new(uow_factory));
    let report_uc = Arc::new(ReportUseCase::new(report_repo));

    let state = AppState {
        product_uc,
        category_uc,
        price_uc,
        stock_uc,
        warranty_uc,
        return_uc,
        sale_uc,
        report_uc,
    };

    let api_routes = Router::new()
        // Produtos
        .route("/products", get(http::product_handler::list_products).post(http::product_handler::create_product))
        .route("/products/{id}", get(http::product_handler::get_product).put(http::product_handler::update_product).delete(http::product_handler::delete_product))
        .route("/products/{id}/status", patch(http::product_handler::update_product_status))
        // Categorias
        .route("/categories", get(http::category_handler::list_categories).post(http::category_handler::create_category))
        .route("/categories/{id}", get(http::category_handler::get_category).put(http::category_handler::update_category).delete(http::category_handler::delete_category))
        .route("/products/{product_id}/categories/{category_id}", post(http::category_handler::link_product_category).delete(http::category_handler::unlink_product_category))
        // Preços
        .route("/products/{id}/prices", get(http::price_handler::list_prices).post(http::price_handler::create_price))
        .route("/prices/{id}", put(http::price_handler::update_price).delete(http::price_handler::delete_price))
        // Estoque
        .route("/stocks", post(http::stock_handler::create_stock))
        .route("/products/{id}/stock", get(http::stock_handler::get_stock))
        .route("/stock/movements", get(http::stock_handler::list_stock_movements).post(http::stock_handler::create_stock_movement))
        .route("/stock/low", get(http::stock_handler::list_low_stock))
        // Garantias
        .route("/warranties", get(http::warranty_handler::list_warranties).post(http::warranty_handler::create_warranty))
        .route("/warranties/{id}", get(http::warranty_handler::get_warranty))
        .route("/warranties/{id}/status", patch(http::warranty_handler::update_warranty_status))
        // Devoluções
        .route("/returns", get(http::return_handler::list_returns).post(http::return_handler::create_return))
        .route("/returns/{id}", get(http::return_handler::get_return))
        .route("/returns/{id}/status", patch(http::return_handler::update_return_status))
        // Vendas
        .route("/sales", post(http::sale_handler::create_sale))
        // Relatórios
        .route("/reports/sales", get(http::report_handler::sales_report))
        .route("/reports/stock", get(http::report_handler::stock_report))
        .route("/reports/returns", get(http::report_handler::returns_report))
        .with_state(state);

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}
