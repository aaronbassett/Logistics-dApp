use frame_support::pallet_macros::*;

#[pallet_section]
mod calls {

	use frame_support::{sp_runtime::SaturatedConversion, traits::ExistenceRequirement};
	use frame_system;
	use sp_std::vec::Vec;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn package_assign(
			origin: OriginFor<T>,
			package_id: PackageId,
			carrier: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists and is still "New"
			ensure!(
				pallet_package::Pallet::<T>::package_is_new(&who, &package_id),
				pallet_package::Error::<T>::InvalidPackage
			);

			// Ensure that proposal exists and is in a valid state to be assigned
			ensure!(
				pallet_carrier::Pallet::<T>::proposal_is_valid(&who, &package_id, &carrier),
				pallet_carrier::Error::<T>::InvalidProposal
			);

			// Reserve Maximum fee amount
			let max_fee: BalanceOf<T> = pallet_carrier::Pallet::<T>::proposal_maximum_fee_amount(
				&who,
				&package_id,
				&carrier,
			)
			.unwrap_or(0)
			.try_into()
			.map_err(|_| Error::<T>::InvalidFee)?;

			<T as pallet::Config>::Currency::reserve(&who, max_fee)
				.map_err(|_| Error::<T>::InsufficientFunds)?;

			// Update package status and carrier
			pallet_package::Pallet::<T>::assign_package(&who, &package_id, &carrier)?;

			// Reject other proposals
			pallet_carrier::Pallet::<T>::reject_proposals(&who, &package_id, &carrier);

			// Accept carrier's proposal
			pallet_carrier::Pallet::<T>::accept_proposal(&who, &package_id, &carrier)?;

			Ok(())
		}

		#[pallet::call_index(20)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn package_cancel(origin: OriginFor<T>, package_id: PackageId) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure package exists and is still "New"
			// We can't cancel a package that's already been assigned
			ensure!(
				pallet_package::Pallet::<T>::package_is_new(&who, &package_id),
				pallet_package::Error::<T>::InvalidPackage
			);

			// Cancel Package
			pallet_package::Pallet::<T>::cancel_package(&who, &package_id)?;

			// Release reserved ernest reserve
			<T as pallet::Config>::Currency::unreserve(
				&who,
				<T as pallet::Config>::ErnestDeposit::get(),
			);

			Ok(())
		}

		#[pallet::call_index(30)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn package_deliver(
			origin: OriginFor<T>,
			package_id: PackageId,
			carrier: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?; // Only client can mark as delivered

			// Ensure package exists and is still "InTransit"
			ensure!(
				pallet_package::Pallet::<T>::package_is_in_transit(&who, &package_id),
				pallet_package::Error::<T>::InvalidPackage
			);

			// Mark package as delivered
			pallet_package::Pallet::<T>::deliver_package(&who, &package_id)?;

			let package = pallet_package::Pallet::<T>::get_package(&who, &package_id)
				.ok_or(pallet_package::Error::<T>::PackageDoesNotExist)?;

			// Pay carrier
			let final_fee = pallet_carrier::Pallet::<T>::calculate_final_fee_amount(
				&who,
				&package_id,
				&carrier,
				package.collected_on.unwrap_or(<frame_system::Pallet<T>>::block_number()),
				<frame_system::Pallet<T>>::block_number(),
			)?;

			// Unreserve Maximum fee amount
			let max_fee: BalanceOf<T> = pallet_carrier::Pallet::<T>::proposal_maximum_fee_amount(
				&who,
				&package_id,
				&carrier,
			)
			.unwrap_or(0)
			.try_into()
			.map_err(|_| Error::<T>::InvalidFee)?;

			<T as pallet::Config>::Currency::unreserve(&who, max_fee);

			// Transfer final fee amount to carrier
			<T as pallet::Config>::Currency::transfer(
				&who,
				&carrier,
				final_fee.saturated_into(),
				ExistenceRequirement::KeepAlive,
			)?;

			// Unreserve ernest deposit
			<T as pallet::Config>::Currency::unreserve(
				&who,
				<T as pallet::Config>::ErnestDeposit::get(),
			);

			Self::deposit_event(Event::PackageDelivered { client: who, package: package_id });

			Ok(())
		}

		#[pallet::call_index(50)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn proposal_create(
			origin: OriginFor<T>,
			client: T::AccountId,
			package_id: PackageId,
			maximum_fee_amount: u128,
			minimum_fee_amount: u128,
			penalty_period: u32,
			penalty_amount: u128,
		) -> DispatchResult {
			pallet_carrier::Pallet::<T>::proposal_create(
				origin,
				client,
				package_id,
				maximum_fee_amount,
				minimum_fee_amount,
				penalty_period,
				penalty_amount,
			)
		}

		#[pallet::call_index(60)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn proposal_accept(
			origin: OriginFor<T>,
			package_id: PackageId,
			carrier: T::AccountId,
		) -> DispatchResult {
			pallet_carrier::Pallet::<T>::proposal_accept(origin, package_id, carrier)
		}

		#[pallet::call_index(70)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
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
			pallet_package::Pallet::<T>::package_create(
				origin,
				package_id,
				pickup,
				destination,
				description,
				length,
				width,
				height,
				weight,
				contains_hazardous_materials,
				requires_climate_controlled,
			)
		}

		#[pallet::call_index(80)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn package_collect(
			origin: OriginFor<T>,
			package_id: PackageId,
			client: T::AccountId,
		) -> DispatchResult {
			pallet_package::Pallet::<T>::package_collect(origin, package_id, client)
		}
	}
}
