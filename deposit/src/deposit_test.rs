use crate::kms::mock::MockKmsProvider;
use crate::kms::provider::KmsProvider;
use crate::{config::Config, deposit::Deposit};
use common::{consts::*, testsuite::postgres::PostgresTestDB, topic::deposit};
use mockall::predicate::*;
use serde_json::json;

#[tokio::test]
async fn test_generate_address_with_invalid_wallet_id() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let user_id = "Alice".to_string();
    let request = deposit::address::Generate {
        user_id: user_id.clone(),
        coin: "PAC".to_string(),
        wallet_id: "valid-wallet-id".to_string(),
    };

    let res = deposit.process_address_generate(request).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_generate_address_eth() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let user_id = "Alice".to_string();
    let request = deposit::address::Generate {
        user_id,
        coin: "PAC".to_string(),
        wallet_id: "wallet_1".to_string(),
    };

    let res = deposit
        .process_address_generate(request)
        .await
        .unwrap()
        .remove(0);
    let event = res
        .as_any()
        .downcast_ref::<deposit::address::Generated>()
        .unwrap();
    assert_eq!(event.deposit_address, "eth_address");
}

#[tokio::test]
async fn test_generate_address_btc() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let request = deposit::address::Generate {
        user_id: "alice".to_owned(),
        coin: "PAC".to_string(),
        wallet_id: "wallet_1".to_owned(),
    };

    let res = deposit
        .process_address_generate(request)
        .await
        .unwrap()
        .remove(0);
    let event = res
        .as_any()
        .downcast_ref::<deposit::address::Generated>()
        .unwrap();
    assert_eq!(event.deposit_address, "???");
}

#[tokio::test]
async fn test_process_duplicate_address_generate() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let request = deposit::address::Generate {
        user_id: "alice".to_owned(),
        coin: "PAC".to_string(),
        wallet_id: "wallet_1".to_owned(),
    };

    let res_1 = deposit
        .process_address_generate(request.clone())
        .await
        .unwrap()
        .remove(0);
    let res_2 = deposit.process_address_generate(request).await;

    let event_1 = res_1
        .as_any()
        .downcast_ref::<deposit::address::Generated>()
        .unwrap();

    assert_eq!(
        event_1.deposit_address,
        "2N4sexvpWpMUjoVHHFXuAUitngG8pwb2sKf"
    );
    assert!(res_2.is_err(), "duplicated address found");
}

#[tokio::test]
async fn test_check_coin_exists() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let request = deposit::address::Generate {
        user_id: "alice".to_owned(),
        coin: "PAC".to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_owned(),
    };

    let res = deposit.process_address_generate(request).await;
    assert!(res.is_err())
}

#[tokio::test]
async fn test_check_duplicate_address() {
    let pq_db = PostgresTestDB::new();

    let db = crate::database::postgres::postgres::PostgresDB::new(&pq_db.con_string(), 1).unwrap();
    let kms = MockKmsProvider::new();
    let deposit = Deposit::new(db, kms);

    let request = deposit::address::Generate {
        user_id: "alice".to_owned(),
        coin: "PAC".to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_owned(),
    };

    let request_1 = deposit::address::Generate {
        user_id: "bob".to_owned(),
        coin: "PAC".to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_owned(),
    };

    let _res = deposit.process_address_generate(request).await;
    let res_1 = deposit.process_address_generate(request_1).await;
    assert!(res_1.is_err())
}
