#![cfg_attr(not(feature = "std"), no_std)]

const CROSS_CONTRACT_ADDRESS: &str = "X";
const SEND_MESSAGE_SELECTOR: [u8; 4] = [0; 4];

use ink_lang as ink;
mod test_trait;

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

    use super::test_trait::{
        Ownable,
        Error,
    };

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error1 {
        NotOwner2(u8),
        NotApproved2,
        TokenExists2,
        TokenNotFound2,
        CannotInsert2,
        CannotFetchValue2,
        NotAllowed2,
    }

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
    pub struct Content {
        contract: String,
        action: String,
        data: Bytes,
    }

    impl PackedAllocate for Content {
        fn allocate_packed(&mut self, at: &Key) {
            // PackedAllocate::allocate_packed(&mut self.contract, at);
            // PackedAllocate::allocate_packed(&mut self.action, at);
            // PackedAllocate::allocate_packed(&mut self.data, at);
        }
    }

    #[derive(SpreadAllocate, PackedLayout, SpreadLayout, Clone, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    pub struct ShowTest {
        test_id: u8,
    }

    #[derive(SpreadAllocate, PackedLayout, SpreadLayout, Clone, Decode, Encode)]
    #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo, StorageLayout))]
    pub struct DeriveTest {
        content: Content,
        test_id: u8,
    }

    impl PackedAllocate for DeriveTest {
        fn allocate_packed(&mut self, at: &Key) {
            PackedAllocate::allocate_packed(&mut self.content, at);
            PackedAllocate::allocate_packed(&mut self.test_id, at)
        }
    }

    #[ink(event)]
    pub struct Transferred {
        // #[ink(topic)]
        // from: Option<AccountId>,

        // #[ink(topic)]
        // to: Option<AccountId>,

        // #[ink(topic)]
        // value: Balance,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        owner: AccountId,
        owner2: Option<AccountId>,
        value: bool,
        str_value: String,
        map: Mapping<u8, Vec<Content>>,
        map1: Mapping<ink_prelude::string::String, Vec<Content>>,
        v: Bytes,
        v_map: Mapping<u8, Bytes>,
        d_t: Vec<DeriveTest>,
        message: u8,
        message2: u8,
        ob: Option<ink_prelude::vec::Vec<u8>>,
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
            self.str_value = String::from("黄海涛");
        }

        #[ink(message)]
        pub fn set_string_value(&mut self, s_value: String) {
            self.str_value = s_value;
        }

        #[ink(message)]
        pub fn get_string_value(& self) -> String {
            self.str_value.clone()
        }

        #[ink(message)]
        pub fn get_string_value_bytes(& self) -> Bytes {
            self.str_value.as_bytes().to_vec()
        }

        #[ink(message)]
        pub fn send_message(&mut self, addr1: AccountId, addr2: AccountId, m: u8) {
            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(addr1)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    // call receive_message
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x3a, 0x6e, 0x96, 0x96]))
                    .push_arg(addr2)
                    .push_arg(m)
                )
                .call_flags(ink_env::CallFlags::default().set_allow_reentry(true))
                .returns::<()>()
                .fire()
                .unwrap();
        }

        #[ink(message)]
        pub fn receive_message(&mut self, addr: AccountId, i: u8) -> Result<(), Error1> {
            self.message = i;
            let ret: Result<(), Error1> = ink_env::call::build_call::<ink_env::DefaultEnvironment>()
                .call_type(
                    ink_env::call::Call::new()
                        .callee(addr)
                        .gas_limit(0)
                        .transferred_value(0))
                .exec_input(
                    // call receive_message2
                    ink_env::call::ExecutionInput::new(ink_env::call::Selector::new([0x03, 0x0e, 0x11, 0xd0]))
                    .push_arg(i)
                )
                .returns::<Result<(), Error1>>()
                .fire()
                .unwrap();

                ret
        }

        #[ink(message)]
        pub fn receive_message2(&mut self, i: u8) -> Result<(), Error> {
            self.message = i;

            let from = self.env().caller();
            let to = self.env().account_id();
            // implementation hidden
            self.env().emit_event(Transferred {
                // from: Some(from),
                // to: Some(to),
                // value: 10,
            });

            Err(Error::NotApproved)
        }

        #[ink(message)]
        pub fn get_message(& self) -> u8 {
            self.message
        }

        #[ink(message)]
        pub fn is_owner(& self) -> bool {
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
                return Err(Error::NotOwner(10));
                // assert!(false, "asdf");
                // panic!("asdf");
            }
            Ok(())
        }
        
        

        #[ink(message)]
        pub fn signatureVerify(&self, msg: ink_prelude::string::String, signature: [u8; 65], acct: AccountId)-> bool {
            let mut msg_hash = <ink_env::hash::Sha2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_bytes::<ink_env::hash::Sha2x256>(&msg.as_bytes(), &mut msg_hash);

            let mut compressed_pubkey = [0; 33];
            ink_env::ecdsa_recover(&signature, &msg_hash, &mut compressed_pubkey);

            let mut addr_hash = <ink_env::hash::Blake2x256 as ink_env::hash::HashOutput>::Type::default();
            ink_env::hash_encoded::<ink_env::hash::Blake2x256, _>(&compressed_pubkey, &mut addr_hash);

            AccountId::from(addr_hash) == acct
        }

        #[ink(message)]
        pub fn get_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
            let mut raw_buffer = ink_prelude::vec![];

            let mut int32_vec = ink_prelude::vec![99 as i32, 88, 77];
            for ele in int32_vec.iter_mut() {
                raw_buffer.append(&mut ink_prelude::vec::Vec::from(ele.to_be_bytes()));
            }

            let some_str = ink_prelude::string::String::from("Hello Nika");
            raw_buffer.append(&mut ink_prelude::vec::Vec::from(some_str.as_bytes()));

            raw_buffer
        }

        #[ink(message)]
        pub fn get_hash(& self, msg: ink_prelude::vec::Vec<u8>) -> [u8; 32] {
            let mut ret = [0_u8; 32];
            // ink_env::hash_bytes::<ink_env::hash::Sha2x256>(msg.as_slice(), &mut ret);
            ink_env::hash_bytes::<ink_env::hash::Keccak256>(msg.as_slice(), &mut ret);
            ret
        }  
    }

    impl TestTrait for Flipper {
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        fn only_owner(&mut self) -> Result<(), Error> {
            let caller = Self::env().caller();
            if self.owner != caller {
                return Err(Error::NotOwner(10));
            }
            Ok(())
        }

        #[ink(message)]
        fn param_test(&mut self) -> Result<DeriveTest, Error> {
            Err(Error::NotOwner(10))
        }
    }

    impl Ownable for Flipper {
        /// Returns the account id of the current owner
        #[ink(message)]
        fn owner(&self) -> Option<AccountId> {
            self.owner2
        }

        /// Renounces ownership of the contract
        #[ink(message)]
        fn renounce_ownership(&mut self) -> Result<(), Error> {
            self.only_owner()?;

            self.owner2 = None;

            Ok(())
        }

        /// Transfer ownership to a new account id
        #[ink(message)]
        fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<(), Error> {
            self.only_owner()?;

            self.owner2 = Some(new_owner);

            Ok(())
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

        #[ink::test]
        fn test_get_hash() {
            let mut flipper = Flipper::new(false);
            let ink_address = ink_prelude::vec![1, 2, 3];
            println!("{:?}", flipper.get_hash(ink_address));
        }
    }
}
