use std::sync::Arc;

use crate::domain::entity::return_model::{Return, CreateReturnInput, UpdateReturnStatusInput, VALID_RETURN_STATUSES};
use crate::domain::error::DomainError;
use crate::domain::port::return_repository::ReturnRepository;
use crate::domain::port::product_repository::ProductRepository;

pub struct ReturnUseCase {
    repo: Arc<dyn ReturnRepository>,
    product_repo: Arc<dyn ProductRepository>,
}

impl ReturnUseCase {
    pub fn new(repo: Arc<dyn ReturnRepository>, product_repo: Arc<dyn ProductRepository>) -> Self {
        Self { repo, product_repo }
    }

    pub async fn list(&self) -> Result<Vec<Return>, DomainError> {
        self.repo.list().await
    }

    pub async fn get(&self, id: &str) -> Result<Return, DomainError> {
        self.repo.find_by_id(id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Devolução com id '{}' não encontrada", id))
        })
    }

    pub async fn create(&self, input: CreateReturnInput) -> Result<Return, DomainError> {
        if !self.product_repo.exists(&input.product_id).await? {
            return Err(DomainError::NotFound(format!(
                "Produto com id '{}' não encontrado", input.product_id
            )));
        }
        self.repo.create(&input).await
    }

    pub async fn update_status(&self, id: &str, input: UpdateReturnStatusInput) -> Result<Return, DomainError> {
        if !VALID_RETURN_STATUSES.contains(&input.status.as_str()) {
            return Err(DomainError::BadRequest(format!(
                "Status inválido. Use: {}",
                VALID_RETURN_STATUSES.join(", ")
            )));
        }
        self.repo.update_status(id, &input).await
    }
}
