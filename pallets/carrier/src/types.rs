use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::sp_runtime::RuntimeDebug;
use frame_system::{self as system, pallet_prelude::BlockNumberFor, Config};
use scale_info::TypeInfo;

pub type PackageId = [u8; 32];

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum ProposalStatus {
	Proposed,
	Accepted,
	Rejected,
	Cancelled,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Proposal<T: Config> {
	pub package: PackageId,
	pub client: T::AccountId,
	pub carrier: T::AccountId,
	pub maximum_fee_amount: u128,
	pub minimum_fee_amount: u128,
	pub penalty_period: u32,
	pub penalty_amount: u128,
	pub proposed_on: BlockNumberFor<T>,
	pub status: ProposalStatus,
}

impl<T: Config> Proposal<T> {
	pub fn new(
		package: PackageId,
		client: T::AccountId,
		carrier: T::AccountId,
		maximum_fee_amount: u128,
		minimum_fee_amount: u128,
		penalty_period: u32,
		penalty_amount: u128,
	) -> Self {
		Proposal {
			package,
			client,
			carrier,
			maximum_fee_amount,
			minimum_fee_amount,
			penalty_period,
			penalty_amount,
			proposed_on: <system::Pallet<T>>::block_number(),
			status: ProposalStatus::Proposed,
		}
	}
}
