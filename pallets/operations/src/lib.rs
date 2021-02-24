#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::{self as system, ensure_signed};
use sensio::{ForWhat, GenericId};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

// use frame_support::debug;

mod mock;
mod tests;

///The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// Not sure what is this.... investigate
pub const PERCEPTUAL_HASH_NAME: &[u8] = b"perceptual_hash";

/// Input params for a generated implementation
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct CustomInputParam {
    ///  'SnByteArray' | 'ProofParams[]' | 'SnBoolean'
    data: Vec<u8>,
    /// The real data type check the outputDecoded in sensio SDK, for more info check the https://gitlab.com/anagolay/node/-/issues/27
    decoded: Vec<u8>,
}

///Operation output definition, more info here https://gitlab.com/anagolay/node/-/issues/27
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationOutput {
    desc: Vec<u8>,
    ///Any valid type from the chain, written as string and converted to the appropriate type in the implementation
    output: Vec<u8>,
    decoded: Vec<u8>,
}
///Operation child output. It can be literally any byte array
pub type ChildOutput = Vec<u8>;

/// Operation Info, this is what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationInfo<A, B> {
    operation: Operation,
    account_id: A,
    block_number: B,
}

/// Operation definition
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationData {
    /// max 128(0.12kb) characters, slugify to use _
    name: Vec<u8>,
    /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
    desc: Vec<u8>,
    /// what operation accepts in the implementation. these are the params of the function with the types
    input: Vec<CustomInputParam>,
    output: OperationOutput,
    hashing_op: Vec<u8>,
    enc_op: Vec<u8>,
    groups: Vec<ForWhat>,
    /// this is the sum of all ops and the ops of the ops. tells how many operations this operation has. Based on this number we will decide which op is going to be executed first. This also tells which op has the longest chain or the deepest child op
    priority: u32,
    /// you can use the ops to build more complex rule and more complex op
    ops: Vec<Operation>,
}

impl Default for OperationData {
    fn default() -> Self {
        OperationData {
            name: b"".to_vec(),
            desc: b"".to_vec(),
            input: vec![],
            output: OperationOutput::default(),
            hashing_op: b"sn_cid".to_vec(),
            enc_op: b"sn_enc_hex".to_vec(),
            groups: vec![ForWhat::SYS],
            priority: 0,
            ops: vec![],
        }
    }
}

/// Operation structure
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Operation {
    id: GenericId,
    data: OperationData,
}

impl Default for Operation {
    fn default() -> Self {
        let data = OperationData::default();

        Operation {
            id: b"".to_vec(),
            data,
        }
    }
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

    trait Store for Module<T: Trait> as OperationStorage
    {
        /// Operations
        pub Operations get(fn operation):  double_map hasher(blake2_128_concat) GenericId, hasher(twox_64_concat) T::AccountId => OperationInfo<T::AccountId, T::BlockNumber>;
        pub OperationCount get(fn operation_count): u64;
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

        /// Create Operation
        #[weight = 10_000]
        fn create(origin, data: Operation) {
            let sender = ensure_signed(origin.clone())?;

            let current_block = <system::Module<T>>::block_number();

            let op_info = Self::build_operation(&data, &sender, &current_block);

            ensure!(!Operations::<T>::contains_key(&op_info.operation.id, &sender), Error::<T>::OperationAlreadyExists);

            Self::insert_operation(&op_info, &sender);


            // Emit an event when operation is created
            Self::deposit_event(RawEvent::OperationCreated(sender, op_info.operation.id.clone()));
        }
    }
}

// The pallet's errors
decl_error! {
  pub enum Error for Module<T: Trait> {

    ///Operation Already exists
    OperationAlreadyExists,
  }
}
// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ///Operation Created
        OperationCreated(AccountId, GenericId),
        ///Default Operations Created
        DefaultOperationCreated(AccountId, u64),
    }
);

impl<T: Trait> Module<T> {
    /// Increase the Rule count
    fn increase_operation_count() -> u64 {
        let rule_count = Self::operation_count();
        let new_rule_count = &rule_count + 1;
        OperationCount::put(new_rule_count);

        new_rule_count
    }
    ///Build the operation structure, this calculates the CID from the data
    fn build_operation(
        data: &Operation,
        account_id: &T::AccountId,
        block_number: &T::BlockNumber,
    ) -> OperationInfo<T::AccountId, T::BlockNumber> {
        // groups cannot contain duplicate values
        // the values are going to be sorted by the index, the way how they are defined in the ForWhat
        // let mut groups = data.groups.clone();
        // groups.sort();
        // groups.dedup();

        // // update the struct
        // let op_data = OperationData {
        //     groups,
        //     ..data.clone()
        // };

        let op_info = OperationInfo {
            operation: data.clone(),
            account_id: account_id.clone(),
            block_number: block_number.clone(),
        };
        op_info
    }

    ///Insert the operation to the Storage
    fn insert_operation(
        data: &OperationInfo<T::AccountId, T::BlockNumber>,
        account_id: &T::AccountId,
    ) {
        Operations::<T>::insert(&data.operation.id, &account_id, data);
        Self::increase_operation_count();
    }
}
