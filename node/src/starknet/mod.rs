pub mod configs;
pub mod constants;
pub mod genesis_block;

use std::path::PathBuf;

use selendra_primitives::Block;
use sc_service::Configuration;

pub type MadaraBackend = mc_db::Backend<Block>;

/// Returns the path to the database of the node.
pub fn db_config_dir(config: &Configuration) -> PathBuf {
    config.base_path.config_dir(config.chain_spec.id())
}