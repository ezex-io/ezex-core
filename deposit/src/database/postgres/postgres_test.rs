use common::testsuite::postgres::PostgresTestDB;

use crate::{
    database::{
        postgres::{config::Config, postgres::PostgresDB},
        provider::{DatabaseReader, DatabaseWriter},
    },
    types::Address,
};

#[test]
fn test_address_operations() {
    let test_db = PostgresTestDB::new();
    let db = PostgresDB::new(&Config {
        database_url: test_db.con_string(),
        pool_size: 1,
    })
    .unwrap();

    let test_cases = [
        Address {
            wallet_id: "wallet_1".into(),
            user_id: "alice".into(),
            chain_id: "pactus".into(),
            asset_id: "PAC".into(),
            address: "pac_addr_1".into(),
            created_at: chrono::Utc::now().naive_local(),
        },
        Address {
            wallet_id: "wallet_1".into(),
            user_id: "bob".into(),
            chain_id: "pactus".into(),
            asset_id: "PAC".into(),
            address: "pac_addr_2".into(),
            created_at: chrono::Utc::now().naive_local(),
        },
        Address {
            wallet_id: "wallet_1".into(),
            user_id: "bob".into(),
            chain_id: "bitcoin".into(),
            asset_id: "PAC".into(),
            address: "btc_addr_1".into(),
            created_at: chrono::Utc::now().naive_local(),
        },
    ];

    // Test address assignment
    for addr in &test_cases {
        db.save_address(addr).unwrap();
    }

    // Verify addresses
    for addr in test_cases {
        let retrieved = db
            .get_address(
                &addr.wallet_id,
                &addr.user_id,
                &addr.chain_id,
                &addr.asset_id,
            )
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.address, addr.address);
    }
}
