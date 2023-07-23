// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! To run this example, a local polkadot node should be running. Example verified against polkadot v0.9.28-9ffe6e9e3da.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.28/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_core::{sr25519, Pair};
use sp_keyring::AccountKeyring;
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Create a client to use:
    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let active_era_addr = polkadot::storage().staking().active_era();
    let era = api
        .storage()
        .at_latest()
        .await?
        .fetch(&active_era_addr)
        .await?
        .unwrap();
    println!(
        "Staking active era: index: {:?}, start: {:?}",
        era.index, era.start
    );

    let alice_id = AccountKeyring::Alice.to_account_id();
    println!("  Alice account id:        {alice_id:?}");

    // Get Alice' Stash account ID
    let alice_stash_id: AccountId32 = sr25519::Pair::from_string("//Alice//stash", None)
        .expect("Could not obtain stash signer pair")
        .public()
        .into();
    println!("  Alice//stash account id: {alice_stash_id:?}");

    // Map from all locked "stash" accounts to the controller account.
    let controller_acc_addr = polkadot::storage().staking().bonded(&alice_stash_id);
    let controller_acc = api
        .storage()
        .at_latest()
        .await?
        .fetch(&controller_acc_addr)
        .await?
        .unwrap();
    println!("    account controlled by: {controller_acc:?}");

    let era_reward_addr = polkadot::storage().staking().eras_reward_points(era.index);
    let era_result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&era_reward_addr)
        .await?;
    println!("Era reward points: {era_result:?}");

    Ok(())
}
