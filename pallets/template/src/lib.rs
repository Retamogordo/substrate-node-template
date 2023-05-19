#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::vec::Vec;
use sp_runtime::RuntimeString;
use sp_inherents::{InherentIdentifier, IsFatalError};
use sp_core::{Encode, Decode};
use sp_runtime::traits::Zero;
//use frame_system::WeightInfo;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
/// Type for Scale-encoded external data
pub type InherentType = Vec<u8>;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		type ExternalDataType: Encode + Decode + Clone + Parameter + Member + MaxEncodedLen;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type ExternalData<T: Config> = StorageValue<_, T::ExternalDataType, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		ExternalDataSet { data: T::ExternalDataType  },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		AlreadySet,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(12_345_678)]
		pub fn set(origin: OriginFor<T>, external_data: T::ExternalDataType) -> DispatchResult {
//			log::info!("!!!!!!!!!!!!!!!!!!!!!!!!!! in set: data: {:?} ", external_data);
			ensure_none(origin)?;
			ensure!(ExternalData::<T>::get().is_none(), Error::<T>::AlreadySet);

			ExternalData::<T>::put(&external_data);
			
            Self::deposit_event(Event::ExternalDataSet { data: external_data });

			Ok(())
		}

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			ExternalData::<T>::kill();

			Zero::zero()
		}
	}
	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = InherentError;
		const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

		fn is_inherent_required(data: &InherentData) -> Result<Option<Self::Error>, Self::Error> {
//			Ok(None)
            Ok(Self::decode_data(data)
				.map(|_| InherentError::Other))
//			.map(|_| InherentError::Other("ExternalDataInherentRequired".into())))
		}

		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			Self::decode_data(data)
				.map(|external_data| Call::set {external_data})
		}

		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::set {..})
		}
	}

	impl<T: Config> Pallet<T> {
		fn decode_data(data: &InherentData) -> Option<T::ExternalDataType> {
			let res = data
				.get_data::<InherentType>(&INHERENT_IDENTIFIER)
				.ok()
				.unwrap_or_default()
                .and_then(|encoded_data| 
//			        Option::<T::ExternalDataType>::decode(&mut &encoded_data[..])
			        T::ExternalDataType::decode(&mut &encoded_data[..])
						.ok()
//						.unwrap_or_default()
				);
//			log::info!("!!!!!!!!!!!!!!!!!!!!! decoded_data: {:?}", res);
			res
		}
	}

}

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"ext_data";

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
	Other,
}

impl IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		match *self {
			InherentError::Other => true,
		}
	}
}
