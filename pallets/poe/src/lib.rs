#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::debug::native;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{traits::Hash, RuntimeDebug};
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};
use system::ensure_signed;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
  /// The overarching event type.
  type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
  // /// Generic rule trait
  // type Rule: Codec + Default + Copy + EncodeLike;
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as PoEModule
  {
    // https://github.com/paritytech/substrate/blob/c34e0641abe52249866b62fdb0c2aeed41903be4/frame/support/procedural/src/lib.rs#L132
    Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, T::Hash);
    // Rules: map hasher(blake2_128_concat) Vec<u8> => Rule;

    // We removed the creator field in favor for the current structure
    // Maybe later it will be useful
    // creator: Vec<u8>, // this can be did:foo:barID or accountID on the blockchain
    Rules:  map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber, Vec<u8>);
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Proof<AccountId, Hash, Rule> {
  account_id: AccountId,
  impl_rule: Vec<Rule>,
  proof: Hash,
  payload: Hash,
  for_what: ForWhat,
}

// /// implement default
// impl<A, H, B, R> Default for Proof<A, H, B, R>
// where
//     A: Default,
//     H: Default,
//     B: Default,
//     R: Default,
// {
//     fn default() -> Self {
//         Proof {
//             account_id: A::default(),
//             block_number: B::default(),
//             proof: H::default(),
//             rules: R::default(),
//             content_hash: H::default(),
//             photo: true,
//         }
//     }
// }

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
    ForWhat::Photo
  }
}

/// Operations that will be performed
#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
struct Operation {
  op: Vec<u8>,
  desc: Vec<u8>,
  hash_algo: Vec<u8>,
  encode_algo: Vec<u8>,
  prefix: Vec<u8>,
  ops: Vec<Operation>, // you can  use the ops to build more complex operations
}

impl Default for Operation {
  fn default() -> Self {
    Operation {
      op: b"".to_vec(),
      desc: b"".to_vec(),
      hash_algo: b"blake2b-128".to_vec(),
      encode_algo: b"hex".to_vec(),
      prefix: b"0x".to_vec(),
      ops: vec![],
    }
  }
}

/// Rule which must be applied to the PoE
#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
  description: Vec<u8>,
  for_what: ForWhat,
  version: u32,
  creator: Vec<u8>,
  build_params: Operation,
  ops: Vec<Operation>,
  parent: Vec<u8>,
}

impl Default for Rule {
  fn default() -> Self {
    Rule {
      version: 0,
      description: b"".to_vec(),
      creator: b"".to_vec(),
      for_what: ForWhat::default(),
      parent: b"".to_vec(),
      ops: vec![],
      build_params: Operation {
        desc: b"Special func".to_vec(),
        op: b"create_payload".to_vec(),
        hash_algo: b"blake2b-128".to_vec(),
        encode_algo: b"hex".to_vec(),
        prefix: b"0x".to_vec(),
        ops: vec![],
      },
    }
  }
}
// The pallet's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

         /// Rules for the PoE
        // const rules: Vec<Rule> = vec![Rule {
        //     name: b"rule 1".to_vec(),
        //     version: 1,
        //     for_what: ForWhat::Photo,
        //     ops: vec![
        //         Operation {
        //             op: b"init".to_vec(),
        //             : b"object".to_vec(),
        //             output: true
        //         },


        //     ]
        // }];

        // Initializing errors
        // this includes information about your errors in the node's metadata.
        // it is needed only if you are using errors in your pallet
        type Error = Error<T>;

        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;

        /// create rule with decoding
        fn create_rule ( origin, rule_cid: Vec<u8>, payload: Vec<u8>) {
            let sender = ensure_signed(origin)?;

            ensure!(!Rules::<T>::contains_key(&rule_cid), Error::<T>::RuleAlreadyCreated);
            native::info!("My payload struct: {:?}", payload.to_vec());

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            Rules::<T>::insert(&rule_cid, (sender.clone(),current_block,  payload));


            // let proof = Rule::decode(&mut &payload[..]);
            // native::info!("My struct: {:?}", proof);

            Self::deposit_event(RawEvent::RuleCreated(sender, rule_cid.clone()));
        }

        /// Create rule with type
        // fn create_rule ( origin, rule_cid: Vec<u8>, payload: Rule) {
        //     let sender = ensure_signed(origin)?;

        //     ensure!(!Rules::contains_key(&rule_cid), Error::<T>::RuleAlreadyCreated);

        //     native::info!("My struct: {:?}", payload);

        //     Self::deposit_event(RawEvent::RuleCreated(sender, rule_cid));
        // }


         /// Allow a user to claim ownership of an unclaimed proof
        fn create_claim(origin, proof: Vec<u8>, _payload: Vec<u8>) {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;
            // let nonce = Self::nonce();
            // Verify that the specified proof has not been claimed yet or error with the message
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);


            let data_hash =<T as system::Trait>::Hashing::hash(b"s");

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();
            //
            // let p = Proof {
            //     account_id : sender.clone(),
            //     block_number: current_block,
            //     proof: proof.clone(),
            //     rules: "uri:ipfs:QM....".as_bytes().to_vec(),
            //     content_hash: data_hash.encode(),
            //     photo: true,
            // };
            //
            // // Store the proof with the sender and the current block number
            // <Proofs::<T>>::insert(&proof, p);
            <Proofs::<T>>::insert(&proof, (sender.clone(), current_block, data_hash));
            //
            // // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }


        /// Allow the owner to revoke their claim
        fn revoke_claim(origin, proof: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }
    }
}

// The pallet's errors
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Value was None
        NoneValue,
        /// Value reached maximum and cannot be incremented further
        StorageOverflow,
         /// This proof has already been claimed
        ProofAlreadyClaimed,
        /// The proof does not exist, so it cannot be revoked
        NoSuchProof,
        /// The proof is claimed by another account, so caller can't revoke it
        NotProofOwner,
        /// Rule already exists
        RuleAlreadyCreated,
    }
}

// The pallet's events
decl_event!(
  pub enum Event<T>
  where
    AccountId = <T as system::Trait>::AccountId,
  {
    /// Event emitted when a proof has been claimed.
    ClaimCreated(AccountId, Vec<u8>),
    /// Event emitted when a claim is revoked by the owner.
    ClaimRevoked(AccountId, Vec<u8>),
    /// Event emitted when a rule is created.
    RuleCreated(AccountId, Vec<u8>),
  }
);

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
