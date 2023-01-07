use rust_data_structures::lists::txn_log::TransactionLog;

fn main() {
    let stmt1 = "INSERT INTO mytable VALUES (1,2,3)".to_string();
    let stmt2 = "INSERT INTO mytable VALUES (4,5,6)".to_string();
    let stmt3 = "INSERT INTO mytable VALUES (7,8,9)".to_string();

    let mut txn_log = TransactionLog::new();
    txn_log.append(stmt1.clone());
    txn_log.append(stmt2.clone());
    txn_log.append(stmt3.clone());

    assert_eq!(txn_log.length(), 3);

    assert_eq!(txn_log.pop(), Some(stmt1));
    assert_eq!(txn_log.pop(), Some(stmt2));
    assert_eq!(txn_log.pop(), Some(stmt3));
    assert_eq!(txn_log.pop(), None);

    println!("The example ran fine.");
}
