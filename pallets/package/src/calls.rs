use frame_support::pallet_macros::*;

#[pallet_section]
mod calls {

	use frame_system;
	use sp_std::vec::Vec;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO: #[pallet::weight(T::WeightInfo::package_create())]
		pub fn package_create(
			origin: OriginFor<T>,
			package_id: PackageId,
			pickup: Vec<u8>,
			destination: Vec<u8>,
			description: Vec<u8>,
			length: u32,
			width: u32,
			height: u32,
			weight: u32,
			contains_hazardous_materials: bool,
			requires_climate_controlled: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let pickup_bounded: BoundedVec<_, _> =
				pickup.try_into().map_err(|_| Error::<T>::InvalidThreeWordAddress)?;
			let destination_bounded: BoundedVec<_, _> =
				destination.try_into().map_err(|_| Error::<T>::InvalidThreeWordAddress)?;
			let description_bounded: BoundedVec<_, _> =
				description.try_into().map_err(|_| Error::<T>::InvalidDescription)?;

			// package IDs must be unique per client
			ensure!(!Packages::<T>::contains_key(&who, &package_id), Error::<T>::PackageExists);

			// Reserve ernest deposit
			T::Currency::reserve(&who, T::ErnestDeposit::get())
				.map_err(|_| Error::<T>::InsufficientFunds)?;

			Packages::<T>::insert(
				&who,
				&package_id,
				Package::new(package_id, who.clone(), pickup_bounded, destination_bounded),
			);

			Manifests::<T>::insert(
				&who,
				&package_id,
				Manifest {
					description: description_bounded,
					dimensions: Dimensions { length, width, height },
					weight,
					hazardous_materials: contains_hazardous_materials,
					climate_controlled: requires_climate_controlled,
				},
			);

			Self::deposit_event(Event::PackageCreated { client: who, package: package_id });

			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO: #[pallet::weight(T::WeightInfo::package_create())]
		pub fn package_assign(
			origin: OriginFor<T>,
			package_id: PackageId,
			carrier: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists
			let mut package =
				Packages::<T>::get(&who, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

			// Ensure that package is still new
			ensure!(package.status == PackageStatus::New, Error::<T>::PackageNotNew);

			package.carrier = Some(carrier.clone());
			package.status = PackageStatus::Assigned;

			Packages::<T>::insert(&who, &package_id, package);

			// TODO: Calculate how much more needs to be reserved

			Self::deposit_event(Event::PackageAssigned {
				client: who,
				package: package_id,
				carrier,
			});

			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO: #[pallet::weight(T::WeightInfo::package_create())]
		pub fn package_collect(
			origin: OriginFor<T>,
			package_id: PackageId,
			client: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists
			let mut package =
				Packages::<T>::get(&client, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

			// Ensure package has been assigned to this carrier
			ensure!(package.carrier == Some(who.clone()), Error::<T>::InvalidCarrier);

			// Ensure that package has not already been collected, delivered, or cancelled
			ensure!(
				package.status == PackageStatus::Assigned,
				Error::<T>::PackageCannotBeCollected
			);

			package.collected_on = Some(<frame_system::Pallet<T>>::block_number());
			package.status = PackageStatus::InTransit;

			Packages::<T>::insert(&client, &package_id, package);

			Self::deposit_event(Event::PackageCollected {
				client,
				package: package_id,
				carrier: who,
			});

			Ok(())
		}

		#[pallet::call_index(30)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO: #[pallet::weight(T::WeightInfo::package_create())]
		pub fn package_deliver(origin: OriginFor<T>, package_id: PackageId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists
			let mut package =
				Packages::<T>::get(&who, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

			// Ensure package is currently in transit
			ensure!(package.status == PackageStatus::InTransit, Error::<T>::PackageNotInTransit);

			package.delivered_on = Some(<frame_system::Pallet<T>>::block_number());
			package.status = PackageStatus::Delivered;

			Packages::<T>::insert(&who, &package_id, package);

			// TODO: Pay Carrier

			Self::deposit_event(Event::PackageDelivered { client: who, package: package_id });

			Ok(())
		}

		#[pallet::call_index(40)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO: #[pallet::weight(T::WeightInfo::package_create())]
		pub fn package_cancel(origin: OriginFor<T>, package_id: PackageId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists
			let mut package =
				Packages::<T>::get(&who, &package_id).ok_or(Error::<T>::PackageDoesNotExist)?;

			// Cannot cancel packages once they've been assigned
			ensure!(package.status == PackageStatus::New, Error::<T>::PackageNotNew);

			package.status = PackageStatus::Cancelled;

			Packages::<T>::insert(&who, &package_id, package);

			// TODO: Release reserved ernest money

			Self::deposit_event(Event::PackageCancelled { client: who, package: package_id });

			Ok(())
		}
	}
}
