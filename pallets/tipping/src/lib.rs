// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(pattern)]

//! Tipping pallet is the core feature in our attempt to build the functionality and features to
//! support creators’ economy in a truly decentralized manner. Every creator can verify their
//! revenue channels like websites, subdomains, or a username on commercial websites and accept
//! payment from anybody in crypto. This pallet, together with the Anagolay Extension, can be used
//! to support open-source projects per user and per repository. To prevent the misuse of the pallet
//! and to make sure that the correct people are supported, the tipping pallet depends on the
//! verification pallet to get the proofs of the domain, username or repository verification, and
//! statements pallet for the ownership.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]

mod benchmarking;
mod functions;
pub mod types;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

pub mod constants {
  use anagolay_support::getter_for_constant;
  getter_for_constant!(MaxTipsPerVerificationContext, u32);
}

#[frame_support::pallet]
pub mod pallet {
  use crate::{constants::*, types::*, weights::WeightInfo};
  use core::convert::TryInto;
  use frame_support::{
    pallet_prelude::*,
    sp_std::{vec::*, *},
    traits::{Currency, ReservableCurrency},
  };
  use verification::types::{VerificationContext, VerificationStatus};

  use frame_support::traits::{ExistenceRequirement, UnixTime};
  use frame_system::pallet_prelude::*;

  pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config + verification::Config {
    /// The overarching event type.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    /// Currency that allows to verify the available balance for the tipper
    type Currency: ReservableCurrency<Self::AccountId>;

    /// Timestamps provider
    type TimeProvider: UnixTime;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Maximum number of Tips recorded for a single VerificationContext. Once reached, old tips
    /// will be discarded to record the new tips
    const MAX_TIPS_PER_VERIFICATION_CONTEXT: u32;
  }

  #[pallet::extra_constants]
  impl<T: Config> Pallet<T> {
    #[pallet::constant_name(MaxTipsPerVerificationContext)]
    fn max_tips_per_verification_context() -> u32 {
      T::MAX_TIPS_PER_VERIFICATION_CONTEXT
    }
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn integrity_test() {
      assert!(
        T::MAX_TIPS_PER_VERIFICATION_CONTEXT > 0u32,
        "`MaxTipsPerVerificationContext` must be greater than 0"
      );
    }
  }

  /// The map of TippingSettings indexed by their respective AccountId and VerificationContext
  #[pallet::storage]
  #[pallet::getter(fn tipping_settings_by_account_id_and_verification_context)]
  pub type TippingSettingsByAccountIdAndVerificationContext<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Twox64Concat,
    VerificationContext,
    TippingSettings<T::AccountId>,
    ValueQuery,
  >;

  /// The map of collection of Tips indexed by their respective receiver AccountId and
  /// VerificationContext
  #[pallet::storage]
  #[pallet::getter(fn tips_by_account_id_and_verification_context)]
  pub type TipsByAccountIdAndVerificationContext<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Twox64Concat,
    VerificationContext,
    BoundedVec<Tip<BalanceOf<T>, T::AccountId, T::BlockNumber>, MaxTipsPerVerificationContextGet<T>>,
    ValueQuery,
  >;

  #[pallet::error]
  pub enum Error<T> {
    /// The verification context is not associated to a successful verification request and cannot
    /// be tipped
    InvalidVerificationContext,
    /// The verification context is not set-up to enable tipping
    InvalidConfiguration,
  }

  /// Events of the Poe pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Produced upon settings update
    TippingSettingsUpdated(T::AccountId, Vec<TippingSettings<T::AccountId>>),
    /// Produced upon the newly created tip
    TipCreated(
      T::AccountId,
      T::AccountId,
      Tip<BalanceOf<T>, T::AccountId, T::BlockNumber>,
    ),
    /// This event is never raised: chain metadata does not include types used only in RPCs so as
    /// workaround we need to include it here
    /// also this is NEVER USED, we had massive issues with the deserialization of the enum on the
    /// JS side. Keeping this for now, to test the regressions
    __TippingLookupTypes(SortTips),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Accepts a collection of [`TippingSettings`] and stores it. A coherency check is performed
    /// on the rightfulness of the caller to configure each setting
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * tipping_settings - the [`TippingSettings`]
    ///
    /// # Events
    /// * `SettingsUpdated` - when the [`TippingSettings`] is successfully updated
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::update_settings())]
    pub fn update_settings(
      origin: OriginFor<T>,
      tipping_settings: Vec<TippingSettings<T::AccountId>>,
    ) -> DispatchResultWithPostInfo {
      let caller = ensure_signed(origin)?;

      let mut updated_settings: Vec<TippingSettings<T::AccountId>> = Vec::new();

      // Iterate through each setting and insert into the storage
      let mut tipping_settings = tipping_settings;
      tipping_settings.iter_mut().for_each(|setting| {
        // Allow only to the verification holder to update the tipping settings
        if let Some(holder) = verification::Pallet::<T>::account_ids_by_verification_context(setting.context.clone())
          .iter()
          .find(|account| caller == **account)
        {
          // Avoid misconfigured tipping settings - default to holder if no account is configured
          let _ = setting.account.get_or_insert(holder.clone());

          // Allow the caller to update the settings only if the status of the verification request related to
          // the context is success
          if let Some(request) = verification::Pallet::<T>::verification_request_by_account_id_and_verification_context(
            holder.clone(),
            setting.context.clone(),
          ) {
            if request.status == VerificationStatus::Success {
              TippingSettingsByAccountIdAndVerificationContext::<T>::insert(
                caller.clone(),
                setting.context.clone(),
                setting.clone(),
              );

              updated_settings.push(setting.clone());
            }
          }
        }
      });

      // Emit an event that tells which settings have been updated
      Self::deposit_event(Event::TippingSettingsUpdated(caller, updated_settings));

      Ok(().into())
    }

    /// Accepts a [`Tip`] for a [`VerificationContext`] and stores them in the
    /// `TipsByAccountIdAndVerificationContext` while it transfers of the required amount from
    /// the account of the sender to the account of the receiver.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * tip - the [`Tip`]
    /// * context - the [`VerificationContext`]
    ///
    /// # Errors
    /// * `InvalidVerificationContext` - If the [`VerificationContext`] is not available for tipping
    /// * `InvalidConfiguration` - If tipping is disabled or not configured for the context
    ///
    /// # Events
    /// * `TipCreated` - when the [`Tip`] is successfully created
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::tip())]
    pub fn tip(origin: OriginFor<T>, amount: BalanceOf<T>, context: VerificationContext) -> DispatchResultWithPostInfo {
      let tipper = ensure_signed(origin)?;

      // Retrieve the successful verification request associated to the context
      let requests =
        verification::Pallet::<T>::get_requests(vec![context.clone()], Some(VerificationStatus::Success), None, 0, 1);
      ensure!(requests.len() == 1, Error::<T>::InvalidVerificationContext);
      let tipped = requests.first().unwrap().holder.clone();

      // Ensure that the tipping is enabled and configured for the context
      let settings = TippingSettingsByAccountIdAndVerificationContext::<T>::get(tipped.clone(), context.clone());
      ensure!(
        settings.enabled && settings.account.is_some(),
        Error::<T>::InvalidConfiguration
      );

      // Fill in tip information
      let receiver_account: T::AccountId = settings.account.unwrap();
      let block_number = <frame_system::Pallet<T>>::block_number();
      let tip = Tip {
        amount,
        sender: tipper.clone(),
        receiver: receiver_account.clone(),
        created_at: T::TimeProvider::now().as_secs(),
        block_number,
      };

      // Make the transfer and store the tip
      <T as Config>::Currency::transfer(&tipper, &receiver_account, tip.amount, ExistenceRequirement::KeepAlive)?;
      TipsByAccountIdAndVerificationContext::<T>::mutate(tipped.clone(), context, |existing_tips| {
        existing_tips.try_push(tip.clone()).unwrap_or_else(|_| {
          existing_tips.slide(0, existing_tips.len());
          existing_tips.force_push(tip.clone());
        })
      });

      // Emit an event that the tip has been created
      Self::deposit_event(Event::TipCreated(tipper, tipped, tip));

      Ok(().into())
    }
  }
}
