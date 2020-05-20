#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
// use frame_support::debug::native;
// use frame_support::debug;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};
use system::ensure_signed;

use cid::{Cid, Codec, Version};
use multibase::Base;
use multihash::Blake2b256;

///The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

pub const PERCEPTUAL_HASH_NAME: &[u8] = b"perceptual_hash";

/// Generic ID, this is the content identifier of the payload, like rule or proof
pub type PoeId = Vec<u8>;

/// Placeholder for SSI and DID
pub type CreatorId = Vec<u8>;

pub trait Skip {
    fn skip() -> Self;
}

/// key-value where key is Operation.op and value is fn(Operation)
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofParams {
    k: Vec<u8>, // key, encoded string
    v: Vec<u8>, // value, encoded string | number | float
}

/// Proof Incoming data
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofData {
    rule_id: PoeId, // which rule is executed
    prev: PoeId,
    creator: CreatorId,
    for_what: ForWhat,
    params: Vec<ProofParams>,
}

/// PoE Proof
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof {
    id: PoeId, // which rule is executed
    data: ProofData,
}
impl Default for Proof {
    fn default() -> Self {
        let data = ProofData {
            rule_id: PoeId::default(),
            prev: PoeId::default(),
            for_what: ForWhat::default(),
            creator: CreatorId::default(),
            params: vec![],
        };
        Proof {
            id: generate_cid(data.clone().encode()),
            data: data,
        }
    }
}

/// Proof Info, this is what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofInfo<Proof, AccountId, BlockNumber> {
    proof: Proof,
    account_id: AccountId,
    block_number: BlockNumber,
}

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
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

/// Operation Info, this is what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationInfo<A, B> {
    op: Operation,
    account_id: A,
    block_number: B,
}

/// Operation structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Operation {
    id: PoeId,
    data: OperationData,
}

/// Operations, they are used to build proofs
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationData {
    op: Vec<u8>,
    desc: Vec<u8>,
    hashing: DefaultsHashing,
    encoding: DefaultsEncoding,
    ops: Vec<Operation>, // you can use the ops to build more complex rule and more complex op
}

impl Default for OperationData {
    fn default() -> Self {
        OperationData {
            op: b"".to_vec(),
            desc: b"".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![],
        }
    }
}
impl Default for Operation {
    fn default() -> Self {
        let data = OperationData::default();
        Operation {
            id: generate_cid(data.clone().encode()),
            data: data,
        }
    }
}

/// Rule which must be applied to the PoE
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
    id: PoeId,
    data: RuleData,
}

impl Default for Rule {
    fn default() -> Self {
        let data = RuleData::default();
        Rule {
            id: generate_cid(data.encode()),
            data: data,
        }
    }
}

/// Rule Data, use this to generate rule_id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleData {
    version: i32,
    name: Vec<u8>,
    desc: Vec<u8>,
    creator: CreatorId,
    for_what: ForWhat,
    parent: PoeId,
    ops: Vec<Operation>,
}

impl Default for RuleData {
    fn default() -> Self {
        RuleData {
            version: 1,
            name: b"".to_vec(),
            desc: b"".to_vec(),
            creator: CreatorId::default(),
            for_what: ForWhat::default(),
            parent: b"".to_vec(),
            ops: vec![],
        }
    }
}

/// Rule Info, what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleInfo<A, B> {
    rule: Rule,
    account_id: A,
    block_number: B,
}

/// PHash Info, what gets stored
#[derive(Encode, Decode, Clone, PartialEq, Default, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct PhashInfo {
    p_hash: Vec<u8>,
    proof_id: PoeId,
}

/// Default values Hashing
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct DefaultsHashing {
    algo: Vec<u8>,
    bits: i32,
    skip: bool,
}

impl Default for DefaultsHashing {
    fn default() -> Self {
        DefaultsHashing {
            algo: b"blake2b".to_vec(),
            bits: 256,
            skip: false,
        }
    }
}

impl Skip for DefaultsHashing {
    fn skip() -> Self {
        DefaultsHashing {
            algo: b"".to_vec(),
            bits: 0,
            skip: true,
        }
    }
}

/// Default values Encoding
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct DefaultsEncoding {
    algo: Vec<u8>,
    prefix: bool,
}
impl Default for DefaultsEncoding {
    fn default() -> Self {
        DefaultsEncoding {
            algo: b"hex".to_vec(),
            prefix: true,
        }
    }
}

/// Default values Content Identifier or CID
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct DefaultsCid {
    version: i32,
    base: Vec<u8>,
    codec: Vec<u8>,
}
impl Default for DefaultsCid {
    fn default() -> Self {
        DefaultsCid {
            version: 1,
            base: b"base32".to_vec(),
            codec: b"raw".to_vec(),
        }
    }
}

/// Default values for this runtime
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct DefaultValues {
    hashing: DefaultsHashing,
    encoding: DefaultsEncoding,
    cid: DefaultsCid,
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as PoEModule
  {
    /// Perceptual hash finder hash(phash) : (PerceptualHash, ProofId)
    PHashes get(p_hashes): map hasher(blake2_128_concat) T::Hash => PhashInfo;

    /// PoE Proofs
    Proofs get(proofs): map hasher(blake2_128_concat) T::Hash=> ProofInfo<Proof, T::AccountId, T::BlockNumber>;

    /// PoE Rules
    Rules get(rules):  map hasher(blake2_128_concat) T::Hash => RuleInfo<T::AccountId, T::BlockNumber>;

    /// PoE Operations
    Operations get(operations):  map hasher(blake2_128_concat) T::Hash => OperationInfo<T::AccountId, T::BlockNumber>;
  }
}
// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        ///Default values for the poe, like encoding scheme and hashing functions
        const defaults: DefaultValues = DefaultValues {
          hashing: DefaultsHashing::default(),
          encoding: DefaultsEncoding::default(),
          cid: DefaultsCid::default()
        };

        ///Use this to build the params for the proof itself
        const build_params: OperationData = OperationData {
          op: b"build_params".to_vec(),
          desc: b"Use this to build the params for the proof itself".to_vec(),
          hashing: DefaultsHashing::default(),
          encoding: DefaultsEncoding::default(),
          ops: vec![],
        };

        ///Default set of operations for the rules. Move this to genesis
        const default_operations: Vec<OperationData> = vec![
          OperationData {
            desc: b"Generic blake2b 256 hashing op. Any kind of data".to_vec(),
            op: b"generic_hash".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract SerialNumber from Metadata".to_vec(),
            op: b"meta_serial_number".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract Make from Metadata".to_vec(),
            op: b"meta_make".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract Model from Metadata".to_vec(),
            op: b"meta_model".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract LensSerialNumber from Metadata".to_vec(),
            op: b"meta_lens_serial_number".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract LensInfo from Metadata".to_vec(),
            op: b"meta_lens_info".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Extract LensModel from Metadata".to_vec(),
            op: b"meta_lens_model".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Hash of full unchanged metadata buffer (or similar). Without raw pixels".to_vec(),
            op: b"metadata_hash".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Metadata must be removed and has must be created off of the RAW PIXELS".to_vec(),
            op: b"raw_pixels_hash".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Perceptual hash calculation, currently implementing http://blockhash.io/".to_vec(),
            op: PERCEPTUAL_HASH_NAME.to_vec(),
            hashing: DefaultsHashing::skip(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Document ID. The common identifier for all versions and renditions of a resource..to_vec() Found under xmp.did:GUID and parsed only the GUID part without the namespace xmp.did:".to_vec(),
            op: b"meta_document_id".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"Original Document ID. The common identifier for the original resource from which the.to_vec() current resource is derived. For example, if you save a resource to a different format, then save that one to another format, each save operationData should generate a new xmpMM:DocumentID that uniquely identifies the resource in that format, but should retain the ID of the source file here.".to_vec(),
            op: b"meta_original_document_id".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"XMP date time original field".to_vec(),
            op: b"meta_date_time_original".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"XMP create date".to_vec(),
            op: b"meta_create_date".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
          OperationData {
            desc: b"XMP copyright field".to_vec(),
            op: b"meta_copyright".to_vec(),
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            ops: vec![]
          },
        ];

        // Initializing errors
        type Error = Error<T>;

        // Initializing events
        fn deposit_event() = default;

        /// Create Rule
        fn create_rule ( origin, rule_data: RuleData ) {
            let sender = ensure_signed(origin)?;

            let rule_id = generate_cid(rule_data.clone().encode());
            let rule_id_hash = rule_id.using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!Rules::<T>::contains_key(&rule_id_hash), Error::<T>::RuleAlreadyCreated);

            let current_block = <system::Module<T>>::block_number();

            let rule = Rule {
              id: rule_id.clone(),
              data: rule_data.clone()
            };

            Rules::<T>::insert(&rule_id_hash, RuleInfo {
              rule: rule.clone(),
              account_id: sender.clone(),
              block_number: current_block,
            });

            // deposit the event
            Self::deposit_event(RawEvent::RuleCreated(sender, rule_id.clone()));
        }

        /// Create proof and claim
        fn create_proof(origin, proof_data: ProofData) {
          let sender = ensure_signed(origin.clone())?;

          let rule_id = &proof_data.rule_id;
          let rule_id_hash = rule_id.using_encoded(<T as system::Trait>::Hashing::hash);

          let rule_record = Self::rules(rule_id_hash);

          // Rule exists?
          ensure!(Rules::<T>::contains_key(&rule_id_hash), Error::<T>::NoSuchRule);

          // The types must match
          if proof_data.for_what != rule_record.rule.data.for_what  {
            ensure!(false, Error::<T>::ProofRuleTypeMismatch);
          }

          let proof_id = generate_cid(proof_data.encode());
          let proof_hash = proof_id.using_encoded(<T as system::Trait>::Hashing::hash);

          // Proof exists?
          ensure!(!Proofs::<T>::contains_key(&proof_hash), Error::<T>::ProofAlreadyClaimed);


          // Call the `system` pallet to get the current block number
          let current_block = <system::Module<T>>::block_number();

          let proof_info = ProofInfo {
            proof: Proof{
              id: proof_id.clone(),
              data: proof_data.clone()
            },
            account_id: sender.clone(),
            block_number: current_block.clone(),
          };

          Proofs::<T>::insert(&proof_hash, proof_info.clone());


          // Emit an event that the proof was created
          Self::deposit_event(RawEvent::ProofCreated(sender, proof_id.clone()));
      }

      /// Create Operation
      fn create_operation(origin, op_data: OperationData) {
        let sender = ensure_signed(origin)?;
        let current_block = <system::Module<T>>::block_number();

        let op_id = generate_cid(op_data.clone().encode());
        let local_id_hash = op_id.using_encoded(<T as system::Trait>::Hashing::hash);

        ensure!(!Operations::<T>::contains_key(&local_id_hash), Error::<T>::OperationAlreadyExists);

        let operation = Operation {
          id: op_id.clone(),
          data: op_data.clone(),
        };
        // debug::info!("OPERATION: {:?}, {:?}",&local_id_hash, &cid_str);
        let op_info = OperationInfo {
          op: operation.clone(),
          account_id: sender.clone(),
          block_number: current_block,
        };

        Operations::<T>::insert(local_id_hash, op_info);

        // Emit an event that the proof was created
        Self::deposit_event(RawEvent::OperationCreated(sender, op_id.clone()));
      }
      /// INDEX storage, save the connection phash <-> proofId for hamming/leven distance calc. Eventually refactor this, for now leave it
      fn save_phash(origin, payload_data: PhashInfo) {
        let sender = ensure_signed(origin)?;

        // Check is do we have the proof, can't add without
        let proof_hash = payload_data.proof_id.using_encoded(<T as system::Trait>::Hashing::hash);
        ensure!(Proofs::<T>::contains_key(&proof_hash), Error::<T>::NoSuchProof);


        let payload_data_digest = payload_data.using_encoded(<T as system::Trait>::Hashing::hash);

        ensure!(!PHashes::<T>::contains_key(&payload_data_digest), Error::<T>::PHashAndProofIdComboAlreadyExist);

        PHashes::<T>::insert(&payload_data_digest, payload_data.clone());

      // Emit an event that the proof was created
      Self::deposit_event(RawEvent::PhashCreated(sender, payload_data_digest));
    }
  }
}

/// Generate CID with multihash for a given input
fn generate_cid(data: Vec<u8>) -> Vec<u8> {
    // gen the multihash with our default algo and bits
    // NOTE don't forget to follow th default hashing algo
    let h = Blake2b256::digest(data.as_slice());

    // ALWAYS use V1, for now base32 is used 'coz it can't be changed
    // TODO double-check on the Codec::Raw
    let cid = Cid::new(Version::V1, Codec::Raw, h).unwrap();

    // create the string slice like `bafk...`
    let cid_str = multibase::encode(Base::Base32Lower, cid.to_bytes().as_slice());

    // make the string slice into vec bytes, usually we use that
    cid_str.into_bytes()
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
      ProofRuleTypeMismatch,
      ///Rule already exists
      RuleAlreadyCreated,
      ///Rule doesn't exits, create one.
      NoSuchRule,
      ///PHash + ProofId already exist
      PHashAndProofIdComboAlreadyExist,
      ///Operation Already exists
      OperationAlreadyExists,
  }
}
// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Hash = <T as system::Trait>::Hash,
    {
        ///Proof is created and claimed
        ProofCreated(AccountId, PoeId),
        ///Rule is created
        RuleCreated(AccountId, PoeId),
        /// Phash is created
        PhashCreated(AccountId, Hash),
        /// Default Operations Created
        OperationCreated(AccountId, PoeId),
    }
);

// impl<T: Trait> Module<T> {
//   /// Get the Rule with operations
//   fn rule(rule_id: T::Hash) -> Rule {
//     let rule_id_hash = rule_id.using_encoded(<T as system::Trait>::Hashing::hash);
//     let rule_info = Rules::<T>::get(rule_id_hash);
//     let mut rule_cloned = rule_info.rule.clone();
//     let mut ops: Vec<Operation> = vec![];
//     for (_, e) in rule_info.rule.ops.iter().enumerate() {
//       let id = e.using_encoded(<T as system::Trait>::Hashing::hash);
//       let op_info = Operations::<T>::get(id);
//       ops.push(op_info.op)
//     }

//     rule_cloned.ops = ops;
//     rule_cloned
//   }
// }

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

    const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

    pub fn build_default_proof(rule_id: Vec<u8>) -> Proof {
        let mut proof = Proof::default();
        proof.data.rule_id = rule_id;
        proof.id = generate_cid(proof.data.clone().encode());
        proof
    }

    pub fn create_default_rule() -> Rule {
        let rule = Rule::default();
        rule
    }

    #[test]
    fn rule_create_default() {
        ExtBuilder::build().execute_with(|| {
            let r = Rule::default();
            let res = Poe::create_rule(Origin::signed(1), r.data);
            assert_ok!(res)
        });
    }
    #[test]
    fn rule_error_on_duplicate() {
        ExtBuilder::build().execute_with(|| {
            let r1 = Rule::default();
            let res1 = Poe::create_rule(Origin::signed(1), r1.data.clone());
            assert_ok!(res1);

            let res2 = Poe::create_rule(Origin::signed(1), r1.data);
            assert_noop!(res2, Error::<Test>::RuleAlreadyCreated);
        });
    }
    #[test]
    fn proof_create_default() {
        ExtBuilder::build().execute_with(|| {
            // todo create default rule, figure out better way
            let r = create_default_rule();
            let res = Poe::create_rule(Origin::signed(1), r.data.clone());
            assert_ok!(res);
            // todo create default rule, figure out better way

            let pd = build_default_proof(r.id);
            let res = Poe::create_proof(Origin::signed(1), pd.data.clone());
            assert_ok!(res)
        });
    }
    #[test]
    fn proof_error_on_duplicate() {
        ExtBuilder::build().execute_with(|| {
            // todo create default rule, figure out better way
            let r = create_default_rule();
            let res = Poe::create_rule(Origin::signed(1), r.data.clone());
            assert_ok!(res);
            // todo create default rule, figure out better way

            // create the proof
            let pd = build_default_proof(r.id);
            let res1 = Poe::create_proof(Origin::signed(1), pd.data.clone());

            assert_ok!(res1);

            // create the proof AGAIN
            let res2 = Poe::create_proof(Origin::signed(1), pd.data.clone());

            assert_noop!(res2, Error::<Test>::ProofAlreadyClaimed);
        });
    }

    #[test]
    fn proof_error_on_no_rule() {
        ExtBuilder::build().execute_with(|| {
            let rule_id = b"dummy-text-never-created".to_vec();
            let pd = build_default_proof(rule_id);
            let res = Poe::create_proof(Origin::signed(1), pd.data.clone());
            assert_noop!(res, Error::<Test>::NoSuchRule);
        });
    }
    #[test]
    fn proof_error_on_for_what_mismatch() {
        ExtBuilder::build().execute_with(|| {
            // todo create default rule, figure out better way
            let mut r = create_default_rule();
            r.data.for_what = ForWhat::Generic;
            let res = Poe::create_rule(Origin::signed(1), r.data.clone());
            assert_ok!(res);
            // todo create default rule, figure out better way

            let mut pd = build_default_proof(r.id);
            pd.data.for_what = ForWhat::Photo;

            let res = Poe::create_proof(Origin::signed(1), pd.data.clone());
            assert_noop!(res, Error::<Test>::ProofRuleTypeMismatch);
        });
    }
    // #[test]
    // fn test_test_phash() {
    //   ExtBuilder::build().execute_with(|| {
    //     // todo create default rule, figure out better way
    //     let mut r = Rule::default();
    //       //     r.for_what = ForWhat::Generic;
    //     // r.ops = vec![
    //     //   Operation {
    //     //     op: b"meta_copyright".to_vec(),
    //     //     name: b"".to_vec(),
    //     //     desc: b"".to_vec(),
    //     //     hash_algo: b"blake2b".to_vec(),
    //     //     hash_bits: 256,
    //     //     enc: b"hex".to_vec(),
    //     //     prefix: b"0x".to_vec(),
    //     //     ops: vec![],
    //     //   },
    //     //   Operation {
    //     //     op: PERCEPTUAL_HASH_NAME.to_vec(),
    //     //     name: b"".to_vec(),
    //     //     desc: b"binary encoded, like 01110111".to_vec(),
    //     //     hash_algo: b"blake2b".to_vec(),
    //     //     hash_bits: 256,
    //     //     enc: b"hex".to_vec(),
    //     //     prefix: b"0x".to_vec(),
    //     //     ops: vec![],
    //     //   },
    //     // ];
    //     let res = Poe::create_rule(Origin::signed(1), r.data.clone());
    //     assert_ok!(res);
    //     let p_hash_name = b"perceptual_hash".to_vec();
    //     // todo create default rule, figure out better way
    //     // let phash_op = r.ops.iter().filter(|x| x.op = "perceptual_hash")
    //     for (pos, e) in r.ops.iter().enumerate() {
    //       if p_hash_name == e.op {
    //         println!("Element at position {}: {:?}", pos, e.op);
    //       }
    //     }
    //   });
    // }
    #[test]
    fn phash_save_phash() {
        ExtBuilder::build().execute_with(|| {
            // todo create default rule, figure out better way
            let r = create_default_rule();
            let res = Poe::create_rule(Origin::signed(1), r.data.clone());
            assert_ok!(res);
            // todo create default rule, figure out better way

            let pd = build_default_proof(r.id);
            let res = Poe::create_proof(Origin::signed(1), pd.data.clone());
            assert_ok!(res);

            let phash = PERCEPTUAL_HASH.clone().to_vec();
            let p_hash_payload = PhashInfo {
                p_hash: phash.clone(),
                proof_id: pd.id.clone(),
            };

            let res = Poe::save_phash(Origin::signed(1), p_hash_payload);
            assert_ok!(res);
        });
    }
    #[test]
    fn phash_save_phash_error_on_duplicate() {
        ExtBuilder::build().execute_with(|| {
            // todo create default rule, figure out better way
            let r = create_default_rule();
            let res = Poe::create_rule(Origin::signed(1), r.data.clone());
            assert_ok!(res);
            // todo create default rule, figure out better way

            let pd = build_default_proof(r.id);
            let res = Poe::create_proof(Origin::signed(1), pd.data.clone());
            assert_ok!(res);

            let phash = PERCEPTUAL_HASH.clone().to_vec();

            let p_hash_payload = PhashInfo {
                p_hash: phash.clone(),
                proof_id: pd.id.clone(),
            };

            let res = Poe::save_phash(Origin::signed(1), p_hash_payload.clone());
            assert_ok!(res);

            let res2 = Poe::save_phash(Origin::signed(1), p_hash_payload.clone());
            assert_noop!(res2, Error::<Test>::PHashAndProofIdComboAlreadyExist);
        });
    }
    #[test]
    fn phash_save_phash_error_no_proof() {
        ExtBuilder::build().execute_with(|| {
      let phash = PERCEPTUAL_HASH.clone().to_vec();

      let proof_id = b"0x6261666b32627a616365616d6c6e766678726c717175743274686f6b6c6a76726b68726f7a787562696a78746f3476743566646f776c6162747733686177".to_vec();

      let p_hash_payload = PhashInfo {
        p_hash: phash.clone(),
        proof_id: proof_id.clone(),
      };

      let res = Poe::save_phash(Origin::signed(1), p_hash_payload);
      assert_noop!(res, Error::<Test>::NoSuchProof);
    });
    }
    // #[test]
    // fn test_cid() {
    //   ExtBuilder::build().execute_with(|| {
    //     // Creating multihash from the struct in substrate
    //     // mainly testing the mh and cid variants
    //     // interesting, here to_string works but in the generate_cid it doesn't
    //     let op = Operation::default();
    //     let s = op.encode();
    //     let h = Blake2b256::digest(s.as_slice());

    //     let cid = Cid::new(Version::V1, Codec::Raw, h).unwrap();
    //     assert_eq!(
    //       cid.to_string(),
    //       "bafk2bzaceb3uveahln5rqipt55vraca7t4obmxwk7ewfklrri2ygep5dtxwnk"
    //     )
    //   });
    // }
    #[test]
    fn operations_create_operation() {
        ExtBuilder::build().execute_with(|| {
            let op = Operation::default();
            let res = Poe::create_operation(Origin::signed(1), op.data.clone());
            assert_ok!(res);
        });
    }
    #[test]
    fn operations_create_operation_error_on_duplicate() {
        ExtBuilder::build().execute_with(|| {
            let op = Operation::default();
            let res_first = Poe::create_operation(Origin::signed(1), op.data.clone());
            assert_ok!(res_first);

            let op = Operation::default();
            let res_duplicate = Poe::create_operation(Origin::signed(1), op.data.clone());
            assert_noop!(res_duplicate, Error::<Test>::OperationAlreadyExists);
        });
    }

    #[test]
    fn test_template() {
        ExtBuilder::build().execute_with(|| {});
    }
}
