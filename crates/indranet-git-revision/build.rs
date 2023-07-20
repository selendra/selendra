use std::process::Command;

fn main() {
    export_git_revision();
}

fn export_git_revision() {
    let cmd = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap()
        .stdout;
    let revision = String::from_utf8_lossy(&cmd);
    let revision = revision.trim();
    let timestamp = Command::new("git")
        .args([
            "show",
            "-s",
            "--format=%cd",
            "--date=format:%Y%m%d%H%M%S",
            revision,
        ])
        .output()
        .unwrap()
        .stdout;
    let timestamp = String::from_utf8_lossy(&timestamp);
    let dirty = !Command::new("git")
        .args(["diff", "HEAD", "--quiet"])
        .output()
        .unwrap()
        .status
        .success();
    let tail = if dirty { "-dirty" } else { "" };
    if revision.is_empty() {
        println!("cargo:warning=⚠️ Failed to get git revision for iruntime.");
        println!("cargo:warning=⚠️ Please ensure you have git installed and are compiling from a git repository.");
    }
    println!("cargo:rustc-env=INDRANET_GIT_REVISION={revision}{tail}");
    println!("cargo:rustc-env=INDRANET_GIT_COMMIT_TS={timestamp}");
    println!("cargo:rustc-env=INDRANET_GIT_REVISION_WITH_TS={revision}-{timestamp}{tail}");
    println!("cargo:rerun-if-changed=always-rerun");
}
