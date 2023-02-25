use super::ingredients::Ingredient;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub ingredients: Vec<Ingredient>,
    pub name: String,
}
