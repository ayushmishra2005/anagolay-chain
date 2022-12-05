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

use anagolay_support::{constants::*, generic_id::GenericId, *};
use codec::{Decode, Encode};
use frame_support::{
  pallet_prelude::*,
  sp_runtime::RuntimeDebug,
  sp_std::{clone::Clone, default::Default},
};
use poe::types::ProofId;
use verification::types::{VerificationInvalidator, VerificationRequest};

getter_for_hardcoded_constant!(MaxSignatureLen, u32, 256);

// Statement id
anagolay_generic_id!(Statement);

// Signature id
anagolay_generic_id!(Signature);

/// Signature
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature {
  /// signing key in urn/did format 'urn:pgp:9cdf8dd38531511968c8d8cb524036585b62f15b'
  pub sig_key: Characters,
  /// Signature sign(prepared_statement, pvtKey(sigKey))
  pub sig: BoundedVec<u8, MaxSignatureLenGet>,
  /// Content identifier of the sig field -- CID(sig)
  pub cid: SignatureId,
}

/// Signatures
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Signatures {
  pub holder: Signature,
  pub issuer: Signature,
}
/// Claim Proportion
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Proportion {
  /// Proportion sign, can be %
  pub sign: Characters,
  pub name: Characters,
  pub value: Characters,
}
/// Validity
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Validity {
  /// When the validity starts, this should be DATE_TIME
  pub from: Characters,
  /// When validity ends, this is calculate Validity.from + Expiration.value
  pub until: Characters,
}

/// Possible Expiration types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Expiration {
  ///Possible Expiration types
  pub expiration_type: ExpirationType,
  ///How long is the expiration, if  ExpirationType::FOREVER then this is empty
  pub value: Characters,
}

/// Claim types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Claim {
  /// Prev Statement id in case this statement is revoked or changed
  pub prev_id: Option<StatementId>,
  /// PoE id of the record in question.
  pub poe_id: ProofId,
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
  pub on_expiration: Characters,
}

/// Copyright data
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementData {
  pub signatures: Signatures,
  pub claim: Claim,
}

impl AnagolayStructureData for StatementData {
  type Id = StatementId;

  fn validate(&self) -> Result<(), Characters> {
    Ok(())
  }
}

#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementExtra {}
impl AnagolayStructureExtra for StatementExtra {}

// Statement entity
anagolay_structure!(Statement, StatementId, StatementData, StatementExtra);

// This produces `StatementRecord<T>`, the Storage record of the Statement.
anagolay_record!(Statement);

/// Implementation for the verification invalidator that revokes the statement associated to
/// the verification
#[derive(Clone)]
pub struct StatementsVerificationInvalidator<T: crate::Config> {
  _marker: PhantomData<T>,
}

impl<T: crate::Config> VerificationInvalidator<T> for StatementsVerificationInvalidator<T> {
  /// Called when a verification request turns out to be no longer valid
  ///
  /// # Arguments
  /// * request - The verification request
  ///
  /// # Return
  /// Result having the unit type if ok, an Error otherwise
  fn invalidate(request: &VerificationRequest<T::AccountId>) -> Result<(), verification::Error<T>> {
    let proof_ids = poe::Pallet::<T>::proof_ids_by_verification_context(request.context.clone())
      .ok_or(verification::Error::<T>::VerificationInvalidationError)?;
    for proof_id in proof_ids {
      let statement_ids = <crate::Pallet<T>>::statement_ids_by_proof_id(proof_id.clone());
      for statement_id in statement_ids {
        <crate::Pallet<T>>::remove_statement(statement_id.clone(), &request.holder)
          .map_err(|_| verification::Error::<T>::VerificationInvalidationError)?;
      }
    }
    Ok(())
  }
}
