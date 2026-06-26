// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{AccountId, AssetsConfig, Balance, BalancesConfig, RuntimeGenesisConfig, SudoConfig, UNIT};
use alloc::{vec, vec::Vec};
use frame_support::build_struct_json_patch;
use serde_json::Value;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::Ss58Codec;
use sp_genesis_builder::{self, PresetId};
use sp_keyring::Sr25519Keyring;

/// Preset name for the live Xcavate testnet (the one deployed to OnFinality).
pub const XCAVATE_TESTNET_RUNTIME_PRESET: &str = "xcavate-testnet";

/// Endowment granted to every prefunded account on the live testnet (100 XCAV).
const TESTNET_ENDOWMENT: Balance = 100 * UNIT;

/// The single Xcavate collator/validator key (sr25519, used for Aura), from `keys.txt`.
const COLLATOR_AURA_SS58: &str = "5CyBrku1V4d2WF965k1DeqvpFc3MmyuyhvtkgFYqJvpJf89S";
/// The Grandpa (ed25519) authority key derived from the same `keys.txt` secret phrase.
const COLLATOR_GRANDPA_SS58: &str = "5DjaSRo2GhKATdUeULsAUEbJAsb3UyWfKV2XPNEe3CBngriX";

/// Build a genesis config patch from the given authorities, prefunded accounts,
/// per-account endowment and sudo/root account.
fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
	endowment: Balance,
	root: AccountId,
) -> Value {
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, endowment))
				.collect::<Vec<_>>(),
		},
		aura: pallet_aura::GenesisConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),
		},
		grandpa: pallet_grandpa::GenesisConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
		},
		assets: AssetsConfig {
			assets: vec![
				(10, root.clone(), true, 1),
				(1337, root.clone(), true, 1),
				(1984, root.clone(), true, 1),
			],
			metadata: vec![
				(10, b"tGBP".to_vec(), b"tGBP".to_vec(), 18),
				(1337, b"USDC".to_vec(), b"USDC".to_vec(), 6),
				(1984, b"USDT".to_vec(), b"USDT".to_vec(), 6),
			],
			accounts: endowed_accounts
				.iter()
				.cloned()
				.flat_map(|x| {
					vec![
						(10, x.clone(), 10_000_000_000_000_000_000u128),
						(1337, x.clone(), 2_000_000_000_000u128),
						(1984, x.clone(), 2_000_000_000_000u128),
					]
				})
				.collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: Some(root) },
	})
}

/// The accounts that are prefunded in the live testnet, read from `seed/accounts.json`.
/// The first entry is also used as the sudo/root account. Mirrors the parachain.
fn seed_accounts() -> Vec<AccountId> {
	let json_data = &include_bytes!("../../seed/accounts.json")[..];
	serde_json::from_slice(json_data)
		.expect("seed/accounts.json must be a valid list of SS58 accounts; qed")
}

/// Return the live Xcavate testnet genesis config: a single collator (from
/// `keys.txt`) and the accounts from `seed/accounts.json` prefunded.
pub fn xcavate_testnet_config_genesis() -> Value {
	let endowed_accounts = seed_accounts();
	let root = endowed_accounts
		.first()
		.cloned()
		.expect("seed/accounts.json must contain at least one account; qed");
	let collator_aura =
		AuraId::from_ss58check(COLLATOR_AURA_SS58).expect("COLLATOR_AURA_SS58 is valid; qed");
	let collator_grandpa = GrandpaId::from_ss58check(COLLATOR_GRANDPA_SS58)
		.expect("COLLATOR_GRANDPA_SS58 is valid; qed");

	testnet_genesis(
		vec![(collator_aura, collator_grandpa)],
		endowed_accounts,
		TESTNET_ENDOWMENT,
		root,
	)
}

/// Return the development genesis config (single Alice validator, well-known keys).
pub fn development_config_genesis() -> Value {
	testnet_genesis(
		vec![(
			sp_keyring::Sr25519Keyring::Alice.public().into(),
			sp_keyring::Ed25519Keyring::Alice.public().into(),
		)],
		vec![
			Sr25519Keyring::Alice.to_account_id(),
			Sr25519Keyring::Bob.to_account_id(),
			Sr25519Keyring::AliceStash.to_account_id(),
			Sr25519Keyring::BobStash.to_account_id(),
		],
		1u128 << 60,
		sp_keyring::Sr25519Keyring::Alice.to_account_id(),
	)
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		XCAVATE_TESTNET_RUNTIME_PRESET => xcavate_testnet_config_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(XCAVATE_TESTNET_RUNTIME_PRESET),
	]
}
