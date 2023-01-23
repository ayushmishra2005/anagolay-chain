// This file is part of Anagolay Network.
// Copyright (C) 2019-2023 Anagolay Network.
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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::sp_std::vec::Vec;
use tipping::types::*;
use verification::types::VerificationContext;

sp_api::decl_runtime_apis! {
    /// Tipping RPC Api
    ///
    /// # Type arguments
    /// - Balance: the `Balance` from the runtime `Config` (in, out)
    /// - AccountId: the `AccountId` from the runtime `Config` (in, out)
    /// - BlockNumber: the `BlockNumber` from the runtime `Config` (out)
    pub trait TippingApi<Balance: Debug + Encode + Decode, AccountId: Debug + Encode + Decode, BlockNumber: Debug + Decode> {

      /// Get the total balance of tips received for a [`VerificationContext`]
      ///
      /// # Arguments
      ///  * account_id - The holder of a successful [`VerificationRequest`] for the verification context
      ///  * verification_context - The [`VerificationContext`] to query
      ///
      /// # Return
      /// Total balance, sum of all [`Tip`]s for the specified verification context
      fn total_received(account_id: AccountId, verification_context: VerificationContext) -> Balance;

      /// Get the count of tips for a [`VerificationContext`]
      ///
      /// # Arguments
      ///  * account_id - The holder of a successful [`VerificationRequest`] for the verification context
      ///  * verification_context - The [`VerificationContext`] to query
      ///
      /// # Return
      /// Count of [`Tip`]s for the specified verification context
      fn total(account_id: AccountId, verification_context: VerificationContext) -> u64;

      /// Get the tips for an Account and a [`VerificationContext`]
      ///
      /// # Arguments
      ///  * account_id - The account to query
      ///  * verification_context - The [`VerificationContext`] to query
      ///  * offset - The index, inside the ids set, of the first Tip on the page
      ///  * limit - The count of Tips on the page
      ///
      /// # Return
      /// Collection of [`Tip`]s sorted by createdAt DESC
      fn get_tips (
        account_id: AccountId,
        verification_context: VerificationContext,
        offset: u64,
        limit: u16, // why this one doesn't have at param?
      ) -> Vec<Tip<Balance, AccountId, BlockNumber>>;
    }
}
