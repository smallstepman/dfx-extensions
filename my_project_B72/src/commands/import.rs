//! Code for the command line: `dfx nns import`
use std::collections::BTreeMap;

use dfx_core::config::model::dfinity::Config;
// use crate::lib::error::DfxResult;
use dfx::lib::info::replica_rev;
use dfx::lib::models::canister_id_store::CanisterIds;
use dfx::lib::nns::install_nns::canisters::NNS_CORE;
use dfx::lib::project::import::{
    get_canisters_json_object, import_canister_definitions, set_remote_canister_ids,
    ImportNetworkMapping,
};
use dfx::lib::project::network_mappings::get_network_mappings;
use dfx::Environment;

use clap::Parser;
// use slog::{info, Logger};

/// Imports the nns canisters
#[derive(Parser)]
pub struct ImportOpts {
    /// Networks to import canisters ids for.
    ///   --network-mapping <network name in both places>
    ///   --network-mapping <network name here>=<network name in project being imported>
    /// Examples:
    ///   --network-mapping ic
    ///   --network-mapping ic=mainnet
    #[clap(long, default_value = "ic=mainnet", multiple_occurrences(true))]
    network_mapping: Vec<String>,
}

/// Executes `dfx nns import`
pub async fn exec(opts: ImportOpts) -> anyhow::Result<()> {
    let config = Config::from_current_dir()?;
    let mut config = config.as_ref().clone();

    let network_mappings = get_network_mappings(&opts.network_mapping)?;
    let ic_commit = std::env::var("DFX_IC_COMMIT").unwrap_or_else(|_| replica_rev().unwrap());

    let dfx_url_str = {
        let ic_project = std::env::var("DFX_IC_SRC").unwrap_or_else(|_| {
            format!("https://raw.githubusercontent.com/dfinity/ic/{ic_commit}")
        });
        format!("{ic_project}/rs/nns/dfx.json")
    };
    import_canister_definitions(
        env.get_logger(),
        &mut config,
        &dfx_url_str,
        Some("nns-"),
        None,
        &network_mappings,
    )
    .await?;

    set_local_nns_canister_ids(env.get_logger(), &mut config)
}

/// Sets local canister IDs
/// The "local" entries at the remote URL are often misssing or do not match our NNS installation.
/// Always set the local values per our local NNS deployment.  We have all the information locally.
fn set_local_nns_canister_ids(logger: &Logger, config: &mut Config) -> DfxResult {
    let local_canister_ids: CanisterIds = NNS_CORE
        .iter()
        .map(|canister| {
            (
                canister.canister_name.to_string(),
                BTreeMap::from([("local".to_string(), canister.canister_id.to_string())]),
            )
        })
        .collect();
    let local_mappings = [ImportNetworkMapping {
        network_name_in_this_project: "local".to_string(),
        network_name_in_project_being_imported: "local".to_string(),
    }];

    let canisters = get_canisters_json_object(config)?;

    for canister in NNS_CORE {
        // Not all NNS canisters may be listed in the remote dfx.json
        let dfx_canister = canisters
            .get_mut(canister.canister_name)
            .and_then(|canister_entry| canister_entry.as_object_mut());
        // If the canister is in dfx.json, set the local canister ID.
        if let Some(dfx_canister) = dfx_canister {
            set_remote_canister_ids(
                logger,
                canister.canister_name,
                &local_mappings,
                &local_canister_ids,
                dfx_canister,
            )?;
        } else {
            info!(
                logger,
                "{} has no local canister ID.", canister.canister_name
            );
        }
    }
    config.save()?;
    Ok(())
}
