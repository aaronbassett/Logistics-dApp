use frame_support::pallet_macros::*;

#[pallet_section]
mod errors {
	#[pallet::error]
	pub enum Error<T> {
		/// Not enough funds available to perform the requested action
		InsufficientFunds,
		/// Could not convert fee amount into valid balance
		InvalidFee,
	}
}
