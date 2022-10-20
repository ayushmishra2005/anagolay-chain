use anagolay_runtime::{
  constants::{currency::UNITS, time::MINUTES},
  AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig,
  VestingConfig, WASM_BINARY,
};
use jsonrpsee::core::to_json_value;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::Ss58Codec, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.anagolay.io/submit/";
// The initial balance for created accounts
const INITIAL_BALANCE: u128 = 100 * UNITS;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
  TPublic::Pair::from_string(&format!("//{}", seed), None)
    .expect("static values are valid; qed")
    .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
  AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
  AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
  (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  let mut props: Properties = Properties::new();

  let value = to_json_value("IDI").unwrap_or_default();
  props.insert("tokenSymbol".to_string(), value);

  Ok(ChainSpec::from_genesis(
    // Name
    "Development",
    // ID
    "dev",
    ChainType::Development,
    move || {
      testnet_genesis(
        wasm_binary,
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
          (get_account_id_from_seed::<sr25519::Public>("Alice"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), INITIAL_BALANCE),
          (
            // daniel
            AccountId::from_ss58check("5Fn9SNUE8LihCm7Lq5dpPgBebGy5D7ZKWESDsWbdjsfV37d4").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // adriano
            AccountId::from_ss58check("5EHkcDMhHgwW7V4GR4Us4dhkPkP9f2k71kdSVbzzzpNsHsYo").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            INITIAL_BALANCE,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            INITIAL_BALANCE,
          ),
        ],
        true,
      )
    },
    // Bootnodes
    vec![],
    // Telemetry
    Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 3)]).unwrap()),
    // Protocol ID
    None,
    // Fork ID
    None,
    // Properties
    Some(props),
    // Extensions
    None,
  ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  let mut props: Properties = Properties::new();

  let value = to_json_value("IDI").unwrap_or_default();
  props.insert("tokenSymbol".to_string(), value);

  Ok(ChainSpec::from_genesis(
    // Name
    "Local Testnet",
    // ID
    "local_testnet",
    ChainType::Local,
    move || {
      testnet_genesis(
        wasm_binary,
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
          (get_account_id_from_seed::<sr25519::Public>("Alice"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Charlie"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Dave"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Eve"), INITIAL_BALANCE),
          (get_account_id_from_seed::<sr25519::Public>("Ferdie"), INITIAL_BALANCE),
          (
            // daniel
            AccountId::from_ss58check("5Fn9SNUE8LihCm7Lq5dpPgBebGy5D7ZKWESDsWbdjsfV37d4").unwrap(),
            INITIAL_BALANCE,
          ),
          (
            // adriano
            AccountId::from_ss58check("5EHkcDMhHgwW7V4GR4Us4dhkPkP9f2k71kdSVbzzzpNsHsYo").unwrap(),
            INITIAL_BALANCE,
          ),
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
        ],
        true,
      )
    },
    // Bootnodes
    vec![],
    // Telemetry
    None,
    // Protocol ID
    None,
    // Fork ID
    None,
    // Properties
    Some(props),
    // Extensions
    None,
  ))
}

#[allow(clippy::identity_op)]
/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
  wasm_binary: &[u8],
  initial_authorities: Vec<(AuraId, GrandpaId)>,
  root_key: AccountId,
  endowed_accounts: Vec<(AccountId, u128)>,
  _enable_println: bool,
) -> GenesisConfig {
  GenesisConfig {
    system: SystemConfig {
      // Add Wasm runtime to storage.
      code: wasm_binary.to_vec(),
    },
    balances: BalancesConfig {
      // Configure endowed accounts with initial balance
      balances: endowed_accounts,
    },
    treasury: Default::default(),
    aura: AuraConfig {
      authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
    },
    grandpa: GrandpaConfig {
      authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
    },
    sudo: SudoConfig {
      // Assign network admin rights.
      key: Some(root_key),
    },
    transaction_payment: Default::default(),
    operations: Default::default(),
    workflows: Default::default(),
    vesting: VestingConfig {
      vesting: vec![
        (
          // daniel
          AccountId::from_ss58check("5Fn9SNUE8LihCm7Lq5dpPgBebGy5D7ZKWESDsWbdjsfV37d4").unwrap(),
          1 * MINUTES,
          10 * MINUTES,
          (INITIAL_BALANCE / 2) as u128,
        ),
        (
          // adriano
          AccountId::from_ss58check("5EHkcDMhHgwW7V4GR4Us4dhkPkP9f2k71kdSVbzzzpNsHsYo").unwrap(),
          1 * MINUTES,
          10 * MINUTES,
          (INITIAL_BALANCE / 2) as u128,
        ),
      ],
    },
  }
}
