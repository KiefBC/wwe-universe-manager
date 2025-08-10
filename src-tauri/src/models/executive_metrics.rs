use serde::{Deserialize, Serialize};

/// Executive dashboard metrics aggregated in a single query for performance
/// Contains key business intelligence indicators for leadership visibility
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutiveMetrics {
    /// Total number of active shows across all promotions
    pub total_shows: i32,
    /// Total number of wrestlers in global talent pool
    pub total_wrestlers: i32,
    /// Total number of championship titles
    pub total_championships: i32,
    /// Total number of scheduled matches
    pub total_matches: i32,
    /// Number of titles with current holders
    pub active_title_holders: i32,
    /// Count of recent activity (matches, title changes, etc.)
    pub recent_activity_count: i32,
}