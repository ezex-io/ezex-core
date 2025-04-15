// @generated automatically by Diesel CLI.

diesel::table! {
    addresses (user_id, chain_id) {
        user_id -> Varchar,
        chain_id -> Varchar,
        wallet_id -> Varchar,
        deposit_address -> Varchar,
        created_at -> Timestamp,
    }
}
