use crate::database::{
    postgres::{config::Config, postgres::PostgresDB, schema::address_book::user_id},
    provider::{DatabaseReader, DatabaseWriter},
};

fn make_test_db() -> PostgresDB {
    let pg_db = PostgresTestDB::new();
    PostgresDB::new(&Config {
        database_url: pg_db.con_string(),
        pool_size: 1,
    })
    .unwrap()
}

#[test]
fn test_address_operations() {
    let db = make_test_db();

    let test_cases = [
        (
            AddressScope {
                wallet_id: Some("wallet_1".into()),
                user_id: "alice".into(),
                chain_id: "pactus".into(),
                asset_id: "PAC".into(),
            },
            "pac_addr_1",
        ),
        (
            AddressScope {
                wallet_id: Some("wallet_1".into()),
                user_id: "bob".into(),
                chain_id: "pactus".into(),
                asset_id: "PAC".into(),
            },
            "pac_addr_2",
        ),
        (
            AddressScope {
                wallet_id: Some("wallet_1".into()),
                user_id: "bob".into(),
                chain_id: "bitcoin".into(),
                asset_id: "PAC".into(),
            },
            "btc_addr_1",
        ),
    ];

    // Test address assignment
    for (scope, addr) in &test_cases {
        db.assign_address(scope, addr).unwrap();
    }

    // Verify addresses
    for (scope, expected_addr) in test_cases {
        let retrieved = db.get_address(&scope).unwrap().unwrap();
        assert_eq!(retrieved.address, expected_addr);
    }
}
