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
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        // #[ink(message)]
        // pub fn flip(&mut self) {
        //     self.value = !self.value;
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get(&self) -> bool {
        //     self.value
        // }

        // fn owner_test1(&mut self) -> Result<(), Error> {
        //     let caller = self.env().caller();
        //     if self.owner != caller {
        //         return Err(Error::NotOwner);
        //     }
        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn test_func(&mut self) -> Result<(), Error> {
        //     let mut item: Vec<Content> = self.map.get(8).unwrap_or(Vec::<Content>::new());
        //     let i: Content = Content::default();
        //     item.push(i);
        //     self.map.insert(8, &item);

        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn test_modify(&mut self, i: u32, contract: String) -> Result<(), Error> {
        //     let mut item: Vec<Content> = self.map.get(8).ok_or(Error::NotOwner)?;
        //     let iu = usize::try_from(i).unwrap();
        //     let mut content: &mut Content = item.get_mut(iu).ok_or(Error::NotApproved)?;
        //     content.contract = contract;
        //     // item.insert(iu, content.clone());
        //     self.map.insert(8, &item);

        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn test_get(& self, i: u32) -> Result<Content, Error> {
        //     let mut item: Vec<Content> = self.map.get(8).ok_or(Error::NotOwner)?;
        //     let iu = usize::try_from(i).unwrap();
        //     let content: &Content = item.get(usize::try_from(iu).unwrap()).ok_or(Error::NotApproved)?;
        //     Ok(content.clone())
        // }

        // #[ink(message)]
        // pub fn test_func2(&mut self) -> Result<(), Error> {
        //     let s = String::from("asdf");
        //     let mut item: Vec<Content> = self.map1.get(&s).ok_or(Error::NotOwner)?;
        //     self.map1.insert(s, &item);

        //     Ok(())
        // }

        // fn is_owner(&mut self) -> bool {
        //     let caller = Self::env().caller();
        //     if self.owner != caller {
        //         return false;
        //     }
        //     true
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn owner_test(&mut self) -> Result<(), Error> {
        //     if !self.is_owner() {
        //         return Err(Error::NotOwner);
        //     }
        //     Ok(())
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get_str_value(& self) -> String {
        //     self.str_value.clone()
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn set_str_value(&mut self, value: String) {
        //     self.str_value = value;
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn enum_get(& self, e: Error) -> Result<(), Error> {
        //     Err(e)
        // }

        // /// Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn option_get(& self, o: Option<u8>) -> Option<u8> {
        //     o
        // }

        // #[ink(message)]
        // pub fn custom_vec_add(&mut self, v: DeriveTest) {
        //     self.d_t.push(v);
        // }

        // #[ink(message)]
        // pub fn custom_vec_get(& self, i: u32) -> DeriveTest {
        //     self.d_t[usize::try_from(i).unwrap()].clone()
        // }

        // #[ink(message)]
        // pub fn custom_vec_length(& self) -> u32 {
        //     self.d_t.len() as u32
        // }

        // #[ink(message)]
        // pub fn string_to_bytes(& self, a: String) -> Bytes {
        //     Bytes::from(a)
        // }

        // #[ink(message)]
        // pub fn emit_event(&mut self) {
        //     let from = self.env().caller();
        //     // implementation hidden
        //     self.env().emit_event(Transferred {
        //         // from: Some(from),
        //         // to: Some(from),
        //         value: 10,
        //     });
        // }

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
        pub fn receive_message(&mut self, addr: AccountId, i: u8) {
            self.message = i;
            ink_env::call::build_call::<ink_env::DefaultEnvironment>()
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
                .returns::<()>()
                .fire()
                .unwrap();
        }

        #[ink(message)]
        pub fn receive_message2(&mut self, i: u8) {
            self.message = i;

            let from = self.env().caller();
            let to = self.env().account_id();
            // implementation hidden
            self.env().emit_event(Transferred {
                // from: Some(from),
                // to: Some(to),
                // value: 10,
            });
        }

        #[ink(message)]
        pub fn get_message(& self) -> u8 {
            self.message
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
                return Err(Error::NotOwner(10));
                // assert!(false, "asdf");
                // panic!("asdf");
            }
            Ok(())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn return_null_test(&mut self) {
            return;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get_str_value(& self) -> String {
            self.str_value.clone()
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn set_str_value(&mut self, value: String) {
            self.str_value = value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn enum_get(& self, e: Error) -> Result<(), Error> {
            Err(e)
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn option_get(& self, o: Option<u8>) -> Option<u8> {
            o
        }

        #[ink(message)]
        pub fn show_test_add(&mut self, v: ShowTest) {
        }

        #[ink(message)]
        pub fn custom_vec_add(&mut self, v: DeriveTest) {
            self.d_t.push(v);
        }

        #[ink(message)]
        pub fn custom_vec_get(& self, i: u32) -> DeriveTest {
            self.d_t[usize::try_from(i).unwrap()].clone()
        }

        #[ink(message)]
        pub fn custom_vec_length(& self) -> u32 {
            self.d_t.len() as u32
        }

        #[ink(message)]
        pub fn string_to_bytes(& self, a: String) -> Bytes {
            Bytes::from(a)
        }

        #[ink(message)]
        pub fn emit_event(&mut self) {
            let from = self.env().caller();

            self.env().emit_event(Transferred {
                // from: Some(from),
                // to: Some(from),
                // value: 10
            });
        
        }

        /// Test
        #[ink(message)]
        pub fn set_bytes(&mut self, v: ink_prelude::vec::Vec<u8>) -> Result<(), ()> {
            self.v = v;
            Ok(())
        }

        /// Test
        #[ink(message)]
        pub fn set_map_bytes(&mut self, key: u8, v: ink_prelude::vec::Vec<u8>) -> Result<(), ()> {
            self.v_map.insert(key, &v);
            Ok(())
        }

        /// Test
        #[ink(message)]
        pub fn test_bytes(&mut self, session: Option<ink_prelude::vec::Vec<u8>>) -> Result<(), ()> {
            self.ob = session;
            Ok(())
        }

        /// Test
        #[ink(message)]
        pub fn test_u8_array(&mut self, session: Option<[u8; 4]>) -> Result<(), ()> {
            Ok(())
        }
        
        #[ink(message)]
        pub fn get_bytes(&mut self) -> Option<ink_prelude::vec::Vec<u8>> {
            struct AA {
                a: u8
            }
            self.ob.clone()
        }
        // #[ink(message)]
        // pub fn get_message_mock(& self) -> u8 {
        //     let i = 12;
        //     i            
        // }
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
