use anagolay_runtime::{AccountId, AuraId, Signature, SudoConfig, VestingConfig, EXISTENTIAL_DEPOSIT, MINUTES, UNITS};
use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::Ss58Codec, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.anagolay.io/submit/";
// The initial balance for created accounts
const INITIAL_BALANCE: u128 = 10_000_000 * UNITS;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<anagolay_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
  TPublic::Pair::from_string(&format!("//{seed}"), None)
    .expect("static values are valid; qed")
    .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
  /// The relay chain of the Parachain.
  pub relay_chain: String,
  /// The id of the Parachain.
  pub para_id: u32,
}

impl Extensions {
  /// Try to get the extension from the given `ChainSpec`.
  pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
    sc_chain_spec::get_extension(chain_spec.extensions())
  }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
  get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
  AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
  AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> anagolay_runtime::SessionKeys {
  anagolay_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
  // Give your base currency a unit name and decimal places
  let mut properties = sc_chain_spec::Properties::new();
  properties.insert("tokenSymbol".into(), "IDI".into());
  properties.insert("tokenDecimals".into(), 12.into());
  properties.insert("ss58Format".into(), 42.into());

  ChainSpec::from_genesis(
    // Name
    "Development",
    // ID
    "dev",
    ChainType::Development,
    move || {
      testnet_genesis(
        // initial collators.
        vec![
          (
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_collator_keys_from_seed("Alice"),
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_collator_keys_from_seed("Bob"),
          ),
        ],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
          (get_account_id_from_seed::<sr25519::Public>("Alice"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Charlie"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Dave"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Eve"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Ferdie"), INITIAL_BALANCE),
          (
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            INITIAL_BALANCE,
          ),
          (
            // daniel
            AccountId::from_ss58check("5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // elena
            AccountId::from_ss58check("5He9bG2frzLPdcvsyE98tyEccLxJ17PM3TCJJoTz3ToiRBci").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // carla
            AccountId::from_ss58check("5FhDaAGVE38GCDPxq5qXTprHu7DtC2Hn3NBuYrR3k3zQkE47").unwrap(),
            INITIAL_BALANCE,
          ),
        ],
        1000.into(),
      )
    },
    // Bootnodes
    Vec::new(),
    // Telemetry
    Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 3)]).unwrap()),
    // Protocol ID
    None,
    // Fork ID
    None,
    // Properties
    Some(properties),
    // Extensions
    Extensions {
      relay_chain: "rococo-dev".into(), // You MUST set this to the correct network!
      para_id: 1000,
    },
  )
}

pub fn local_testnet_config() -> ChainSpec {
  // Give your base currency a unit name and decimal places
  let mut properties = sc_chain_spec::Properties::new();
  properties.insert("tokenSymbol".into(), "IDI".into());
  properties.insert("tokenDecimals".into(), 12.into());
  properties.insert("ss58Format".into(), 42.into());

  ChainSpec::from_genesis(
    // Name
    "Local Testnet",
    // ID
    "local_testnet",
    ChainType::Local,
    move || {
      testnet_genesis(
        // initial collators.
        vec![
          (
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_collator_keys_from_seed("Alice"),
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_collator_keys_from_seed("Bob"),
          ),
        ],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
          (get_account_id_from_seed::<sr25519::Public>("Alice"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Charlie"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Dave"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Eve"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Ferdie"), INITIAL_BALANCE),
          (
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            INITIAL_BALANCE,
          ),
          (
            // daniel
            AccountId::from_ss58check("5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // cuteolaf
            AccountId::from_ss58check("5DJ2NvYUb6idjT6AgMqhRd3Mxw7hx4ZxQwyp1X395897u27m").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // elena
            AccountId::from_ss58check("5He9bG2frzLPdcvsyE98tyEccLxJ17PM3TCJJoTz3ToiRBci").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // carla
            AccountId::from_ss58check("5FhDaAGVE38GCDPxq5qXTprHu7DtC2Hn3NBuYrR3k3zQkE47").unwrap(),
            INITIAL_BALANCE,
          ),
        ],
        1000.into(),
      )
    },
    // Bootnodes
    Vec::new(),
    // Telemetry
    Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 3)]).unwrap()),
    // Protocol ID
    Some("idiyanale-local"),
    // Fork ID
    None,
    // Properties
    Some(properties),
    // Extensions
    Extensions {
      relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
      para_id: 1000,
    },
  )
}

fn testnet_genesis(
  invulnerables: Vec<(AccountId, AuraId)>,
  root_key: AccountId,
  endowed_accounts: Vec<(AccountId, u128)>,
  id: ParaId,
) -> anagolay_runtime::GenesisConfig {
  anagolay_runtime::GenesisConfig {
    system: anagolay_runtime::SystemConfig {
      code: anagolay_runtime::WASM_BINARY
        .expect("WASM binary was not build, please build it!")
        .to_vec(),
    },
    balances: anagolay_runtime::BalancesConfig {
      balances: endowed_accounts,
    },
    treasury: Default::default(),
    parachain_info: anagolay_runtime::ParachainInfoConfig { parachain_id: id },
    collator_selection: anagolay_runtime::CollatorSelectionConfig {
      invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
      candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
      ..Default::default()
    },
    session: anagolay_runtime::SessionConfig {
      keys: invulnerables
        .into_iter()
        .map(|(acc, aura)| {
          (
            acc.clone(),                 // account id
            acc,                         // validator id
            template_session_keys(aura), // session keys
          )
        })
        .collect(),
    },
    sudo: SudoConfig {
      // Assign network admin rights.
      key: Some(root_key),
    },
    // no need to pass anything to aura, in fact it will panic if we do. Session will take care
    // of this.
    aura: Default::default(),
    aura_ext: Default::default(),
    parachain_system: Default::default(),
    polkadot_xcm: anagolay_runtime::PolkadotXcmConfig {
      safe_xcm_version: Some(SAFE_XCM_VERSION),
    },
    operations: Default::default(),
    workflows: Default::default(),
    vesting: VestingConfig {
      vesting: vec![(
        // daniel test account
        AccountId::from_ss58check("5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY").unwrap(),
        1 * MINUTES,
        10 * MINUTES,
        (INITIAL_BALANCE / 2) as u128,
      )],
    },
  }
}
