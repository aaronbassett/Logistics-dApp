use frame_support::pallet_macros::*;

#[pallet_section]
mod storages {
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
