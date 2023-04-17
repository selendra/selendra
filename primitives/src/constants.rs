use crate::{ConsensusEngineId, EraIndex, KeyTypeId, Perbill, SessionCount, SessionIndex, Version};

pub const TOKEN_DECIMALS: u32 = 18;
pub const TOKEN: u128 = 10u128.pow(TOKEN_DECIMALS);

pub const DEFAULT_SESSION_PERIOD: u32 = 900;
pub const DEFAULT_SESSIONS_PER_ERA: SessionIndex = 96;

pub const DEFAULT_UNIT_CREATION_DELAY: u64 = 300;
pub const HEAP_PAGES: u64 = 4096;

pub const DEFAULT_COMMITTEE_SIZE: u32 = 4;

pub const DEFAULT_BAN_MINIMAL_EXPECTED_PERFORMANCE: Perbill = Perbill::from_percent(0);
pub const DEFAULT_BAN_SESSION_COUNT_THRESHOLD: SessionCount = 3;
pub const DEFAULT_BAN_REASON_LENGTH: u32 = 300;
pub const DEFAULT_MAX_WINNERS: u32 = u32::MAX;

pub const DEFAULT_CLEAN_SESSION_COUNTER_DELAY: SessionCount = 960;
pub const DEFAULT_BAN_PERIOD: EraIndex = 10;

/// Version returned when no version has been set.
pub const DEFAULT_FINALITY_VERSION: Version = 0;
/// Current version of abft.
pub const CURRENT_FINALITY_VERSION: u16 = LEGACY_FINALITY_VERSION + 1;
/// Legacy version of abft.
pub const LEGACY_FINALITY_VERSION: u16 = 1;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"indr");

pub const SELENDRA_ENGINE_ID: ConsensusEngineId = *b"FRNK";
