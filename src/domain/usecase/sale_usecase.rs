use std::sync::Arc;

use crate::domain::entity::sale::{Sale, CreateSaleInput};
use crate::domain::entity::stock::CreateStockMovementInput;
use crate::domain::error::DomainError;
use crate::domain::port::UnitOfWorkFactory;

pub struct SaleUseCase {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
}

impl SaleUseCase {
    pub fn new(uow_factory: Arc<dyn UnitOfWorkFactory>) -> Self {
        Self { uow_factory }
    }

    pub async fn create(&self, input: CreateSaleInput) -> Result<Sale, DomainError> {
        // Validações de entrada
        if input.quantity <= 0 {
            return Err(DomainError::BadRequest("Quantidade deve ser maior que zero".to_string()));
        }
        if input.unit_price <= 0.0 {
            return Err(DomainError::BadRequest("Preço unitário deve ser maior que zero".to_string()));
        }

        // Inicia transação via UnitOfWork
        let mut uow = self.uow_factory.begin().await?;

        // Verifica se o produto existe e está disponível
        let product = uow.products().find_by_id(&input.product_id).await?;
        match product {
            None => {
                let _ = uow.rollback().await;
                return Err(DomainError::NotFound(format!(
                    "Produto com id '{}' não encontrado", input.product_id
                )));
            }
            Some(p) if p.status != "available" => {
                let _ = uow.rollback().await;
                return Err(DomainError::BadRequest("Produto não está disponível para venda".to_string()));
            }
            _ => {}
        }

        // Decrementa estoque atomicamente (verifica saldo + atualiza em 1 SQL)
        let _new_qty = match uow.stocks().atomic_decrement(&input.product_id, input.quantity).await {
            Ok(q) => q,
            Err(e) => {
                let _ = uow.rollback().await;
                return Err(e);
            }
        };

        let total_price = input.unit_price * input.quantity as f64;

        // Registra a venda
        let sale = match uow.sales().create(&input, total_price).await {
            Ok(s) => s,
            Err(e) => {
                let _ = uow.rollback().await;
                return Err(e);
            }
        };

        // Registra movimentação
        let movement_input = CreateStockMovementInput {
            product_id: input.product_id.clone(),
            movement_type: "out".to_string(),
            quantity: input.quantity,
            reason: Some("Venda".to_string()),
            reference: Some(sale.id.clone()),
        };
        if let Err(e) = uow.stocks().create_movement(&movement_input).await {
            let _ = uow.rollback().await;
            return Err(e);
        }

        // Commit
        uow.commit().await?;

        Ok(sale)
    }
}
