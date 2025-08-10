use serde::{Deserialize, Serialize};

/// Comprehensive analytics data for business intelligence dashboard
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsData {
    // Overall metrics
    pub total_shows: i32,
    pub total_wrestlers: i32,
    pub total_championships: i32,
    pub total_matches: i32,
    pub active_title_holders: i32,

    // Performance analytics
    pub top_wrestler_win_rate: f64,
    pub total_active_wrestlers: i32,
    pub championship_contenders: i32,
    pub roster_utilization_rate: f64,
    pub avg_matches_per_show: f64,
    pub title_matches_percentage: i32,

    // Championship analytics
    pub average_prestige_tier: f64,
    pub vacant_titles: i32,

    // Strategic planning
    pub business_health_score: f64,
    pub growth_opportunities: i32,
    pub strategic_priorities: i32,

    // Detailed analytics
    pub top_wrestlers: Vec<WrestlerAnalytics>,
    pub show_analytics: Vec<ShowAnalytics>,
    pub title_analytics: Vec<TitleAnalytics>,
    pub performance_tiers: PerformanceTiers,
}

/// Wrestler performance analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct WrestlerAnalytics {
    pub id: i32,
    pub name: String,
    pub wins: i32,
    pub losses: i32,
    pub total_matches: i32,
    pub win_rate: f64,
    pub championship_count: i32,
    pub shows_assigned: i32,
}

/// Show performance analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct ShowAnalytics {
    pub id: i32,
    pub show_name: String,
    pub roster_size: i32,
    pub total_matches: i32,
    pub title_matches: i32,
    pub avg_match_quality: f64,
}

/// Title/Championship analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct TitleAnalytics {
    pub id: i32,
    pub title_name: String,
    pub prestige_tier: i32,
    pub has_holder: bool,
    pub days_held: Option<i32>,
    pub show_assignment: Option<String>,
}

/// Performance tier distribution for talent analytics
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceTiers {
    pub elite_count: i32,
    pub elite_percentage: f64,
    pub strong_count: i32,
    pub strong_percentage: f64,
    pub average_count: i32,
    pub average_percentage: f64,
    pub developing_count: i32,
    pub developing_percentage: f64,
}