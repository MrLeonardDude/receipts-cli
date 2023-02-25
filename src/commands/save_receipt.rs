use crate::domain::Receipt;
use crate::infrastructure::ReceiptRepository;
use crate::infrastructure::receipts_repositories::Repository;
use crate::infrastructure::receipts_repositories::load_receipts;


pub fn save_receipt(
    receipt: Receipt
) -> bool {

    let repository = ReceiptRepository::default();
    
    let current_repository = load_receipts(
        repository
    );
    
    current_repository.save_receipt(
        receipt
    );

    return true;
}
