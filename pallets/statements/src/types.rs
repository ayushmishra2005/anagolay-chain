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

use anagolay::{CreatorId, GenericId};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec::Vec};

/// Anagolay Signature
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolaySignature {
  /// signing key in urn/did format 'urn:pgp:9cdf8dd38531511968c8d8cb524036585b62f15b'
  pub sig_key: Vec<u8>,
  /// Signature sign(prepared_statement, pvtKey(sigKey)) and encoded using multibase
  /// https://gitlab.com/sensio_group/sensio-faas/-/blob/master/sp-api/src/plugins/copyright/helpers.ts#L76
  pub sig: Vec<u8>,
  /// Content identifier of the sig field -- CID(sig)
  pub cid: GenericId,
}

/// Anagolay Signatures
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolaySignatures {
  pub holder: AnagolaySignature,
  pub issuer: AnagolaySignature,
}
/// Anagolay Claim Proportion
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Proportion {
  /// Proportion sign, can be %
  pub sign: Vec<u8>,
  pub name: Vec<u8>,
  pub value: Vec<u8>,
}
/// Anagolay Validity
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Validity {
  /// When the validity starts, this should be DATE_TIME
  pub from: Vec<u8>,
  /// When validity ends, this is calculate Validity.from + Expiration.value
  pub until: Vec<u8>,
}

/// Possible Expiration types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ExpirationType {
  Forever,
  Years,
  Months,
  Days,
  Minutes,
  Seconds,
}

impl Default for ExpirationType {
  fn default() -> Self {
    ExpirationType::Forever
  }
}

/// Anagolay Claim Expiration
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Expiration {
  ///Possible Expiration types
  pub expiration_type: ExpirationType,
  ///How long is the expiration, if  ExpirationType::FOREVER then this is empty
  pub value: Vec<u8>,
}

/// Anagolay Claim types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum AnagolayClaimType {
  Copyright,
  Ownership,
}

impl Default for AnagolayClaimType {
  fn default() -> Self {
    AnagolayClaimType::Copyright
  }
}

/// Anagolay Generic Claim
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolayClaim {
  /// Prev Anagolay Statement id in case this statement is revoked or changed
  pub prev_id: GenericId,
  /// PoE id of the record in question.
  pub poe_id: GenericId,
  /// Implemented rule
  pub rule_id: GenericId,
  /// In which proportion the statement is held
  pub proportion: Proportion,
  /// ATM this is the same as poe_id @TODO this should be unique representation of the subject that is NOT poe
  pub subject_id: GenericId,
  /// ATM this is the did representation of the substrate based account in format 'did:substrate:5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY/anagolay-network'
  pub holder: CreatorId,
  /// ATM this is the did representation of the substrate based account in format 'did:substrate:Hcd78R7frJfUZHsqgpPEBLeiCZxV29uyyyURaPxB71ojNjy/anagolay-network'
  pub issuer: Vec<u8>,
  /// Generic type, ATM is Copyright or Ownership
  pub claim_type: AnagolayClaimType,
  /// How long this statement is valid
  pub valid: Validity,
  /// Setting when the statement should end
  pub expiration: Expiration,
  /// What happens after the expiration? this is default rule or smart contract that automatically does stuff,
  /// like move it to the public domain, transfer to relatives etc... need better definition
  pub on_expiration: Vec<u8>,
}

/// Copyright data
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct StatementData {
  pub signatures: AnagolaySignatures,
  pub claim: AnagolayClaim,
}

/// Anagolay copyright statement
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolayStatement {
  pub id: GenericId,
  pub data: StatementData,
}

impl Default for AnagolayStatement {
  fn default() -> Self {
    AnagolayStatement {
      id: b"".to_vec(),
      data: StatementData::default(),
    }
  }
}

/// Statement DB entry
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct StatementInfo<AccountId, BlockNumber> {
  /// Generated statement data
  pub statement: AnagolayStatement,
  pub account_id: AccountId,
  pub block_number: BlockNumber,
}
