//! The Substrate Node Anagolay runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use grandpa::{fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
  create_runtime_str, generic, impl_opaque_keys,
  traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, IdentityLookup, NumberFor, Verify},
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature,
};
use sp_std::{marker::PhantomData, prelude::*};

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use frame_support::{
  construct_runtime, parameter_types,
  traits::{Get, KeyOwnerProofSystem, Randomness},
  weights::{
    constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
    Weight, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
  },
  StorageValue,
};

use smallvec::smallvec;
use transaction_payment::CurrencyAdapter;

pub mod constants;
pub use constants::{currency::*, time::*};

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
pub use timestamp::Call as TimestampCall;

/// Importing a anagolay pallet
pub use anagolay_support;

/// Importing a operations pallet
pub use operations;

/// Importing a statements pallet
// pub use an_statements;

/// Importing workflows pallet
pub use workflows;

/// Importing a poe pallet
// pub use an_poe;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
  use super::*;

  pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

  /// Opaque block header type.
  pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
  /// Opaque block type.
  pub type Block = generic::Block<Header, UncheckedExtrinsic>;
  /// Opaque block identifier type.
  pub type BlockId = generic::BlockId<Block>;

  impl_opaque_keys! {
      pub struct SessionKeys {
          pub aura: Aura,
          pub grandpa: Grandpa,
      }
  }
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
  spec_name: create_runtime_str!("node-anagolay"),
  impl_name: create_runtime_str!("node-anagolay"),
  authoring_version: 1,
  spec_version: 1,
  impl_version: 1,
  apis: RUNTIME_API_VERSIONS,
  transaction_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
  NativeVersion {
    runtime_version: VERSION,
    can_author_with: Default::default(),
  }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: system::limits::BlockWeights = system::limits::BlockWeights
        ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
    pub BlockLength: system::limits::BlockLength = system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 42;
    pub const Version: RuntimeVersion = VERSION;
}

impl system::Config for Runtime {
  /// The basic call filter to use in dispatchable.
  type BaseCallFilter = ();
  /// Block & extrinsics weights: base values and limits.
  type BlockWeights = BlockWeights;
  /// The maximum length of a block (in bytes).
  type BlockLength = BlockLength;
  /// The identifier used to distinguish between accounts.
  type AccountId = AccountId;
  /// The aggregated dispatch type that is available for extrinsics.
  type Call = Call;
  /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
  type Lookup = IdentityLookup<AccountId>;
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
  type Event = Event;
  /// The ubiquitous origin type.
  type Origin = Origin;
  /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
  type BlockHashCount = BlockHashCount;
  /// The weight of database operations that the runtime can invoke.
  type DbWeight = RocksDbWeight;
  /// Version of the runtime.
  type Version = Version;
  /// Converts a module to the index of the module in `construct_runtime!`.
  ///
  /// This type is being generated by `construct_runtime!`.
  type PalletInfo = PalletInfo;
  /// What to do if a new account is created.
  type OnNewAccount = ();
  /// What to do if an account is fully reaped from the system.
  type OnKilledAccount = ();
  /// The data to be stored in an account.
  type AccountData = balances::AccountData<Balance>;
  /// Weight information for the extrinsics of this pallet.
  type SystemWeightInfo = system::weights::SubstrateWeight<Runtime>;
  /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
  type SS58Prefix = SS58Prefix;
}

impl aura::Config for Runtime {
  type AuthorityId = AuraId;
}

impl grandpa::Config for Runtime {
  type Event = Event;
  type Call = Call;

  type KeyOwnerProofSystem = ();

  type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

  type KeyOwnerIdentification =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;

  type HandleEquivocation = ();

  type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl timestamp::Config for Runtime {
  /// A timestamp: milliseconds since the unix epoch.
  type Moment = Moment;
  type OnTimestampSet = Aura;
  type MinimumPeriod = MinimumPeriod;
  type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 10 * CENTS;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
}

impl balances::Config for Runtime {
  type MaxLocks = MaxLocks;
  /// The type for recording an account's balance.
  type Balance = Balance;
  /// The ubiquitous event type.
  type Event = Event;
  type DustRemoval = ();
  type ExistentialDeposit = ExistentialDeposit;
  type AccountStore = System;
  type WeightInfo = balances::weights::SubstrateWeight<Runtime>;
}

/// Convert from weight to fee via a simple coefficient multiplication. The associated type C
/// encapsulates an integer constant in units of balance per weight.
pub struct LinearWeightToFee<C>(PhantomData<C>);

impl<C> WeightToFeePolynomial for LinearWeightToFee<C>
where
  C: Get<Balance>,
{
  type Balance = Balance;

  fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
    let coefficient = WeightToFeeCoefficient {
      coeff_integer: C::get(),
      coeff_frac: Perbill::zero(),
      negative: false,
      degree: 1,
    };

    // Return a smallvec of coefficients. Order does not need to match degrees
    // because each coefficient has an explicit degree annotation.
    smallvec!(coefficient)
  }
}

parameter_types! {
    // Establish the byte-fee. It is used in all configurations.
    // pub const TransactionByteFee: Balance = 1;
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;

    // @TODO_FEES
    /// Used with LinearWeightToFee conversion.
    pub const FeeWeightRatio: u128 = 1;

}

impl transaction_payment::Config for Runtime {
  // The asset in which fees will be collected, and what to do with the imbalance.
  type OnChargeTransaction = CurrencyAdapter<Balances, ()>; // The balances pallet

  // Byte fee is multiplied by the length of the
  // serialized transaction in bytes
  type TransactionByteFee = TransactionByteFee;

  // Convert dispatch weight to a chargeable fee.
  // type WeightToFee = IdentityFee<Balance>;
  type WeightToFee = LinearWeightToFee<FeeWeightRatio>;

  //TODO
  type FeeMultiplierUpdate = ();
  //type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

impl pallet_utility::Config for Runtime {
  type Event = Event;
  type Call = Call;
  type WeightInfo = ();
}

impl sudo::Config for Runtime {
  type Event = Event;
  type Call = Call;
}

// Anagolay pallets:
// ------------------------------------------------------------------------------------------------
impl anagolay_support::Config for Runtime {}

impl operations::Config for Runtime {
  type Event = Event;
  type WeightInfo = operations::weights::AnagolayWeight<Runtime>;
  type TimeProvider = timestamp::Pallet<Runtime>;
}

//impl an_statements::Config for Runtime {
//  type Event = Event;
//  type WeightInfo = an_statements::weights::AnagolayWeight<Runtime>;
//}

impl workflows::Config for Runtime {
  type Event = Event;
  type WeightInfo = workflows::weights::AnagolayWeight<Runtime>;
  type TimeProvider = timestamp::Pallet<Runtime>;
}

//impl an_poe::Config for Runtime {
//  type Event = Event;
//  type ExternalRulesStorage = an_rules::Pallet<Runtime>;
//  type WeightInfo = an_poe::weights::AnagolayWeight<Runtime>;
//}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{Module, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: randomness_collective_flip::{Module, Call, Storage},
        Timestamp: timestamp::{Module, Call, Storage, Inherent},
        Aura: aura::{Module, Config<T>},
        Grandpa: grandpa::{Module, Call, Storage, Config, Event},
        Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: transaction_payment::{Module, Storage},
        Sudo: sudo::{Module, Call, Config<T>, Storage, Event<T>},

        // Customizations
        Utility: pallet_utility::{Module, Call, Event},

        // Used for the module anagolay
        Anagolay: anagolay_support::{Module},
        Operations: operations::{Module, Call, Storage, Event<T>, Config<T>},
//        Statements: an_statements::{Module, Call, Storage, Event<T>},
        Workflows: workflows::{Module, Call, Storage, Event<T>, Config<T>},
//        Poe: an_poe::{Module, Call, Storage, Event<T>},
    }
);

/// The address format for describing accounts.
pub type Address = AccountId;
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
  system::CheckSpecVersion<Runtime>,
  system::CheckTxVersion<Runtime>,
  system::CheckGenesis<Runtime>,
  system::CheckEra<Runtime>,
  system::CheckNonce<Runtime>,
  system::CheckWeight<Runtime>,
  transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllModules>;

impl_runtime_apis! {
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
            Runtime::metadata().into()
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

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed()
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }

        fn authorities() -> Vec<AuraId> {
            Aura::authorities()
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }

        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            impl frame_system_benchmarking::Config for Runtime {}

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

            add_benchmark!(params, batches, operations, Operations);

//            add_benchmark!(params, batches, an_poe, Poe);

            add_benchmark!(params, batches, workflows, Workflows);

//            add_benchmark!(params, batches, an_statements, Statements);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }
}
