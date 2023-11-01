use frame_support::pallet_macros::*;

/// A [`pallet_section`] that defines the events for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod events {
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New package created [Client Account ID, Package ID]
		PackageCreated { client: T::AccountId, package: PackageId },
		/// Package has been assigned to a carrier [Client Account ID, Package ID, Carrier Account
		/// ID]
		PackageAssigned { client: T::AccountId, package: PackageId, carrier: T::AccountId },
		/// Package collected by carrier
		PackageCollected { client: T::AccountId, package: PackageId, carrier: T::AccountId },
		/// Package has been delivered [Client Account ID, Package ID]
		PackageDelivered { client: T::AccountId, package: PackageId },
		/// Package has been cancelled [Client Account ID, Package ID]
		PackageCancelled { client: T::AccountId, package: PackageId },
	}
}
