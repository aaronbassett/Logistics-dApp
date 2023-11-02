use frame_support::pallet_macros::*;

#[pallet_section]
mod errors {
	#[pallet::error]
	pub enum Error<T> {
		/// A proposal already exists from that carrier
		ProposalExists,
		/// A proposal does not exist from that carrier
		ProposalDoesNotExist,
		/// Proposal is not valid for requested action
		InvalidProposal,
	}
}
