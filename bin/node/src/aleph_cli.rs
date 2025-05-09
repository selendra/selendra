use std::path::PathBuf;

use finality_aleph::UnitCreationDelay;
use log::warn;
use primitives::{DEFAULT_MAX_NON_FINALIZED_BLOCKS, DEFAULT_UNIT_CREATION_DELAY};
use sc_cli::clap::{self, ArgGroup, Parser};

#[derive(Debug, Parser, Clone)]
#[command(group(ArgGroup::new("backup")))]
pub struct AlephCli {
    #[arg(long, default_value_t = DEFAULT_UNIT_CREATION_DELAY)]
    unit_creation_delay: u64,

    /// The addresses at which the node will be externally reachable for validator network
    /// purposes. Have to be provided for validators.
    #[arg(long)]
    public_validator_addresses: Option<Vec<String>>,

    /// The port on which to listen to validator network connections.
    #[arg(long, default_value_t = 30343)]
    validator_port: u16,

    /// Turn off backups, at the cost of limiting crash recoverability.
    ///
    /// If backups are turned off and the node crashes, it most likely will not be able to continue
    /// the session during which it crashed. It will join AlephBFT consensus in the next session.
    #[arg(long, group = "backup")]
    no_backup: bool,
    /// The path to save backups to.
    ///
    /// Backups created by the node are saved under this path. When restarted after a crash,
    /// the backups will be used to recover the node's state, helping prevent auto-forks. The layout
    /// of the directory is unspecified. This flag must be specified unless backups are turned off
    /// with `--no-backup`, but note that that limits crash recoverability.
    #[arg(long, value_name = "PATH", group = "backup")]
    backup_path: Option<PathBuf>,

    /// The maximum number of nonfinalized blocks, after which block production should be locally
    /// stopped. DO NOT CHANGE THIS, PRODUCING MORE OR FEWER BLOCKS MIGHT BE CONSIDERED MALICIOUS
    /// BEHAVIOUR AND PUNISHED ACCORDINGLY!
    #[arg(long, default_value_t = DEFAULT_MAX_NON_FINALIZED_BLOCKS)]
    max_nonfinalized_blocks: u32,

    /// Enable database pruning. It removes older entries in the state-database. Pruning of blocks is not supported.
    /// Note that we only support pruning with ParityDB database backend.
    /// See also `--state-pruning` option for more details.
    #[arg(long, default_value_t = false)]
    enable_pruning: bool,

    /// Maximum bit-rate in bits per second of the alephbft validator network.
    #[arg(long, default_value_t = 768 * 1024)]
    alephbft_network_bit_rate: u64,

    /// Maximum bit-rate in bits per second of the substrate network.
    #[arg(long, default_value_t = 5*1024*1024)]
    substrate_network_bit_rate: u64,

    /// Don't spend some extra time to collect more debugging data (e.g. validator network details).
    /// By default collecting is enabled, as the impact on performance is negligible, if any.
    #[arg(long, default_value_t = true)]
    collect_validator_network_data: bool,
}

impl AlephCli {
    pub fn unit_creation_delay(&self) -> UnitCreationDelay {
        UnitCreationDelay(self.unit_creation_delay)
    }

    pub fn external_addresses(&self) -> Vec<String> {
        self.public_validator_addresses.clone().unwrap_or_default()
    }

    pub fn set_dummy_external_addresses(&mut self) {
        self.public_validator_addresses = Some(vec!["192.0.2.43:30343".to_string()])
    }

    pub fn validator_port(&self) -> u16 {
        self.validator_port
    }

    pub fn backup_path(&self) -> Option<PathBuf> {
        self.backup_path.clone()
    }

    pub fn no_backup(&self) -> bool {
        self.no_backup
    }

    pub fn max_nonfinalized_blocks(&self) -> u32 {
        if self.max_nonfinalized_blocks != DEFAULT_MAX_NON_FINALIZED_BLOCKS {
            warn!("Running block production with a value of max-nonfinalized-blocks {}, which is not the default of 20. THIS MIGHT BE CONSIDERED MALICIOUS BEHAVIOUR AND RESULT IN PENALTIES!", self.max_nonfinalized_blocks);
        }
        self.max_nonfinalized_blocks
    }

    pub fn enable_pruning(&self) -> bool {
        self.enable_pruning
    }

    pub fn alephbft_network_bit_rate(&self) -> u64 {
        self.alephbft_network_bit_rate
    }

    pub fn substrate_network_bit_rate(&self) -> u64 {
        self.substrate_network_bit_rate
    }

    pub fn collect_validator_network_data(&self) -> bool {
        self.collect_validator_network_data
    }
}
