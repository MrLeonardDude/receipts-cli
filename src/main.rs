pub mod domain;
pub mod infrastructure;
pub mod commands;

use commands::save_receipt;
use domain::Receipt;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    receipt: String,
}

fn main() {
    let input = Cli::parse();
    let receipt: Receipt = serde_json::from_str(&input.receipt).unwrap();
    
    save_receipt(receipt);
}


