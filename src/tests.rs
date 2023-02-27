pub mod domain;
use domain::*;

#[test]
fn test_domain() {
    let ingredient = Ingredient {
        name: "test",
        quantity: 1,
    };
    assert_eq!(true, true);
}
