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

//! Autogenerated weights for workflows
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-06, STEPS: `50`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `anagolay-anagolay-yq922khj19q`, CPU: `AMD EPYC 7B13`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/anagolay
// benchmark
// pallet
// --chain
// dev
// --steps
// 50
// --repeat
// 100
// --pallet
// workflows
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --output
// ./pallets/workflows/src/weights.rs
// --template
// ./templates/module-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
  sp_std::marker::PhantomData,
  traits::Get,
  weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for workflows.
pub trait WeightInfo {
  fn create() -> Weight;
}

/// Weights for workflows using the Substrate node and recommended hardware.
pub struct AnagolayWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AnagolayWeight<T> {
  // Storage: Workflows WorkflowByWorkflowIdAndAccountId (r:1 w:1)
  // Storage: Workflows VersionIdsByWorkflowId (r:1 w:1)
  // Storage: Anagolay ArtifactsByArtifactId (r:1 w:1)
  // Storage: Workflows Total (r:1 w:1)
  // Storage: Timestamp Now (r:1 w:0)
  // Storage: Workflows VersionByVersionId (r:0 w:1)
  fn create() -> Weight {
    // Minimum execution time: 75_750 nanoseconds.
    Weight::from_ref_time(86_151_000)
      .saturating_add(T::DbWeight::get().reads(5))
      .saturating_add(T::DbWeight::get().writes(5))
  }
}

// For backwards compatibility and tests
impl WeightInfo for () {
  // Storage: Workflows WorkflowByWorkflowIdAndAccountId (r:1 w:1)
  // Storage: Workflows VersionIdsByWorkflowId (r:1 w:1)
  // Storage: Anagolay ArtifactsByArtifactId (r:1 w:1)
  // Storage: Workflows Total (r:1 w:1)
  // Storage: Timestamp Now (r:1 w:0)
  // Storage: Workflows VersionByVersionId (r:0 w:1)
  fn create() -> Weight {
    // Minimum execution time: 75_750 nanoseconds.
    Weight::from_ref_time(86_151_000)
      .saturating_add(RocksDbWeight::get().reads(5))
      .saturating_add(RocksDbWeight::get().writes(5))
  }
}
