#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod calls;
mod config;
mod errors;
mod events;
pub mod types;

use crate::types::*;
use frame_support::{pallet_macros::*, pallet_prelude::*, sp_runtime::SaturatedConversion};
use frame_system::pallet_prelude::*;

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
	#[pallet::getter(fn get_proposal)]
	pub type Proposals<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>, // Client's account id
			NMapKey<Blake2_128Concat, PackageId>,
			NMapKey<Blake2_128Concat, T::AccountId>, // Carrier's account id
		),
		Proposal<T>,
	>;
}

impl<T: Config> Pallet<T> {
	pub fn proposal_is_valid(
		client: &T::AccountId,
		package_id: &PackageId,
		carrier: &T::AccountId,
	) -> bool {
		let proposal = Self::get_proposal((&client, &package_id, &carrier));
		proposal.is_some() && proposal.unwrap().status == ProposalStatus::Proposed
	}

	pub fn proposal_maximum_fee_amount(
		client: &T::AccountId,
		package_id: &PackageId,
		carrier: &T::AccountId,
	) -> Option<u128> {
		match Self::get_proposal((&client, &package_id, &carrier)) {
			Some(proposal) => Some(proposal.maximum_fee_amount),
			None => None,
		}
	}

	pub fn reject_proposals(client: &T::AccountId, package_id: &PackageId, carrier: &T::AccountId) {
		for package_proposal in Proposals::<T>::iter_prefix((&client, &package_id)) {
			let mut rejected_proposal = package_proposal.1;
			if rejected_proposal.carrier != *carrier {
				rejected_proposal.status = ProposalStatus::Rejected;
				Proposals::<T>::insert(
					(&client, &package_id, &rejected_proposal.carrier),
					rejected_proposal.clone(),
				);

				Self::deposit_event(Event::<T>::ProposalRejected {
					client: client.clone(),
					package: package_id.clone(),
					carrier: rejected_proposal.carrier,
				});
			}
		}
	}

	pub fn accept_proposal(
		client: &T::AccountId,
		package_id: &PackageId,
		carrier: &T::AccountId,
	) -> DispatchResult {
		let mut proposal = Proposals::<T>::get((&client, &package_id, &carrier))
			.ok_or(Error::<T>::ProposalDoesNotExist)?;

		proposal.status = ProposalStatus::Accepted;
		Proposals::<T>::insert((&client, &package_id, &carrier), proposal);

		Self::deposit_event(Event::<T>::ProposalAccepted {
			client: client.clone(),
			package: package_id.clone(),
			carrier: carrier.clone(),
		});

		Ok(())
	}

	pub fn calculate_final_fee_amount(
		client: &T::AccountId,
		package_id: &PackageId,
		carrier: &T::AccountId,
		collected_on: BlockNumberFor<T>,
		delivered_on: BlockNumberFor<T>,
	) -> Result<u128, frame_support::dispatch::DispatchError> {
		let proposal = Proposals::<T>::get((&client, &package_id, &carrier))
			.ok_or(Error::<T>::ProposalDoesNotExist)?;

		// For every penalty period passed, deduct the penalty amount from the final fee
		let blocks_taken = delivered_on - collected_on;
		let periods = blocks_taken / proposal.penalty_period.into();
		let penalty: u128 = periods.saturated_into::<u128>() * proposal.penalty_amount;

		let final_fee = if proposal.maximum_fee_amount - penalty > proposal.minimum_fee_amount {
			proposal.maximum_fee_amount - penalty
		} else {
			proposal.minimum_fee_amount
		};

		Ok(final_fee)
	}

	pub fn remove_concluded_proposals(prefix: (T::AccountId, PackageId)) {
		let _ = Proposals::<T>::clear_prefix(prefix, u32::MAX, None);
	}
}
