// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

use super::*;
use crate::{
  strategies::*,
  types::{VerificationAction, VerificationContext, VerificationStrategy},
};
use frame_support::sp_std::vec;

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
}
