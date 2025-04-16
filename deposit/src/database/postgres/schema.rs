diesel::table! {
    #[sql_name = "ezex_deposit_wallets"]
    wallets (id) {
        id -> Uuid,
        status -> SmallInt,
        wallet_id -> Varchar,
        chain_id -> Varchar,
        wallet_type -> Varchar,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    #[sql_name = "ezex_deposit_address_book"]
    address_book (id) {
        id -> Uuid,
        user_id -> Varchar,
        chain_id -> Varchar,
        wallet_id -> Varchar,
        address -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(wallets, address_book,);
