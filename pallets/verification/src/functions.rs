// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2022 Anagolay Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
