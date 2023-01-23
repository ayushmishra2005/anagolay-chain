// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

use super::*;
use crate::{
  strategies::*,
  types::{VerificationAction, VerificationContext, VerificationRequest, VerificationStatus, VerificationStrategy},
};
use frame_support::sp_std::{vec, vec::Vec};

/// Internal implementation of the verification pallet
impl<T: Config> Pallet<T> {
  /// Collect all verification strategies and filter them by the given arguments to find the one
  /// that supports them
  ///
  /// # Arguments
  /// * context - the [`VerificationContext`]
  /// * action - the [`VerificationAction`]
  ///
  /// # Return
  /// An VerificationStrategy that passed the filter if some, none otherwise
  pub fn find_strategy(
    context: &VerificationContext,
    action: &VerificationAction,
  ) -> Option<impl VerificationStrategy<Config = T>> {
    // Collect all verification strategies. For now we only have the dns verification strategy
    let dns_verification_strategy = DnsVerificationStrategy::<T>::default();
    vec![dns_verification_strategy]
      .iter()
      .find(|s| s.supports(context, action))
      .cloned()
  }

  /// Get a subset of [`VerificationRequest`] representing a page, given the full set of the
  /// [`VerificationContext`] to paginate, a filter on the request status and on the holder
  /// account and the pagination information
  ///
  /// # Arguments
  ///  * request_contexts - The full set of [`VerificationContext`]. If empty, all
  ///    [`VerificationRequest`] will be considered
  ///  * status - Additional filter on the status of the requests
  ///  * account - Additional filter on the holder account
  ///  * offset - The index, inside the ids set, of the first Operation on the page
  ///  * limit - The count of Operations on the page
  ///
  /// # Return
  /// Collection of [`VerificationRequest`]
  pub fn get_requests(
    request_contexts: Vec<VerificationContext>,
    status: Option<VerificationStatus>,
    account: Option<T::AccountId>,
    offset: u64,
    limit: u16,
  ) -> Vec<VerificationRequest<T::AccountId>> {
    let mut requests = Vec::new();

    // Retrieve the contexts matching the filters. If an account filter is specified filter out
    // immediately the requerst not held by such account for better performance
    let request_contexts = if request_contexts.is_empty() {
      let mut contexts = Vec::new();
      VerificationRequestByAccountIdAndVerificationContext::<T>::iter_keys().for_each(|(k1, k2)| {
        let matches = match account.clone() {
          Some(account) => k1 == account,
          _ => true,
        };
        if matches {
          contexts.push(k2)
        };
      });
      contexts
    } else {
      request_contexts
    };

    // Pagination offset
    let (_, request_contexts) = request_contexts.split_at(offset as usize);

    for request_context in request_contexts.iter() {
      // Pagination limit
      if requests.len() >= limit as usize {
        break;
      }

      // A request may be attempted by several accounts altoghether. Add those that match the filter
      let holders = AccountIdsByVerificationContext::<T>::get(request_context);
      for holder in holders {
        let request: Option<VerificationRequest<T::AccountId>> =
          VerificationRequestByAccountIdAndVerificationContext::<T>::get(holder, request_context);

        match (request, status.clone(), account.clone()) {
          (Some(request), None, None) => {
            if !requests.contains(&request) {
              requests.push(request)
            }
          }
          (Some(request), Some(status), None) => {
            if !requests.contains(&request) && request.status == status {
              requests.push(request)
            }
          }
          (Some(request), None, Some(account)) => {
            if !requests.contains(&request) && request.holder == account {
              requests.push(request)
            }
          }
          (Some(request), Some(status), Some(account)) => {
            if !requests.contains(&request) && request.status == status && request.holder == account {
              requests.push(request)
            }
          }
          _ => (),
        }
      }
    }

    requests
  }
}
