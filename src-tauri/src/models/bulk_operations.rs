use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

/// Result structure for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationResult {
    /// Operation name/description
    pub operation_name: String,
    /// Success count
    pub success_count: u32,
    /// Error count
    pub error_count: u32,
    /// Success messages
    pub success_messages: Vec<String>,
    /// Error messages
    pub error_messages: Vec<String>,
    /// Operation timestamp
    pub timestamp: String,
}

impl BulkOperationResult {
    /// Create a new bulk operation result
    pub fn new(operation_name: &str) -> Self {
        Self {
            operation_name: operation_name.to_string(),
            success_count: 0,
            error_count: 0,
            success_messages: Vec::new(),
            error_messages: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// Add a success message
    pub fn add_success(&mut self, message: String) {
        self.success_count += 1;
        self.success_messages.push(message);
    }
    
    /// Add an error message
    pub fn add_error(&mut self, message: String) {
        self.error_count += 1;
        self.error_messages.push(message);
    }
    
    /// Check if operation was successful overall
    pub fn is_success(&self) -> bool {
        self.error_count == 0 && self.success_count > 0
    }
    
    /// Get overall success rate
    pub fn success_rate(&self) -> f64 {
        if self.success_count + self.error_count == 0 {
            0.0
        } else {
            (self.success_count as f64) / ((self.success_count + self.error_count) as f64)
        }
    }
}

/// Bulk wrestler power rating update data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkWrestlerPowerUpdate {
    pub wrestler_id: i32,
    pub overall_rating: i32,
    pub striking: i32,
    pub grappling: i32,
    pub aerial: i32,
    pub submission: i32,
    pub durability: i32,
    pub charisma: i32,
}

/// Bulk title update data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkTitleUpdate {
    pub title_id: i32,
    pub new_wrestler_id: Option<i32>,
    pub event_name: String,
    pub event_date: Option<NaiveDateTime>,
}

/// Bulk operation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationStatus {
    /// Current operation in progress
    pub current_operation: Option<String>,
    /// Progress percentage (0-100)
    pub progress: u8,
    /// Status message
    pub status_message: String,
    /// Items processed
    pub items_processed: u32,
    /// Total items
    pub total_items: u32,
    /// Operation start time
    pub started_at: Option<String>,
    /// Estimated completion time
    pub estimated_completion: Option<String>,
}

impl Default for BulkOperationStatus {
    fn default() -> Self {
        Self {
            current_operation: None,
            progress: 0,
            status_message: "Ready".to_string(),
            items_processed: 0,
            total_items: 0,
            started_at: None,
            estimated_completion: None,
        }
    }
}

/// Template type for bulk match creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchTemplate {
    Championship,
    Tournament,
    Rivalry,
    Custom(String),
}

impl MatchTemplate {
    pub fn as_str(&self) -> &str {
        match self {
            MatchTemplate::Championship => "championship",
            MatchTemplate::Tournament => "tournament",
            MatchTemplate::Rivalry => "rivalry",
            MatchTemplate::Custom(name) => name,
        }
    }
}

/// Bulk match creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkMatchCreationRequest {
    pub show_ids: Vec<i32>,
    pub template: MatchTemplate,
    pub custom_stipulations: Option<Vec<String>>,
    pub auto_assign_participants: bool,
}

/// Bulk roster assignment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkRosterAssignmentRequest {
    pub wrestler_ids: Vec<i32>,
    pub show_ids: Vec<i32>,
    pub remove_from_other_shows: bool,
}

/// Bulk wrestler creation batch (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkWrestlerCreationRequest {
    pub wrestler_count: u32,
    pub auto_assign_to_show_id: Option<i32>,
    pub default_power_rating: Option<i32>,
}

/// Progress tracking for long-running bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOperationProgress {
    pub operation_id: String,
    pub operation_type: String,
    pub status: BulkOperationProgressStatus,
    pub current_step: String,
    pub steps_completed: u32,
    pub total_steps: u32,
    pub progress_percentage: f64,
    pub started_at: String,
    pub updated_at: String,
    pub results: Option<BulkOperationResult>,
    pub estimated_completion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BulkOperationProgressStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

impl BulkOperationProgress {
    pub fn new(operation_id: String, operation_type: String, total_steps: u32) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            operation_id,
            operation_type,
            status: BulkOperationProgressStatus::Pending,
            current_step: "Initializing...".to_string(),
            steps_completed: 0,
            total_steps,
            progress_percentage: 0.0,
            started_at: now.clone(),
            updated_at: now,
            results: None,
            estimated_completion: None,
        }
    }
    
    pub fn update_progress(&mut self, steps_completed: u32, current_step: String) {
        self.steps_completed = steps_completed;
        self.current_step = current_step;
        self.progress_percentage = if self.total_steps > 0 {
            (steps_completed as f64 / self.total_steps as f64) * 100.0
        } else {
            0.0
        };
        self.updated_at = chrono::Utc::now().to_rfc3339();
        
        if steps_completed >= self.total_steps {
            self.status = BulkOperationProgressStatus::Completed;
        } else {
            self.status = BulkOperationProgressStatus::InProgress;
        }
    }
}

/// Simplified export summary for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkExportSummary {
    pub wrestler_count: u32,
    pub show_count: u32,
    pub title_count: u32,
    pub match_count: u32,
    pub export_timestamp: String,
    pub total_records: u32,
}

impl BulkExportSummary {
    pub fn new(wrestler_count: u32, show_count: u32, title_count: u32, match_count: u32) -> Self {
        let total_records = wrestler_count + show_count + title_count + match_count;
        Self {
            wrestler_count,
            show_count,
            title_count,
            match_count,
            export_timestamp: chrono::Utc::now().to_rfc3339(),
            total_records,
        }
    }
}

/// Validation results for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkValidationResult {
    pub is_valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub validation_summary: String,
}

impl BulkValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
            validation_summary: "Validation passed".to_string(),
        }
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }
    
    pub fn finalize(&mut self) {
        if self.is_valid {
            if self.warnings.is_empty() {
                self.validation_summary = "All validations passed successfully".to_string();
            } else {
                self.validation_summary = format!("Validation passed with {} warnings", self.warnings.len());
            }
        } else {
            self.validation_summary = format!("Validation failed with {} errors", self.errors.len());
        }
    }
}