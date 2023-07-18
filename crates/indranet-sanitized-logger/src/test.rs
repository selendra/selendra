use super::*;

#[test]
fn whitelist_works() {
    let allowed: Vec<_> = include_str!("all-log-targets.txt")
        .split('\n')
        .filter(|t| target_allowed(t))
        .collect();
    assert_eq!(
        allowed,
        [
            "gk_computing",
            "indratory",
            "indratory::benchmark",
            "indratory::bin_api_service",
            "indratory::contracts::pink",
            "indratory::contracts::support",
            "indratory::contracts::support::keeper",
            "indratory::light_validation",
            "indratory::light_validation::justification::communication",
            "indratory::irpc_service",
            "indratory::storage::storage_ext",
            "indratory::system",
            "indratory::system::gk",
            "indratory::system::master_key",
            "indratory_api::storage_sync",
            "indranet_mq",
            "indranet_node_runtime",
            "indranet_pallets::mining::pallet::migrations",
            "pink",
            "pink::contract",
            "pink::runtime::extension",
            "pink_extension_runtime",
            "irpc_measuring",
            "iruntime",
            "iruntime::api_server",
            "iruntime::ias",
            "iruntime::pal_gramine",
            "iruntime::runtime",
            "rocket::launch",
            "rocket::launch_",
            "rocket::server",
            "sidevm",
            "sidevm_env::tasks",
            "sidevm_host_runtime::instrument",
            "sidevm_host_runtime::resource",
        ]
    );
}

#[test]
fn see_log() {
    use log::info;

    init_env_logger(true);
    info!(target: "pink", "target pink");
    info!(target: "other", "target other");
}
