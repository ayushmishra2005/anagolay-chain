#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]
// let clippy disregard expressions like 1 * UNITS
#![allow(clippy::identity_op)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

mod weights;
pub mod xcm_config;

use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use smallvec::smallvec;
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
  create_runtime_str, generic, impl_opaque_keys,
  traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, ConstU128, ConstU16, IdentifyAccount, Verify},
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature,
};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
  construct_runtime,
  dispatch::DispatchClass,
  pallet_prelude::*,
  parameter_types,
  traits::{
    AsEnsureOriginWithArg, EqualPrivilegeOnly, Everything, Imbalance, InstanceFilter, OnUnbalanced, WithdrawReasons,
  },
  weights::{
    constants::WEIGHT_PER_SECOND, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
  },
  PalletId,
};
use frame_system::{
  limits::{BlockLength, BlockWeights},
  EnsureRoot, EnsureSigned,
};
use pallet_balances::NegativeImbalance;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::traits::ConvertInto;
pub use sp_runtime::{MultiAddress, Perbill, Permill};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Polkadot imports
use polkadot_runtime_common::{BlockHashCount, SlowAdjustingFeeUpdate};

use weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight};

// XCM Imports
use xcm::latest::prelude::BodyId;
use xcm_executor::XcmExecutor;

/// Importing anagolay pallet
pub use anagolay_support;

/// Importing operations pallet
pub use operations;

/// Importing statements pallet
pub use statements;

/// Importing workflows pallet
pub use workflows;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
  frame_system::CheckNonZeroSender<Runtime>,
  frame_system::CheckSpecVersion<Runtime>,
  frame_system::CheckTxVersion<Runtime>,
  frame_system::CheckGenesis<Runtime>,
  frame_system::CheckEra<Runtime>,
  frame_system::CheckNonce<Runtime>,
  frame_system::CheckWeight<Runtime>,
  pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive =
  frame_executive::Executive<Runtime, Block, frame_system::ChainContext<Runtime>, Runtime, AllPalletsWithSystem>;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
  type Balance = Balance;
  fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
    // in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNITS:
    // in our template, we map to 1/10 of that, or 1/10 MILLIUNITS
    let p = MILLIUNITS / 10;
    let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
    smallvec![WeightToFeeCoefficient {
      degree: 1,
      negative: false,
      coeff_frac: Perbill::from_rational(p % q, q),
      coeff_integer: p / q,
    }]
  }
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
  use super::*;
  use sp_runtime::{generic, traits::BlakeTwo256};

  pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
  /// Opaque block header type.
  pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
  /// Opaque block type.
  pub type Block = generic::Block<Header, UncheckedExtrinsic>;
  /// Opaque block identifier type.
  pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
      pub struct SessionKeys {
          pub aura: Aura,
  }
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
  spec_name: create_runtime_str!("idiyanale"),
  impl_name: create_runtime_str!("idiyanale"),
  authoring_version: 1,
  // The version of the runtime specification. A full node will not attempt to use its native
  //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
  //   `spec_version`, and `authoring_version` are the same between Wasm and native.
  // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
  //   the compatible custom types.
  spec_version: 117,
  impl_version: 1,
  apis: RUNTIME_API_VERSIONS,
  transaction_version: 1,
  state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
pub const WEEKS: BlockNumber = DAYS * 7;

// Provide a scaling factor
pub const SUPPLY_FACTOR: Balance = 10;

// Unit = the base number of indivisible units for balances
pub const UNITS: Balance = 1_000_000_000_000;
pub const MILLIUNITS: Balance = UNITS / 1_000; // 1_000_000_000
pub const MICROUNITS: Balance = UNITS / 1_000_000; // 1_000_000
pub const NANOUNITS: Balance = UNITS / 1_000_000_000; // 1_000
pub const PICOUNITS: Balance = UNITS / 1_000_000_000_000; // 1

pub const TRANSACTION_BYTE_FEE: Balance = 1 * PICOUNITS * SUPPLY_FACTOR;
pub const STORAGE_BYTE_FEE: Balance = 100 * PICOUNITS * SUPPLY_FACTOR;
pub const WEIGHT_FEE: Balance = 50 * PICOUNITS * SUPPLY_FACTOR;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
  items as Balance * 15 * PICOUNITS * SUPPLY_FACTOR + (bytes as Balance) * STORAGE_BYTE_FEE
}

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNITS;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = WEIGHT_PER_SECOND
  .saturating_div(2)
  .set_proof_size(cumulus_primitives_core::relay_chain::v2::MAX_POV_SIZE as u64);

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
  NativeVersion {
    runtime_version: VERSION,
    can_author_with: Default::default(),
  }
}

parameter_types! {
  pub const Version: RuntimeVersion = VERSION;

  // This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
  //  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
  // `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
  // the lazy contract deletion.
  pub RuntimeBlockLength: BlockLength =
    BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
  pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
    .base_block(BlockExecutionWeight::get())
    .for_class(DispatchClass::all(), |weights| {
      weights.base_extrinsic = ExtrinsicBaseWeight::get();
    })
    .for_class(DispatchClass::Normal, |weights| {
      weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
    })
    .for_class(DispatchClass::Operational, |weights| {
      weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
      // Operational transactions have some extra reserved space, so that they
      // are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
      weights.reserved = Some(
        MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
      );
    })
    .avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
    .build_or_panic();
  pub const SS58Prefix: u16 = 42;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
  /// The identifier used to distinguish between accounts.
  type AccountId = AccountId;
  /// The aggregated dispatch type that is available for extrinsics.
  type RuntimeCall = RuntimeCall;
  /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
  type Lookup = AccountIdLookup<AccountId, ()>;
  /// The index type for storing how many extrinsics an account has signed.
  type Index = Index;
  /// The index type for blocks.
  type BlockNumber = BlockNumber;
  /// The type for hashing blocks and tries.
  type Hash = Hash;
  /// The hashing algorithm used.
  type Hashing = BlakeTwo256;
  /// The header type.
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  /// The ubiquitous event type.
  type RuntimeEvent = RuntimeEvent;
  /// The ubiquitous origin type.
  type RuntimeOrigin = RuntimeOrigin;
  /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
  type BlockHashCount = BlockHashCount;
  /// Runtime version.
  type Version = Version;
  /// Converts a module to an index of this module in the runtime.
  type PalletInfo = PalletInfo;
  /// The data to be stored in an account.
  type AccountData = pallet_balances::AccountData<Balance>;
  /// What to do if a new account is created.
  type OnNewAccount = ();
  /// What to do if an account is fully reaped from the system.
  type OnKilledAccount = ();
  /// The weight of database operations that the runtime can invoke.
  type DbWeight = RocksDbWeight;
  /// The basic call filter to use in dispatchable.
  type BaseCallFilter = Everything;
  /// Weight information for the extrinsics of this pallet.
  type SystemWeightInfo = ();
  /// Block & extrinsics weights: base values and limits.
  type BlockWeights = RuntimeBlockWeights;
  /// The maximum length of a block (in bytes).
  type BlockLength = RuntimeBlockLength;
  /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
  type SS58Prefix = SS58Prefix;
  /// The action to take on a Runtime Upgrade
  type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
  type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
  pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
  /// A timestamp: milliseconds since the unix epoch.
  type Moment = u64;
  type OnTimestampSet = Aura;
  type MinimumPeriod = MinimumPeriod;
  type WeightInfo = ();
}

parameter_types! {
  pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
  type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
  type UncleGenerations = UncleGenerations;
  type FilterUncle = ();
  type EventHandler = (CollatorSelection,);
}

parameter_types! {
  pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
  pub const MaxLocks: u32 = 50;
  pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
  type MaxLocks = MaxLocks;
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type RuntimeEvent = RuntimeEvent;
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
  type MaxReserves = MaxReserves;
  type ReserveIdentifier = [u8; 8];
}

parameter_types! {
  /// Relay Chain `TransactionByteFee` / 10
  pub const TransactionByteFee: Balance = 10 * MICROUNITS;
  pub const OperationalFeeMultiplier: u8 = 5;
}

pub struct DealWithFees<R>(sp_std::marker::PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
  R: pallet_balances::Config + pallet_treasury::Config,
  pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
{
  // this is called for substrate-based transactions
  fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
    if let Some(fees) = fees_then_tips.next() {
      // for fees, 80% are burned, 20% to the treasury
      let (_, to_treasury) = fees.ration(80, 20);
      // Balances pallet automatically burns dropped Negative Imbalances by decreasing
      // total_supply accordingly
      <pallet_treasury::Pallet<R> as OnUnbalanced<_>>::on_unbalanced(to_treasury);
    }
  }
}

/// Uses a polynomial curve to have transaction pay according to their length, which means
/// the amount of data they potentially want to store on chain
pub struct LengthToFee;
impl WeightToFeePolynomial for LengthToFee {
  type Balance = Balance;

  fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
    smallvec![
      WeightToFeeCoefficient {
        degree: 1,
        coeff_frac: Perbill::zero(),
        coeff_integer: TRANSACTION_BYTE_FEE,
        negative: false,
      },
      WeightToFeeCoefficient {
        degree: 3,
        coeff_frac: Perbill::zero(),
        coeff_integer: SUPPLY_FACTOR,
        negative: false,
      },
    ]
  }
}

impl pallet_transaction_payment::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
  type WeightToFee = WeightToFee;
  type LengthToFee = LengthToFee;
  type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
  type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

parameter_types! {
  pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
  pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type OnSystemEvent = ();
  type SelfParaId = parachain_info::Pallet<Runtime>;
  type OutboundXcmpMessageSource = XcmpQueue;
  type DmpMessageHandler = DmpQueue;
  type ReservedDmpWeight = ReservedDmpWeight;
  type XcmpMessageHandler = XcmpQueue;
  type ReservedXcmpWeight = ReservedXcmpWeight;
  type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type XcmExecutor = XcmExecutor<XcmConfig>;
  type ChannelInfo = ParachainSystem;
  type VersionWrapper = ();
  type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
  type ControllerOrigin = EnsureRoot<AccountId>;
  type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
  type WeightInfo = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type XcmExecutor = XcmExecutor<XcmConfig>;
  type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
  pub const Period: u32 = 6 * HOURS;
  pub const Offset: u32 = 0;
  pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_session::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type ValidatorId = <Self as frame_system::Config>::AccountId;
  // we don't have stash and controller, thus we don't need the convert as well.
  type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
  type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
  type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
  type SessionManager = CollatorSelection;
  // Essentially just Aura, but lets be pedantic.
  type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
  type Keys = SessionKeys;
  type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_aura::Config for Runtime {
  type AuthorityId = AuraId;
  type DisabledValidators = ();
  type MaxAuthorities = MaxAuthorities;
}

parameter_types! {
  pub const PotId: PalletId = PalletId(*b"PotStake");
  pub const MaxCandidates: u32 = 1000;
  pub const MinCandidates: u32 = 5;
  pub const SessionLength: BlockNumber = 6 * HOURS;
  pub const MaxInvulnerables: u32 = 100;
  pub const ExecutiveBody: BodyId = BodyId::Executive;
}

// We allow root only to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EnsureRoot<AccountId>;

impl pallet_collator_selection::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type Currency = Balances;
  type UpdateOrigin = CollatorSelectionUpdateOrigin;
  type PotId = PotId;
  type MaxCandidates = MaxCandidates;
  type MinCandidates = MinCandidates;
  type MaxInvulnerables = MaxInvulnerables;
  // should be a multiple of session or things will get inconsistent
  type KickThreshold = Period;
  type ValidatorId = <Self as frame_system::Config>::AccountId;
  type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
  type ValidatorRegistration = Session;
  type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
}

parameter_types! {
  pub const ProposalBond: Permill = Permill::from_percent(5);
  pub const TreasuryId: PalletId = PalletId(*b"py/trsry");
}

impl pallet_treasury::Config for Runtime {
  type PalletId = TreasuryId;
  type Currency = Balances;
  // root is required to approve a proposal
  type ApproveOrigin = EnsureRoot<AccountId>;
  // root is required to reject a proposal
  type RejectOrigin = EnsureRoot<AccountId>;
  type RuntimeEvent = RuntimeEvent;
  // If spending proposal rejected, transfer proposer bond to treasury
  type OnSlash = Treasury;
  type ProposalBond = ProposalBond;
  type ProposalBondMinimum = ConstU128<{ 1 * UNITS }>;
  type SpendPeriod = ConstU32<{ 6 * DAYS }>;
  type Burn = ();
  type BurnDestination = ();
  type MaxApprovals = ConstU32<100>;
  type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
  type SpendFunds = ();
  type ProposalBondMaximum = ();
  type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>; // Same as Polkadot
}

impl pallet_utility::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
  type PalletsOrigin = OriginCaller;
  type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
  pub const MinVestedTransfer: Balance = 1 * UNITS;
  pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
    WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type Currency = Balances;
  type BlockNumberToBalance = ConvertInto;
  type MinVestedTransfer = MinVestedTransfer;
  type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
  type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;

  const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
  pub MaximumSchedulerWeight: Weight = Weight::from_ref_time(10_000_000);
  pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeOrigin = RuntimeOrigin;
  type PalletsOrigin = OriginCaller;
  type RuntimeCall = RuntimeCall;
  type MaximumWeight = MaximumSchedulerWeight;
  type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
  type MaxScheduledPerBlock = MaxScheduledPerBlock;
  type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
  type OriginPrivilegeCmp = EqualPrivilegeOnly;
  type Preimages = ();
}

parameter_types! {
  pub const CollectionDeposit: Balance = 10 * UNITS; // 10 UNIT deposit to create uniques class
  pub const ItemDeposit: Balance = UNITS / 100; // 1 / 100 UNIT deposit to create uniques instance
  pub const KeyLimit: u32 = 32;	// Max 32 bytes per key
  pub const ValueLimit: u32 = 64;	// Max 64 bytes per value
  pub const UniquesMetadataDepositBase: Balance = deposit(1, 129);
  pub const AttributeDepositBase: Balance = deposit(1, 0);
  pub const DepositPerByte: Balance = deposit(0, 1);
  pub const UniquesStringLimit: u32 = 128;
}

impl pallet_uniques::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type CollectionId = u32;
  type ItemId = u32;
  type Currency = Balances;
  type ForceOrigin = EnsureRoot<AccountId>;
  type CollectionDeposit = CollectionDeposit;
  type ItemDeposit = ItemDeposit;
  type MetadataDepositBase = UniquesMetadataDepositBase;
  type AttributeDepositBase = AttributeDepositBase;
  type DepositPerByte = DepositPerByte;
  type StringLimit = UniquesStringLimit;
  type KeyLimit = KeyLimit;
  type ValueLimit = ValueLimit;
  type WeightInfo = pallet_uniques::weights::SubstrateWeight<Runtime>;
  #[cfg(feature = "runtime-benchmarks")]
  type Helper = ();
  type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
  type Locker = ();
}

parameter_types! {
  // One storage item; key size 32, value size 8; .
  pub const ProxyDepositBase: Balance = deposit(1, 8);
  // Additional storage item size of 33 bytes.
  pub const ProxyDepositFactor: Balance = deposit(0, 33);
  pub const AnnouncementDepositBase: Balance = deposit(1, 8);
  pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
  Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum ProxyType {
  Any,
  NonTransfer,
}
impl Default for ProxyType {
  fn default() -> Self {
    Self::Any
  }
}
impl InstanceFilter<RuntimeCall> for ProxyType {
  fn filter(&self, c: &RuntimeCall) -> bool {
    match self {
      ProxyType::Any => true,
      ProxyType::NonTransfer => !matches!(
        c,
        RuntimeCall::Balances(..) |
          RuntimeCall::Uniques(..) |
          RuntimeCall::Vesting(pallet_vesting::Call::vested_transfer { .. })
      ),
    }
  }
  fn is_superset(&self, o: &Self) -> bool {
    match (self, o) {
      (x, y) if x == y => true,
      (ProxyType::Any, _) => true,
      (_, ProxyType::Any) => false,
      (ProxyType::NonTransfer, _) => true,
    }
  }
}

impl pallet_proxy::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
  type Currency = Balances;
  type ProxyType = ProxyType;
  type ProxyDepositBase = ProxyDepositBase;
  type ProxyDepositFactor = ProxyDepositFactor;
  type MaxProxies = ConstU32<32>;
  type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
  type MaxPending = ConstU32<32>;
  type CallHasher = BlakeTwo256;
  type AnnouncementDepositBase = AnnouncementDepositBase;
  type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

parameter_types! {
  // One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
  pub const DepositBase: Balance = deposit(1, 88);
  // Additional storage item size of 32 bytes.
  pub const DepositFactor: Balance = deposit(0, 32);
}

impl pallet_multisig::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type RuntimeCall = RuntimeCall;
  type Currency = Balances;
  type DepositBase = DepositBase;
  type DepositFactor = DepositFactor;
  type MaxSignatories = ConstU16<100>;
  type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
  pub const BasicDeposit: Balance = 10 * UNITS;       // 258 bytes on-chain
  pub const FieldDeposit: Balance = 2500 * MILLIUNITS;  // 66 bytes on-chain
  pub const SubAccountDeposit: Balance = 2 * UNITS;   // 53 bytes on-chain
  pub const MaxSubAccounts: u32 = 100;
  pub const MaxAdditionalFields: u32 = 100;
  pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type Currency = Balances;
  type BasicDeposit = BasicDeposit;
  type FieldDeposit = FieldDeposit;
  type SubAccountDeposit = SubAccountDeposit;
  type MaxSubAccounts = MaxSubAccounts;
  type MaxAdditionalFields = MaxAdditionalFields;
  type MaxRegistrars = MaxRegistrars;
  type Slashed = Treasury;
  type ForceOrigin = EnsureRoot<AccountId>;
  type RegistrarOrigin = EnsureRoot<AccountId>;
  type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}

// Anagolay pallets:
// ------------------------------------------------------------------------------------------------
impl anagolay_support::Config for Runtime {
  const MAX_ARTIFACTS: u32 = 1_000_000;
}

impl operations::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = operations::weights::AnagolayWeight<Runtime>;
  type TimeProvider = pallet_timestamp::Pallet<Runtime>;

  const MAX_VERSIONS_PER_OPERATION: u32 = 100;
}

impl workflows::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = workflows::weights::AnagolayWeight<Runtime>;
  type TimeProvider = pallet_timestamp::Pallet<Runtime>;

  const MAX_VERSIONS_PER_WORKFLOW: u32 = 100;
}

impl statements::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = statements::weights::AnagolayWeight<Runtime>;

  const MAX_STATEMENTS_PER_PROOF: u32 = 16;
}

impl poe::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type WeightInfo = poe::weights::AnagolayWeight<Runtime>;

  const MAX_PROOFS_PER_WORKFLOW: u32 = 1;
}

impl verification::Config for Runtime {
  type AuthorityId = verification::crypto::VerificationAuthId;
  type RuntimeEvent = RuntimeEvent;
  type VerificationKeyGenerator = poe::types::PoeVerificationKeyGenerator<Runtime>;
  type VerificationInvalidator = statements::types::StatementsVerificationInvalidator<Runtime>;
  type WeightInfo = verification::weights::AnagolayWeight<Runtime>;
  type Currency = Balances;

  const REGISTRATION_FEE: u128 = 1 * UNITS;
  const MAX_REQUESTS_PER_CONTEXT: u32 = 1000;
}

impl tipping::Config for Runtime {
  type RuntimeEvent = RuntimeEvent;
  type Currency = Balances;
  type TimeProvider = pallet_timestamp::Pallet<Runtime>;
  type WeightInfo = tipping::weights::AnagolayWeight<Runtime>;

  const MAX_TIPS_PER_VERIFICATION_CONTEXT: u32 = 10000;
}

impl frame_system::offchain::SigningTypes for Runtime {
  type Public = <Signature as Verify>::Signer;
  type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
  RuntimeCall: From<C>,
{
  type OverarchingCall = RuntimeCall;
  type Extrinsic = UncheckedExtrinsic;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    // System support stuff.
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 0,
    ParachainSystem: cumulus_pallet_parachain_system::{
      Pallet, Call, Config, Storage, Inherent, Event<T>, ValidateUnsigned,
    } = 1,
    Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent} = 2,
    ParachainInfo: parachain_info::{Pallet, Storage, Config} = 3,
    Sudo: pallet_sudo = 7,
    Treasury: pallet_treasury = 8,

    // Monetary stuff.
    Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 10,
    TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>} = 11,


    // Collator support. The order of these 4 are important and shall not change.
    Authorship: pallet_authorship::{Pallet, Call, Storage} = 20,
    CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>, Config<T>} = 21,
    Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>} = 22,
    Aura: pallet_aura::{Pallet, Storage, Config<T>} = 23,
    AuraExt: cumulus_pallet_aura_ext::{Pallet, Storage, Config} = 24,

    // XCM helpers.
    XcmpQueue: cumulus_pallet_xcmp_queue::{Pallet, Call, Storage, Event<T>} = 30,
    PolkadotXcm: pallet_xcm::{Pallet, Call, Event<T>, Origin, Config} = 31,
    CumulusXcm: cumulus_pallet_xcm::{Pallet, Event<T>, Origin} = 32,
    DmpQueue: cumulus_pallet_dmp_queue::{Pallet, Call, Storage, Event<T>} = 33,

    // Customizations
    Utility: pallet_utility = 40,
    // Vesting. Usable initially, but removed once all vesting is finished.
    Vesting: pallet_vesting = 41,
    Scheduler: pallet_scheduler = 42,
    Uniques: pallet_uniques = 43,
    Proxy: pallet_proxy = 44,
    MultiSig: pallet_multisig = 45,
    Identity: pallet_identity = 46,

    // Used for anagolay blockchain
    Anagolay: anagolay_support::{Pallet} = 50,
    Operations: operations::{Pallet, Call, Storage, Event<T>, Config<T>} = 51,
    Poe: poe::{Pallet, Call, Storage, Event<T>} = 52,
    Statements: statements::{Pallet, Call, Storage, Event<T>} = 53,
    Workflows: workflows::{Pallet, Call, Storage, Event<T>, Config<T>} = 54,

    // Verification and Tipping
    Verification: verification::{Pallet, Call, Storage, Event<T>, ValidateUnsigned} = 55,
    Tipping: tipping::{Pallet, Call, Storage, Event<T>} = 56,
  }
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
  define_benchmarks!(
    [frame_system, SystemBench::<Runtime>]
    [pallet_balances, Balances]
    [pallet_session, SessionBench::<Runtime>]
    [pallet_timestamp, Timestamp]
    [pallet_collator_selection, CollatorSelection]
    [cumulus_pallet_xcmp_queue, XcmpQueue]
    [pallet_treasury, Treasury]
    [pallet_utility, Utility]
    [pallet_vesting, Vesting]
    [pallet_scheduler, Scheduler]
    [pallet_uniques, Uniques]
    [pallet_proxy, Proxy]
    [pallet_multisig, MultiSig]
    [pallet_identity, Identity]
    [operations, Operations]
    [poe, Poe]
    [statements, Statements]
    [workflows, Workflows]
    [verification, Verification]
    [tipping, Tipping]
  );
}

impl_runtime_apis! {
  impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
    fn slot_duration() -> sp_consensus_aura::SlotDuration {
      sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
    }

    fn authorities() -> Vec<AuraId> {
      Aura::authorities().into_inner()
    }
  }

    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

  impl sp_api::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
      OpaqueMetadata::new(Runtime::metadata().into())
    }
  }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

  impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
      source: TransactionSource,
      tx: <Block as BlockT>::Extrinsic,
      block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
      Executive::validate_transaction(source, tx, block_hash)
    }
  }

  impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
      Executive::offchain_worker(header)
    }
  }

  impl sp_session::SessionKeys<Block> for Runtime {
    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
      SessionKeys::generate(seed)
    }

    fn decode_session_keys(
      encoded: Vec<u8>,
    ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
      SessionKeys::decode_into_raw_public_keys(&encoded)
    }
  }

  impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
    fn account_nonce(account: AccountId) -> Index {
      System::account_nonce(account)
    }
  }

  impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
    fn query_info(
      uxt: <Block as BlockT>::Extrinsic,
      len: u32,
    ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
      TransactionPayment::query_info(uxt, len)
    }
    fn query_fee_details(
      uxt: <Block as BlockT>::Extrinsic,
      len: u32,
    ) -> pallet_transaction_payment::FeeDetails<Balance> {
      TransactionPayment::query_fee_details(uxt, len)
    }
  }

  impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
    for Runtime
  {
    fn query_call_info(
      call: RuntimeCall,
      len: u32,
    ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
      TransactionPayment::query_call_info(call, len)
    }
    fn query_call_fee_details(
      call: RuntimeCall,
      len: u32,
    ) -> pallet_transaction_payment::FeeDetails<Balance> {
      TransactionPayment::query_call_fee_details(call, len)
    }
  }

  impl operations_rpc_runtime_api::OperationsApi<Block> for Runtime {
    fn get_operations_by_ids(
      operation_ids: Vec<operations::types::OperationId>,
      offset: u64,
      limit: u16,
    ) -> Vec<operations::types::Operation> {
      Operations::get_operations_by_ids(operation_ids, offset, limit)
    }
    fn get_operation_versions_by_ids(
      operation_versions_ids: Vec<operations::types::OperationVersionId>,
      offset: u64,
      limit: u16,
    ) -> Vec<operations::types::OperationVersion> {
      Operations::get_operation_versions_by_ids(operation_versions_ids, offset, limit)
    }
  }

  impl workflows_rpc_runtime_api::WorkflowsApi<Block> for Runtime {
    fn get_workflows_by_ids(
      workflow_ids: Vec<workflows::types::WorkflowId>,
      offset: u64,
      limit: u16,
    ) -> Vec<workflows::types::Workflow> {
      Workflows::get_workflows_by_ids(workflow_ids, offset, limit)
    }
    fn get_workflow_versions_by_ids(
      workflow_version_ids: Vec<workflows::types::WorkflowVersionId>,
      offset: u64,
      limit: u16,
    ) -> Vec<workflows::types::WorkflowVersion> {
      Workflows::get_workflow_versions_by_ids(workflow_version_ids, offset, limit)
    }
  }

  impl verification_rpc_runtime_api::VerificationApi<Block, AccountId> for Runtime {
    fn get_requests(
      contexts: Vec<verification::types::VerificationContext>,
      status: Option<verification::types::VerificationStatus>,
      offset: u64,
      limit: u16,
    ) -> Vec<verification::types::VerificationRequest<AccountId>> {
      Verification::get_requests(contexts, status, None, offset, limit)
    }
    fn get_requests_for_account(
      account: AccountId,
      status: Option<verification::types::VerificationStatus>,
      offset: u64,
      limit: u16,
    ) -> Vec<verification::types::VerificationRequest<AccountId>> {
      Verification::get_requests(vec![], status, Some(account), offset, limit)
    }
  }

  impl tipping_rpc_runtime_api::TippingApi<Block, Balance, AccountId, BlockNumber> for Runtime {
    fn total_received(account_id: AccountId, verification_context: verification::types::VerificationContext) -> Balance {
      Tipping::total_received(account_id, verification_context)
    }
    fn total(account_id: AccountId, verification_context: verification::types::VerificationContext) -> u64 {
      Tipping::total(account_id, verification_context)
    }
    fn get_tips (
      account_id: AccountId,
      verification_context: verification::types::VerificationContext,
      offset: u64,
      limit: u16,
    ) -> Vec<tipping::types::Tip<Balance, AccountId, BlockNumber>> {
      Tipping::get_tips(account_id, verification_context, offset, limit)
    }
  }

 impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
    fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
      ParachainSystem::collect_collation_info(header)
    }
  }

  #[cfg(feature = "try-runtime")]
  impl frame_try_runtime::TryRuntime<Block> for Runtime {
    fn on_runtime_upgrade() -> (Weight, Weight) {
      log::info!("try-runtime::on_runtime_upgrade parachain-template.");
      let weight = Executive::try_runtime_upgrade().unwrap();
      (weight, RuntimeBlockWeights::get().max_block)
    }

    fn execute_block(block: Block, state_root_check: bool, select: frame_try_runtime::TryStateSelect) -> Weight {
      log::info!(
        target: "runtime::parachain-template", "try-runtime: executing block #{} ({:?}) / root checks: {:?} / sanity-checks: {:?}",
        block.header.number,
        block.header.hash(),
        state_root_check,
        select,
      );
      Executive::try_execute_block(block, state_root_check, select).expect("try_execute_block failed")
    }
  }


  #[cfg(feature = "runtime-benchmarks")]
  impl frame_benchmarking::Benchmark<Block> for Runtime {
    fn benchmark_metadata(extra: bool) -> (
      Vec<frame_benchmarking::BenchmarkList>,
      Vec<frame_support::traits::StorageInfo>,
    ) {
      use frame_benchmarking::{Benchmarking, BenchmarkList};
      use frame_support::traits::StorageInfoTrait;
      use frame_system_benchmarking::Pallet as SystemBench;
      use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

      let mut list = Vec::<BenchmarkList>::new();
      list_benchmarks!(list, extra);

      let storage_info = AllPalletsWithSystem::storage_info();
      return (list, storage_info)
    }

    fn dispatch_benchmark(
      config: frame_benchmarking::BenchmarkConfig
    ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
      use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

      use frame_system_benchmarking::Pallet as SystemBench;
      impl frame_system_benchmarking::Config for Runtime {}

      use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
      impl cumulus_pallet_session_benchmarking::Config for Runtime {}

      let whitelist: Vec<TrackedStorageKey> = vec![
        // Block Number
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
        // Total Issuance
        hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
        // Execution Phase
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
        // Event Count
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
        // System Events
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
      ];

      let mut batches = Vec::<BenchmarkBatch>::new();
      let params = (&config, &whitelist);
      add_benchmarks!(params, batches);

      if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
      Ok(batches)
    }
  }
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
  fn check_inherents(
    block: &Block,
    relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
  ) -> sp_inherents::CheckInherentsResult {
    let relay_chain_slot = relay_state_proof
      .read_slot()
      .expect("Could not read the relay chain slot from the proof");

    let inherent_data = cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
      relay_chain_slot,
      sp_std::time::Duration::from_secs(6),
    )
    .create_inherent_data()
    .expect("Could not create the timestamp inherent data");

    inherent_data.check_extrinsics(block)
  }
}

cumulus_pallet_parachain_system::register_validate_block! {
  Runtime = Runtime,
  BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
  CheckInherents = CheckInherents,
}
