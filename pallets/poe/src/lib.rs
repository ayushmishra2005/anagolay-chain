#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::debug::native;
// use frame_support::debug;
use frame_support::codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::{self as system, ensure_signed};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

use operations::Trait as OperationTrait;
use rules::{PutInStorage, Trait as RulesTrait};
use sensio::{CreatorId, ForWhat, GenericId};

mod mock;
mod tests;

///The pallet's configuration trait. Including the Operation trait too.
pub trait Trait: system::Trait + OperationTrait + RulesTrait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    // type ExternalRulesStorage: PutInStorage<Self::AccountId, Self::BlockNumber>;
    type ExternalRulesStorage: PutInStorage;
}

/// key-value where key is Operation.op and value is fn(Operation)
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofParams {
    /// Operation.name, hex encoded using Parity scale codec
    k: Vec<u8>,
    /// operation Output value serialized using cbor and represented as CID
    v: Vec<u8>,
}

/// Proof Incoming data
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct ProofData {
    rule_id: GenericId,
    // which rule is executed
    prev_id: GenericId,
    creator: CreatorId,
    groups: Vec<ForWhat>,
    // must be the same as for the rule
    params: Vec<ProofParams>,
}

impl Default for ProofData {
    fn default() -> Self {
        ProofData {
            rule_id: GenericId::default(),
            prev_id: GenericId::default(),
            groups: vec![ForWhat::default()],
            creator: CreatorId::default(),
            params: vec![],
        }
    }
}

/// PoE Proof
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof {
    id: GenericId,
    // which rule is executed
    data: ProofData,
}

impl Default for Proof {
    fn default() -> Self {
        let data = ProofData::default();
        Proof {
            id: b"".to_vec(),
            data,
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

/// PHash Info, what gets stored
#[derive(Encode, Decode, Clone, PartialEq, Default, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct PhashInfo {
    p_hash: Vec<u8>,
    proof_id: GenericId,
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as PoeStorage
  {
    /// Perceptual hash finder hash(phash) : (PerceptualHash, ProofId)
    PHashes get(fn p_hashes): double_map hasher(blake2_128_concat) T::Hash, hasher(twox_64_concat) T::AccountId => PhashInfo;

    /// PHashes count
    PHashCount get(fn phash_count): u128;

    /// PoE Proofs
    pub Proofs get(fn proofs): double_map hasher(blake2_128_concat) GenericId, hasher(twox_64_concat) T::AccountId => ProofInfo<Proof, T::AccountId, T::BlockNumber>;

    /// Proofs count
    ProofsCount get(fn proofs_count): u128;
  }
}

// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        type Error = Error<T>;

        // Initializing events
        fn deposit_event() = default;

        /// Create proof and claim
        #[weight = 10_000]
        fn create_proof(origin, proof: Proof) {
          let sender = ensure_signed(origin.clone())?;

          let rule_id = &proof.data.rule_id;

          let proof_id = proof.id.clone();

          let rule_record = rules::Module::<T>::rules(rule_id, &sender);

          // @TODO somehow figure this out. we don't need it NOW but must be done before the Milestone 2 is submitted
          // ensure!(&rule_record, Error::<T>::NoSuchRule);

          // The types must match
          if proof.data.groups != rule_record.rule.data.groups  {
            ensure!(false, Error::<T>::ProofRuleTypeMismatch);
          }


          // Proof exists?
          ensure!(!Proofs::<T>::contains_key(&proof_id,&sender), Error::<T>::ProofAlreadyClaimed);

          // Call the `system` pallet to get the current block number
          let current_block = <system::Module<T>>::block_number();

          let proof_info = ProofInfo {
            proof: proof.clone(),
            account_id: sender.clone(),
            block_number: current_block.clone(),
          };

          Proofs::<T>::insert(&proof_id, &sender, proof_info.clone());

          Self::increase_proof_count();

          // Emit an event that the proof was created
          Self::deposit_event(RawEvent::ProofCreated(sender, proof_id.clone()));
      }

      /// INDEX storage, save the connection phash <-> proofId for hamming/leven distance calc. Eventually refactor this, for now leave it
      #[weight = 10_000]
      fn save_phash(origin, payload_data: PhashInfo) {
        let sender = ensure_signed(origin)?;

        // Check is do we have the proof, can't add without
        ensure!(Proofs::<T>::contains_key(&payload_data.proof_id, &sender), Error::<T>::NoSuchProof);

        let payload_data_digest = payload_data.using_encoded(<T as system::Trait>::Hashing::hash);


        ensure!(!PHashes::<T>::contains_key(&payload_data_digest, &sender), Error::<T>::PHashAndProofIdComboAlreadyExist);

        PHashes::<T>::insert(&payload_data_digest, &sender, payload_data.clone());

        Self::increase_phash_count();

        // Emit an event that the proof was created
        Self::deposit_event(RawEvent::PhashCreated(sender, payload_data_digest));
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
        ProofRuleTypeMismatch,
        ///Proof Belongs to another account
        ProofBelongsToAnotherAccount,
        ///PHash + ProofId already exist
        PHashAndProofIdComboAlreadyExist,
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
        ProofCreated(AccountId, GenericId),
        /// Phash is created
        PhashCreated(AccountId, Hash),
    }
);

impl<T: Trait> Module<T> {
    fn increase_proof_count() -> u128 {
        let count = Self::proofs_count();
        let new_count = &count + 1;
        ProofsCount::put(new_count);
        new_count
    }
    fn increase_phash_count() -> u128 {
        let count = Self::phash_count();
        let new_count = &count + 1;
        PHashCount::put(new_count);
        new_count
    }
}
