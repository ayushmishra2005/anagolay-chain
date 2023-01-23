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

use frame_support::sp_std::vec::Vec;
use operations::types::{Operation, OperationId, OperationVersion, OperationVersionId};

sp_api::decl_runtime_apis! {
    /// Operations RPC Api
    pub trait OperationsApi {
        /// Get a subset of Operations representing a page, given the full set of the ids to paginate
        /// and the pagination information
        ///
        /// # Arguments
        ///  * operation_ids - The full set of OperationIds. If empty, all Operations will be considered
        ///  * offset - The index, inside the ids set, of the first Operation on the page
        ///  * limit - The count of Operations on the page
        ///
        /// # Return
        /// Collection of Operations
        fn get_operations_by_ids (
            operation_ids: Vec<OperationId>,
            offset: u64,
            limit: u16,
        ) -> Vec<Operation>;

        /// Get a subset of OperationVersions representing a page, given the full set of the ids to paginate
        /// and the pagination information
        ///
        /// # Arguments
        ///  * version_ids - The full set of OperationVersionIds. If empty, all OperationVersions will be considered
        ///  * offset - The index, inside the ids set, of the first Operation on the page
        ///  * limit - The count of Operations on the page
        ///
        /// # Return
        /// Collection of OperationVersions
        fn get_operation_versions_by_ids (
            operation_ids: Vec<OperationVersionId>,
            offset: u64,
            limit: u16,
        ) -> Vec<OperationVersion>;
    }
}
