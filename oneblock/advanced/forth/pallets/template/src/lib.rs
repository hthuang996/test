#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use frame_support::inherent::Vec;

	use serde::{Deserialize};
	use codec::{Decode, Encode};


	use sp_runtime::{
		offchain::{
			storage::{StorageValueRef},
		},
		traits::Zero,
	};

	use sp_io::offchain_index;

	#[derive(Debug, Deserialize, Encode, Decode, Default)]
	struct IndexingData(Vec<u8>, u64);

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
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		Key(Vec::<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn write_offchain_storage(origin: OriginFor<T>, key_index: u32, something: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins

			// let key = Self::derive_key(block_number);
			let key = Self::derive_key(key_index.into());
			let data = IndexingData(b"submit_number_signed".to_vec(), something);

			Self::deposit_event(Event::Key(key.clone()));

			//  write or mutate tuple content to key
			offchain_index::set(&key, &data.encode());

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn write_offchain_storage_fail(origin: OriginFor<T>, something: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins

			let block_number = <frame_system::Pallet<T>>::block_number();

			let key = Self::derive_key(block_number);
			let data = IndexingData(b"submit_number_signed".to_vec(), something);
			Self::deposit_event(Event::Key(key.clone()));

			//  write or mutate tuple content to key
			offchain_index::set(&key, &data.encode());

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("Hello World from offchain workers!: {:?}", block_number);

			// if block_number % 2u32.into() != Zero::zero() {
            //     // // odd
            //     // let key = Self::derive_key(block_number);
            //     // let val_ref = StorageValueRef::persistent(&key);
                
            //     // //  get a local random value 
            //     // let random_slice = sp_io::offchain::random_seed();
                
            //     // //  get a local timestamp
            //     // let timestamp_u64 = sp_io::offchain::timestamp().unix_millis();

            //     // // combine to a tuple and print it  
            //     // let value = (random_slice, timestamp_u64);
            //     // log::info!("in odd block, value to write: {:?}", value);

            //     // //  write or mutate tuple content to key
            //     // val_ref.set(&value);

            // } else {
                // even
                let key = Self::derive_key(block_number - 1u32.into());

                let mut val_ref = StorageValueRef::persistent(&key);

                // get from db by key
                if let Ok(Some(value)) = val_ref.get::<IndexingData>() {
                    // print values
                    log::info!("in even block, value read: {:?}, {:?}", key, value);
                    // delete that key
                    val_ref.clear();
                }
				else {
					log::info!("No value to read");
				}
            // }

            log::info!("Leave from offchain workers!: {:?}", block_number);
        }

        fn on_initialize(_n: T::BlockNumber) -> Weight {
            log::info!("in on_initialize!");
            0
        }

        fn on_finalize(_n: T::BlockNumber) {
            log::info!("in on_finalize!");
        }

        fn on_idle(_n: T::BlockNumber, _remaining_weight: Weight) -> Weight {
            log::info!("in on_idle!");
            0
        }

    }

	impl<T: Config> Pallet<T> {

        #[deny(clippy::clone_double_ref)]
        fn derive_key(block_number: T::BlockNumber) -> Vec<u8> {
            block_number.using_encoded(|encoded_bn| {
                Vec::<u8>::from(encoded_bn)
            })
        }

    }
}
