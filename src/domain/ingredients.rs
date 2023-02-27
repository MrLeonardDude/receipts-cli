use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ingredient() {
        let ingredient = Ingredient {
            name: String::from("test"),
            quantity: 1
        };

        assert_eq!(
            ingredient.name,
            "test"
        );
    }
}
