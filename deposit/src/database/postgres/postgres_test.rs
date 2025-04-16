use crate::database::{
    postgres::postgres::PostgresDB,
    provider::{
        DatabaseReader,
        DatabaseWriter,
    },
};
use common::{
    consts::*,
    test_tools::PostgresTestDB,
};

#[test]
fn test_save_address() {
    let pq_db = PostgresTestDB::new();

    let pq = PostgresDB::new(&pq_db.con_string(), 1).unwrap();

    let user_id_1 = "alice".to_string();
    let user_id_2 = "bob".to_string();

    pq.assign_address(&user_id_1, chain::id::BITCOIN, "wallet_1", "btc_addr_1")
        .unwrap();
    pq.assign_address(&user_id_2, chain::id::BITCOIN, "wallet_2", "btc_addr_2")
        .unwrap();
    pq.assign_address(&user_id_2, chain::id::ETHEREUM, "wallet_2", "eth_addr_1")
        .unwrap();

    let result_1 = pq.get_address(&user_id_1, chain::id::BITCOIN).unwrap();
    assert_eq!(result_1.unwrap().deposit_address, "btc_addr_1");
    let result_2 = pq.get_address(&user_id_1, chain::id::ETHEREUM).unwrap();
    assert_eq!(result_2, None);
    let result_3 = pq.get_address(&user_id_2, chain::id::ETHEREUM).unwrap();
    assert_eq!(result_3.unwrap().deposit_address, "eth_addr_1");
}
