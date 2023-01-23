// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

use super::*;
use crate::types::{offchain::*, *};
use frame_support::log::error;
use frame_system::offchain::SubmitTransaction;

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
  ///   [`VerificationIndexingData`] has an unexpected status different from `Pending`
  ///
  /// # Return
  /// A result which is the unit type in case of success, or one of the pallet errors otherwise
  pub fn process_pending_verification(indexing_data: VerificationIndexingData<T>) -> Result<(), Error<T>> {
    let mut verification_data = indexing_data.clone();
    let request = indexing_data.request;
    match request.status {
      VerificationStatus::Pending => {
        // At this point we are sure that a strategy supporting this request exists
        if let Some(strategy) = Self::find_strategy(&request.context, &request.action) {
          match strategy.verify(&request) {
            Ok(status) => {
              verification_data.request.status = status;
              let call = Call::submit_verification_status { verification_data };
              SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()).map_err(|_err| {
                error!("Failed call to submit_verification_status() by off-chain worker");
                Error::<T>::OffChainVerificationError
              })
            }
            Err(err) => {
              error!("Error in verification process for request {:?}: {:?}", request, err);
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
