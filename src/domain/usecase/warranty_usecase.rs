use std::sync::Arc;

use crate::domain::entity::warranty::{Warranty, CreateWarrantyInput, VALID_WARRANTY_STATUSES};
use crate::domain::error::DomainError;
use crate::domain::port::warranty_repository::WarrantyRepository;
use crate::domain::port::product_repository::ProductRepository;

pub struct WarrantyUseCase {
    repo: Arc<dyn WarrantyRepository>,
    product_repo: Arc<dyn ProductRepository>,
}

impl WarrantyUseCase {
    pub fn new(repo: Arc<dyn WarrantyRepository>, product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo, product_repo }
    }

    pub async fn list(&self) -> Result<Vec<Warranty>, DomainError> {
        self.repo.list().await
    }

    pub async fn get(&self, id: &str) -> Result<Warranty, DomainError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Garantia com id '{}' não encontrada", id))
        })
    }

    pub async fn create(&self, input: CreateWarrantyInput) -> Result<Warranty, DomainError> {
        if !self.product_repo.exists(&input.product_id).await? {
            return Err(DomainError::NotFound(format!(
                "Produto com id '{}' não encontrado", input.product_id
            )));
        }

        if input.warranty_days <= 0 {
            return Err(DomainError::BadRequest("Dias de garantia deve ser maior que zero".to_string()));
        }

        // Calcula data de expiração
        let purchase_date = chrono::NaiveDate::parse_from_str(&input.purchase_date, "%Y-%m-%d")
            .map_err(|_| DomainError::BadRequest("Formato de data inválido. Use YYYY-MM-DD".to_string()))?;
        let expires_at = purchase_date + chrono::Duration::days(input.warranty_days as i64);
        let expires_at_str = expires_at.format("%Y-%m-%d").to_string();

        self.repo.create(&input, &expires_at_str).await
    }

    pub async fn update_status(&self, id: &str, status: &str) -> Result<Warranty, DomainError> {
        if !VALID_WARRANTY_STATUSES.contains(&status) {
            return Err(DomainError::BadRequest(format!(
                "Status inválido. Use: {}",
                VALID_WARRANTY_STATUSES.join(", ")
            )));
        }
        self.repo.update_status(id, status).await
    }
}
