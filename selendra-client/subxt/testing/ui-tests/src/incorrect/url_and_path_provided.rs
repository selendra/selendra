#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata.scale",
    runtime_metadata_url = "wss://rpc.polkadot.io:443"
)]
pub mod node_runtime {}

fn main() {}
