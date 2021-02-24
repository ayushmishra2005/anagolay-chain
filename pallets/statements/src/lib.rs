#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
// use frame_support::debug::native;
use frame_support::debug;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use sensio::{CreatorId, GenericId};

use frame_system::{self as system, ensure_signed};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec::Vec};

mod mock;
mod tests;

const LOG: &str = "sensio";

///The pallet's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
/// Sensio Signature
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct SensioSignature {
    ///signing key in urn/did format 'urn:pgp:9cdf8dd38531511968c8d8cb524036585b62f15b'
    sig_key: Vec<u8>,
    ///Signature sign(prepared_statement, pvtKey(sigKey)) and encoded using multibase
    //https://gitlab.com/sensio_group/sensio-faas/-/blob/master/sp-api/src/plugins/copyright/helpers.ts#L76
    sig: Vec<u8>,
    ///Content identifier of the sig field -- CID(sig)
    cid: GenericId,
}

/// Sensio Signatures
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct SensioSignatures {
    holder: SensioSignature,
    issuer: SensioSignature,
}
/// Sensio Claim Proportion
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Proportion {
    /// Proportion sign, can be %
    sign: Vec<u8>,
    name: Vec<u8>,
    value: Vec<u8>,
}
/// Sensio Validity
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Validity {
    /// When the validity starts, this should be DATE_TIME
    from: Vec<u8>,
    /// When validity ends, this is calculate Validity.from + Expiration.value
    until: Vec<u8>,
}

///Possible Expiration types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub enum ExpirationType {
    FOREVER,
    YEARS,
    MONTHS,
    DAYS,
    MINUTES,
    SECONDS,
}

impl Default for ExpirationType {
    fn default() -> Self {
        ExpirationType::FOREVER
    }
}

/// Sensio Claim Expiration
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct Expiration {
    /// Proportion sign, can be %
    expiration_type: ExpirationType,
    ///How long is the expiration, if  ExpirationType::FOREVER then this is empty
    value: Vec<u8>,
}

///Sensio Claim types
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub enum SensioClaimType {
    COPYRIGHT,
    OWNERSHIP,
}

impl Default for SensioClaimType {
    fn default() -> Self {
        SensioClaimType::COPYRIGHT
    }
}

/// Sensio Generic Claim
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct SensioClaim {
    ///Prev Sensio Statement id in case this statement is revoked or changed
    prev_id: GenericId,
    ///PoE id of the record in question.
    poe_id: GenericId,
    ///Implemented rule
    rule_id: GenericId,
    ///In which proportion the statement is held
    proportion: Proportion,
    ///ATM this is the same as poe_id @TODO this should be unique representation of the subject that is NOT poe
    subject_id: GenericId,
    ///ATM this is the did representation of the substrate based account in format 'did:substrate:5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY/sensio-network', @NOTE this is part of the SENSIO ID which will come later this year
    holder: CreatorId,
    ///ATM this is the did representation of the substrate based account in format 'did:substrate:Hcd78R7frJfUZHsqgpPEBLeiCZxV29uyyyURaPxB71ojNjy/sensio-network', @NOTE this is part of the SENSIO ID which will come later this year
    issuer: Vec<u8>,
    /// Generic type, ATM is Copyright or Ownership
    claim_type: SensioClaimType,
    ///How long this statement is valid
    valid: Validity,
    /// Setting when the statement should end
    expiration: Expiration,
    ///What happens after the expiration? this is default rule or smart contract that automatically does stuff, like move it to the public domain, transfer to relatives etc... need better definition
    on_expiration: Vec<u8>,
}

/// Statement DB entry
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct StatementInfo<AccountId, BlockNumber> {
    ///Generated statement data
    statement: SensioStatement,
    account_id: AccountId,
    block_number: BlockNumber,
}

///Copyright data
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct StatementData {
    signatures: SensioSignatures,
    claim: SensioClaim,
}

/// Sensio copyright statement
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct SensioStatement {
    id: GenericId,
    data: StatementData,
}

// This pallet's storage items.
decl_storage! {
  // It is important to update your storage name so that your pallet's
  // storage items are isolated from other pallets.

  trait Store for Module<T: Trait> as StatementStorage
  {
    /// ALL statements
    pub Statements get(fn statements):  double_map hasher(blake2_128_concat) GenericId, hasher(twox_64_concat) T::AccountId => StatementInfo<T::AccountId, T::BlockNumber>;

    ///Statement to previous statement index table for quick check. The StatementB has a parent StatementA in `prev_id` field this will be
    ///Example:

    /// ```ts
    /// const aStatement = {
    ///   //   ... normal as the rest,
    ///   prev_id: ''
    /// }

    /// const bStatement = {
    ///   //  ... normal as the rest,
    ///   prev_id: aStatement.id
    /// }
    /// ```
    /// so this will be a map of bStatement.GenericId => aStatement.GenericId
    /// And now we try to revoke the `aStatement` it will fail, because it is the part of the `bStatement`
    pub StatementToPrevious get(fn prev_statement): map hasher(blake2_128_concat) GenericId => GenericId;

    /// Amount of saved Statements
    pub StatementsCount get(fn statements_count): u128;

    ///List of the statements connected to the Proof. If the statement claim is 100% then there will be only one entry, if it's not then as many entries is needed to get to 100%
    pub ProofValidStatements get (fn proof_valid_statement): map hasher(blake2_128_concat) GenericId => Vec<GenericId>;
  }
}

// The pallet's dispatchable functions.
decl_module! {
    ///The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        // Initializing errors
        type Error = Error<T>;

        // Initializing events
        fn deposit_event() = default;

        /// Create Copyright
        #[weight = 10_000]
        fn create_copyright ( origin, statement: SensioStatement )  {
            debug::RuntimeLogger::init();
            let sender = ensure_signed(origin)?;
            let current_block = <system::Module<T>>::block_number();

            // Statement must be type of copyright
            ensure!(statement.data.claim.claim_type == SensioClaimType::COPYRIGHT, Error::<T>::WrongClaimType );
            // Ensure that ProofCurrentStatements has or not the statement

            debug::debug!(target: LOG, "issue {:?}", statement);


            ensure!(statement.data.claim.prev_id.is_empty(),Error::<T>::CreatingChildStatementNotSupported );

            Self::check_statements_for_proof(&statement)?;

            // Do we have such a statement
            ensure!(!Statements::<T>::contains_key(&statement.id, &sender), Error::<T>::CopyrightAlreadyCreated);

            //@FUCK this needs fixing, it's a work-around for https://gitlab.com/anagolay/node/-/issues/31
            let statement_info = Self::build_statement_info(&statement, &sender, &current_block);
            Self::add_statement_to_proof(&statement.data.claim.poe_id, &statement.id)?;
            Self::insert_statement(&statement_info, &sender);

            // Emit an event when operation is created
            Self::deposit_event(RawEvent::CopyrightCreated(sender,statement.id.clone()));
        }

        /// Create Ownership
        #[weight = 10_000]
        fn create_ownership ( origin, statement: SensioStatement ) {
            let sender = ensure_signed(origin)?;
            let current_block = <system::Module<T>>::block_number();

            // Statement must be type of copyright
            ensure!(statement.data.claim.claim_type == SensioClaimType::OWNERSHIP,Error::<T>::WrongClaimType );

            ensure!(statement.data.claim.prev_id.is_empty(),Error::<T>::CreatingChildStatementNotSupported );
             // Ensure that ProofCurrentStatements has or not the statement
            Self::check_statements_for_proof(&statement)?;

            // Do we have such a statement
            ensure!(!Statements::<T>::contains_key(&statement.id, &sender), Error::<T>::OwnershipAlreadyCreated);

            //@FUCK this needs fixing, it's a work-around for https://gitlab.com/anagolay/node/-/issues/31
            let statement_info = Self::build_statement_info(&statement, &sender, &current_block);
            Self::add_statement_to_proof(&statement.data.claim.poe_id, &statement.id)?;
            Self::insert_statement(&statement_info, &sender);

            // Emit an event when operation is created
            Self::deposit_event(RawEvent::OwnershipCreated(sender, statement.id.clone()));
        }
        /// Allow the owner to revoke their statement.
        #[weight = 10_000]
        fn revoke (origin, statement_id: GenericId) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let sender = ensure_signed(origin)?;

            ensure!(StatementToPrevious::contains_key(&statement_id), Error::<T>::StatementHasChildStatement);

            // Verify that the specified statement has been claimed.
            ensure!(Statements::<T>::contains_key(&statement_id, &sender), Error::<T>::NoSuchStatement);

            Self::remove_statement(&statement_id, &sender)?;
            // Emit an event that the claim was erased.
            Self::deposit_event(RawEvent::StatementRevoked(sender, statement_id));
        }
        /// Revoke ALL statements -- test only
        #[weight = 10_000_000]
        fn revoke_all (origin) {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let _sender = ensure_signed(origin)?;

            // Emit an event that the claim was erased.

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
        /// Copyright already exists
        CopyrightAlreadyCreated,
        /// Ownership already exists
        OwnershipAlreadyCreated,
        /// Copyright doesn't exits, create one.
        NoSuchCopyright,
        /// Copyright doesn't exist
        CopyrightDoesntExist,
        /// Wrong claim type
        WrongClaimType,
        /// Proof already has this statement
        ProofHasStatement,
        /// Statement already exist
        StatementExist,
        /// Statement doesn't exits.
        NoSuchStatement,
        /// Statement has child statement and ite cannot be revoked
        StatementHasChildStatement,
        /// Create child statement is not yet supported
        CreatingChildStatementNotSupported,
    }
}
// The pallet's events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        ///Statement is created
        StatementCreated(AccountId, GenericId),

        ///Copyright is created
        CopyrightCreated(AccountId, GenericId),

        ///Copyright exists
        CopyrightExists(AccountId, GenericId),

        ///Ownership is created
        OwnershipCreated(AccountId, GenericId),

        ///Ownership exists
        OwnershipExists(AccountId, GenericId),

        ///Statement revoked
        StatementRevoked(AccountId, GenericId),
    }
);

impl<T: Trait> Module<T> {
    /// Increase the statements count
    fn decrease_statements_count() {
        StatementsCount::mutate(|v| *v -= 1);
    }

    /// Increase the statements count
    fn increase_statements_count() {
        StatementsCount::mutate(|v| *v += 1);
    }

    /// Remove the statement from the Storage
    fn remove_statement(
        statement_id: &GenericId,
        account_id: &T::AccountId,
    ) -> Result<bool, Error<T>> {
        let statement_info: StatementInfo<T::AccountId, T::BlockNumber> =
            Statements::<T>::get(&statement_id, &account_id);
        Self::remove_statement_from_proof(
            &statement_info.statement.data.claim.poe_id,
            &statement_info.statement.id,
        )?;
        Statements::<T>::remove(&statement_id, &account_id);
        Self::decrease_statements_count();
        Ok(true)
    }

    /// Insert the operation to the Storage
    fn insert_statement(
        data: &StatementInfo<T::AccountId, T::BlockNumber>,
        account_id: &T::AccountId,
    ) {
        Statements::<T>::insert(&data.statement.id, &account_id, data.clone());
        Self::increase_statements_count();
    }

    ///Build the Statement info, storing to the DB
    fn build_statement_info(
        data: &SensioStatement,
        account_id: &T::AccountId,
        block_number: &T::BlockNumber,
    ) -> StatementInfo<T::AccountId, T::BlockNumber> {
        let statement_info = StatementInfo {
            statement: data.clone(),
            account_id: account_id.clone(),
            block_number: block_number.clone(),
        };

        statement_info
    }

    ///Check does the Proof list is empty or not
    fn check_statements_for_proof(statement: &SensioStatement) -> Result<bool, Error<T>> {
        let proof_statement_list: Vec<GenericId> =
            ProofValidStatements::get(&statement.data.claim.poe_id);

        // We have never seen this proof getting the statement
        if !proof_statement_list.is_empty() {
            // check here for existence of the statement given the condition where proportion is 100% or less
            // For now return error since we only can have one statement 100% per proof
            Err(Error::<T>::ProofHasStatement.into())
        } else {
            // ProofValidStatements::insert(&poe_id, vec![]);
            Ok(true)
        }
    }

    ///Add Statement to the Proof
    fn remove_statement_from_proof(
        poe_id: &GenericId,
        statement_id: &GenericId,
    ) -> Result<bool, Error<T>> {
        let mut proof_statement_list: Vec<GenericId> = ProofValidStatements::get(&poe_id);

        match proof_statement_list.binary_search(&statement_id) {
            Ok(removal_index) => {
                proof_statement_list.remove(removal_index);
                ProofValidStatements::insert(&poe_id, proof_statement_list);
                Ok(true)
            }
            // If the search fails, the caller is not a member and we learned the index where
            // they should be inserted
            Err(_) => Err(Error::<T>::ProofHasStatement.into()),
        }
    }

    ///Add Statement to the Proof
    fn add_statement_to_proof(
        poe_id: &GenericId,
        statement_id: &GenericId,
    ) -> Result<bool, Error<T>> {
        let mut proof_statement_list: Vec<GenericId> = ProofValidStatements::get(&poe_id);

        match proof_statement_list.binary_search(&statement_id) {
            // If the search succeeds, the caller is already a member, so just return
            Ok(_) => Err(Error::<T>::ProofHasStatement.into()),
            // If the search fails, the caller is not a member and we learned the index where
            // they should be inserted
            Err(index) => {
                // update the list
                proof_statement_list.insert(index, statement_id.clone());
                ProofValidStatements::insert(&poe_id, proof_statement_list);
                Ok(true)
            }
        }
    }
}

// match values.binary_search(value) {
//     Ok(removal_index) =>,
//     Err(_) => {} // value not contained.
//   }
