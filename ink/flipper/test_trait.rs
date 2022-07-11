use ink_env::AccountId;
use ink_lang as ink;
use scale::{
    Decode,
    Encode,
};

#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotOwner,
    NotApproved,
    TokenExists,
    TokenNotFound,
    CannotInsert,
    CannotFetchValue,
    NotAllowed,
}

/// Trait for owner
#[ink::trait_definition]
pub trait Ownable {
    /// Returns the account id of the current owner
    #[ink(message)]
    fn owner(&self) -> Option<AccountId>;
    /// Renounces ownership of the contract
    #[ink(message)]
    fn renounce_ownership(&mut self) -> Result<(), Error>;
    /// Transfer ownership to a new account id
    #[ink(message)]
    fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), Error>;
}