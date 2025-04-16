use crate::types::{
    Address,
    Wallet,
};

/// Provides read access to the database.
pub trait DatabaseReader {
    /// Returns wallet details for the given `chain_id`, if it exists.
    ///
    /// # Errors
    /// Returns an error if the wallet associated with the identifier cannot be retrieved.
    fn get_wallet(&self, chain_id: &str) -> anyhow::Result<Wallet>;

    /// Returns the address details for a user, if one has been generated for the specified chain.
    ///
    /// # Errors
    /// Returns an error if the address associated with the user cannot be retrieved.
    fn get_address(&self, user_id: &str, chain_id: &str) -> anyhow::Result<Option<Address>>;

    /// Checks whether the specified address exists for the given chain.
    ///
    /// # Errors
    /// Returns an error if the check fails.
    fn has_address(&self, address: &str, chain_id: &str) -> anyhow::Result<bool>;
}

/// Provides write access to the database.
pub trait DatabaseWriter {
    /// Assigns a new address to a user.
    ///
    /// # Errors
    /// Returns an error if an address is already assigned for the given identifier and user ID,
    /// or if the address cannot be stored for any reason.
    fn assign_address(
        &self,
        user_id: &str,
        chain_id: &str,
        wallet_id: &str,
        wallet_address: &str,
    ) -> anyhow::Result<()>;
}

/// Combines both read and write capabilities for database access.
pub trait DatabaseProvider: DatabaseReader + DatabaseWriter {}

impl<T: DatabaseReader + DatabaseWriter> DatabaseProvider for T {}
