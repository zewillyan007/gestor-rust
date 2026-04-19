use std::sync::Arc;

use crate::domain::entity::stock::{
    Stock, StockMovement, CreateStockMovementInput, CreateStockInput, LowStockProduct,
    VALID_MOVEMENT_TYPES,
};
use crate::domain::error::DomainError;
use crate::domain::port::stock_repository::StockRepository;
use crate::domain::port::product_repository::ProductRepository;
use crate::domain::port::UnitOfWorkFactory;

pub struct StockUseCase {
    repo: Arc<dyn StockRepository>,
    product_repo: Arc<dyn ProductRepository>,
    uow_factory: Arc<dyn UnitOfWorkFactory>,
}

impl StockUseCase {
    pub fn new(
        repo: Arc<dyn StockRepository>,
        product_repo: Arc<dyn ProductRepository>,
        uow_factory: Arc<dyn UnitOfWorkFactory>,
    ) -> Self {
        Self { repo, product_repo, uow_factory }
    }

    pub async fn get_by_product(&self, product_id: &str) -> Result<Stock, DomainError> {
        self.repo.find_by_product(product_id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Estoque do produto '{}' não encontrado", product_id))
        })
    }

    pub async fn create(&self, input: CreateStockInput) -> Result<Stock, DomainError> {
        if let Some(qty) = input.quantity {
            if qty < 0 {
                return Err(DomainError::BadRequest("Quantidade não pode ser negativa".to_string()));
            }
        }
        if let Some(min_qty) = input.min_quantity {
            if min_qty < 0 {
                return Err(DomainError::BadRequest("Quantidade mínima não pode ser negativa".to_string()));
            }
        }
        if !self.product_repo.exists(&input.product_id).await? {
            return Err(DomainError::NotFound(format!(
                "Produto com id '{}' não encontrado", input.product_id
            )));
        }
        self.repo.create(&input).await
    }

    pub async fn create_movement(&self, input: CreateStockMovementInput) -> Result<StockMovement, DomainError> {
        if input.quantity <= 0 {
            return Err(DomainError::BadRequest("Quantidade deve ser maior que zero".to_string()));
        }
        if !VALID_MOVEMENT_TYPES.contains(&input.movement_type.as_str()) {
            return Err(DomainError::BadRequest(
                "Tipo de movimentação inválido. Use: in, out".to_string()
            ));
        }

        // Valida que o produto existe
        if !self.product_repo.exists(&input.product_id).await? {
            return Err(DomainError::NotFound(format!(
                "Produto com id '{}' não encontrado", input.product_id
            )));
        }

        // Usa transação para garantir atomicidade entre atualização de estoque e registro de movimentação
        let mut uow = self.uow_factory.begin().await?;

        match input.movement_type.as_str() {
            "in" => {
                if let Err(e) = uow.stocks().atomic_increment(&input.product_id, input.quantity).await {
                    let _ = uow.rollback().await;
                    return Err(e);
                }
            }
            "out" => {
                if let Err(e) = uow.stocks().atomic_decrement(&input.product_id, input.quantity).await {
                    let _ = uow.rollback().await;
                    return Err(e);
                }
            }
            _ => unreachable!("movement_type já validado acima"),
        }

        let movement = match uow.stocks().create_movement(&input).await {
            Ok(m) => m,
            Err(e) => {
                let _ = uow.rollback().await;
                return Err(e);
            }
        };

        uow.commit().await?;

        Ok(movement)
    }

    pub async fn list_movements(&self) -> Result<Vec<StockMovement>, DomainError> {
        self.repo.list_movements().await
    }

    pub async fn list_low_stock(&self) -> Result<Vec<LowStockProduct>, DomainError> {
        self.repo.list_low_stock().await
    }
}
