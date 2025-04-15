use crate::model::*;

pub trait DatabaseReader {
    /// "get_wallet" will return wallet Details for existing identifier's wallet
    ///
    /// # Errors
    /// - could not retrieve wallet associated to identifier it will Error out
    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Option<Wallet>>;

    /// "get_address" will return address Details for user if is stored
    ///
    /// # Errors
    /// - could not retrieve address associated to user it will Error out
    fn get_address(&self, user_id: &str, chain_id: &str) -> anyhow::Result<Option<WalletAddress>>;

    /// "has_address" will return false, if address is not exist in database.
    ///
    /// # Errors
    /// - return true, if address is already generated and save in database.
    fn has_address(&self, address: &str, chain_id: &str) -> anyhow::Result<bool>;
}

pub trait DatabaseWriter {
    /// "assign_address" will assign new Address to a user
    ///
    /// # Errors
    /// - will Error if there is a identifier+user_id already in datastore
    /// - it will fail if be not able to store address for any reason
    fn assign_address(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()>;
}

pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}
