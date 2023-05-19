

// // use sp_core::sr25519;
//use sp_std::vec::Vec;
//use sp_runtime::RuntimeString;
use sp_inherents::{InherentIdentifier, IsFatalError};
use sp_core::{Encode, Decode};
use sp_runtime::traits::Zero;
use frame_system::WeightInfo;

pub use pallet::*;
/// Type for Scale-encoded external data
//pub type InherentType = Vec<u8>;
pub type InherentType = [u8; 100];

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

    #[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;

		type ExternalDataType: Encode + Decode + Clone + Parameter + Member + MaxEncodedLen;
	}

    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ExternalDataSet { data: T::ExternalDataType  },
	}

	#[pallet::error]
	pub enum Error<T> {
	    AlreadySet,
	}

	#[pallet::storage]
	pub type ExternalData<T: Config> = StorageValue<_, T::ExternalDataType, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	{
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
//            .map(|_| InherentError::Other("ExternalDataInherentRequired".into())))
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

// /// A trait to find the author (miner) of the block.
// pub trait BlockAuthor<AccountId: From<sr25519::Public>> {
// 	fn block_author() -> Option<AccountId>;
// }

// impl<AccountId: From<sr25519::Public>> BlockAuthor<AccountId> for () {
// 	fn block_author() -> Option<AccountId> {
// 		None
// 	}
// }

// impl<T: Config> BlockAuthor<T::AccountId> for Pallet<T> 
// where
// 	<T as frame_system::Config>::AccountId: From<sp_core::sr25519::Public>
// {
// 	fn block_author() -> Option<T::AccountId> {
// 		Author::<T>::get().map(|a| a.into())
// 	}
// }

pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"ext_data";

#[derive(Encode)]
//#[cfg_attr(feature = "std", derive(Debug, Decode))]
pub enum InherentError {
//	Other(RuntimeString),
	Other,
}

impl IsFatalError for InherentError {
	fn is_fatal_error(&self) -> bool {
		match *self {
//			InherentError::Other(_) => true,
			InherentError::Other => true,
		}
	}
}

// impl InherentError {
// 	/// Try to create an instance ouf of the given identifier and data.
// 	#[cfg(feature = "std")]
// 	pub fn try_from(id: &InherentIdentifier, data: &[u8]) -> Option<Self> {
// 		if id == &INHERENT_IDENTIFIER {
// 			<InherentError as parity_scale_codec::Decode>::decode(&mut &data[..]).ok()
// 		} else {
// 			None
// 		}
// 	}
// }


