use frame_support::pallet_macros::*;

#[pallet_section]
mod config {

	use frame_support::traits::{Currency, OnUnbalanced, ReservableCurrency};

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Ernest deposit amount
		#[pallet::constant]
		type ErnestDeposit: Get<BalanceOf<Self>>;

		/// What to do with slashed funds.
		type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// We don't want anyone storing the entirety of the Bee Movie script on-chain
		#[pallet::constant]
		type DescriptionMaxLength: Get<u32>;
	}
}
