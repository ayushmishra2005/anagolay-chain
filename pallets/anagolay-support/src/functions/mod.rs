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

//! Business logic is located here.
//!
//! Each pallet must have this file.

/// Package that contains support functions to export
pub mod public;
pub use public::*;

use super::constants::*;
use frame_support::{
  sp_std::{vec, vec::Vec},
  BoundedVec,
};

impl<T: Config> Pallet<T> {
  /// Produces a CID v1 out of some bytes using an Anagolay workflow
  ///
  /// # Arguments
  /// * holder - The verification holder (unused)
  /// * context - The verification context (unused)
  /// * identifier - The identifier to use to compute the CID
  ///
  /// # Return
  /// The CID string ("bafk...") in the form of a collection of bytes
  pub fn produce_cid(bytes: Vec<u8>) -> Vec<u8> {
    extern crate alloc;
    use alloc::rc::Rc;
    use core::any::Any;

    let inputs: Vec<Rc<dyn Any>> = vec![Rc::new(bytes)];
    let workflow = wf_cidv1_from_array::Workflow::new();
    if let Ok(result) = workflow.next(inputs) {
      let result = result.as_ref();
      let cid_str = result
        .get_output()
        .unwrap()
        .downcast_ref::<alloc::string::String>()
        .unwrap()
        .clone();
      cid_str.as_bytes().to_vec()
    } else {
      "".as_bytes().to_vec()
    }
  }

  /// Verifies if the package passed as argument is already stored
  ///
  /// # Arguments
  ///  * artifact - The artifact to check
  ///
  /// # Return
  /// True if the artifact is already stored, false otherwise
  pub fn is_existing_artifact(artifact: &AnagolayArtifactStructure<impl ArtifactType>) -> bool {
    match ArtifactsByArtifactId::<T>::try_get() {
      Ok(artifacts) => artifacts.contains(&artifact.ipfs_cid),
      _ => false,
    }
  }

  /// Store all artifacts passed as parameter.
  ///
  /// # Arguments
  ///  * artifacts - The artifacts to store
  ///
  /// # Return
  /// An [`Result`] having an unit-type `Ok` if all insertion succeeded or unit-type `Err` if any
  /// insertion failed
  pub fn store_artifacts(
    artifacts: &BoundedVec<AnagolayArtifactStructure<impl ArtifactType>, MaxArtifactsPerVersionGet>,
  ) -> Result<(), Error<T>> {
    ArtifactsByArtifactId::<T>::try_mutate(|stored_artifacts| {
      artifacts.iter().try_for_each(|artifact| {
        stored_artifacts
          .try_push(artifact.ipfs_cid.clone())
          .map_err(|_err| Error::<T>::MaxArtifactsLimitReached)
      })
    })
  }

  /// Retrieve all stored artifacts
  ///
  /// # Return
  /// Collection of artifact Ids
  pub fn get_artifacts() -> BoundedVec<ArtifactId, MaxArtifactsGet<T>> {
    ArtifactsByArtifactId::<T>::get()
  }
}
