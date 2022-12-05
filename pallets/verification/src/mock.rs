// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

//! Test utilities

use crate as verification;
use crate::{types::*, Config};
use core::convert::{TryFrom, TryInto};
use frame_support::parameter_types;
use pallet_balances::AccountData;
use sp_core::{sr25519, H256};
use sp_runtime::{
  testing::{Header, TestXt},
  traits::{BlakeTwo256, IdentityLookup},
};

type Extrinsic = TestXt<Call, ()>;
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
  type Origin = Origin;
  type Call = Call;
  type Index = u64;
  type BlockNumber = u64;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type AccountId = sr25519::Public;
  type Lookup = IdentityLookup<Self::AccountId>;
  type Header = Header;
  type Event = Event;
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
  type Event = Event;
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = ();
}

impl Config for Test {
  type Event = Event;
  type VerificationKeyGenerator = NaiveVerificationKeyGenerator<Self>;
  type VerificationInvalidator = NaiveVerificationInvalidator<Self>;
  type WeightInfo = ();
  type Currency = Balances;

  const REGISTRATION_FEE: u64 = 10;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
  Call: From<LocalCall>,
{
  type OverarchingCall = Call;
  type Extrinsic = Extrinsic;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(balances: Vec<(sr25519::Public, u64)>) -> sp_io::TestExternalities {
  let mut ext = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
  pallet_balances::GenesisConfig::<Test> { balances }
    .assimilate_storage(&mut ext)
    .unwrap();

  ext.into()
}
