// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2021 Anagolay Foundation.
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

use anagolay::{CreatorId, ForWhat, GenericId};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

/// Rule which must be applied
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
  pub id: GenericId,
  pub data: RuleData,
}

impl Default for Rule {
  fn default() -> Self {
    Rule {
      id: b"".to_vec(),
      data: RuleData::default(),
    }
  }
}

///OperationReference by id instead of full
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationReference {
  id: GenericId,
  children: Vec<OperationReference>,
}

/// Rule Data, use this to generate rule_id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleData {
  /// Version, maybe we remove this, we will see
  pub version: u32,
  /// max 128(0.12kb) characters, slugify to use _
  pub name: Vec<u8>,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
  pub desc: Vec<u8>,
  pub creator: CreatorId,
  pub groups: Vec<ForWhat>,
  pub parent_id: GenericId,
  pub ops: Vec<OperationReference>,
}

impl Default for RuleData {
  fn default() -> Self {
    RuleData {
      version: 1,
      name: b"".to_vec(),
      desc: b"".to_vec(),
      creator: CreatorId::default(),
      groups: vec![ForWhat::default()],
      parent_id: b"".to_vec(),
      ops: vec![],
    }
  }
}
