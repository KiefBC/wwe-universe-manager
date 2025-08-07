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
/// 
/// # Type Parameters
/// * `R` - Expected return type (must be deserializable)
/// 
/// # Arguments
/// * `command` - Name of the Tauri command to invoke
/// 
/// # Returns
/// * `Ok(R)` - Deserialized response from the command
/// * `Err(String)` - Error message if invocation or deserialization fails
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

/// Fetches all shows from the backend
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - List of all shows
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_shows() -> Result<Vec<Show>, String> {
    invoke_tauri_no_args("get_shows").await
}

/// Creates a new show via Tauri command
/// 
/// # Arguments
/// * `show_data` - Show name and description
/// 
/// # Returns
/// * `Ok(Show)` - The newly created show
/// * `Err(String)` - Error message if creation fails
pub async fn create_show(show_data: ShowData) -> Result<Show, String> {
    console::log_1(&format!("create_show called with: {:?}", show_data).into());

    let args = serde_json::json!({
        "showData": show_data
    });

    invoke_tauri("create_show", args).await
}


/// Fetches all wrestlers from the backend
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - List of all wrestlers
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_wrestlers() -> Result<Vec<Wrestler>, String> {
    invoke_tauri_no_args("get_wrestlers").await
}

/// Fetches all unassigned wrestlers (not on any show roster)
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - List of wrestlers not assigned to any show
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_unassigned_wrestlers() -> Result<Vec<Wrestler>, String> {
    invoke_tauri_no_args("get_unassigned_wrestlers").await
}

/// Fetches wrestlers assigned to a specific show's roster
/// 
/// # Arguments
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - List of wrestlers on the show's roster
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_wrestlers_for_show(show_id: i32) -> Result<Vec<Wrestler>, String> {
    console::log_1(&format!("fetch_wrestlers_for_show called with show_id: {}", show_id).into());

    let args = serde_json::json!({
        "showId": show_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameter 'show_id' for fetch_wrestlers_for_show(): {} (show_id: {})", e, show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_wrestlers_for_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize wrestlers response from fetch_wrestlers_for_show(): {} (show_id: {})", e, show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Assigns a wrestler to a show's roster
/// 
/// # Arguments
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to assign
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if assignment fails
pub async fn assign_wrestler_to_show(show_id: i32, wrestler_id: i32) -> Result<String, String> {

    let args = serde_json::json!({
        "showId": show_id,
        "wrestlerId": wrestler_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        format!("Failed to serialize parameters for assign_wrestler_to_show(): {} (show_id: {}, wrestler_id: {})", e, show_id, wrestler_id)
    })?;

    let result_js = invoke("assign_wrestler_to_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        format!("Failed to deserialize assignment response from assign_wrestler_to_show(): {} (show_id: {}, wrestler_id: {})", e, show_id, wrestler_id)
    })
}

/// Removes a wrestler from a show's roster
/// 
/// # Arguments
/// * `show_id` - ID of the show
/// * `wrestler_id` - ID of the wrestler to remove
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if removal fails
pub async fn remove_wrestler_from_show(show_id: i32, wrestler_id: i32) -> Result<String, String> {

    let args = serde_json::json!({
        "showId": show_id,
        "wrestlerId": wrestler_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        format!("Failed to serialize parameters for remove_wrestler_from_show(): {} (show_id: {}, wrestler_id: {})", e, show_id, wrestler_id)
    })?;

    let result_js = invoke("remove_wrestler_from_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        format!("Failed to deserialize removal response from remove_wrestler_from_show(): {} (show_id: {}, wrestler_id: {})", e, show_id, wrestler_id)
    })
}

/// Gets the shows that a wrestler is currently assigned to
/// 
/// # Arguments
/// * `wrestler_id` - ID of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - List of shows the wrestler is assigned to
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_shows_for_wrestler(wrestler_id: i32) -> Result<Vec<Show>, String> {
    console::log_1(&format!("fetch_shows_for_wrestler called with wrestler_id: {}", wrestler_id).into());

    let args = serde_json::json!({
        "wrestlerId": wrestler_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameter 'wrestler_id' for fetch_shows_for_wrestler(): {} (wrestler_id: {})", e, wrestler_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_shows_for_wrestler", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize shows response from fetch_shows_for_wrestler(): {} (wrestler_id: {})", e, wrestler_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

// Match-related types and functions

/// Match data structure representing a wrestling match
/// 
/// Matches are contests that take place on shows and can be
/// singles, tag team, or multi-person matches with various stipulations
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

/// Data transfer object for creating new matches
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

/// Match participant record linking wrestlers to matches
/// 
/// Tracks which wrestlers are in a match, their team assignment
/// (for tag matches), and entrance order
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MatchParticipant {
    pub id: i32,
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

/// Data transfer object for adding wrestlers to matches
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MatchParticipantData {
    pub match_id: i32,
    pub wrestler_id: i32,
    pub team_number: Option<i32>,
    pub entrance_order: Option<i32>,
}

/// Fetches all matches scheduled for a specific show
/// 
/// # Arguments
/// * `show_id` - ID of the show
/// 
/// # Returns
/// * `Ok(Vec<Match>)` - List of matches ordered by match order
/// * `Err(String)` - Error message if fetch fails
pub async fn fetch_matches_for_show(show_id: i32) -> Result<Vec<Match>, String> {
    console::log_1(&format!("fetch_matches_for_show called with show_id: {}", show_id).into());

    let args = serde_json::json!({
        "showId": show_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameter 'show_id' for fetch_matches_for_show(): {} (show_id: {})", e, show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_matches_for_show", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize matches response from fetch_matches_for_show(): {} (show_id: {})", e, show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Creates a new match for a show
/// 
/// # Arguments
/// * `match_data` - Match details including type, stipulation, and order
/// 
/// # Returns
/// * `Ok(Match)` - The newly created match
/// * `Err(String)` - Error message if creation fails
pub async fn create_match(match_data: MatchData) -> Result<Match, String> {
    console::log_1(&format!("create_match called with: {:?}", match_data).into());

    let args = serde_json::json!({
        "matchData": match_data
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize match data for create_match(): {} (match_type: {}, show_id: {})", e, match_data.match_type, match_data.show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("create_match", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize match response from create_match(): {} (match_type: {}, show_id: {})", e, match_data.match_type, match_data.show_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Adds a wrestler as a participant in a match
/// 
/// # Arguments
/// * `match_id` - ID of the match
/// * `wrestler_id` - ID of the wrestler to add
/// * `team_number` - Optional team assignment for tag matches
/// * `entrance_order` - Optional entrance order
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if addition fails
pub async fn add_wrestler_to_match(match_id: i32, wrestler_id: i32, team_number: Option<i32>, entrance_order: Option<i32>) -> Result<String, String> {
    console::log_1(&format!("add_wrestler_to_match called with match_id: {}, wrestler_id: {}, team_number: {:?}, entrance_order: {:?}", match_id, wrestler_id, team_number, entrance_order).into());

    let args = serde_json::json!({
        "matchId": match_id,
        "wrestlerId": wrestler_id,
        "teamNumber": team_number,
        "entranceOrder": entrance_order
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameters for add_wrestler_to_match(): {} (match_id: {}, wrestler_id: {}, team_number: {:?})", e, match_id, wrestler_id, team_number);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("add_wrestler_to_match", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize response from add_wrestler_to_match(): {} (match_id: {}, wrestler_id: {}, team_number: {:?})", e, match_id, wrestler_id, team_number);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Fetches all participants in a specific match
/// 
/// # Arguments
/// * `match_id` - ID of the match
/// 
/// # Returns
/// * `Ok(Vec<MatchParticipant>)` - List of participants with details
/// * `Err(String)` - Error message if fetch fails
#[allow(dead_code)]
pub async fn fetch_match_participants(match_id: i32) -> Result<Vec<MatchParticipant>, String> {
    console::log_1(&format!("fetch_match_participants called with match_id: {}", match_id).into());

    let args = serde_json::json!({
        "matchId": match_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameter 'match_id' for fetch_match_participants(): {} (match_id: {})", e, match_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("get_match_participants", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize match participants response from fetch_match_participants(): {} (match_id: {})", e, match_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Sets the winner of a completed match
/// 
/// # Arguments
/// * `match_id` - ID of the match
/// * `winner_id` - ID of the winning wrestler
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if update fails
#[allow(dead_code)]
pub async fn set_match_winner(match_id: i32, winner_id: i32) -> Result<String, String> {
    console::log_1(&format!("set_match_winner called with match_id: {}, winner_id: {}", match_id, winner_id).into());

    let args = serde_json::json!({
        "matchId": match_id,
        "winnerId": winner_id
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize parameters for set_match_winner(): {} (match_id: {}, winner_id: {})", e, match_id, winner_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    let result_js = invoke("set_match_winner", args_value).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize response from set_match_winner(): {} (match_id: {}, winner_id: {})", e, match_id, winner_id);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}
