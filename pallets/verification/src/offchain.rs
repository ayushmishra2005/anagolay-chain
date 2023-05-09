// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

use super::*;
use crate::types::{offchain::*, *};
use frame_support::log;
use frame_system::offchain::{SendUnsignedTransaction, Signer};

/// Internal implementation of the verification pallet, this module is grouping the off-chain
/// functionality
impl<T: Config> Pallet<T> {
  /// Offchain process of `VerificationIndexData`. Runs only for `Pending` requests and
  /// invokes the appropriate verification strategy
  ///
  /// # Arguments
  /// * indexing_data - The indexing data of the current block
  ///
  /// # Errors
  /// * `OffChainVerificationError` - The strategy verification failed, or it was impossible to call
  ///   the runtime
  /// * `NoMatchingVerificationStrategy` - If none of the registered verification strategies is
  ///   suitable to respond to the request
  /// * `InvalidVerificationStatus` - The ['VerificationRequest] contained in the
  ///   [`VerificationIndexingInputData`] has an unexpected status different from `Pending`
  ///
  /// # Return
  /// A result which is the unit type in case of success, or one of the pallet errors otherwise
  pub fn process_pending_verification(
    indexing_data: VerificationIndexingInputData<T::AccountId>,
  ) -> Result<(), Error<T>> {
    let request = indexing_data.request;
    let verifier = indexing_data.verifier;
    match request.status {
      VerificationStatus::Pending => {
        // At this point we are sure that a strategy supporting this request exists
        if let Some(strategy) = Self::find_strategy(&request.context, &request.action) {
          match strategy.verify(&request) {
            Ok(status) => {
              // Retrieve the signer to sign the payload
              let signer = Signer::<T, T::AuthorityId>::any_account();

              // `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
              //	 The returned result means:
              // 	 - `None`: no account is available for sending transaction
              // 	 - `Some((account, Ok(())))`: transaction is successfully sent
              // 	 - `Some((account, Err(())))`: error occurred when sending the transaction
              if let Some((_, res)) = signer.send_unsigned_transaction(
                // this line is to prepare and return payload
                |acct| {
                  // Update payload with status and signature
                  let mut verification_data = VerificationIndexingOutputData {
                    verifier: verifier.clone(),
                    request: request.clone(),
                    public: acct.public.clone(),
                  };
                  verification_data.request.status = status.clone();
                  verification_data
                },
                |verification_data, signature| Call::submit_verification_status {
                  verification_data,
                  signature,
                },
              ) {
                res.map_err(|_| Error::<T>::OffChainVerificationError)
              } else {
                // The case of `None`: no account is available for sending
                log::error!("No local accounts available. Consider adding one via `author_insertKey` RPC.");
                Err(Error::<T>::OffChainVerificationError)
              }
            }
            Err(err) => {
              log::error!("Error in verification process for request {:?}: {:?}", request, err);
              Err(Error::<T>::OffChainVerificationError)
            }
          }
        } else {
          Err(Error::<T>::NoMatchingVerificationStrategy)
        }
      }
      _ => Err(Error::<T>::InvalidVerificationStatus),
    }
  }
}
