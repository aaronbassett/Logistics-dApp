#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod calls;
mod config;
mod errors;
mod events;
pub mod types;

use crate::types::*;
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

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			// Remove all Delivered & Cancelled packages
			// along with associated manifests and proposals
			let concluded_packages =
				pallet_package::Pallet::<T>::remove_concluded_packages_and_manifests();

			if let Some(concluded_packages) = concluded_packages {
				for concluded_package in concluded_packages.iter() {
					pallet_carrier::Pallet::<T>::remove_concluded_proposals(
						concluded_package.clone(),
					);
				}
			}

			Weight::zero()
		}
	}
}
