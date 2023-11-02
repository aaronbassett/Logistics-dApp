#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod calls;
mod config;
mod errors;
mod events;
pub mod types;

use crate::types::*;
use frame_support::{pallet_macros::*, pallet_prelude::*};
use frame_system::{self as system, pallet_prelude::*};

#[import_section(events::events)]
#[import_section(errors::errors)]
#[import_section(config::config)]
#[import_section(calls::calls)]
#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn get_package)]
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

	#[pallet::storage]
	pub type ConcludedPackages<T: Config> =
		StorageValue<_, BoundedVec<(T::AccountId, PackageId), T::MaxConcludedPackages>>;
}

impl<T: Config> Pallet<T> {
	pub fn package_is_new(client: &T::AccountId, package_id: &PackageId) -> bool {
		let package = Self::get_package(&client, &package_id);
		package.is_some() && package.unwrap().status == PackageStatus::New
	}

	pub fn package_is_in_transit(client: &T::AccountId, package_id: &PackageId) -> bool {
		let package = Self::get_package(&client, &package_id);
		package.is_some() && package.unwrap().status == PackageStatus::InTransit
	}

	pub fn assign_package(
		client: &T::AccountId,
		package_id: &PackageId,
		carrier: &T::AccountId,
	) -> DispatchResult {
		let mut package =
			Packages::<T>::get(&client, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

		package.carrier = Some(carrier.clone());
		package.status = PackageStatus::Assigned;
		Packages::<T>::insert(&client, &package_id, package);

		Self::deposit_event(Event::<T>::PackageAssigned {
			client: client.clone(),
			package: package_id.clone(),
			carrier: carrier.clone(),
		});

		Ok(())
	}

	pub fn cancel_package(client: &T::AccountId, package_id: &PackageId) -> DispatchResult {
		let mut package =
			Packages::<T>::get(&client, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

		package.status = PackageStatus::Cancelled;
		Packages::<T>::insert(&client, &package_id, package);

		Self::deposit_event(Event::<T>::PackageCancelled {
			client: client.clone(),
			package: package_id.clone(),
		});

		Ok(())
	}

	pub fn deliver_package(client: &T::AccountId, package_id: &PackageId) -> DispatchResult {
		let mut package =
			Packages::<T>::get(&client, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

		package.delivered_on = Some(<system::Pallet<T>>::block_number());
		package.status = PackageStatus::Delivered;
		Packages::<T>::insert(&client, &package_id, package);

		Self::deposit_event(Event::<T>::PackageDelivered {
			client: client.clone(),
			package: package_id.clone(),
		});

		Ok(())
	}

	pub fn remove_concluded_packages_and_manifests(
	) -> Option<BoundedVec<(T::AccountId, PackageId), T::MaxConcludedPackages>> {
		let concluded_packages = ConcludedPackages::<T>::take();

		if let Some(concluded_packages) = concluded_packages.clone() {
			for concluded_package in concluded_packages.iter() {
				Packages::<T>::remove(&concluded_package.0, &concluded_package.1);
				Manifests::<T>::remove(&concluded_package.0, &concluded_package.1);
			}
		}

		concluded_packages
	}
}
