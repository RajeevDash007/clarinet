use clarinet_utils::get_bip39_seed_from_mnemonic;
use std::collections::BTreeMap;

use super::FileLocation;
use bip39::{Language, Mnemonic};
use clarity_repl::clarity::util::hash::bytes_to_hex;
use clarity_repl::clarity::util::secp256k1::Secp256k1PublicKey;
use clarity_repl::clarity::util::StacksAddress;
use libsecp256k1::{PublicKey, SecretKey};
use orchestra_types::{BitcoinNetwork, StacksNetwork};
use tiny_hderive::bip32::ExtendedPrivKey;
use toml::value::Value;

#[cfg(feature = "cli")]
use bitcoin;

pub const DEFAULT_DERIVATION_PATH: &str = "m/44'/5757'/0'/0/0";
pub const DEFAULT_BITCOIN_NODE_IMAGE: &str = "quay.io/hirosystems/bitcoind:devnet-v2";
pub const DEFAULT_STACKS_NODE_IMAGE: &str = "quay.io/hirosystems/stacks-node:devnet-v2";
pub const DEFAULT_BITCOIN_EXPLORER_IMAGE: &str = "quay.io/hirosystems/bitcoin-explorer:devnet";
pub const DEFAULT_STACKS_API_IMAGE: &str = "blockstack/stacks-blockchain-api:latest";
pub const DEFAULT_STACKS_EXPLORER_IMAGE: &str = "hirosystems/explorer:latest";
pub const DEFAULT_POSTGRES_IMAGE: &str = "postgres:alpine";
pub const DEFAULT_HYPERCHAINS_IMAGE: &str = "hirosystems/hyperchains:103-merge-stretch";
pub const DEFAULT_HYPERCHAIN_CONTRACT_ID: &str =
    "ST3A7S7GFKR8E3TVZ41Z2N441265CKXS0QPZ94N6B.hc-alpha-2";

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkManifestFile {
    network: NetworkConfigFile,
    accounts: Option<Value>,
    devnet: Option<DevnetConfigFile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkConfigFile {
    name: String,
    node_rpc_address: Option<String>,
    stacks_node_rpc_address: Option<String>,
    bitcoin_node_rpc_address: Option<String>,
    deployment_fee_rate: Option<u64>,
    sats_per_bytes: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DevnetConfigFile {
    pub orchestrator_port: Option<u16>,
    pub orchestrator_control_port: Option<u16>,
    pub bitcoin_node_p2p_port: Option<u16>,
    pub bitcoin_node_rpc_port: Option<u16>,
    pub stacks_node_p2p_port: Option<u16>,
    pub stacks_node_rpc_port: Option<u16>,
    pub stacks_node_events_observers: Option<Vec<String>>,
    pub stacks_api_port: Option<u16>,
    pub stacks_api_events_port: Option<u16>,
    pub bitcoin_explorer_port: Option<u16>,
    pub stacks_explorer_port: Option<u16>,
    pub bitcoin_node_username: Option<String>,
    pub bitcoin_node_password: Option<String>,
    pub miner_mnemonic: Option<String>,
    pub miner_derivation_path: Option<String>,
    pub faucet_mnemonic: Option<String>,
    pub faucet_derivation_path: Option<String>,
    pub bitcoin_controller_block_time: Option<u32>,
    pub bitcoin_controller_automining_disabled: Option<bool>,
    pub working_dir: Option<String>,
    pub postgres_port: Option<u16>,
    pub postgres_username: Option<String>,
    pub postgres_password: Option<String>,
    pub postgres_database: Option<String>,
    pub pox_stacking_orders: Option<Vec<PoxStackingOrder>>,
    pub execute_script: Option<Vec<ExecuteScript>>,
    pub bitcoin_node_image_url: Option<String>,
    pub bitcoin_explorer_image_url: Option<String>,
    pub stacks_node_image_url: Option<String>,
    pub stacks_api_image_url: Option<String>,
    pub stacks_explorer_image_url: Option<String>,
    pub postgres_image_url: Option<String>,
    pub disable_bitcoin_explorer: Option<bool>,
    pub disable_stacks_explorer: Option<bool>,
    pub disable_stacks_api: Option<bool>,
    pub bind_containers_volumes: Option<bool>,
    pub enable_hyperchain_node: Option<bool>,
    pub hyperchain_image_url: Option<String>,
    pub hyperchain_leader_mnemonic: Option<String>,
    pub hyperchain_leader_derivation_path: Option<String>,
    pub hyperchain_node_p2p_port: Option<u16>,
    pub hyperchain_node_rpc_port: Option<u16>,
    pub hyperchain_events_ingestion_port: Option<u16>,
    pub hyperchain_node_events_observers: Option<Vec<String>>,
    pub hyperchain_contract_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PoxStackingOrderFile {
    pub start_at_cycle: u32,
    pub end_at_cycle: u32,
    pub amount_locked: u32,
    pub wallet_label: String,
    pub bitcoin_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteScript {
    pub script: String,
    pub allow_wallets: bool,
    pub allow_write: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountConfigFile {
    mnemonic: Option<String>,
    derivation: Option<String>,
    balance: Option<u64>,
    is_mainnet: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkManifest {
    pub network: NetworkConfig,
    pub accounts: BTreeMap<String, AccountConfig>,
    pub devnet: Option<DevnetConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkConfig {
    name: String,
    pub stacks_node_rpc_address: Option<String>,
    pub bitcoin_node_rpc_address: Option<String>,
    pub deployment_fee_rate: u64,
    pub sats_per_bytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DevnetConfig {
    pub orchestrator_ingestion_port: u16,
    pub orchestrator_control_port: u16,
    pub bitcoin_node_p2p_port: u16,
    pub bitcoin_node_rpc_port: u16,
    pub bitcoin_node_username: String,
    pub bitcoin_node_password: String,
    pub stacks_node_p2p_port: u16,
    pub stacks_node_rpc_port: u16,
    pub stacks_node_events_observers: Vec<String>,
    pub stacks_api_port: u16,
    pub stacks_api_events_port: u16,
    pub stacks_explorer_port: u16,
    pub bitcoin_explorer_port: u16,
    pub bitcoin_controller_block_time: u32,
    pub bitcoin_controller_automining_disabled: bool,
    pub miner_stx_address: String,
    pub miner_secret_key_hex: String,
    pub miner_btc_address: String,
    pub miner_mnemonic: String,
    pub miner_derivation_path: String,
    pub faucet_stx_address: String,
    pub faucet_secret_key_hex: String,
    pub faucet_btc_address: String,
    pub faucet_mnemonic: String,
    pub faucet_derivation_path: String,
    pub working_dir: String,
    pub postgres_port: u16,
    pub postgres_username: String,
    pub postgres_password: String,
    pub postgres_database: String,
    pub pox_stacking_orders: Vec<PoxStackingOrder>,
    pub execute_script: Vec<ExecuteScript>,
    pub bitcoin_node_image_url: String,
    pub stacks_node_image_url: String,
    pub stacks_api_image_url: String,
    pub stacks_explorer_image_url: String,
    pub postgres_image_url: String,
    pub bitcoin_explorer_image_url: String,
    pub disable_bitcoin_explorer: bool,
    pub disable_stacks_explorer: bool,
    pub disable_stacks_api: bool,
    pub bind_containers_volumes: bool,
    pub enable_hyperchain_node: bool,
    pub hyperchain_image_url: String,
    pub hyperchain_leader_stx_address: String,
    pub hyperchain_leader_secret_key_hex: String,
    pub hyperchain_leader_btc_address: String,
    pub hyperchain_leader_mnemonic: String,
    pub hyperchain_leader_derivation_path: String,
    pub hyperchain_node_p2p_port: u16,
    pub hyperchain_node_rpc_port: u16,
    pub hyperchain_events_ingestion_port: u16,
    pub hyperchain_node_events_observers: Vec<String>,
    pub hyperchain_contract_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoxStackingOrder {
    pub start_at_cycle: u32,
    pub duration: u32,
    pub wallet: String,
    pub slots: u64,
    pub btc_address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountConfig {
    pub label: String,
    pub mnemonic: String,
    pub derivation: String,
    pub balance: u64,
    pub stx_address: String,
    pub btc_address: String,
    pub is_mainnet: bool,
}

impl NetworkManifest {
    pub fn from_project_manifest_location(
        project_manifest_location: &FileLocation,
        networks: &(BitcoinNetwork, StacksNetwork),
    ) -> Result<NetworkManifest, String> {
        let network_manifest_location =
            project_manifest_location.get_network_manifest_location(&networks.1)?;
        NetworkManifest::from_location(&network_manifest_location, networks)
    }

    pub fn from_location(
        location: &FileLocation,
        networks: &(BitcoinNetwork, StacksNetwork),
    ) -> Result<NetworkManifest, String> {
        let network_manifest_file_content = location.read_content()?;
        let mut network_manifest_file: NetworkManifestFile =
            toml::from_slice(&network_manifest_file_content[..]).unwrap();
        Ok(NetworkManifest::from_network_manifest_file(
            &mut network_manifest_file,
            networks,
        ))
    }

    pub fn from_network_manifest_file(
        network_manifest_file: &mut NetworkManifestFile,
        networks: &(BitcoinNetwork, StacksNetwork),
    ) -> NetworkManifest {
        let stacks_node_rpc_address = match (
            &network_manifest_file.network.node_rpc_address,
            &network_manifest_file.network.stacks_node_rpc_address,
        ) {
            (Some(_), Some(url)) | (None, Some(url)) | (Some(url), None) => Some(url.clone()),
            _ => None,
        };
        let network = NetworkConfig {
            name: network_manifest_file.network.name.clone(),
            stacks_node_rpc_address: stacks_node_rpc_address,
            bitcoin_node_rpc_address: network_manifest_file
                .network
                .bitcoin_node_rpc_address
                .clone(),
            deployment_fee_rate: network_manifest_file
                .network
                .deployment_fee_rate
                .unwrap_or(10),
            sats_per_bytes: network_manifest_file.network.sats_per_bytes.unwrap_or(10),
        };

        let mut accounts = BTreeMap::new();
        let is_mainnet = networks.1.is_mainnet();

        match &network_manifest_file.accounts {
            Some(Value::Table(entries)) => {
                for (account_name, account_settings) in entries.iter() {
                    match account_settings {
                        Value::Table(account_settings) => {
                            let balance = match account_settings.get("balance") {
                                Some(Value::Integer(balance)) => *balance as u64,
                                _ => 0,
                            };

                            let mnemonic = match account_settings.get("mnemonic") {
                                Some(Value::String(words)) => {
                                    Mnemonic::parse_in_normalized(Language::English, words)
                                        .unwrap()
                                        .to_string()
                                }
                                _ => {
                                    let entropy = &[
                                        0x33, 0xE4, 0x6B, 0xB1, 0x3A, 0x74, 0x6E, 0xA4, 0x1C, 0xDD,
                                        0xE4, 0x5C, 0x90, 0x84, 0x6A, 0x79,
                                    ]; // TODO(lgalabru): rand
                                    Mnemonic::from_entropy(entropy).unwrap().to_string()
                                }
                            };

                            let derivation = match account_settings.get("derivation") {
                                Some(Value::String(derivation)) => derivation.to_string(),
                                _ => DEFAULT_DERIVATION_PATH.to_string(),
                            };

                            let (stx_address, btc_address, _) =
                                compute_addresses(&mnemonic, &derivation, networks);

                            accounts.insert(
                                account_name.to_string(),
                                AccountConfig {
                                    label: account_name.to_string(),
                                    mnemonic: mnemonic.to_string(),
                                    derivation,
                                    balance,
                                    stx_address,
                                    btc_address,
                                    is_mainnet,
                                },
                            );
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        let devnet = if networks.1.is_devnet() {
            let mut devnet_config = match network_manifest_file.devnet.take() {
                Some(conf) => conf,
                _ => DevnetConfigFile::default(),
            };

            let now = clarity_repl::clarity::util::get_epoch_time_secs();
            let mut dir = std::env::temp_dir();
            dir.push(format!("stacks-devnet-{}/", now));
            let default_working_dir = dir.display().to_string();

            let miner_mnemonic = devnet_config.miner_mnemonic.take().unwrap_or("fragile loan twenty basic net assault jazz absorb diet talk art shock innocent float punch travel gadget embrace caught blossom hockey surround initial reduce".to_string());
            let miner_derivation_path = devnet_config
                .miner_derivation_path
                .take()
                .unwrap_or(DEFAULT_DERIVATION_PATH.to_string());
            let (miner_stx_address, miner_btc_address, miner_secret_key_hex) =
                compute_addresses(&miner_mnemonic, &miner_derivation_path, networks);

            let faucet_mnemonic = devnet_config.faucet_mnemonic.take().unwrap_or("shadow private easily thought say logic fault paddle word top book during ignore notable orange flight clock image wealth health outside kitten belt reform".to_string());
            let faucet_derivation_path = devnet_config
                .faucet_derivation_path
                .take()
                .unwrap_or(DEFAULT_DERIVATION_PATH.to_string());
            let (faucet_stx_address, faucet_btc_address, faucet_secret_key_hex) =
                compute_addresses(&faucet_mnemonic, &faucet_derivation_path, networks);
            // If unset, we'll reuse the miner's keypair for the hyperchain leader
            let (
                hyperchain_leader_stx_address,
                hyperchain_leader_btc_address,
                hyperchain_leader_secret_key_hex,
                hyperchain_leader_derivation_path,
                hyperchain_leader_mnemonic,
            ) = if let Some(mnemonic) = devnet_config.hyperchain_leader_mnemonic.take() {
                let derivation_path = devnet_config
                    .hyperchain_leader_derivation_path
                    .take()
                    .unwrap_or(DEFAULT_DERIVATION_PATH.to_string());
                let (stx_address, btc_address, secret_key_hex) = compute_addresses(
                    &mnemonic,
                    &derivation_path,
                    &StacksNetwork::Devnet.get_networks(),
                );
                (
                    stx_address,
                    btc_address,
                    secret_key_hex,
                    derivation_path,
                    mnemonic,
                )
            } else {
                (
                    miner_stx_address.clone(),
                    miner_btc_address.clone(),
                    miner_secret_key_hex.clone(),
                    miner_derivation_path.clone(),
                    miner_mnemonic.clone(),
                )
            };

            let enable_hyperchain_node = devnet_config.enable_hyperchain_node.unwrap_or(false);
            let hyperchain_events_ingestion_port = devnet_config
                .hyperchain_events_ingestion_port
                .unwrap_or(30445);

            let mut stacks_node_events_observers = devnet_config
                .stacks_node_events_observers
                .take()
                .unwrap_or(vec![]);

            if enable_hyperchain_node {
                // add hyperchain node to stacks-node observers
                let label = "hyperchain-leader";
                stacks_node_events_observers.push(format!(
                    "host.docker.internal:{}",
                    hyperchain_events_ingestion_port
                ));
                accounts.insert(
                    label.to_string(),
                    AccountConfig {
                        label: label.to_string(),
                        mnemonic: hyperchain_leader_mnemonic.clone(),
                        derivation: hyperchain_leader_derivation_path.clone(),
                        balance: super::DEFAULT_DEVNET_BALANCE,
                        stx_address: hyperchain_leader_stx_address.clone(),
                        btc_address: hyperchain_leader_btc_address.clone(),
                        is_mainnet,
                    },
                );
            }

            let mut config = DevnetConfig {
                orchestrator_ingestion_port: devnet_config.orchestrator_port.unwrap_or(20445),
                orchestrator_control_port: devnet_config.orchestrator_control_port.unwrap_or(20446),
                bitcoin_node_p2p_port: devnet_config.bitcoin_node_p2p_port.unwrap_or(18444),
                bitcoin_node_rpc_port: devnet_config.bitcoin_node_rpc_port.unwrap_or(18443),
                bitcoin_node_username: devnet_config
                    .bitcoin_node_username
                    .take()
                    .unwrap_or("devnet".to_string()),
                bitcoin_node_password: devnet_config
                    .bitcoin_node_password
                    .take()
                    .unwrap_or("devnet".to_string()),
                bitcoin_controller_block_time: devnet_config
                    .bitcoin_controller_block_time
                    .unwrap_or(30_000),
                bitcoin_controller_automining_disabled: devnet_config
                    .bitcoin_controller_automining_disabled
                    .unwrap_or(false),
                stacks_node_p2p_port: devnet_config.stacks_node_p2p_port.unwrap_or(20444),
                stacks_node_rpc_port: devnet_config.stacks_node_rpc_port.unwrap_or(20443),
                stacks_node_events_observers,
                stacks_api_port: devnet_config.stacks_api_port.unwrap_or(3999),
                stacks_api_events_port: devnet_config.stacks_api_events_port.unwrap_or(3700),
                stacks_explorer_port: devnet_config.stacks_explorer_port.unwrap_or(8000),
                bitcoin_explorer_port: devnet_config.bitcoin_explorer_port.unwrap_or(8001),
                miner_btc_address,
                miner_stx_address,
                miner_mnemonic,
                miner_secret_key_hex,
                miner_derivation_path,
                faucet_btc_address,
                faucet_stx_address,
                faucet_mnemonic,
                faucet_secret_key_hex,
                faucet_derivation_path,
                working_dir: devnet_config
                    .working_dir
                    .take()
                    .unwrap_or(default_working_dir),
                postgres_port: devnet_config.postgres_port.unwrap_or(5432),
                postgres_username: devnet_config
                    .postgres_username
                    .take()
                    .unwrap_or("postgres".to_string()),
                postgres_password: devnet_config
                    .postgres_password
                    .take()
                    .unwrap_or("postgres".to_string()),
                postgres_database: devnet_config
                    .postgres_database
                    .take()
                    .unwrap_or("postgres".to_string()),
                execute_script: devnet_config.execute_script.take().unwrap_or(vec![]),
                bitcoin_node_image_url: devnet_config
                    .bitcoin_node_image_url
                    .take()
                    .unwrap_or(DEFAULT_BITCOIN_NODE_IMAGE.to_string()),
                stacks_node_image_url: devnet_config
                    .stacks_node_image_url
                    .take()
                    .unwrap_or(DEFAULT_STACKS_NODE_IMAGE.to_string()),
                stacks_api_image_url: devnet_config
                    .stacks_api_image_url
                    .take()
                    .unwrap_or(DEFAULT_STACKS_API_IMAGE.to_string()),
                postgres_image_url: devnet_config
                    .postgres_image_url
                    .take()
                    .unwrap_or(DEFAULT_POSTGRES_IMAGE.to_string()),
                stacks_explorer_image_url: devnet_config
                    .stacks_explorer_image_url
                    .take()
                    .unwrap_or(DEFAULT_STACKS_EXPLORER_IMAGE.to_string()),
                bitcoin_explorer_image_url: devnet_config
                    .bitcoin_explorer_image_url
                    .take()
                    .unwrap_or(DEFAULT_BITCOIN_EXPLORER_IMAGE.to_string()),
                pox_stacking_orders: devnet_config.pox_stacking_orders.take().unwrap_or(vec![]),
                disable_bitcoin_explorer: devnet_config.disable_bitcoin_explorer.unwrap_or(false),
                disable_stacks_api: devnet_config.disable_stacks_api.unwrap_or(false),
                disable_stacks_explorer: devnet_config.disable_stacks_explorer.unwrap_or(false),
                bind_containers_volumes: devnet_config.bind_containers_volumes.unwrap_or(false),
                enable_hyperchain_node,
                hyperchain_image_url: devnet_config
                    .hyperchain_image_url
                    .take()
                    .unwrap_or(DEFAULT_HYPERCHAINS_IMAGE.to_string()),
                hyperchain_leader_btc_address,
                hyperchain_leader_stx_address,
                hyperchain_leader_mnemonic,
                hyperchain_leader_secret_key_hex,
                hyperchain_leader_derivation_path,
                hyperchain_node_p2p_port: devnet_config.stacks_node_p2p_port.unwrap_or(30444),
                hyperchain_node_rpc_port: devnet_config.stacks_node_rpc_port.unwrap_or(30443),
                hyperchain_events_ingestion_port,
                hyperchain_node_events_observers: devnet_config
                    .hyperchain_node_events_observers
                    .take()
                    .unwrap_or(vec![]),
                hyperchain_contract_id: devnet_config
                    .hyperchain_contract_id
                    .unwrap_or(DEFAULT_HYPERCHAIN_CONTRACT_ID.to_string()),
            };
            if !config.disable_stacks_api && config.disable_stacks_api {
                config.disable_stacks_api = false;
            }

            Some(config)
        } else {
            None
        };
        let config = NetworkManifest {
            network,
            accounts,
            devnet,
        };

        config
    }
}

pub fn compute_addresses(
    mnemonic: &str,
    derivation_path: &str,
    networks: &(BitcoinNetwork, StacksNetwork),
) -> (String, String, String) {
    let bip39_seed = match get_bip39_seed_from_mnemonic(&mnemonic, "") {
        Ok(bip39_seed) => bip39_seed,
        Err(_) => panic!(),
    };

    let ext = ExtendedPrivKey::derive(&bip39_seed[..], derivation_path).unwrap();

    let secret_key = SecretKey::parse_slice(&ext.secret()).unwrap();

    // Enforce a 33 bytes secret key format, expected by Stacks
    let mut secret_key_bytes = secret_key.serialize().to_vec();
    secret_key_bytes.push(1);
    let miner_secret_key_hex = bytes_to_hex(&secret_key_bytes);

    let public_key = PublicKey::from_secret_key(&secret_key);
    let pub_key = Secp256k1PublicKey::from_slice(&public_key.serialize_compressed()).unwrap();
    let version = if networks.1.is_mainnet() {
        clarity_repl::clarity::util::C32_ADDRESS_VERSION_MAINNET_SINGLESIG
    } else {
        clarity_repl::clarity::util::C32_ADDRESS_VERSION_TESTNET_SINGLESIG
    };

    let stx_address = StacksAddress::from_public_key(version, pub_key).unwrap();

    let btc_address = compute_btc_address(&public_key, &networks.0);

    (stx_address.to_string(), btc_address, miner_secret_key_hex)
}

#[cfg(feature = "cli")]
fn compute_btc_address(public_key: &PublicKey, network: &BitcoinNetwork) -> String {
    let public_key = bitcoin::PublicKey::from_slice(&public_key.serialize_compressed())
        .expect("Unable to recreate public key");
    let btc_address = bitcoin::Address::p2pkh(
        &public_key,
        match network {
            BitcoinNetwork::Regtest => bitcoin::Network::Regtest,
            BitcoinNetwork::Testnet => bitcoin::Network::Testnet,
            BitcoinNetwork::Mainnet => bitcoin::Network::Bitcoin,
        },
    );
    btc_address.to_string()
}

#[cfg(feature = "wasm")]
fn compute_btc_address(_public_key: &PublicKey, _network: &BitcoinNetwork) -> String {
    format!("__not_implemented__")
}