pub mod file_repository;

pub use file_repository::*;
use crate::domain::Receipt;
use std::fs::File;
use std::io::prelude::*;
use std::fmt::Debug;


pub trait Repository {
    fn save_receipt(self, receipt: Receipt) -> Self;
}

pub fn load_receipts(
    mut repository: (impl Repository + Debug)
) -> (impl Repository +Debug) {
    let mut file = File::open("receipt.txt").unwrap();    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let receipts: Vec<Receipt> = serde_json::from_str(&content).unwrap();

    for receipt in receipts {
        repository = repository.save_receipt(receipt);
    }

    repository
}
