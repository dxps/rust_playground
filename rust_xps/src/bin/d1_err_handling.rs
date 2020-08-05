use serde_derive::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

// This method uses the standard error handling mechanism.
pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(fname) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string()),
    };
    let txns: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(txns) => txns,
        Err(e) => return Err(e.to_string()),
    };
    Ok(txns)
}

// This method uses the `.map_err` and `.and_then` methods of `Result`
// helping with a more concise result.
fn get_transactions_b(fname: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(fname)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}

fn main() {
    let txns = get_transactions_b("data/transactions.json").expect("Could not load transactions file");
    for txn in txns {
        println!("{:?}", txn);
    }
}
