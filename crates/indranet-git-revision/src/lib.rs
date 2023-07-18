#![no_std]

pub fn git_revision() -> &'static str {
    env!("INDRANET_GIT_REVISION")
}

pub fn git_commit_timestamp() -> &'static str {
    env!("INDRANET_GIT_COMMIT_TS")
}

pub fn git_revision_with_ts() -> &'static str {
    env!("INDRANET_GIT_REVISION_WITH_TS")
}
