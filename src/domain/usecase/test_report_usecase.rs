use std::sync::Arc;

use crate::domain::entity::report::{SalesReportItem, StockReportItem, ReturnReportItem, ReportFilter};
use crate::domain::error::DomainError;
use crate::domain::port::mocks::MockReportRepository;
use crate::domain::usecase::report_usecase::ReportUseCase;

fn make_uc() -> (ReportUseCase, Arc<MockReportRepository>) {
    let repo = Arc::new(MockReportRepository::new());
    let uc = ReportUseCase::new(repo.clone());
    (uc, repo)
}

fn sample_sales_item() -> SalesReportItem {
    SalesReportItem {
        product_id: "p1".to_string(),
        product_name: "Colar".to_string(),
        sku: "COL-001".to_string(),
        total_quantity: 10,
        total_revenue: 599.0,
    }
}

fn sample_stock_item() -> StockReportItem {
    StockReportItem {
        product_id: "p1".to_string(),
        product_name: "Colar".to_string(),
        sku: "COL-001".to_string(),
        quantity: 50,
        min_quantity: 10,
        status: "available".to_string(),
    }
}

fn sample_return_item() -> ReturnReportItem {
    ReturnReportItem {
        id: "r1".to_string(),
        product_id: "p1".to_string(),
        product_name: "Colar".to_string(),
        reason: "Defeito".to_string(),
        status: "approved".to_string(),
        refund_amount: Some(59.90),
        created_at: "2026-01-01 00:00:00".to_string(),
    }
}

#[tokio::test]
async fn test_sales_report_ok() {
    let (uc, repo) = make_uc();
    repo.sales_items.lock().unwrap().push(sample_sales_item());
    let filter = ReportFilter { start_date: None, end_date: None };
    let report = uc.sales_report(filter).await.unwrap();
    assert_eq!(report.items.len(), 1);
    assert_eq!(report.total_revenue, 599.0);
    assert_eq!(report.total_items_sold, 10);
}

#[tokio::test]
async fn test_sales_report_with_dates() {
    let (uc, repo) = make_uc();
    repo.sales_items.lock().unwrap().push(sample_sales_item());
    let filter = ReportFilter {
        start_date: Some("2026-01-01".to_string()),
        end_date: Some("2026-12-31".to_string()),
    };
    let report = uc.sales_report(filter).await.unwrap();
    assert_eq!(report.items.len(), 1);
}

#[tokio::test]
async fn test_sales_report_partial_start_date() {
    let (uc, _) = make_uc();
    let filter = ReportFilter {
        start_date: Some("2026-01-01".to_string()),
        end_date: None,
    };
    let err = uc.sales_report(filter).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("ambas as datas")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_sales_report_partial_end_date() {
    let (uc, _) = make_uc();
    let filter = ReportFilter {
        start_date: None,
        end_date: Some("2026-12-31".to_string()),
    };
    let err = uc.sales_report(filter).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("ambas as datas")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_stock_report_ok() {
    let (uc, repo) = make_uc();
    repo.stock_items.lock().unwrap().push(sample_stock_item());
    let report = uc.stock_report().await.unwrap();
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].quantity, 50);
}

#[tokio::test]
async fn test_returns_report_ok() {
    let (uc, repo) = make_uc();
    repo.return_items.lock().unwrap().push(sample_return_item());
    let filter = ReportFilter { start_date: None, end_date: None };
    let report = uc.returns_report(filter).await.unwrap();
    assert_eq!(report.len(), 1);
}

#[tokio::test]
async fn test_returns_report_partial_start_date() {
    let (uc, _) = make_uc();
    let filter = ReportFilter {
        start_date: Some("2026-01-01".to_string()),
        end_date: None,
    };
    let err = uc.returns_report(filter).await.unwrap_err();
    match err {
        DomainError::BadRequest(msg) => assert!(msg.contains("ambas as datas")),
        _ => panic!("Esperava BadRequest"),
    }
}

#[tokio::test]
async fn test_sales_report_empty() {
    let (uc, _) = make_uc();
    let filter = ReportFilter { start_date: None, end_date: None };
    let report = uc.sales_report(filter).await.unwrap();
    assert!(report.items.is_empty());
    assert_eq!(report.total_revenue, 0.0);
    assert_eq!(report.total_items_sold, 0);
}

#[tokio::test]
async fn test_stock_report_empty() {
    let (uc, _) = make_uc();
    let report = uc.stock_report().await.unwrap();
    assert!(report.is_empty());
}
