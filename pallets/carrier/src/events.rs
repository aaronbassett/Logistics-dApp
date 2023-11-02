use frame_support::pallet_macros::*;

#[pallet_section]
mod events {
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New proposal created [Client ID, Package ID, Carrier ID, Max Fee, Min Fee]
		ProposalCreated {
			client: T::AccountId,
			package: PackageId,
			carrier: T::AccountId,
			maximum_fee: u128,
			minimum_fee: u128,
		},
		/// Proposal accepted [Client ID, Package ID, Carrier ID]
		ProposalAccepted { client: T::AccountId, package: PackageId, carrier: T::AccountId },
		/// Proposal rejected [Client ID, Package ID, Carrier ID]
		ProposalRejected { client: T::AccountId, package: PackageId, carrier: T::AccountId },
	}
}
