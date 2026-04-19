use std::sync::Arc;

use crate::domain::entity::price::{Price, CreatePriceInput, UpdatePriceInput};
use crate::domain::error::DomainError;
use crate::domain::port::price_repository::PriceRepository;
use crate::domain::port::product_repository::ProductRepository;

pub struct PriceUseCase {
    repo: Arc<dyn PriceRepository>,
    product_repo: Arc<dyn ProductRepository>,
}

impl PriceUseCase {
    pub fn new(repo: Arc<dyn PriceRepository>, product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo, product_repo }
    }

    pub async fn list_by_product(&self, product_id: &str) -> Result<Vec<Price>, DomainError> {
        self.repo.list_by_product(product_id).await
    }

    pub async fn create(&self, product_id: &str, input: CreatePriceInput) -> Result<Price, DomainError> {
        if input.cost_price <= 0.0 || input.sale_price <= 0.0 {
            return Err(DomainError::BadRequest("Preços devem ser maiores que zero".to_string()));
        }
        if !self.product_repo.exists(product_id).await? {
            return Err(DomainError::NotFound(format!("Produto com id '{}' não encontrado", product_id)));
        }
        self.repo.create(product_id, &input).await
    }

    pub async fn update(&self, id: &str, input: UpdatePriceInput) -> Result<Price, DomainError> {
        if let Some(cp) = input.cost_price {
            if cp <= 0.0 {
                return Err(DomainError::BadRequest("Preço de custo deve ser maior que zero".to_string()));
            }
        }
        if let Some(sp) = input.sale_price {
            if sp <= 0.0 {
                return Err(DomainError::BadRequest("Preço de venda deve ser maior que zero".to_string()));
            }
        }
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Preço com id '{}' não encontrado", id))
        })?;
        self.repo.update(id, &input).await
    }

    pub async fn delete(&self, id: &str) -> Result<(), DomainError> {
        self.repo.delete(id).await
    }
}
