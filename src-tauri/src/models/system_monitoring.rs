use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// System health monitoring data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemHealth {
    /// Overall system status
    pub status: String,
    /// System uptime in seconds
    pub uptime_seconds: i64,
    /// Database health information
    pub database_health: DatabaseHealth,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Current active alerts
    pub active_alerts: Vec<SystemAlert>,
    /// Recent operations log
    pub recent_operations: Vec<OperationLog>,
    /// Pending decisions or issues requiring attention
    pub pending_decisions: Vec<String>,
    /// System version information
    pub version: String,
    /// Database size in bytes
    pub database_size: i64,
    /// Memory usage in MB
    pub memory_usage: i32,
}

/// Database health metrics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseHealth {
    /// Average query response time in milliseconds
    pub avg_response_time: i32,
    /// Database connection pool status
    pub connection_pool_healthy: bool,
    /// Health score (0-100)
    pub health_score: i32,
    /// Number of active connections
    pub active_connections: i32,
    /// Total number of queries in last hour
    pub queries_last_hour: i32,
}

/// System performance metrics
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    /// Database response time in milliseconds
    pub db_response_time: i32,
    /// Database health score (0-100)
    pub db_health_score: i32,
    /// Memory usage in MB
    pub memory_usage: i32,
    /// CPU usage percentage (0-100)
    pub cpu_usage: i32,
    /// Request throughput (requests per minute)
    pub requests_per_minute: i32,
    /// Error rate percentage (0-100)
    pub error_rate: f64,
}

/// System alert with priority and details
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemAlert {
    /// Alert message
    pub message: String,
    /// Alert priority level
    pub priority: AlertPriority,
    /// Timestamp when alert was created
    pub created_at: DateTime<Utc>,
    /// Alert category (performance, security, business, etc.)
    pub category: String,
    /// Whether the alert requires immediate action
    pub requires_action: bool,
}

/// Alert priority levels
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AlertPriority {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Operation log entry for system activities
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationLog {
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// Operation description
    pub operation: String,
    /// Operation status (Success, Warning, Error)
    pub status: String,
    /// User or system that performed the operation
    pub performed_by: String,
    /// Additional details about the operation
    pub details: Option<String>,
    /// Duration of the operation in milliseconds
    pub duration_ms: Option<i32>,
}