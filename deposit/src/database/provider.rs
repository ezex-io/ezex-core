use crate::types::{Address, Wallet};

/// Provides read access to the database.
pub trait DatabaseReader {
    /// Returns wallet details for the given `chain_id`, if available.
    ///
    /// # Errors
    /// Returns an error if the wallet associated with the identifier cannot be retrieved.
    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Option<Wallet>>;

    /// Returns the address details for a user, if one has been generated for the specified asset.
    ///
    /// # Errors
    /// Returns an error if the address associated with the user cannot be retrieved.
    fn get_address(
        &self,
        wallet_id: &str,
        user_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<Option<Address>>;

    /// Checks whether the specified address exists for the given asset.
    ///
    /// # Errors
    /// Returns an error if the check fails.
    fn has_address(
        &self,
        wallet_id: &str,
        user_id: &str,
        chain_id: &str,
        asset_id: &str,
    ) -> anyhow::Result<bool>;
}

/// Provides write access to the database.
pub trait DatabaseWriter {
    /// Saves a newly generated address to the database.
    ///
    /// # Errors
    /// Returns an error if an address has already been assigned to the user,
    /// or if the address cannot be stored for any reason.
    fn save_address(&self, address: &Address) -> anyhow::Result<()>;
}

/// Combines both read and write capabilities for database access.
pub trait DatabaseProvider: DatabaseReader + DatabaseWriter + Sync + Send + 'static {}

impl<T: DatabaseReader + DatabaseWriter + Sync + Send + 'static> DatabaseProvider for T {}
