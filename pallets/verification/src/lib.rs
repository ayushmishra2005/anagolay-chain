// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(pattern)]

//! This pallet’s responsibility is to keep records ofThis pallet’s responsibility is to keep
//! records of the verified items and their proofs to know how to handle different types of
//! verification processes and how to store them. There can be any number of Strategies implemented
//! to handle several different verification scenarios. In the following description, we’ll speak of
//! DNS verification, but the same procedure applies similarly to other verification strategies as
//! well.
//!
//! Having done so, due to DNS propagation, the process can halt and DoH (DNS over HTTPS) queries
//! can be performed off-chain before the `perform_verification` extrinsic is called, because this
//! call will incur transaction costs. When the DNS propagation happened, process can resume. Other
//! verification strategies may be more or less immediate.
//!
//! Any verifier account, even different from the holder, can call `perform_verification` at any
//! time to update the state of the request to //! `Pending`, signaling to the off-chain worker
//! that, on its next execution, the challenge must be verified. If the verification status is
//! already `Failed`, however, the call to perform verification will result in an error since the
//! verification must be requested again from the holder in order to pay the registration fee.
//!
//! At execution of the off-chain worker, the appropriate verification strategy is instantiated,
//! `DNSVerificationStrategy` in our case. It //! performs a call to the DNS resolve provider to
//! verify the presence and the exactness of the aforementioned key. The `VerificationRequest` is
//! then updated on chain with the call to a local unsigned extrinsic to store the appropriate
//! status; `Success` or `Failure`. If the verification fails, the registration fee is attributed to
//! the verifier account, which is the origin of the call to perform verification, in appreciation
//! of the behavior of external actors that validate that `VerificationRequest` validity is not
//! expired, or for the holder to claim back the registration fee.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
mod functions;
mod offchain;
mod strategies;
pub mod types;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

const ONCHAIN_TX_KEY: &[u8] = b"verification::strategy::worker";

pub mod consts {
  /// Getter for the configurable constant MAX_REQUESTS_PER_CONTEXT
  #[derive(
    codec::Encode,
    codec::Decode,
    Clone,
    PartialEq,
    Eq,
    frame_support::sp_runtime::RuntimeDebug,
    frame_support::pallet_prelude::TypeInfo,
  )]
  pub struct MaxVerificationRequestsPerContextGet<T>(frame_support::pallet_prelude::PhantomData<T>);
  /// Implementation of the ['Get'] trait for the getter of MAX_REQUESTS_PER_CONTEXT
  impl<T: crate::pallet::Config> frame_support::pallet_prelude::Get<u32> for MaxVerificationRequestsPerContextGet<T> {
    fn get() -> u32 {
      T::MAX_REQUESTS_PER_CONTEXT
    }
  }
}

#[frame_support::pallet]
pub mod pallet {
  use crate::{
    types::{offchain::*, *},
    weights::WeightInfo,
    ONCHAIN_TX_KEY,
  };
  use core::convert::TryInto;
  use frame_support::{
    pallet_prelude::*,
    sp_std::{vec::*, *},
    traits::{Currency, ReservableCurrency},
  };

  use crate::consts::*;
  use frame_support::traits::tokens::BalanceStatus;
  use frame_system::{offchain::SendTransactionTypes, pallet_prelude::*};
  use sp_io::offchain_index;
  use sp_runtime::offchain::storage::StorageValueRef;

  pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

  #[deny(clippy::clone_double_ref)]
  fn derived_key<T: Config>(block_number: T::BlockNumber) -> Vec<u8> {
    block_number.using_encoded(|encoded_bn| {
      ONCHAIN_TX_KEY
        .iter()
        .chain(b"/".iter())
        .chain(encoded_bn)
        .copied()
        .collect::<Vec<u8>>()
    })
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    /// Validate unsigned call to this module.
    ///
    /// By default unsigned transactions are disallowed, but implementing the validator
    /// here we make sure that some particular calls (the ones produced by offchain worker)
    /// are being whitelisted and marked as valid.
    fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      // We allow calls only from the local OCW engine.
      match source {
        TransactionSource::Local | TransactionSource::InBlock => {}
        _ => return InvalidTransaction::Call.into(),
      }

      let valid_tx = |provide| {
        ValidTransaction::with_tag_prefix("verification")
          .priority(1)
          .and_provides([&provide])
          .longevity(3)
          .propagate(true)
          .build()
      };

      match call {
        Call::submit_verification_status { verification_data: _ } => valid_tx(b"submit_verification_status".to_vec()),
        _ => InvalidTransaction::Call.into(),
      }
    }
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: SendTransactionTypes<Call<Self>> + frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// The key generator used to produce the verification key
    type VerificationKeyGenerator: VerificationKeyGenerator<Self>;

    /// The callback that allows to perform some operation when a verification turns out to be no
    /// longer valid
    type VerificationInvalidator: VerificationInvalidator<Self>;

    /// Currency that allows to lock the registration fee
    type Currency: ReservableCurrency<Self::AccountId>;

    /// The amount to pay in order to issue a verification
    const REGISTRATION_FEE: BalanceOf<Self>;

    /// The maximum number of accounts requesting verification of the same context
    const MAX_REQUESTS_PER_CONTEXT: u32;
  }

  #[pallet::extra_constants]
  impl<T: Config> Pallet<T> {
    /// The amount to pay in order to issue a verification. It will be refunded to any account that
    /// update the verification status to invalid. Thus, the higher the amount, the more
    /// incentive for external actors to validate the verification status and for the holder
    /// to keep the verification status valid; the lower the amount, the more encouraging to issue a
    /// verification.
    #[pallet::constant_name(RegistrationFee)]
    fn registration_fee() -> BalanceOf<T> {
      T::REGISTRATION_FEE
    }
    /// The maximum number of accounts requesting verification for the same context. It should be
    /// high enough to prevent a malicious actor controlling several accounts to lock the
    /// verification of a given context to failed status
    #[pallet::constant_name(MaxRequestsPerContext)]
    fn max_requests_per_context() -> u32 {
      T::MAX_REQUESTS_PER_CONTEXT
    }
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn integrity_test() {
      assert!(
        T::REGISTRATION_FEE > 0u32.into(),
        "`RegistrationFee` must be greater than 0"
      );
      assert!(
        T::MAX_REQUESTS_PER_CONTEXT > 0,
        "`MaxRequestsPerContext` must be greater than 0"
      );
    }
    fn offchain_worker(block_number: T::BlockNumber) {
      let key = derived_key::<T>(block_number);
      let oci_mem = StorageValueRef::persistent(&key);

      if let Ok(Some(indexing_data)) = oci_mem.get::<VerificationIndexingData<T>>() {
        let _res = Self::process_pending_verification(indexing_data);
      }
    }
  }

  /// The map of account id of the holder indexed by the verification contexts of the
  /// correspondent verification request
  #[pallet::storage]
  #[pallet::getter(fn account_ids_by_verification_context)]
  pub type AccountIdsByVerificationContext<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    VerificationContext,
    BoundedVec<T::AccountId, MaxVerificationRequestsPerContextGet<T>>,
    ValueQuery,
  >;

  /// The map of verification requests indexed by the account id of the holder and the
  /// associated verification context
  #[pallet::storage]
  #[pallet::getter(fn verification_request_by_account_id_and_verification_context)]
  pub type VerificationRequestByAccountIdAndVerificationContext<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Twox64Concat,
    VerificationContext,
    VerificationRequest<T::AccountId>,
    OptionQuery,
  >;

  #[pallet::error]
  pub enum Error<T> {
    /// The VerificationContext is submitted twice, no matter the VerificationStatus
    VerificationAlreadyIssued,
    /// The holder can't afford to reserve the amount requested for the verification registration
    /// fee
    CannotReserveRegistrationFee,
    /// The verification key generation failed
    VerificationKeyGenerationError,
    /// The verification invalidation callback failed
    VerificationInvalidationError,
    /// No registered [`VerificationStrategy'] could match the request
    NoMatchingVerificationStrategy,
    /// The [`VerificationRequest'] is expected to be stored for the given [`VerificationContext`]
    /// but none could be found
    NoSuchVerificationRequest,
    /// The off-chain worker encountered an error while attempting verification
    OffChainVerificationError,
    /// Some processing was attempted on a ['VerificationRequest`] which has an inappropriate status
    InvalidVerificationStatus,
    /// There are already a number of accounts attempting to verify the same context and no more
    /// will be accepted
    MaxVerificationRequestsPerContextLimitReached,
  }

  /// Events of the Poe pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Produced upon newly requested verification to communicate to the holder the key to use for
    /// the agreed action or that the verification is ongoing
    VerificationRequested(T::AccountId, VerificationRequest<T::AccountId>),
    /// Produced upon successful verification
    VerificationSuccessful(T::AccountId, VerificationRequest<T::AccountId>),
    /// Produced upon failed verification, intended to be received by both the verifier and the
    /// holder, also provides a textual explaination of what went wrong
    VerificationFailed(T::AccountId, T::AccountId, VerificationRequest<T::AccountId>, Bytes),
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Accepts a [`VerificationContext`] and a desired [`VerificationAction`] and
    /// produces the information for the holder about the action to take in order for the
    /// verification to succeed. The registration fee is reserved on the holder funds: it will be
    /// possible to claim this amount back later.
    ///
    /// A [`VerificationRequest`] is initialized, iterating through all known
    /// [`VerificationStrategy`] in order to find the one that supports the
    /// [`VerificationContext`] the [`VerificationRequest`] is stored in.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * context - the [`VerificationContext`]
    /// * action - the [`VerificationAction`]
    ///
    /// # Errors
    /// * `VerificationAlreadyIssued` - if a request for the same context was already created by the
    ///   caller or by another user
    /// * `CannotReserveRegistrationFee` - if the holder does not have enough funds to reserve the
    ///   required registration fee
    /// * `NoMatchingVerificationStrategy` - if none of the registered verification strategies is
    ///   suitable to respond to the request
    /// * `MaxVerificationRequestsPerContextLimitReached` - if the maximum number of verification
    ///   requests has already been submitted for this context
    ///
    /// # Events
    /// * `VerificationRequested` - having `Waiting` status and providing further verification
    ///   instructions
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::request_verification())]
    pub fn request_verification(
      origin: OriginFor<T>,
      context: VerificationContext,
      action: VerificationAction,
    ) -> DispatchResultWithPostInfo {
      let holder = ensure_signed(origin)?;

      // Ensure a verification request for same context and holder is not contained in storage, or it is
      // failed
      let existing = VerificationRequestByAccountIdAndVerificationContext::<T>::get(holder.clone(), context.clone());
      ensure!(
        existing.is_none() || matches!(existing.unwrap().status, VerificationStatus::Failure(_)),
        Error::<T>::VerificationAlreadyIssued
      );

      let strategy = Self::find_strategy(&context, &action).ok_or(Error::<T>::NoMatchingVerificationStrategy)?;

      // Reserve the registration fee on the holder account
      T::Currency::reserve(&holder, T::REGISTRATION_FEE).map_err(|_| Error::<T>::CannotReserveRegistrationFee)?;

      // Use the strategy to create a new pending request
      let request = strategy.new_request(holder.clone(), context.clone(), action)?;
      VerificationRequestByAccountIdAndVerificationContext::<T>::insert(
        holder.clone(),
        context.clone(),
        request.clone(),
      );
      AccountIdsByVerificationContext::<T>::try_mutate(context, |stored_accounts| {
        // Insert the account of the holder only once even if a failed request is resubmitted
        if !stored_accounts.iter().any(|stored_account| *stored_account == holder) {
          stored_accounts
            .try_push(holder.clone())
            .map_err(|_err| Error::<T>::MaxVerificationRequestsPerContextLimitReached)
        } else {
          Ok(())
        }
      })?;

      // Emit an event that the verification request is awaiting action
      Self::deposit_event(Event::VerificationRequested(holder, request));

      Ok(().into())
    }

    /// Accepts a [`VerificationRequest`] that may contain some `id` and signals that the holder has
    /// taken the appropriate action in order for the verification to succeed. The respective
    /// VerificationRequest from VerificationRequestByContext is stored in the off-chain
    /// worker indexing database with the status `Pending`. As soon as the off-chain worker runs, it
    /// finds the pending request in the off-chain worker indexing database and instantiates the
    /// required strategy to perform the verification, which depends on the
    /// specific implementation. At this point, an unsigned local transaction is submitted to
    /// `submit_verification_status()`, passing the VerificationStatus.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * request - the [`VerificationRequest`] returned by `request_verification`, optionally
    ///   augmented with some value for the `id` field
    ///
    /// # Errors
    /// * `NoSuchVerificationRequest` - if the request context is not associated to any stored
    ///   [`VerificationRequest`]
    /// * `NoMatchingVerificationStrategy` - if none of the registered verification strategies is
    ///   suitable to respond to the request
    ///
    /// # Events
    /// * `VerificationRequested` - having `Pending` status and awaiting to be processed off-chain
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::perform_verification())]
    pub fn perform_verification(
      origin: OriginFor<T>,
      request: VerificationRequest<T::AccountId>,
    ) -> DispatchResultWithPostInfo {
      let verifier = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      // Ensure that the stored request exists and augment it with some value passed in the `id` field,
      // plus Pending status
      let stored_request = VerificationRequestByAccountIdAndVerificationContext::<T>::try_mutate(
        request.holder.clone(),
        request.context.clone(),
        |stored_request| match stored_request {
          Some(stored_request) => {
            stored_request.id = request.id.clone();
            match stored_request.status {
              // If the Verification has previously failed, it must go through request_verification() again to pay the
              // registration fee
              VerificationStatus::Failure(_) => Err(Error::<T>::InvalidVerificationStatus),
              _ => {
                stored_request.status = VerificationStatus::Pending;
                Ok(stored_request.clone())
              }
            }
          }
          None => Err(Error::<T>::NoSuchVerificationRequest),
        },
      )?;

      // Insert the request in the off-chain indexed database for further processing by the off-chain
      // worker
      let key = derived_key::<T>(current_block);
      let data = VerificationIndexingData::<T> {
        verifier: verifier.clone(),
        request: stored_request.clone(),
      };
      offchain_index::set(&key, &data.encode());

      // Emit an event that the verification request is pending processing
      Self::deposit_event(Event::VerificationRequested(verifier, stored_request));

      Ok(().into())
    }

    /// Accepts a [`VerificationIndexingData`] from an unsigned local transaction submitted by
    /// the off-chain worker. This will unreserve the registration fee of the holder, and will try
    /// to transfer it to the verifier if they are not the same account.
    ///
    /// # Arguments
    /// * origin - the None origin
    /// * verification_data - the [`VerificationIndexingData`] structure updated by the off-chain
    ///   worker
    ///
    /// # Errors
    /// * `NoSuchVerificationRequest` - if the request context is not associated to any stored
    ///   [`VerificationRequest`]
    /// * `InvalidVerificationStatus` - if the request does not have status `Success` or `Failure`
    ///
    /// # Events
    /// * `VerificationSuccessful` - for the verifier account to indicate that his verification
    ///   request was successful
    /// * `VerificationFailed` - for the verifier account and for the holder account to indicate the
    ///   verification is no longer valid and the registration fee has been claimed
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(0)]
    pub fn submit_verification_status(
      origin: OriginFor<T>,
      verification_data: VerificationIndexingData<T>,
    ) -> DispatchResultWithPostInfo {
      ensure_none(origin)?;

      // Mutate the request in the store with the status obtained from the verification data
      let stored_request = VerificationRequestByAccountIdAndVerificationContext::<T>::try_mutate(
        verification_data.request.holder.clone(),
        verification_data.request.context.clone(),
        |stored_request| match stored_request {
          Some(stored_request) => {
            stored_request.status = verification_data.request.status.clone();
            Ok(stored_request.clone())
          }
          None => Err(Error::<T>::NoSuchVerificationRequest),
        },
      )?;

      // Notify of the updated verification status. In case of failure, the verifier also rescues the
      // registration fee
      match &stored_request.status {
        VerificationStatus::Success => {
          // Emit an event that the verification is successful
          Self::deposit_event(Event::VerificationSuccessful(
            verification_data.verifier,
            stored_request,
          ));
          Ok(())
        }
        VerificationStatus::Failure(error_msg) => {
          let verifier = verification_data.verifier;
          let holder = stored_request.holder.clone();

          // Unreserve funds of the holder and, if he's not the same account as the verifier, transfer the
          // registration fee to the latter
          if verifier == holder {
            T::Currency::unreserve(&holder, T::REGISTRATION_FEE);
          } else {
            T::Currency::repatriate_reserved(&holder, &verifier, T::REGISTRATION_FEE, BalanceStatus::Free).map_err(
              |err| {
                // Failed to transfer the registration fee to the verifier. It will be unreserved for the holder in
                // any case
                T::Currency::unreserve(&holder, T::REGISTRATION_FEE);
                err
              },
            )?;
          }

          // Callback to the generator in order to notify the invalidation of the key
          T::VerificationInvalidator::invalidate(&stored_request)?;

          // Emit an event that the verification is failed
          Self::deposit_event(Event::VerificationFailed(
            verifier,
            holder,
            stored_request.clone(),
            error_msg.clone(),
          ));
          Ok(())
        }
        _ => Err(Error::<T>::InvalidVerificationStatus),
      }?;

      Ok(().into())
    }
  }
}
