#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch};
use frame_system::{self as system, ensure_signed};
use sp_std::cmp::{Ord, PartialOrd};
use sp_std::{clone::Clone, default::Default, vec::Vec};

///The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    // It is important to update your storage name so that your pallet's
    // storage items are isolated from other pallets.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as SensioModule {
        // Just a dummy storage item.
        // Here we are declaring a StorageValue, `Something` as a Option<u32>
        // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
        Something get(fn something): Option<u32>;
    }
}

// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Just a dummy event.
        /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
        /// To emit this event, we call the deposit function, from our runtime functions
        SomethingStored(u32, AccountId),
    }
);

/// Generic ID, this is the content identifier of the payload, like rule or proof. for now it's CID string
pub type GenericId = Vec<u8>;

/// Placeholder for SSI and DID
pub type CreatorId = Vec<u8>;

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub enum ForWhat {
    /// WE are creating it For what? This can be a part of the group
    GENERIC, // 0
    PHOTO,       // 1
    CAMERA,      // 2
    LENS,        // 3
    SMARTPHONE,  // 4
    USER,        // 5
    SYS,         // 6
    FLOWCONTROL, // 7
}

impl Default for ForWhat {
    fn default() -> Self {
        ForWhat::GENERIC
    }
}

/// Default values Hashing
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultsHashing {
    algo: Vec<u8>,
    bits: u32,
}

impl Default for DefaultsHashing {
    fn default() -> Self {
        DefaultsHashing {
            algo: b"blake2b".to_vec(),
            bits: 256,
        }
    }
}

/// Default values Encoding
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultsEncoding {
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
pub struct DefaultsCid {
    version: u8,
    base: Vec<u8>,
    codec: Vec<u8>,
}

impl Default for DefaultsCid {
    fn default() -> Self {
        DefaultsCid {
            version: 1,
            base: b"base32".to_vec(),
            codec: b"dag-cbor".to_vec(),
        }
    }
}

/// Default values for this runtime
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultValues {
    hashing: DefaultsHashing,
    encoding: DefaultsEncoding,
    cid: DefaultsCid,
}

impl Default for DefaultValues {
    fn default() -> Self {
        DefaultValues {
            hashing: DefaultsHashing::default(),
            encoding: DefaultsEncoding::default(),
            cid: DefaultsCid::default(),
        }
    }
}

decl_module! {

    /// The Sensio module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing errors
        type Error = Error<T>;
        // Initializing events
        // this is needed only if you are using events in your pallet
        fn deposit_event() = default;
        ///Default values for the poe, like encoding scheme and hashing functions
        const defaults: DefaultValues = DefaultValues::default();

        #[weight = 10_000]
        pub fn foo(origin, something: u32)-> dispatch::DispatchResult{
            // Check it was signed and get the signer. See also: ensure_root and ensure_none
            let who = ensure_signed(origin)?;

            // Code to execute when something calls this.
            // For example: the following line stores the passed in u32 in the storage
            Something::put(something);

            // Here we are raising the Something event
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
    }
}

// The pallet's errors
decl_error! {
  pub enum Error for Module<T: Trait> {
      ///Value was None
      NoneValue,
      /// Value reached maximum and cannot be incremented further
        StorageOverflow,
  }
}
