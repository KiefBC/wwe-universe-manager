use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::Other => write!(f, "Other"),
        }
    }
}

impl From<String> for Gender {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "male" | "m" => Gender::Male,
            "female" | "f" => Gender::Female,
            _ => Gender::Other,
        }
    }
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        Gender::from(s.to_string())
    }
}

impl From<Gender> for String {
    fn from(gender: Gender) -> Self {
        gender.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ShowData {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Title {
    pub id: i32,
    pub name: String,
    pub current_holder_id: Option<i32>,
    pub title_type: String,
    pub division: String,
    pub prestige_tier: i32,
    pub gender: String,
    pub show_id: Option<i32>,
    pub is_active: bool,
    pub is_user_created: Option<bool>,
}


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Generic helper function for Tauri command invocation with proper error handling
pub async fn invoke_tauri<T, R>(command: &str, args: T) -> Result<R, String>
where
    T: Serialize,
    R: for<'de> Deserialize<'de>,
{
    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize args for command '{}': {}", command, e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    console::log_1(&format!("Invoking Tauri command '{}'...", command).into());
    let result_js = invoke(command, args_value).await;
    console::log_1(&format!("Tauri command '{}' returned result", command).into());

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize result from command '{}': {}", command, e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Generic helper for Tauri commands that don't need arguments
pub async fn invoke_tauri_no_args<R>(command: &str) -> Result<R, String>
where
    R: for<'de> Deserialize<'de>,
{
    console::log_1(&format!("Invoking Tauri command '{}' (no args)...", command).into());
    let result_js = invoke(command, JsValue::NULL).await;
    console::log_1(&format!("Tauri command '{}' returned result", command).into());

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize result from command '{}': {}", command, e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Fetches shows from the backend via Tauri
pub async fn fetch_shows() -> Result<Vec<Show>, String> {
    invoke_tauri_no_args("get_shows").await
}

/// Creates a new show via Tauri
pub async fn create_show(show_data: ShowData) -> Result<Show, String> {
    console::log_1(&format!("create_show called with: {:?}", show_data).into());

    let args = serde_json::json!({
        "showData": show_data
    });

    invoke_tauri("create_show", args).await
}


/// Fetches wrestlers from the backend via Tauri
pub async fn fetch_wrestlers() -> Result<Vec<Wrestler>, String> {
    invoke_tauri_no_args("get_wrestlers").await
}


/// Fetches wrestlers assigned to a specific show
pub async fn fetch_wrestlers_for_show(show_id: i32) -> Result<Vec<Wrestler>, String> {
    console::log_1(&format!("fetch_wrestlers_for_show called with show_id: {}", show_id).into());

    let args = serde_json::json!({
        "showId": show_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize show_id: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_wrestlers_for_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize wrestlers for show: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Assigns a wrestler to a show roster
pub async fn assign_wrestler_to_show(show_id: i32, wrestler_id: i32) -> Result<String, String> {
    console::log_1(&format!("assign_wrestler_to_show called with show_id: {}, wrestler_id: {}", show_id, wrestler_id).into());

    let args = serde_json::json!({
        "showId": show_id,
        "wrestlerId": wrestler_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize assignment data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("assign_wrestler_to_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize assignment result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Removes a wrestler from a show roster
pub async fn remove_wrestler_from_show(show_id: i32, wrestler_id: i32) -> Result<String, String> {
    console::log_1(&format!("remove_wrestler_from_show called with show_id: {}, wrestler_id: {}", show_id, wrestler_id).into());

    let args = serde_json::json!({
        "showId": show_id,
        "wrestlerId": wrestler_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize removal data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("remove_wrestler_from_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize removal result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

// Match-related types and functions

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Match {
    pub id: i32,
    pub show_id: i32,
    pub match_name: Option<String>,
    pub match_type: String,
    pub match_stipulation: Option<String>,
    pub scheduled_date: Option<String>,
    pub match_order: Option<i32>,
    pub winner_id: Option<i32>,
    pub is_title_match: bool,
    pub title_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MatchData {
    pub show_id: i32,
    pub match_name: Option<String>,
    pub match_type: String,
    pub match_stipulation: Option<String>,
    pub scheduled_date: Option<String>,
    pub match_order: Option<i32>,
    pub is_title_match: bool,
    pub title_id: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MatchParticipant {
    pub id: i32,
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MatchParticipantData {
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

/// Fetches matches for a specific show
pub async fn fetch_matches_for_show(show_id: i32) -> Result<Vec<Match>, String> {
    console::log_1(&format!("fetch_matches_for_show called with show_id: {}", show_id).into());

    let args = serde_json::json!({
        "showId": show_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize show_id: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_matches_for_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize matches: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Creates a new match
pub async fn create_match(match_data: MatchData) -> Result<Match, String> {
    console::log_1(&format!("create_match called with: {:?}", match_data).into());

    let args = serde_json::json!({
        "matchData": match_data
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize match data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("create_match", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize match result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Adds a wrestler to a match
pub async fn add_wrestler_to_match(match_id: i32, wrestler_id: i32, team_number: Option<i32>, entrance_order: Option<i32>) -> Result<String, String> {
    console::log_1(&format!("add_wrestler_to_match called with match_id: {}, wrestler_id: {}, team_number: {:?}, entrance_order: {:?}", match_id, wrestler_id, team_number, entrance_order).into());

    let args = serde_json::json!({
        "matchId": match_id,
        "wrestlerId": wrestler_id,
        "teamNumber": team_number,
        "entranceOrder": entrance_order
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize match participant data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("add_wrestler_to_match", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize add wrestler result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Fetches participants for a specific match
pub async fn fetch_match_participants(match_id: i32) -> Result<Vec<MatchParticipant>, String> {
    console::log_1(&format!("fetch_match_participants called with match_id: {}", match_id).into());

    let args = serde_json::json!({
        "matchId": match_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize match_id: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_match_participants", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize match participants: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Sets the winner of a match
pub async fn set_match_winner(match_id: i32, winner_id: i32) -> Result<String, String> {
    console::log_1(&format!("set_match_winner called with match_id: {}, winner_id: {}", match_id, winner_id).into());

    let args = serde_json::json!({
        "matchId": match_id,
        "winnerId": winner_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize winner data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("set_match_winner", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize winner result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}
