#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod calls;
mod config;
mod errors;
mod events;
pub mod types;

use frame_support::pallet_macros::*;

#[import_section(events::events)]
#[import_section(errors::errors)]
#[import_section(config::config)]
#[import_section(calls::calls)]
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use crate::types::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Packages<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		PackageId,
		Package<T>,
	>;

	#[pallet::storage]
	pub type Manifests<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		PackageId,
		Manifest<T>,
	>;
}
