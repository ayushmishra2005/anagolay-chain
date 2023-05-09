// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Test utilities

use crate as verification;
use crate::{types::*, Config};
use core::convert::{TryFrom, TryInto};
use frame_support::parameter_types;
use pallet_balances::AccountData;
use sp_core::{sr25519, sr25519::Signature, H256};
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::{
  testing::{Header, TestXt, UintAuthorityId},
  traits::{BlakeTwo256, IdentityLookup},
  RuntimeAppPublic,
};
use std::sync::Arc;

type Extrinsic = TestXt<RuntimeCall, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
      System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
      Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
      Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
      VerificationTest: verification::{Pallet, Call, Storage, Event<T>, ValidateUnsigned} = 18,
    }
);

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const SS58Prefix: u8 = 42;

  pub static ExistentialDeposit: u32 = 0;
  pub const MaxLocks: u32 = 50;
  pub const MaxReserves: u32 = 2;
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
  type AccountData = AccountData<u64>;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type SystemWeightInfo = ();
  type SS58Prefix = SS58Prefix;
  type OnSetCode = ();
  type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
  type MaxLocks = ();
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  type Balance = u64;
  type DustRemoval = ();
  type RuntimeEvent = RuntimeEvent;
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = ();
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

impl Config for Test {
  type AuthorityId = crate::crypto::VerificationAuthId;
  type RuntimeEvent = RuntimeEvent;
  type VerificationKeyGenerator = NaiveVerificationKeyGenerator<Self>;
  type VerificationInvalidator = NaiveVerificationInvalidator<Self>;
  type WeightInfo = ();
  type Currency = Balances;

  const REGISTRATION_FEE: u64 = 10;
  const MAX_REQUESTS_PER_CONTEXT: u32 = 2;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
  RuntimeCall: From<LocalCall>,
{
  type OverarchingCall = RuntimeCall;
  type Extrinsic = Extrinsic;
}

impl frame_system::offchain::SigningTypes for Test {
  type Public = <Signature as sp_runtime::traits::Verify>::Signer;
  type Signature = Signature;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(balances: Vec<(sr25519::Public, u64)>) -> sp_io::TestExternalities {
  let mut ext = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
  pallet_balances::GenesisConfig::<Test> { balances }
    .assimilate_storage(&mut ext)
    .unwrap();
  let mut ext: sp_io::TestExternalities = ext.into();

  const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";
  let keystore = KeyStore::new();

  SyncCryptoStore::sr25519_generate_new(
    &keystore,
    crate::crypto::Public::ID,
    Some(&format!("{}/hunter1", PHRASE)),
  )
  .unwrap();

  let _public_key = *SyncCryptoStore::sr25519_public_keys(&keystore, crate::crypto::Public::ID)
    .get(0)
    .unwrap();
  ext.register_extension(KeystoreExt(Arc::new(keystore)));

  ext
}
