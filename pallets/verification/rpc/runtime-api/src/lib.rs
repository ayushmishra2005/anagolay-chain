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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use core::fmt::Debug;
use frame_support::sp_std::vec::Vec;
use verification::types::*;

sp_api::decl_runtime_apis! {
    /// Verification RPC Api
    ///
    /// # Type arguments
    /// - AccountId: the `AccountId` from the runtime `Config`
    pub trait VerificationApi<AccountId: Debug + Decode> {
        /// Get a subset of [`VerificationRequest`] representing a page, given the full set of the [`VerificationContext`] to paginate,
        /// a filter on the request status and the pagination information
        ///
        /// # Arguments
        ///  * contexts - The full set of [`VerificationContext`]. If empty, all [`VerificationRequest`] will be considered
        ///  * offset - The index, inside the ids set, of the first Operation on the page
        ///  * limit - The count of Operations on the page
        ///
        /// # Return
        /// Collection of [`VerificationRequest`]
        fn get_requests (
            contexts: Vec<VerificationContext>,
            status: Option<VerificationStatus>,
            offset: u64,
            limit: u16,
        ) -> Vec<VerificationRequest<AccountId>>;
    }
}
