use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{sp_runtime::RuntimeDebug, traits::ConstU32, BoundedVec};
use frame_system::{self as system, pallet_prelude::BlockNumberFor, Config};
use scale_info::TypeInfo;

use crate::pallet;

pub type PackageId = [u8; 32];

pub type ThreeWordAddress = BoundedVec<u8, ConstU32<256>>;

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum PackageStatus {
	New,
	Assigned,
	InTransit,
	Delivered,
	Cancelled,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum ProposalStatus {
	Proposed,
	Accepted,
	Rejected,
	Cancelled,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct Dimensions {
	pub length: u32,
	pub width: u32,
	pub height: u32,
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Package<T: Config> {
	pub(super) id: PackageId,
	pub(super) client: T::AccountId,
	pub(super) carrier: Option<T::AccountId>,
	pub(super) pickup: ThreeWordAddress,
	pub(super) destination: ThreeWordAddress,
	pub(super) requested_on: BlockNumberFor<T>,
	pub(super) collected_on: Option<BlockNumberFor<T>>,
	pub(super) delivered_on: Option<BlockNumberFor<T>>,
	pub(super) status: PackageStatus,
}

impl<T: Config> Package<T> {
	pub fn new(
		id: PackageId,
		client: T::AccountId,
		pickup: ThreeWordAddress,
		destination: ThreeWordAddress,
	) -> Self {
		Package {
			id,
			client,
			carrier: None,
			pickup,
			destination,
			requested_on: <system::Pallet<T>>::block_number(),
			collected_on: None,
			delivered_on: None,
			status: PackageStatus::New,
		}
	}
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Manifest<T: Config + pallet::Config> {
	pub description: BoundedVec<u8, <T>::DescriptionMaxLength>,
	pub dimensions: Dimensions,
	pub weight: u32,
	pub hazardous_materials: bool,
	pub climate_controlled: bool,
}
