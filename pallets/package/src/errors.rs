use frame_support::pallet_macros::*;

/// A [`pallet_section`] that defines the events for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod errors {
	#[pallet::error]
	pub enum Error<T> {
		/// A package with that ID already exists
		PackageExists,
		/// A package with that ID does not exist
		PackageDoesNotExist,
		/// Attempting to assign/cancel a package whose status is not New
		PackageNotNew,
		/// A package has already been assigned to a carrier
		PackageAlreadyAssigned,
		/// Package is in a state which can no longer be collected
		PackageCannotBeCollected,
		/// Attempting to modify a delivered package
		PackageDelivered,
		/// Attempting to modify a cancelled package
		PackageCancelled,
		/// Package is In Transit
		PackageInTransit,
		/// Package is *not* In Transit
		PackageNotInTransit,
		/// Requested package is not valid for requested action
		InvalidPackage,
		/// Action attempted on package by carrier who is not assigned
		InvalidCarrier,
		/// Three Word Address format is invalid
		InvalidThreeWordAddress,
		/// Manifest Description is too long
		InvalidDescription,
		/// Not enough funds available to perform the requested action
		InsufficientFunds,
	}
}
