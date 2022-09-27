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

use frame_support::sp_std::vec::Vec;
use workflows::types::{Workflow, WorkflowId, WorkflowVersion, WorkflowVersionId};

sp_api::decl_runtime_apis! {
    /// Workflows RPC Api
    pub trait WorkflowsApi {
        /// Get a subset of Workflows representing a page, given the full set of the ids to paginate
        /// and the pagination information
        ///
        /// # Arguments
        ///  * workflow_ids - The full set of WorkflowIds. If empty, all Workflows will be considered
        ///  * offset - The index, inside the ids set, of the first Workflow on the page
        ///  * limit - The count of Workflows on the page
        ///
        /// # Return
        /// Collection of Workflows
        fn get_workflows_by_ids (
            workflow_ids: Vec<WorkflowId>,
            offset: u64,
            limit: u16,
        ) -> Vec<Workflow>;

        /// Get a subset of WorkflowVersions representing a page, given the full set of the ids to paginate
        /// and the pagination information
        ///
        /// # Arguments
        ///  * version_ids - The full set of WorkflowVersionIds. If empty, all WorkflowVersions will be considered
        ///  * offset - The index, inside the ids set, of the first Workflow on the page
        ///  * limit - The count of Workflows on the page
        ///
        /// # Return
        /// Collection of WorkflowVersions
        fn get_workflow_versions_by_ids (
            workflow_versions: Vec<WorkflowVersionId>,
            offset: u64,
            limit: u16,
        ) -> Vec<WorkflowVersion>;
    }
}
