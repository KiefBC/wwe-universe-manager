use crate::schema::signature_moves;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(crate::models::wrestler::Wrestler))]
#[diesel(table_name = signature_moves)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SignatureMove {
    pub id: Option<i32>,
    pub wrestler_id: i32,
    pub move_name: String,
    pub move_type: String, // "primary" or "secondary"
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = signature_moves)]
pub struct NewSignatureMove {
    pub wrestler_id: i32,
    pub move_name: String,
    pub move_type: String,
}

#[derive(Deserialize)]
pub struct SignatureMoveData {
    pub move_name: String,
    pub move_type: String, // "primary" or "secondary"
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Primary,
    Secondary,
}

impl From<String> for MoveType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "primary" => MoveType::Primary,
            "secondary" => MoveType::Secondary,
            _ => MoveType::Secondary, // Default to secondary for invalid values
        }
    }
}

impl From<MoveType> for String {
    fn from(move_type: MoveType) -> Self {
        match move_type {
            MoveType::Primary => "primary".to_string(),
            MoveType::Secondary => "secondary".to_string(),
        }
    }
}

impl SignatureMove {
    /// Get the CSS color class for this move type
    pub fn get_color_class(&self) -> &'static str {
        match self.move_type.as_str() {
            "primary" => "bg-red-600/80", // Red for primary finishers
            "secondary" => "bg-blue-600/80", // Blue for secondary finishers
            _ => "bg-gray-600/80", // Gray for unknown
        }
    }
    
    /// Get the border color class for this move type
    pub fn get_border_color_class(&self) -> &'static str {
        match self.move_type.as_str() {
            "primary" => "border-red-500",
            "secondary" => "border-blue-500",
            _ => "border-gray-500",
        }
    }
}