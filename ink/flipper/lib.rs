#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod flipper {
    use ink_lang as ink;
    use scale::{
        Decode,
        Encode,
    };

    use ink_primitives::{
        Key,
        KeyPtr,
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

    use ink_storage::{
        traits::{
            SpreadAllocate,
            SpreadLayout,
            StorageLayout,
            PackedLayout,
            PackedAllocate,
        },
        Mapping,
    };

    use ink_prelude::{
        vec::Vec,
        string::String,
    };

    type Byte = u8;
    type Bytes = Vec<Byte>;

    #[ink::trait_definition]
    pub trait TestTrait {
        #[ink(message)]
        fn only_owner(&mut self) -> Result<(), Error>;
        #[ink(message)]
        fn param_test(&mut self) -> Result<DeriveTest, Error>;
    }

    /// Content structure
    #[derive(SpreadAllocate, SpreadLayout, PackedLayout, Clone, Default, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    struct Content {
        contract: String,
        action: String,
        data: Bytes,
    }

    impl PackedAllocate for Content {
        fn allocate_packed(&mut self, at: &Key) {
            PackedAllocate::allocate_packed(&mut self.contract, at);
            PackedAllocate::allocate_packed(&mut self.action, at);
            PackedAllocate::allocate_packed(&mut self.data, at);
        }
    }

    #[derive(SpreadAllocate, PackedLayout, SpreadLayout, Clone, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    pub struct DeriveTest {
        content: Content,
    }

    impl PackedAllocate for DeriveTest {
        fn allocate_packed(&mut self, at: &Key) {
            PackedAllocate::allocate_packed(&mut self.content, at)
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        value: bool,
        map: Mapping<u8, Vec<Content>>,
        map1: Mapping<ink_prelude::string::String, Vec<Content>>,
        v: Vec<Bytes>,
        d_t: Vec<DeriveTest>,
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            // let m = Mapping<u128, u128>::new();
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract)
            })
        }

        fn new_init(&mut self) {
            let caller = Self::env().caller();
            self.owner = caller;
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        fn owner_test1(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.owner != caller {
                return Err(Error::NotOwner);
            }
            Ok(())
        }

        fn test_func(&mut self) -> Result<(), Error> {
            let mut item: Vec<Content> = self.map.get(8).ok_or(Error::NotOwner)?;
            let i: Content = Content::default();
            item.push(i);
            self.map.insert(8, &item);

            Ok(())
        }

        #[ink(message)]
        pub fn test_func2(&mut self) -> Result<(), Error> {
            let s = String::from("asdf");
            let mut item: Vec<Content> = self.map1.get(&s).ok_or(Error::NotOwner)?;
            self.map1.insert(s, &item);

            Ok(())
        }

        fn is_owner(&mut self) -> bool {
            let caller = Self::env().caller();
            if self.owner != caller {
                return false;
            }
            true
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn owner_test(&mut self) -> Result<(), Error> {
            if !self.is_owner() {
                return Err(Error::NotOwner);
            }
            Ok(())
        }
    }

    impl TestTrait for Flipper {
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn only_owner(&mut self) -> Result<(), Error> {
            let caller = Self::env().caller();
            if self.owner != caller {
                return Err(Error::NotOwner);
            }
            Ok(())
        }

        #[ink(message)]
        fn param_test(&mut self) -> Result<DeriveTest, Error> {
            Err(Error::NotOwner)
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn owner_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>();
            let mut flipper = Flipper::new(false);
            set_caller(accounts.bob);
            println!("{:?}", flipper.map.get(0));
        }

        fn set_caller(sender: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(sender);
        }
    }
}
