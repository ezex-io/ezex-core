use crate::database::{
    postgres::postgres::PostgresDB,
    provider::{
        DatabaseReader,
        DatabaseWriter,
    },
};
use common::{
    consts::*,
    testsuite::postgres::PostgresTestDB,
};

#[test]
fn test_save_address() {
    let pq_db = PostgresTestDB::new();

    let pq = PostgresDB::new(&pq_db.con_string(), 1).unwrap();

    let user_id_1 = "alice".to_string();
    let user_id_2 = "bob".to_string();

    pq.assign_address(&user_id_1, "PAC", "wallet_1", "pac_addr_1")
        .unwrap();
    pq.assign_address(&user_id_2, "PAC", "wallet_2", "pac_addr_2")
        .unwrap();
    pq.assign_address(&user_id_2, "BTC", "wallet_2", "btc_addr_1")
        .unwrap();

    let result_1 = pq.get_address(&user_id_1, "PAC").unwrap();
    assert_eq!(result_1.unwrap().address, "pac_addr_1");
    let result_2 = pq.get_address(&user_id_1, "PAC").unwrap();
    assert_eq!(result_2, None);
    let result_3 = pq.get_address(&user_id_2, "BTC").unwrap();
    assert_eq!(result_3.unwrap().address, "btc_addr_1");
}
