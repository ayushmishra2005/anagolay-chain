#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
// use frame_support::debug::native;
// use frame_support::debug;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::{self as system, ensure_signed};
use sensio::{CreatorId, ForWhat, GenericId};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

// Local files
mod mock;
mod tests;

///The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// Rule which must be applied
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Rule {
    pub id: GenericId,
    pub data: RuleData,
}

impl Default for Rule {
    fn default() -> Self {
        let data = RuleData::default();
        Rule {
            id: b"demo-cid".to_vec(),
            data,
        }
    }
}

///OperationReference by id instead of full
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationReference {
    id: GenericId,
    children: Vec<OperationReference>,
}

/// Rule Data, use this to generate rule_id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleData {
    /// Version, maybe we remove this, we will see
    pub version: u32,
    /// max 128(0.12kb) characters, slugify to use _
    pub name: Vec<u8>,
    /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
    pub desc: Vec<u8>,
    pub creator: CreatorId,
    pub groups: Vec<ForWhat>,
    pub parent_id: GenericId,
    pub ops: Vec<OperationReference>,
}

impl Default for RuleData {
    fn default() -> Self {
        RuleData {
            version: 1,
            name: b"".to_vec(),
            desc: b"".to_vec(),
            creator: CreatorId::default(),
            groups: vec![ForWhat::default()],
            parent_id: b"".to_vec(),
            ops: vec![],
        }
    }
}

/// Rule Info, what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct RuleInfo<A, B> {
    pub rule: Rule,
    pub account_id: A,
    pub block_number: B,
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as RuleStorage
  {
    /// Rules
    pub Rules get(fn rules):  double_map hasher(blake2_128_concat) GenericId , hasher(twox_64_concat) T::AccountId=> RuleInfo<T::AccountId, T::BlockNumber>;

    /// Amount of saved rules
    pub RuleCount get(fn rule_count): u32;
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

        /// Create Rule
        #[weight = 10_000]
        fn create_rule ( origin, rule: Rule ) {
            let sender = ensure_signed(origin)?;
            let current_block = <system::Module<T>>::block_number();


            ensure!(
                !Rules::<T>::contains_key(&rule.id, &sender),
                Error::<T>::RuleAlreadyCreated
            );

            let rule_info = Self::create(&sender, &current_block, &rule);

            // deposit the event
            Self::deposit_event(RawEvent::RuleCreated(sender, rule_info.rule.id.clone()));
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
      ///Rule already exists
      RuleAlreadyCreated,
      ///Rule doesn't exits, create one.
      NoSuchRule,
  }
}
// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ///Rule is created
        RuleCreated(AccountId, GenericId),
    }
);

/// Pallets external access to the saving in the storage
pub trait PutInStorage {}

impl<T: Trait> PutInStorage for Module<T> {}

impl<T: Trait> Module<T> {
    /// Increase the Rule count
    fn increase_rule_count() -> u32 {
        let rule_count = Self::rule_count();
        let new_rule_count = &rule_count + 1;
        RuleCount::put(new_rule_count);

        new_rule_count
    }

    /// Save the Rule to the Storage
    pub fn create(
        account_id: &T::AccountId,
        block_number: &T::BlockNumber,
        rule: &Rule,
    ) -> RuleInfo<T::AccountId, T::BlockNumber> {
        // Create the CID

        let rule_info = RuleInfo {
            rule: rule.clone(),
            account_id: account_id.clone(),
            block_number: block_number.clone(),
        };

        Rules::<T>::insert(&rule.id, &account_id, rule_info.clone());
        Self::increase_rule_count();

        rule_info
    }
}
