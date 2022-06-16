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

use anagolay_support::{
  AnagolayRecord, AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, Characters, CreatorId, ProofId,
  SignatureId, StatementId, WorkflowId,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec::Vec};

/// Signature
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Signature {
  /// signing key in urn/did format 'urn:pgp:9cdf8dd38531511968c8d8cb524036585b62f15b'
  pub sig_key: Characters,
  /// Signature sign(prepared_statement, pvtKey(sigKey)) and encoded using multibase
  /// https://gitlab.com/sensio_group/sensio-faas/-/blob/master/sp-api/src/plugins/copyright/helpers.ts#L76
  pub sig: Vec<u8>,
  /// Content identifier of the sig field -- CID(sig)
  pub cid: SignatureId,
}

/// Signatures
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Signatures {
  pub holder: Signature,
  pub issuer: Signature,
}
/// Claim Proportion
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Proportion {
  /// Proportion sign, can be %
  pub sign: Vec<u8>,
  pub name: Vec<u8>,
  pub value: Vec<u8>,
}
/// Validity
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

/// Claim Expiration
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Expiration {
  ///Possible Expiration types
  pub expiration_type: ExpirationType,
  ///How long is the expiration, if  ExpirationType::FOREVER then this is empty
  pub value: Vec<u8>,
}

/// Claim types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum ClaimType {
  Copyright,
  Ownership,
}

impl Default for ClaimType {
  fn default() -> Self {
    ClaimType::Copyright
  }
}

/// Generic Claim
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Claim {
  /// Prev Statement id in case this statement is revoked or changed
  pub prev_id: Option<StatementId>,
  /// PoE id of the record in question.
  pub poe_id: ProofId,
  /// Implemented workflow id
  pub workflow_id: WorkflowId,
  /// In which proportion the statement is held
  pub proportion: Proportion,
  /// ATM this is the same as poe_id @TODO this should be unique representation of the subject that
  /// is NOT poe
  pub subject_id: ProofId,
  /// ATM this is the did representation of the substrate based account in format
  /// 'did:substrate:5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY/anagolay-network'
  pub holder: CreatorId,
  /// ATM this is the did representation of the substrate based account in format
  /// 'did:substrate:Hcd78R7frJfUZHsqgpPEBLeiCZxV29uyyyURaPxB71ojNjy/anagolay-network'
  pub issuer: CreatorId,
  /// Generic type, ATM is Copyright or Ownership
  pub claim_type: ClaimType,
  /// How long this statement is valid
  pub valid: Validity,
  /// Setting when the statement should end
  pub expiration: Expiration,
  /// What happens after the expiration? this is default rule or smart contract that automatically
  /// does stuff, like move it to the public domain, transfer to relatives etc... need better
  /// definition
  pub on_expiration: Vec<u8>,
}

/// Copyright data
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct StatementData {
  pub signatures: Signatures,
  pub claim: Claim,
}

impl AnagolayStructureData for StatementData {
  fn validate(&self) -> Result<(), Characters> {
    Ok(())
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct StatementExtra {}
impl AnagolayStructureExtra for StatementExtra {}
impl Default for StatementExtra {
  fn default() -> Self {
    StatementExtra {}
  }
}

pub type Statement = AnagolayStructure<StatementData, StatementExtra>;
pub type StatementRecord<T> =
  AnagolayRecord<Statement, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;
