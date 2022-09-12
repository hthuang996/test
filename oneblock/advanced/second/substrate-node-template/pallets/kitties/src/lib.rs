#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {    
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::traits::{
		ExistenceRequirement,
		Currency,
		ReservableCurrency,
		Randomness,
		tokens::fungible::Transfer,
	};
	use frame_support::transactional;
	use sp_io::hashing::blake2_128;
	use sp_std::{cmp, fmt::Debug, mem, ops::BitOr, prelude::*, result};
	use codec::{Codec, Decode, Encode, MaxEncodedLen};
	use sp_runtime::{
		traits::{
			AtLeast32BitUnsigned, AtLeast32Bit, Bounded, CheckedAdd, CheckedSub, MaybeSerializeDeserialize,
			Saturating, StaticLookup, Zero,
		},
		ArithmeticError, DispatchError, RuntimeDebug,
	};

	type BalanceOf<T> =
		<<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::type_value]
	pub fn GetDefaultValue<T: Config>() -> T::KittyIndex {
		0_u32.into()
	}
	
	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

    /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Balances: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type KittyIndex: Parameter
			+ Member
			+ AtLeast32Bit
			+ Codec
			+ Default
			+ Copy
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaxEncodedLen
			+ TypeInfo;
		#[pallet::constant]
		type MaxKittyOwned: Get<u32>;
		#[pallet::constant]
		type ReserveBalance: Get<BalanceOf<Self>>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type NextKittyId<T:Config> = StorageValue<_, T::KittyIndex, ValueQuery, GetDefaultValue<T>>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T:Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T:Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn orders)]
	pub type Orders<T:Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, BalanceOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn owner_kitties)]
	pub type OwnerKitties<T:Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::KittyIndex, T::MaxKittyOwned>>;

    // Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyCreated(T::AccountId, T::KittyIndex, Kitty),
		KittyBred(T::AccountId, T::KittyIndex, Kitty),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidKittyId,
		SameKittyId,
		NotOwner,
		InvalidOrder,
		KittyIdOverflow,
		ReserveFailed,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		#[transactional]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let dna = Self::random_value(&sender);

			let kitty = Self::create_kitty(dna, kitty_id, sender.clone())?;

			Self::deposit_event(Event::KittyCreated(sender, kitty_id, kitty));
			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);

			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			ensure!(Some(sender.clone()) == Self::kitty_owner(kitty_id_1), Error::<T>::NotOwner);
			ensure!(Some(sender.clone()) == Self::kitty_owner(kitty_id_2), Error::<T>::NotOwner);

			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			let selector = Self::random_value(&sender);			
			let data = Self::mix(&kitty_1, &kitty_2, selector);
			
			let kitty = Self::create_kitty(data, kitty_id, sender.clone())?;

			Self::deposit_event(Event::KittyBred(sender, kitty_id, kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn transfer(origin: OriginFor<T>, kitty_id: T::KittyIndex, new_owner: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			ensure!(Some(sender.clone()) == Self::kitty_owner(kitty_id), Error::<T>::NotOwner);

			Self::transfer_kitty(kitty_id, &sender, &new_owner)?;

			Self::deposit_event(Event::KittyTransferred(sender, new_owner, kitty_id));

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn sell_kitty(origin: OriginFor<T>, kitty_id: T::KittyIndex, price: BalanceOf<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
			
			ensure!(Some(sender.clone()) == Self::kitty_owner(kitty_id), Error::<T>::NotOwner);

			Orders::<T>::insert(kitty_id, price);

			Ok(())
		}

		#[pallet::weight(10_000)]
		#[transactional]
		pub fn buy_kitty(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let price = Self::get_order(kitty_id).map_err(|_| Error::<T>::InvalidOrder)?;
			let owner = Self::kitty_owner(kitty_id).unwrap();

			// T::Balances::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive);
			// T::Balances::reserve(&sender, price)?;

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// random
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet::<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		fn get_next_id() -> Result<T::KittyIndex, ()> {
			let max = T::KittyIndex::max_value();
			let next_id: T::KittyIndex = Self::next_kitty_id();
			if max == next_id {
				Err(())
			}
			else {
				Ok(next_id)
			}
		}

		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}

		fn get_order(kitty_id: T::KittyIndex) -> Result<BalanceOf<T>, ()> {
			match Self::orders(kitty_id) {
				Some(price) => Ok(price),
				None => Err(())
			}
		}

		fn mix(kitty_1: &Kitty, kitty_2: &Kitty, selector: [u8; 16]) -> [u8; 16] {
			let mut data = [0_u8; 16];
			for i in 0..kitty_1.0.len() {
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}
			data
		}

		fn create_kitty(dna: [u8; 16], kitty_id: T::KittyIndex, owner: T::AccountId) -> Result<Kitty, Error<T>> {
			let kitty = Kitty(dna);

			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &owner);
			let next_id: T::KittyIndex = kitty_id.checked_add(&(1_u32.into()))
											.ok_or(Error::<T>::KittyIdOverflow)?;
			NextKittyId::<T>::set(next_id);
			// let ownerKitties = OwnerKitties::<T>::get(owner).unwrap_or(BoundedVec::<T::KittyIndex, T::MaxKittyOwned>::with_bounded_capacity(T::MaxKittyOwned::get()));
			let mut owner_kitties = OwnerKitties::<T>::get(&owner).unwrap_or(BoundedVec::<T::KittyIndex, T::MaxKittyOwned>::default());
			owner_kitties.try_push(kitty_id).map_err(|_| Error::<T>::KittyIdOverflow)?;
			OwnerKitties::<T>::insert(&owner, owner_kitties);

			T::Balances::reserve(&owner, T::ReserveBalance::get()).map_err(|_| Error::<T>::ReserveFailed)?;

			Ok(kitty)
		}

		fn transfer_kitty(kitty_id: T::KittyIndex, from: &T::AccountId, to: &T::AccountId) -> Result<(), Error<T>> {
			Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
			KittyOwner::<T>::insert(kitty_id, &to);

			let mut owner_kitties = OwnerKitties::<T>::get(&from).unwrap_or(BoundedVec::<T::KittyIndex, T::MaxKittyOwned>::default());
			let index = owner_kitties.iter().position(|&r| r == kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			owner_kitties.remove(index);
			OwnerKitties::<T>::insert(&from, owner_kitties);
			
			let mut new_owner_kitties = OwnerKitties::<T>::get(&to).unwrap_or(BoundedVec::<T::KittyIndex, T::MaxKittyOwned>::default());
			new_owner_kitties.try_push(kitty_id).map_err(|_| Error::<T>::KittyIdOverflow)?;
			OwnerKitties::<T>::insert(&to, new_owner_kitties);
			
			T::Balances::unreserve(&from, T::ReserveBalance::get());
			T::Balances::reserve(&to, T::ReserveBalance::get()).map_err(|_| Error::<T>::ReserveFailed)?;

			Ok(())
		}
	}
}