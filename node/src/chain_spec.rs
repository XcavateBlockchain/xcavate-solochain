use sc_service::ChainType;
use xcavate_runtime::{genesis_config_presets::XCAVATE_TESTNET_RUNTIME_PRESET, WASM_BINARY};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Token properties of the Xcavate network (mirrors the parachain).
const TOKEN_SYMBOL: &str = "XCAV";
const TOKEN_DECIMALS: u32 = 12;
/// SS58 prefix used for address display (mirrors the parachain).
const SS58_FORMAT: u32 = 0;

fn chain_properties() -> serde_json::Map<String, serde_json::Value> {
	let mut properties = serde_json::Map::new();
	properties.insert("tokenSymbol".into(), TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());
	properties
}

pub fn development_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
	.with_properties(chain_properties())
	.build())
}

/// The live Xcavate testnet: a single collator (from `keys.txt`) and the accounts
/// from `seed/accounts.json` prefunded. The genesis itself is defined in the
/// runtime preset `XCAVATE_TESTNET_RUNTIME_PRESET`.
pub fn xcavate_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Xcavate testnet wasm not available".to_string())?,
		None,
	)
	.with_name("Xcavate Testnet")
	.with_id("xcavate_testnet")
	.with_chain_type(ChainType::Live)
	.with_protocol_id("xcavate")
	.with_genesis_config_preset_name(XCAVATE_TESTNET_RUNTIME_PRESET)
	.with_properties(chain_properties())
	.build())
}
