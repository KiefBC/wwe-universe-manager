#![allow(dead_code)]

/// Application constants for the WWE Universe Manager
/// 
/// This module contains all magic numbers and configuration values
/// used throughout the application to improve maintainability.
///
/// **Note**: Many constants are future infrastructure for upcoming features.

pub mod ui_constants;

/// URL checking and polling constants
pub const URL_CHECK_INTERVAL_MS: i32 = 500;

/// Power rating constraints
pub const MIN_POWER_RATING: i32 = 0;
pub const MAX_POWER_RATING: i32 = 10;

/// UI timing constants
pub const SUCCESS_REDIRECT_DELAY_MS: u64 = 1500;

/// Default values
pub const DEFAULT_PROMOTION_ID: i32 = 1;

/// Title prestige tiers
pub const WORLD_CHAMPIONSHIP_TIER: i32 = 1;
pub const SECONDARY_CHAMPIONSHIP_TIER: i32 = 2;
pub const TAG_TEAM_CHAMPIONSHIP_TIER: i32 = 3;
pub const SPECIALTY_CHAMPIONSHIP_TIER: i32 = 4;

/// Match type constraints
pub const SINGLES_MATCH_HOLDERS: usize = 1;
pub const TAG_TEAM_MATCH_HOLDERS: usize = 2;
pub const TRIPLE_TAG_TEAM_MATCH_HOLDERS: usize = 3;