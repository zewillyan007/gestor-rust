pub mod product;
pub mod category;
pub mod price;
pub mod stock;
pub mod warranty;
pub mod return_model;
pub mod sale;
pub mod report;

use uuid::Uuid;

/// Gera um novo ID único (UUID v4).
pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}
