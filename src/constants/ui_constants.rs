/// UI Constants for consistent styling and behavior across components
/// 
/// This module contains hardcoded values that were previously scattered
/// throughout the codebase for better maintainability and consistency.

/// Prestige tier configurations for championship titles
pub mod prestige_tiers {
    /// Prestige tier definitions with styling information
    #[derive(Debug, Clone)]
    pub struct PrestigeTierInfo {
        pub name: &'static str,
        pub text_color: &'static str,
        pub border_color: &'static str,
        pub background_color: &'static str,
    }
    
    /// World Championship (Tier 1) - Gold styling
    pub const WORLD_CHAMPIONSHIP: PrestigeTierInfo = PrestigeTierInfo {
        name: "World Championship",
        text_color: "text-warning",
        border_color: "border-warning",
        background_color: "bg-warning/20",
    };
    
    /// Secondary Championship (Tier 2) - Silver styling
    pub const SECONDARY_CHAMPIONSHIP: PrestigeTierInfo = PrestigeTierInfo {
        name: "Secondary Championship",
        text_color: "text-base-content/70",
        border_color: "border-base-300",
        background_color: "bg-base-200",
    };
    
    /// Tag Team Championship (Tier 3) - Bronze styling
    pub const TAG_TEAM_CHAMPIONSHIP: PrestigeTierInfo = PrestigeTierInfo {
        name: "Tag Team Championship",
        text_color: "text-accent",  
        border_color: "border-accent",
        background_color: "bg-accent/20",
    };
    
    /// Specialty Championship (Tier 4+) - Purple styling
    pub const SPECIALTY_CHAMPIONSHIP: PrestigeTierInfo = PrestigeTierInfo {
        name: "Specialty Championship",
        text_color: "text-secondary",
        border_color: "border-secondary", 
        background_color: "bg-secondary/20",
    };
    
    /// Get prestige tier information by tier number
    pub fn get_prestige_info(tier: i32) -> &'static PrestigeTierInfo {
        match tier {
            1 => &WORLD_CHAMPIONSHIP,
            2 => &SECONDARY_CHAMPIONSHIP,
            3 => &TAG_TEAM_CHAMPIONSHIP,
            _ => &SPECIALTY_CHAMPIONSHIP,
        }
    }
}

/// Layout and sizing constants
pub mod layout {
    /// Maximum height for scrollable areas (Tailwind: max-h-96)
    pub const SCROLLABLE_MAX_HEIGHT: &str = "max-h-96";
    
    /// Standard spacing between sections
    pub const SECTION_SPACING: &str = "space-y-2";
    
    /// Standard padding for containers
    pub const CONTAINER_PADDING: &str = "p-6";
    
    /// Standard margin for components
    pub const COMPONENT_MARGIN: &str = "mb-6";
    
    /// Grid layout for two-column responsive design
    pub const TWO_COLUMN_GRID: &str = "grid grid-cols-1 lg:grid-cols-2 gap-6";
    
    /// Standard card styling
    pub const CARD_STYLE: &str = "card bg-base-200 border border-base-300";
}

/// Loading and interactive states
pub mod states {
    /// Loading spinner size variants
    pub const SPINNER_SMALL: &str = "loading loading-spinner loading-sm";
    pub const SPINNER_MEDIUM: &str = "loading loading-spinner loading-md";
    pub const SPINNER_LARGE: &str = "loading loading-spinner loading-lg";
    
    /// Standard loading message
    pub const LOADING_MESSAGE: &str = "Loading...";
    
    /// Standard success message
    pub const SUCCESS_MESSAGE: &str = "Operation completed successfully";
    
    /// Standard error message prefix
    pub const ERROR_PREFIX: &str = "An error occurred: ";
}

/// Form-related constants
pub mod forms {
    /// Default championship change method
    pub const DEFAULT_CHANGE_METHOD: &str = "won";
    
    /// Change method options for championship transfers
    pub const CHANGE_METHODS: &[(&str, &str)] = &[
        ("won", "Won"),
        ("awarded", "Awarded"),
        ("stripped", "Previous champion stripped"),
        ("vacated", "Previous champion vacated"),
    ];
    
    /// Match type options
    pub const MATCH_TYPES: &[&str] = &[
        "Singles",
        "Tag Team", 
        "Triple Threat",
        "Fatal 4-Way",
        "Battle Royal",
        "Ladder Match",
        "Cage Match",
    ];
    
    /// Match stipulation options
    pub const MATCH_STIPULATIONS: &[&str] = &[
        "Standard",
        "No Holds Barred",
        "Street Fight",
        "Last Man Standing",
        "Submission Match",
        "Iron Man Match",
        "Hell in a Cell",
    ];
}

/// Messages and text content
pub mod messages {
    /// Empty state messages
    pub const NO_WRESTLERS: &str = "No wrestlers available";
    pub const NO_TITLES: &str = "No titles available";
    pub const NO_MATCHES: &str = "No matches scheduled";
    pub const NO_SHOWS: &str = "No shows available";
    
    /// Placeholder text
    pub const SELECT_WRESTLER: &str = "Select a wrestler...";
    pub const SELECT_SHOW: &str = "Select a show...";
    pub const SELECT_TITLE: &str = "Select a title...";
    
    /// Action confirmations
    pub const CONFIRM_DELETE: &str = "Are you sure you want to delete";
    pub const CANNOT_UNDO: &str = "This action cannot be undone.";
    
    /// Validation messages
    pub const REQUIRED_FIELD: &str = "This field is required";
    pub const INVALID_SELECTION: &str = "Please make a valid selection";
    
    /// Wrestling-specific messages
    pub const TITLE_VACANT: &str = "Vacant";
    pub const NEW_CHAMPION: &str = "New champion";
    pub const NO_CURRENT_HOLDER: &str = "No current holder";
    pub const CHAMPIONSHIP_REIGN: &str = "Championship Reign";
}

/// Icon and visual elements
pub mod icons {
    /// SVG paths for common icons (simplified for brevity)
    pub const ERROR_ICON_PATH: &str = "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";
    pub const SUCCESS_ICON_PATH: &str = "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z";
    pub const WARNING_ICON_PATH: &str = "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z";
    pub const DELETE_ICON_PATH: &str = "M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16";
    
    /// Title/championship icon path
    pub const CHAMPIONSHIP_ICON_PATH: &str = "M5 16L3 14l5.5-5.5L10 10l4-4 4 4 1.5-1.5L15 3l-4 4L7 3 2.5 8.5 5 11v5zm2.5 2.5L9 17l1.5 1.5L12 17l1.5 1.5L15 17l1.5 1.5L18 17v-2l-1.5-1.5L15 15l-1.5-1.5L12 15l-1.5-1.5L9 15l-1.5 1.5L6 17v2l1.5-1.5z";
}

/// Time and duration formatting
pub mod time {
    /// Format days into human readable text
    pub fn format_days_held(days: i32) -> String {
        match days {
            0 => "Today".to_string(),
            1 => "1 day".to_string(),
            d if d < 7 => format!("{} days", d),
            d => {
                let weeks = d / 7;
                let remaining_days = d % 7;
                if remaining_days > 0 {
                    format!("{} weeks, {} days", weeks, remaining_days)
                } else {
                    format!("{} weeks", weeks)
                }
            }
        }
    }
}