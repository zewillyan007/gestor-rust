use sqlx::Row;
use sqlx::sqlite::SqliteRow;
use crate::domain::entity::product::Product;
use crate::domain::entity::category::Category;
use crate::domain::entity::price::Price;
use crate::domain::entity::stock::{Stock, StockMovement, LowStockProduct};
use crate::domain::entity::warranty::Warranty;
use crate::domain::entity::return_model::Return;
use crate::domain::entity::sale::Sale;
use crate::domain::entity::report::{SalesReportItem, StockReportItem, ReturnReportItem};

pub fn map_product(row: &SqliteRow) -> Product {
    Product {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        sku: row.get("sku"),
        brand: row.get("brand"),
        status: row.get("status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn map_category(row: &SqliteRow) -> Category {
    Category {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        parent_id: row.get("parent_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn map_price(row: &SqliteRow) -> Price {
    Price {
        id: row.get("id"),
        product_id: row.get("product_id"),
        cost_price: row.get("cost_price"),
        sale_price: row.get("sale_price"),
        effective_date: row.get("effective_date"),
        created_at: row.get("created_at"),
    }
}

pub fn map_stock(row: &SqliteRow) -> Stock {
    Stock {
        id: row.get("id"),
        product_id: row.get("product_id"),
        quantity: row.get("quantity"),
        min_quantity: row.get("min_quantity"),
        location: row.get("location"),
        updated_at: row.get("updated_at"),
    }
}

pub fn map_stock_movement(row: &SqliteRow) -> StockMovement {
    StockMovement {
        id: row.get("id"),
        product_id: row.get("product_id"),
        movement_type: row.get("movement_type"),
        quantity: row.get("quantity"),
        reason: row.get("reason"),
        reference: row.get("reference"),
        created_at: row.get("created_at"),
    }
}

pub fn map_warranty(row: &SqliteRow) -> Warranty {
    Warranty {
        id: row.get("id"),
        product_id: row.get("product_id"),
        customer_name: row.get("customer_name"),
        customer_contact: row.get("customer_contact"),
        purchase_date: row.get("purchase_date"),
        warranty_days: row.get("warranty_days"),
        expires_at: row.get("expires_at"),
        status: row.get("status"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn map_return(row: &SqliteRow) -> Return {
    Return {
        id: row.get("id"),
        product_id: row.get("product_id"),
        warranty_id: row.get("warranty_id"),
        reason: row.get("reason"),
        status: row.get("status"),
        refund_amount: row.get("refund_amount"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

pub fn map_sale(row: &SqliteRow) -> Sale {
    Sale {
        id: row.get("id"),
        product_id: row.get("product_id"),
        quantity: row.get("quantity"),
        unit_price: row.get("unit_price"),
        total_price: row.get("total_price"),
        sale_date: row.get("sale_date"),
        customer_name: row.get("customer_name"),
        created_at: row.get("created_at"),
    }
}

pub fn map_sales_report_item(row: &SqliteRow) -> SalesReportItem {
    SalesReportItem {
        product_id: row.get("product_id"),
        product_name: row.get("product_name"),
        sku: row.get("sku"),
        total_quantity: row.get("total_quantity"),
        total_revenue: row.get("total_revenue"),
    }
}

pub fn map_stock_report_item(row: &SqliteRow) -> StockReportItem {
    StockReportItem {
        product_id: row.get("product_id"),
        product_name: row.get("product_name"),
        sku: row.get("sku"),
        quantity: row.get("quantity"),
        min_quantity: row.get("min_quantity"),
        status: row.get("status"),
    }
}

pub fn map_return_report_item(row: &SqliteRow) -> ReturnReportItem {
    ReturnReportItem {
        id: row.get("id"),
        product_id: row.get("product_id"),
        product_name: row.get("product_name"),
        reason: row.get("reason"),
        status: row.get("status"),
        refund_amount: row.get("refund_amount"),
        created_at: row.get("created_at"),
    }
}

pub fn map_low_stock_product(row: &SqliteRow) -> LowStockProduct {
    LowStockProduct {
        product_id: row.get("product_id"),
        product_name: row.get("product_name"),
        sku: row.get("sku"),
        quantity: row.get("quantity"),
        min_quantity: row.get("min_quantity"),
    }
}
