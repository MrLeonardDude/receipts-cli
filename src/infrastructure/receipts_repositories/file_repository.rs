use super::Repository;
use crate::domain::Receipt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct ReceiptRepository {
    pub receipts: Vec<Receipt>
}

impl Repository for ReceiptRepository {
    fn save_receipt(mut self, receipt: Receipt) -> Self {
        self.receipts.push(receipt);

        let new_repo = serde_json::to_string(
            &self.receipts.clone()
        ).unwrap();

        let mut file = File::create("receipt.txt")
            .unwrap();
        file.write(
            new_repo.as_bytes()
        ).unwrap();

        self
    }
}
