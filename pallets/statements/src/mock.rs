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

//! Test utilities

#![cfg(test)]

use crate as statements;
use crate::{types::StatementsVerificationInvalidator, Config};
use frame_support::parameter_types;
use sp_core::{sr25519, sr25519::Signature, H256};
use sp_runtime::{
  testing::{Header, TestXt, UintAuthorityId},
  traits::{BlakeTwo256, IdentityLookup},
};
use std::convert::{TryFrom, TryInto};
use verification::types::NaiveVerificationKeyGenerator;

type Extrinsic = TestXt<RuntimeCall, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

use core::time::Duration;
use frame_support::traits::UnixTime;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        Poe: poe::{Pallet, Call, Storage, Event<T>},
        Verification: verification::{Pallet, Call, Storage, Event<T>, ValidateUnsigned},
        TestStatements: statements::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockWeights = ();
  type BlockLength = ();
  type RuntimeOrigin = RuntimeOrigin;
  type RuntimeCall = RuntimeCall;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = sr25519::Public;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type RuntimeEvent = RuntimeEvent;
  type BlockHashCount = BlockHashCount;
  type DbWeight = ();
  type Version = ();
  type PalletInfo = PalletInfo;
  type AccountData = ();
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type SystemWeightInfo = ();
  type SS58Prefix = SS58Prefix;
  type OnSetCode = ();
  type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
  pub const Period: u32 = 360000;
  pub const Offset: u32 = 0;
}

impl pallet_session::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type ValidatorId = sr25519::Public;
  type ValidatorIdOf = ();
  type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
  type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
  type SessionManager = ();
  type SessionHandler = pallet_session::TestSessionHandler;
  type Keys = UintAuthorityId;
  type WeightInfo = ();
}

impl verification::Config for Test {
  type AuthorityId = verification::crypto::VerificationAuthId;
  type RuntimeEvent = RuntimeEvent;
  type VerificationKeyGenerator = NaiveVerificationKeyGenerator<Self>;
  type VerificationInvalidator = StatementsVerificationInvalidator<Self>;
  type WeightInfo = ();
  type Currency = ();

  const REGISTRATION_FEE: u32 = 10;
  const MAX_REQUESTS_PER_CONTEXT: u32 = 1;
}

impl<VerificationCall> frame_system::offchain::SendTransactionTypes<VerificationCall> for Test
where
  RuntimeCall: From<VerificationCall>,
{
  type OverarchingCall = RuntimeCall;
  type Extrinsic = Extrinsic;
}

impl frame_system::offchain::SigningTypes for Test {
  type Public = <Signature as sp_runtime::traits::Verify>::Signer;
  type Signature = Signature;
}

impl poe::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = ();

  const MAX_PROOFS_PER_WORKFLOW: u32 = 1;
}

pub struct MockTime {}

impl UnixTime for MockTime {
  fn now() -> Duration {
    core::time::Duration::from_millis(1000)
  }
}

type WorkflowsEvent = workflows::Event<Test>;
impl From<WorkflowsEvent> for RuntimeEvent {
  fn from(_: WorkflowsEvent) -> Self {
    todo!()
  }
}
impl workflows::Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = ();
  type TimeProvider = MockTime;

  const MAX_VERSIONS_PER_WORKFLOW: u32 = 100;
}

impl anagolay_support::Config for Test {
  const MAX_ARTIFACTS: u32 = 1_000_000;
}

impl Config for Test {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = crate::weights::AnagolayWeight<Test>;
  const MAX_STATEMENTS_PER_PROOF: u32 = 16;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
  frame_system::GenesisConfig::default()
    .build_storage::<Test>()
    .unwrap()
    .into()
}
