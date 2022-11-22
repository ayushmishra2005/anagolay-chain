// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

use codec::{Decode, Encode};
use frame_support::{pallet_prelude::*, sp_runtime::RuntimeDebug, sp_std::vec::Vec};
/// Module containing types used for off-chain processing
pub mod offchain {
  use super::*;
  use codec::{Decode, Encode};
  use frame_support::sp_std::clone::Clone;

  /// Structure used in the offchain indexing to signal that there is a
  /// [`VerificationRequest`] to process. This same type is submitted back to the runtime
  /// to update the status of the [`VerificationRequest`] on-chain
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebugNoBound, TypeInfo)]
  #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
  #[scale_info(skip_type_params(T))]
  pub struct VerificationIndexingData<T: frame_system::Config> {
    /// The caller of perform verification extrinsic
    pub verifier: T::AccountId,
    /// The [`VerificationRequest`] to process
    pub request: VerificationRequest<T>,
  }
}

/// Getter for the hard-coded constant defining Maximum length of a Bytes type.
/// This approach has advantages over using `ConstU32` since it implements all the traits used in
/// the datamodel
#[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct MaxBytesLenGet();
/// Implementation of the ['Get'] trait for the getter for [`MaxBytesLenGet`]
impl Get<u32> for MaxBytesLenGet {
  fn get() -> u32 {
    256u32
  }
}

/// Newtype around BoundedVec<u8, MaxBytesLenGet>
#[derive(codec::Encode, codec::Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Bytes(BoundedVec<u8, MaxBytesLenGet>);

/// Delegation of the inner type methods
impl Bytes {
  pub fn into_inner(self) -> Vec<u8> {
    self.0.into_inner()
  }
}

impl From<&str> for Bytes {
  fn from(string: &str) -> Bytes {
    string.as_bytes().to_vec().into()
  }
}

impl From<Vec<u8>> for Bytes {
  fn from(vec: Vec<u8>) -> Bytes {
    use core::convert::TryInto;
    Self(vec.try_into().unwrap_or_default())
  }
}

/// Enumeration representing the possible outcomes of a verification
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum VerificationStatus {
  /// The verification strategy is waiting for an external action prior to starting the challenge
  Waiting,
  /// The verification strategy is ready to execute the challenge
  Pending,
  /// The verification challenge has failed
  Failure(Bytes),
  /// The verification challende is successful
  Success,
}

/// An enumeration providing the switch to verify a context (full URL + breakdown)
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum VerificationContext {
  /// No context to verify
  #[default]
  Unbounded,
  /// URL, Domain - e.g: https://anagolay.network
  UrlForDomain(Bytes, Bytes),
  /// URL, Domain, Username - e.g: https://github.com/anagolay/
  UrlForDomainWithUsername(Bytes, Bytes, Bytes),
  /// URL, Domain, Subdomain - e.g: https://app.anagolay.network
  UrlForDomainWithSubdomain(Bytes, Bytes, Bytes),
  /// URL, Domain, Username, Repository - e.g: https://github.com/anagolay/anagolay-chain
  UrlForDomainWithUsernameAndRepository(Bytes, Bytes, Bytes, Bytes),
}

/// An enumeration providing the instructions of an action to perform in order to verify a context
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum VerificationAction {
  /// Instruct the verification holder to update the DNS TXT record
  DnsTxtRecord,
}

/// A structure representing the request to verify. Requires to be typed with the runtime
/// configuration and with the expected [`VerificationContext`]. It's used to keep trace of the
/// state of a verification request.
///
/// # Type arguments
/// - T: the runtime `Config`
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[scale_info(skip_type_params(T))]
pub struct VerificationRequest<T: frame_system::Config> {
  /// The context of the verification, for instance a domain
  pub context: VerificationContext,
  /// An indication of the action that the holder must perform to pass verification
  pub action: VerificationAction,
  /// The current state of the request
  pub status: VerificationStatus,
  /// The holder of the verification
  pub holder: T::AccountId,
  /// Contains a challenge string. This is in complete control of the invoked Strategy
  pub key: Bytes,
  /// The feedback from the holder pointing at the exact place where the verification should happen
  /// (TweetId, etc…)
  pub id: Option<Bytes>,
}

/// Default implementation for the verification key generator
#[derive(Clone)]
pub struct NaiveVerificationKeyGenerator<T: crate::Config> {
  _marker: PhantomData<T>,
}

impl<T: crate::Config> VerificationKeyGenerator<T> for NaiveVerificationKeyGenerator<T> {
  /// Produces a 2 characters key by overflowing subtract the bytes of the identifier argument
  ///
  /// # Arguments
  /// * holder - The verification holder (unused)
  /// * context - The verification context (unused)
  /// * identifier - The identifier to use for the hakey generationsh
  ///
  /// # Return
  /// The hex encoded identifier in the form of a collection of utf8 bytes
  fn generate(
    _holder: &T::AccountId,
    _context: &VerificationContext,
    identifier: Vec<u8>,
  ) -> Result<Vec<u8>, crate::Error<T>> {
    identifier
      .into_iter()
      .reduce(|a, b| a.overflowing_sub(b).0)
      .map(|byte| hex::encode(&[byte]).as_bytes().to_vec())
      .ok_or(crate::Error::<T>::VerificationKeyGenerationError)
  }
}

/// A trait that mimics the behavior of verification strategies on a VerificationContext trait,
/// providing the common methods
///
/// # Type arguments
/// - Config: the runtime `Config`
pub trait VerificationStrategy: Clone {
  type Config: frame_system::Config;
  type VerificationError: core::fmt::Debug;

  /// Creates a new [`VerificationRequest`]
  ///
  /// # Arguments
  /// - holder: The verification holder
  /// - context: The [`VerificationContext`] to check
  /// - action: The [`VerificationAction`] the end user has chosen to perform
  ///
  /// # Return
  /// A [`VerificationRequest`] from the given context and action
  fn new_request(
    &self,
    holder: <Self::Config as frame_system::Config>::AccountId,
    context: VerificationContext,
    action: VerificationAction,
  ) -> Result<VerificationRequest<Self::Config>, crate::Error<Self::Config>>;

  /// Defines whether a [`VerificationContext`] is supported or not
  ///
  /// # Arguments
  /// - context: The [`VerificationContext`] to check
  /// - action: The [`VerificationAction`] the end user has chosen to perform
  ///
  /// # Return
  /// True if the context is supported by this strategy, false otherwise
  fn supports(&self, context: &VerificationContext, action: &VerificationAction) -> bool;

  /// Performs an HTTP call to check the required criterion to pass the verification
  ///
  /// # Arguments
  /// - request: The `VerificationRequest` to verify
  ///
  /// # Return
  /// A `VerificationStatus` resulting from the verification
  fn verify(&self, request: &VerificationRequest<Self::Config>) -> Result<VerificationStatus, Self::VerificationError>;
}

/// A trait that mimics the behavior of a key generator. The default implementation uses an
/// Anagolay workflow to generate a cid out of an identifier (usually the concatenation of some
/// strategy-related information and the verification holder account). However, the pallet
/// configuration allow to define another implementation of this trait so that the key generation
/// can be tweaked.
///
/// # Type arguments
/// - T: the runtime `Config`
pub trait VerificationKeyGenerator<T: frame_system::Config>: Clone {
  /// Produces a verification key out of an identifier
  ///
  /// # Arguments
  /// * holder - The verification holder
  /// * context - The verification context
  /// * identifier - The identifier bytes
  ///
  /// # Return
  /// Verification key bytes
  fn generate(
    holder: &T::AccountId,
    context: &VerificationContext,
    identifier: Vec<u8>,
  ) -> Result<Vec<u8>, crate::Error<T>>;
}
