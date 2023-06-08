// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

use super::*;
use crate::types::*;
use core::{cmp::Ordering, convert::TryInto};
use frame_support::sp_std::vec::Vec;

use verification::types::VerificationContext;

/// Internal implementation of the tipping pallet
impl<T: Config> Pallet<T> {
  /// Get the total balance of tips received for a [`VerificationContext`]
  ///
  /// # Arguments
  ///  * holder - The holder of a successful [`VerificationRequest`] for the verification context
  ///  * verification_context - The [`VerificationContext`] to query
  ///
  /// # Return
  /// Total balance, sum of all [`Tip`]s for the specified verification context
  pub fn total_received(holder: T::AccountId, verification_context: VerificationContext) -> BalanceOf<T> {
    TipsByAccountIdAndVerificationContext::<T>::get(holder, verification_context)
      .iter()
      // Review potential overflow situations where operators +, -, * and / are used.
      // If operations are safe it is a good practice to add a comment about why.
      // Use `checked_add`, `checked_sub`, `checked_mul`, `checked_div`, instead.
      .fold(0u32.into(), |acc, tip| acc + tip.amount)
  }

  /// Get the count of tips for a [`VerificationContext`]
  ///
  /// # Arguments
  ///  * holder - The holder of a successful [`VerificationRequest`] for the verification context
  ///  * verification_context - The [`VerificationContext`] to query
  ///
  /// # Return
  /// Count of [`Tip`]s for the specified verification context
  pub fn total(holder: T::AccountId, verification_context: VerificationContext) -> u64 {
    TipsByAccountIdAndVerificationContext::<T>::get(holder, verification_context)
      .iter()
      .count()
      .try_into()
      .unwrap_or(0u64)
  }

  /// Get the tips for an Account and a [`VerificationContext`]
  ///
  /// # Arguments
  ///  * account_id - The account to query
  ///  * context - The [`VerificationContext`] to query
  ///  * offset - The index, inside the ids set, of the first Tip on the page. Default is 0
  ///  * limit - The count of Tips on the page. Default is 100
  ///
  /// # Return
  /// Collection of [`Tip`]
  pub fn get_tips(
    account_id: T::AccountId,
    context: VerificationContext,
    offset: u64,
    limit: u16,
  ) -> Vec<Tip<BalanceOf<T>, T::AccountId, T::BlockNumber>> {
    let mut page = Vec::new();
    let mut tips = TipsByAccountIdAndVerificationContext::<T>::get(account_id, context);
    tips.sort_by(|a, b| b.created_at.partial_cmp(&a.created_at).unwrap_or(Ordering::Equal));
    tips
      .iter()
      .skip(offset.try_into().unwrap_or(0))
      .take(limit.try_into().unwrap_or(100))
      .for_each(|tip| page.push(tip.clone()));

    page
  }
}
