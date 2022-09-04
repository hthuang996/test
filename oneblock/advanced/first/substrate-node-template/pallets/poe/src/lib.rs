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
	use sp_std::vec::Vec;

    /// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Proofs<T:Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    // Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId, Vec::<u8>),
		ClaimRevoked(T::AccountId, Vec::<u8>),
		ClaimTransferred(T::AccountId, T::AccountId, Vec::<u8>),
	}

    // Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		ClaimAlreadyExist,
		/// Errors should have helpful documentation associated with them.
		ClaimNotExist,
		/// There have be doc.
		OnlyOwnerCanRevoke,
		NotAbleToTransferToSelf,
		OnlyOwnerCanTransfer,
		ClaimLengthError,
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(claim.len() == 8, Error::<T>::ClaimLengthError);

            // Check if the claim exists.
            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyExist);

			// Update storage.
			<Proofs<T>>::insert(
                &claim,
                (sender.clone(), frame_system::Pallet::<T>::block_number())
            );

			// Emit an event.
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

            // Check if the claim exists.
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::OnlyOwnerCanRevoke);

            // Update storage
            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(owner, claim));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, receiver: T::AccountId, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Can not transfer the claim to self.
            ensure!(sender != receiver, Error::<T>::NotAbleToTransferToSelf);

            // Check if the claim exists.
            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>::OnlyOwnerCanTransfer);

            // Update storage
            Proofs::<T>::remove(&claim);
			<Proofs<T>>::insert(
                &claim,
                (receiver.clone(), frame_system::Pallet::<T>::block_number())
            );

            Self::deposit_event(Event::ClaimTransferred(owner, receiver, claim));

			Ok(())
		}
	}
}