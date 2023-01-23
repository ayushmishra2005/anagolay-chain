// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

use codec::{Decode, Encode};
use core::fmt::Debug;
use frame_support::pallet_prelude::*;
use verification::types::VerificationContext;

/// NOT USED ATM. Keeping it because we might need it in the future.
/// An enum used in RPCs to indicate the order with which Tips are paged
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all(deserialize = "camelCase")))]
pub enum SortTips {
  Asc,
  Desc,
}

/// A structure associated with every `VerificationContext`, providing the tipping settings
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct TippingSettings<Account: Debug> {
  /// The verification context this setting applies to
  pub context: VerificationContext,
  /// Specifies that the tipping is enabled or not
  pub enabled: bool,
  /// Specifies to which wallet to send the tips given in some kind of token
  pub account: Option<Account>,
}

impl<Account: Debug> Default for TippingSettings<Account> {
  fn default() -> Self {
    TippingSettings {
      context: VerificationContext::default(),
      enabled: false,
      account: None,
    }
  }
}

/// Structure representing a tip
#[derive(Encode, Decode, Clone, Default, PartialEq, Eq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Tip<Balance: Debug, Account: Debug, BlockNumber: Debug> {
  /// Quantity of tokens tipped
  pub amount: Balance,
  /// The user that is tipping
  pub sender: Account,
  /// The account that is receiving the tip
  pub receiver: Account,
  /// Timestamp of the tip
  pub created_at: u64,
  /// Block where the tip was inserted
  pub block_number: BlockNumber,
}
