use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: i32,
}
