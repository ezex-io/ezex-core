use crate::{
    database::postgres::{config::Config as PostgresConfig, postgres::PostgresDB},
    deposit::DepositHandler,
    event_bus::{events::address::Generated, provider::MockPublisherProvider},
    grpc::deposit::GenerateAddressRequest,
    kms::provider::MockKmsProvider,
    types::Address,
};
use common::testsuite::postgres::PostgresTestDB;
use mockall::predicate::*;
use tonic::Request;

struct TestData {
    pub mocked_kms: MockKmsProvider,
    pub mocked_publisher: MockPublisherProvider,
    pub db: PostgresDB,
}

impl TestData {
    pub fn setup(test_db: &PostgresTestDB) -> Self {
        let db = PostgresDB::new(&PostgresConfig {
            database_url: test_db.con_string(),
            pool_size: 1,
        })
        .unwrap();
        let mocked_kms = MockKmsProvider::new();
        let mocked_publisher = MockPublisherProvider::new();

        // Manually create a test wallet before running tests
        test_db.execute(
            r#"
        INSERT INTO ezex_deposit_wallets
        (status, wallet_id, chain_id, description)
        VALUES
        (1, 'wallet-id-1', 'Pactus', 'test-wallet')
        "#,
        );

        Self {
            mocked_kms,
            mocked_publisher,
            db,
        }
    }

    pub fn deposit_handler(self) -> DepositHandler {
        DepositHandler::new(
            Box::new(self.db),
            Box::new(self.mocked_kms),
            Box::new(self.mocked_publisher),
        )
    }
}

#[tokio::test]
async fn test_generate_address_unknown_chain() {
    let test_db = PostgresTestDB::new();
    let td = TestData::setup(&test_db);

    let request = Request::new(GenerateAddressRequest {
        user_id: "alice".into(),
        chain_id: "unknown_chain".into(),
        asset_id: "PAC".into(),
    });

    let res = td.deposit_handler().generate_address(request).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_generate_address() {
    let test_db = PostgresTestDB::new();
    let mut td = TestData::setup(&test_db);

    let expected_address = Address {
        wallet_id: "wallet-id-1".into(),
        user_id: "alice".into(),
        chain_id: "Pactus".into(),
        asset_id: "PAC".into(),
        address: "address-alice".into(),
        created_at: chrono::Utc::now().naive_utc(),
    };
    let cloned_expected_address = expected_address.clone();

    td.mocked_kms
        .expect_generate_address()
        .once()
        .withf(
            move |wallet_id: &str, user_id: &str, chain_id: &str, asset_id: &str| {
                wallet_id == expected_address.wallet_id
                    && user_id == expected_address.user_id
                    && chain_id == expected_address.chain_id
                    && asset_id == expected_address.asset_id
            },
        )
        .return_once(|_, _, _, _| Ok(cloned_expected_address));

    td.mocked_publisher
        .expect_publish()
        .once()
        .withf(|evt| evt.key() == Generated::event_key)
        .return_once(|_| Ok(()));

    let request = GenerateAddressRequest {
        user_id: "alice".into(),
        chain_id: "Pactus".into(),
        asset_id: "PAC".into(),
    };

    let deposit = td.deposit_handler();

    let res = deposit
        .generate_address(Request::new(request.clone()))
        .await
        .unwrap();
    assert_eq!(res.get_ref().address.as_str(), "address-alice");

    // Test Duplicated Address
    let res = deposit.generate_address(Request::new(request)).await;
    assert!(res.is_err());
}
