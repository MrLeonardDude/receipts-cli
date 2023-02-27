use super::ingredients::Ingredient;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Receipt {
    pub ingredients: Vec<Ingredient>,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt() {
        let receipt = Receipt {
            ingredients: Vec::new(),
            name: String::from("Receita")
        };

        assert_eq!(
            receipt.name,
            "Receita"
        );
    }
}
