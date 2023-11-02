use frame_support::pallet_macros::*;

#[pallet_section]
mod calls {

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
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
			let who = ensure_signed(origin)?;

			// Can only submit 1 proposal at a time
			ensure!(
				!Proposals::<T>::contains_key((&client, &package_id, &who)),
				Error::<T>::ProposalExists
			);

			// Insert new proposal into storage
			Proposals::<T>::insert(
				(&client, &package_id, &who),
				Proposal::new(
					package_id,
					client.clone(),
					who.clone(),
					maximum_fee_amount,
					minimum_fee_amount,
					penalty_period,
					penalty_amount,
				),
			);

			Self::deposit_event(Event::ProposalCreated {
				client,
				package: package_id,
				carrier: who,
				maximum_fee: maximum_fee_amount,
				minimum_fee: minimum_fee_amount,
			});

			Ok(())
		}

		#[pallet::call_index(10)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn proposal_accept(
			origin: OriginFor<T>,
			package_id: PackageId,
			carrier: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Ensure proposal exists and is valid to be accepted
			ensure!(
				Self::proposal_is_valid(&who, &package_id, &carrier),
				Error::<T>::InvalidProposal
			);

			// Reject all proposals
			for package_proposal in Proposals::<T>::iter_prefix((&who, &package_id)) {
				let mut rejected_proposal = package_proposal.1;
				rejected_proposal.status = ProposalStatus::Rejected;
				Proposals::<T>::insert(
					(&who, &package_id, &rejected_proposal.carrier),
					rejected_proposal.clone(),
				);
			}

			// Mark proposal as accepted
			let mut proposal = Proposals::<T>::get((&who, &package_id, &carrier))
				.ok_or(Error::<T>::ProposalDoesNotExist)?;
			proposal.status = ProposalStatus::Accepted;
			Proposals::<T>::insert((&who, &package_id, &carrier), proposal);

			// Send proposal accepted event
			Self::deposit_event(Event::ProposalAccepted {
				client: who.clone(),
				package: package_id,
				carrier: carrier.clone(),
			});

			Ok(())
		}
	}
}
