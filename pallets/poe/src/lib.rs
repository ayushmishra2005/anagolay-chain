#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
// use frame_support::debug::native;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};
use system::ensure_signed;

///The pallet's configuration trait.
pub trait Trait: system::Trait {
  /// The overarching event type.
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as PoEModule
  {
    /// [Proof, AccountId BlockNumber]
    Proofs get(proof): map hasher(blake2_128_concat) T::Hash=> (Proof, T::AccountId, T::BlockNumber);
    /// [Rule, AccountId BlockNumber]
    Rules get(rule):  map hasher(blake2_128_concat) T::Hash => (Rule, T::AccountId, T::BlockNumber);
  }
}

/// PoE Proof
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof {
  id: Vec<u8>,   // hexEncode(cid(body))
  body: Vec<u8>, // this is generic now, a string, make it GENERIC rust way.
  created_at: u64,
  prev: Vec<u8>,
  rule_id: Vec<u8>, // which rule is executed
  for_what: ForWhat,
}
// This is how payload can look like
// const body = {
//   ruleId: string,
//   params: string, // JSON.stringify(executedOps.op[])
//   owner: string, // this will be a DID or URN
//   forWhat: forWhat,
// };

/// implement default
impl Default for Proof {
  fn default() -> Self {
    Proof {
      id: b"".to_vec(),
      body: b"".to_vec(),
      created_at: 0, // millis
      prev: b"".to_vec(),
      rule_id: b"".to_vec(),
      for_what: ForWhat::default(),
    }
  }
}

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
enum ForWhat {
  /// general hash of a payload
  Generic = 0,
  /// Any photo
  Photo = 1,
  /// Any camera, not a smartphone
  Camera = 2,
  /// Any Lens
  Lens = 3,
  /// Any Smartphone
  SmartPhone = 4,
}

impl Default for ForWhat {
  fn default() -> Self {
    ForWhat::Generic
  }
}

/// Operations that will be performed
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct Operation {
  op: Vec<u8>,
  name: Vec<u8>,
  desc: Vec<u8>,
  hash_algo: Vec<u8>,
  hash_bits: u32,
  encode_algo: Vec<u8>,
  prefix: Vec<u8>,
  ops: Vec<Operation>, // you can  use the ops to build more complex rule
}

impl Default for Operation {
  fn default() -> Self {
    Operation {
      op: b"".to_vec(),
      name: b"".to_vec(),
      desc: b"".to_vec(),
      hash_algo: b"blake2b".to_vec(),
      hash_bits: 256,
      encode_algo: b"hex".to_vec(),
      prefix: b"0x".to_vec(),
      ops: vec![],
    }
  }
}
/// Rule which must be applied to the PoE
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleBody {
  version: u32,
  name: Vec<u8>,
  description: Vec<u8>,
  creator: Vec<u8>,
  for_what: ForWhat,
  parent: Vec<u8>,
  ops: Vec<Operation>,
  build_params: Operation,
  create_proof: Operation,
  // optional_number: Option<u32>,
}

/// Rule which must be applied to the PoE
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
  id: Vec<u8>, // a CID the body
  created_at: u64,
  prev: Vec<u8>,
  body: RuleBody,
}

impl Default for Rule {
  fn default() -> Self {
    Rule {
      id: b"".to_vec(),
      created_at: 0,
      prev: b"".to_vec(),
      body: RuleBody {
        version: 1,
        name: b"".to_vec(),
        description: b"".to_vec(),
        creator: b"".to_vec(),
        for_what: ForWhat::default(),
        parent: b"".to_vec(),
        ops: vec![],
        build_params: Operation {
          op: b"create_payload".to_vec(),
          name: b"Special func".to_vec(),
          desc: b"Special func description".to_vec(),
          hash_algo: b"blake2b".to_vec(),
          hash_bits: 256,
          encode_algo: b"hex".to_vec(),
          prefix: b"0x".to_vec(),
          ops: vec![],
        },
        create_proof: Operation {
          op: b"create_proof".to_vec(),
          name: b"How Proof should be created".to_vec(),
          desc: b"When applying this rule, use this to create the proof".to_vec(),
          hash_algo: b"blake2b".to_vec(),
          hash_bits: 256,
          encode_algo: b"hex".to_vec(),
          prefix: b"0x".to_vec(),
          ops: vec![],
        },
      },
    }
  }
}

/// Default values for this runtime
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct DefaultValues {
  hash_algo: Vec<u8>,
  hash_bits: u32,
  encoding_algo: Vec<u8>,
  encoding_prefix: Vec<u8>,
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        /// Default values for the poe, like encoding scheme and hashing functions
        const defaults: DefaultValues = DefaultValues{
          hash_algo : b"blake2b".to_vec(),
          hash_bits : 256,
          encoding_algo : b"hex".to_vec(),
          encoding_prefix : b"0x".to_vec(),
        };

        // Initializing errors
        type Error = Error<T>;

        // Initializing events
        fn deposit_event() = default;

        /// Create Rule
        fn create_rule ( origin, rule:Rule ) {
            let sender = ensure_signed(origin)?;

            let rule_id = rule.id.clone();
            let rule_id_hash = rule_id.using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!Rules::<T>::contains_key(&rule_id_hash), Error::<T>::RuleAlreadyCreated);

            let current_block = <system::Module<T>>::block_number();

            Rules::<T>::insert(&rule_id_hash, ( rule, sender.clone(), current_block));

            // deposit the event
            Self::deposit_event(RawEvent::RuleCreated(sender, rule_id));
        }

        /// Create proof and claim
        fn create_proof(origin, proof: Proof) {
          let sender = ensure_signed(origin)?;

          let rule_id = proof.rule_id.clone();
          let rule_id_hash = rule_id.using_encoded(<T as system::Trait>::Hashing::hash);

          let rule = Rules::<T>::get(rule_id_hash);

          // native::info!("My rule_id: {:?}", rule);
          ensure!(Rules::<T>::contains_key(&rule_id_hash), Error::<T>::NoSuchRule);

          // the 0 comes from first element in the tuple value -- storage
          if proof.for_what != rule.0.body.for_what  {
            ensure!(false, Error::<T>::TypeForClaimRuleMismatch);
          }

          let id = proof.id.clone();

          // Check is Proof claimed already
          let proof_hash = id.using_encoded(<T as system::Trait>::Hashing::hash);
          ensure!(!Proofs::<T>::contains_key(&proof_hash), Error::<T>::ProofAlreadyClaimed);

          // Call the `system` pallet to get the current block number
          let current_block = <system::Module<T>>::block_number();

          // native::info!("My proof: {:?}", &proof);

          // Store the proof with the sender and the current block number

          <Proofs::<T>>::insert(&proof_hash, ( proof, sender.clone(), current_block));
          // Emit an event that the claim was created
          Self::deposit_event(RawEvent::ProofCreated(sender, id));
      }
  }
}

// The pallet's errors
decl_error! {
  pub enum Error for Module<T: Trait> {
      ///Value was None
      NoneValue,
      ///Value reached maximum and cannot be incremented further
      StorageOverflow,
       ///This proof has already been claimed
      ProofAlreadyClaimed,
      ///The proof does not exist, so it cannot be revoked
      NoSuchProof,
      ///The proof is claimed by another account, so caller can't revoke it
      NotProofOwner,
      ///ForWhat mismatch
      TypeForClaimRuleMismatch,
      ///Rule already exists
      RuleAlreadyCreated,
      ///Rule doesn't exits, create one.
      NoSuchRule
  }
}
// The pallet's events
decl_event!(
  pub enum Event<T>
  where
    AccountId = <T as system::Trait>::AccountId,
  {
    ///Proof is created and claimed
    ProofCreated(AccountId, Vec<u8>),
    ///Rule is created
    RuleCreated(AccountId, Vec<u8>),
  }
);

#[cfg(test)]
mod tests {
  use super::*;

  use frame_support::{
    assert_noop, assert_ok, impl_outer_origin, parameter_types, weights::Weight,
  };
  use sp_core::H256;
  use sp_io::TestExternalities;
  use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
  };

  impl_outer_origin! {
      pub enum Origin for Test {}
  }

  // For testing the pallet, we construct most of a mock runtime. This means
  // first constructing a configuration type (`Test`) which `impl`s each of the
  // configuration traits of pallets we want to use.
  #[derive(Clone, Eq, PartialEq)]
  pub struct Test;
  parameter_types! {
      pub const BlockHashCount: u64 = 250;
      pub const MaximumBlockWeight: Weight = 1024;
      pub const MaximumBlockLength: u32 = 2 * 1024;
      pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
  }
  impl system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type ModuleToIndex = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
  }

  impl Trait for Test {
    type Event = ();
  }

  pub type Poe = Module<Test>;

  pub struct ExtBuilder;
  impl ExtBuilder {
    pub fn build() -> TestExternalities {
      let storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
      TestExternalities::from(storage)
    }
  }

  #[test]
  fn rule_create_default() {
    ExtBuilder::build().execute_with(|| {
      let mut r = Rule::default();
      r.id = b"dummy-text".to_vec();

      let res = Poe::create_rule(Origin::signed(1), r);
      assert_ok!(res)
    });
  }
  #[test]
  fn rule_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
      let mut r1 = Rule::default();
      r1.id = b"dummy-text".to_vec();

      let res1 = Poe::create_rule(Origin::signed(1), r1.clone());
      assert_ok!(res1);

      let res2 = Poe::create_rule(Origin::signed(1), r1);
      assert_noop!(res2, Error::<Test>::RuleAlreadyCreated);
    });
  }
  #[test]
  fn proof_create_default() {
    ExtBuilder::build().execute_with(|| {
      // todo create default rule, figure out better way
      let mut r = Rule::default();
      r.id = b"dummy-text".to_vec();
      let res = Poe::create_rule(Origin::signed(1), r.clone());
      assert_ok!(res);
      // todo create default rule, figure out better way

      let mut proof = Proof::default();
      proof.id = b"proof-id".to_vec();
      proof.rule_id = r.id.clone();

      let res = Poe::create_proof(Origin::signed(1), proof.clone());
      assert_ok!(res)
    });
  }
  #[test]
  fn proof_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
      // todo create default rule, figure out better way
      let mut r = Rule::default();
      r.id = b"dummy-text".to_vec();

      let res = Poe::create_rule(Origin::signed(1), r.clone());
      assert_ok!(res);
      // todo create default rule, figure out better way

      let mut proof = Proof::default();
      proof.id = b"proof-id".to_vec();
      proof.rule_id = r.id.clone();
      // create the proof
      let res1 = Poe::create_proof(Origin::signed(1), proof.clone());

      assert_ok!(res1);

      // create the proof AGAIN
      let res2 = Poe::create_proof(Origin::signed(1), proof.clone());

      assert_noop!(res2, Error::<Test>::ProofAlreadyClaimed);
    });
  }

  #[test]
  fn proof_error_on_no_rule() {
    ExtBuilder::build().execute_with(|| {
      let mut proof = Proof::default();
      proof.id = b"proof-id".to_vec();
      let rule_id = b"dummy-text-never-created".to_vec();
      proof.rule_id = rule_id;
      let res = Poe::create_proof(Origin::signed(1), proof);
      assert_noop!(res, Error::<Test>::NoSuchRule);
    });
  }
  #[test]
  fn proof_error_on_for_what_mismatch() {
    ExtBuilder::build().execute_with(|| {
      // todo create default rule, figure out better way
      let mut r = Rule::default();
      r.id = b"dummy-text".to_vec();
      r.body.for_what = ForWhat::Generic;
      let res = Poe::create_rule(Origin::signed(1), r.clone());
      assert_ok!(res);
      // todo create default rule, figure out better way

      let mut proof = Proof::default();
      proof.id = b"proof-id".to_vec();
      proof.rule_id = r.id.clone();
      proof.for_what = ForWhat::Photo;

      let res = Poe::create_proof(Origin::signed(1), proof);
      assert_noop!(res, Error::<Test>::TypeForClaimRuleMismatch);
    });
  }
  #[test]
  fn test_template() {
    ExtBuilder::build().execute_with(|| {});
  }
}
