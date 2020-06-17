#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet proof of existence with necessary imports

use frame_support::{storage::{StorageMap},
	decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	traits::{Get},
};
use frame_support::traits::{Currency, ExistenceRequirement};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

	// 附加题答案
	type MaxClaimLength: Get<u32>;

	// 添加Currency类型
	type Currency: Currency<Self::AccountId>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
		Prices get(fn price): map hasher(blake2_128_concat) Vec<u8> => BalanceOf<T>;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId,
	   Balance = BalanceOf<T>,
	  {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		PriceSet(AccountId, Vec<u8>, Balance),
		BuyClaimOk(AccountId,Vec<u8>, Balance),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ProofTooLong,
		CanNotBuyOwnClaim,
		BuyPriceCanNOtLessThanSellPrice,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			// 附加题答案
			ensure!(T::MaxClaimLength::get() >= claim.len() as u32, Error::<T>::ProofTooLong);

			Proofs::<T>::insert(&claim, (sender.clone(), system::Module::<T>::block_number()));

            // 存证创建时设置默认价格为0
			let price: BalanceOf<T> = 0.into();
			Prices::<T>::insert(&claim,&price);

			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));

			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Prices::<T>::remove(&claim);

			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));

			Ok(())
		}

		// 第二题答案
		#[weight = 0]
		pub fn transfer_claim(origin, claim: Vec<u8>, dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);

			let (owner, _block_number) = Proofs::<T>::get(&claim);

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			let dest = T::Lookup::lookup(dest)?;

			Proofs::<T>::insert(&claim, (dest, system::Module::<T>::block_number()));

			Ok(())
		}

		#[weight = 0]
		pub fn set_price(origin, claim: Vec<u8>,price: BalanceOf<T>) -> dispatch::DispatchResult {
		    let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
			let (s,_) = Proofs::<T>::get(&claim);
			ensure!(s == sender,Error::<T>::NotClaimOwner);
			Prices::<T>::insert(&claim,&price);

			Self::deposit_event(RawEvent::PriceSet(sender,claim,price));

			Ok(())
		}
		/**
		 * origin buy claim user
		 * seller sell claim user
		 * sellPrice
		 * buyPrice
		 */
		 #[weight = 0]
		 pub fn buy_claim(origin,claim: Vec<u8>,buy_price: BalanceOf<T>) -> dispatch::DispatchResult {
		     let buyer = ensure_signed(origin)?;
		 	 ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);

			 let(owner,_) = Proofs::<T>::get(&claim);
			 ensure!(buyer!=owner,Error::<T>::CanNotBuyOwnClaim);
			 let sell_pric_least =  Prices::<T>::get(&claim);

			 ensure!(buy_price > sell_pric_least, Error::<T>::BuyPriceCanNOtLessThanSellPrice);

		 	 T::Currency::transfer(&buyer,&owner,sell_pric_least,ExistenceRequirement::AllowDeath)?;

			 Proofs::<T>::insert(&claim, (&buyer, system::Module::<T>::block_number()));

			 Prices::<T>::insert(&claim,&sell_pric_least);

			 Self::deposit_event(RawEvent::BuyClaimOk(buyer,claim,sell_pric_least));

			 Ok(())

		 }
	}
}
