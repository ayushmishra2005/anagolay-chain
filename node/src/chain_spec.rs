use anagolay_runtime::{
  AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use jsonrpc_core::serde_json::json;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::Ss58Codec, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

// Note this is the URL for the telemetry server
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.anagolay.io/submit/";
// The minimum balance for created accounts
const EXISTENCIAL_DEPOSIT: u128 = 1 << 60;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
  TPublic::Pair::from_string(&format!("//{}", seed), None)
    .expect("static values are valid; qed")
    .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
  AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
  AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
  (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  let mut props: Properties = Properties::new();

  let value = json!("IDI");
  props.insert("tokenSymbol".to_string(), value);

  Ok(ChainSpec::from_genesis(
    "Development",
    "dev",
    ChainType::Development,
    move || {
      testnet_genesis(
        wasm_binary,
        vec![authority_keys_from_seed("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
          (
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            EXISTENCIAL_DEPOSIT,
          ),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), EXISTENCIAL_DEPOSIT),
          (
            AccountId::from_ss58check("5GukQt4gJW2XqzFwmm3RHa7x6sYuVcGhuhz72CN7oiBsgffx").unwrap(),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
        ],
        true,
      )
    },
    vec![],
    Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 3)]).unwrap()),
    None,
    Some(props),
    None,
  ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  let mut props: Properties = Properties::new();

  let value = json!("IDI");
  props.insert("tokenSymbol".to_string(), value);

  Ok(ChainSpec::from_genesis(
    "Local Testnet",
    "local_testnet",
    ChainType::Local,
    move || {
      testnet_genesis(
        wasm_binary,
        vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![
          (
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            EXISTENCIAL_DEPOSIT,
          ),
          (get_account_id_from_seed::<sr25519::Public>("Bob"), EXISTENCIAL_DEPOSIT),
          (
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            EXISTENCIAL_DEPOSIT,
          ),
          (get_account_id_from_seed::<sr25519::Public>("Dave"), EXISTENCIAL_DEPOSIT),
          (get_account_id_from_seed::<sr25519::Public>("Eve"), EXISTENCIAL_DEPOSIT),
          (
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            AccountId::from_ss58check("5GukQt4gJW2XqzFwmm3RHa7x6sYuVcGhuhz72CN7oiBsgffx").unwrap(),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
          (
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
            EXISTENCIAL_DEPOSIT,
          ),
        ],
        true,
      )
    },
    vec![],
    None,
    None,
    Some(props),
    None,
  ))
}

fn testnet_genesis(
  wasm_binary: &[u8],
  initial_authorities: Vec<(AuraId, GrandpaId)>,
  root_key: AccountId,
  endowed_accounts: Vec<(AccountId, u128)>,
  _enable_println: bool,
) -> GenesisConfig {
  GenesisConfig {
    system: Some(SystemConfig {
      code: wasm_binary.to_vec(),
      changes_trie_config: Default::default(),
    }),
    balances: Some(BalancesConfig {
      balances: endowed_accounts,
    }),
    aura: Some(AuraConfig {
      authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
    }),
    grandpa: Some(GrandpaConfig {
      authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
    }),
    sudo: Some(SudoConfig { key: root_key }),
    operations: Default::default(),
    workflows: Default::default(),
  }
}
