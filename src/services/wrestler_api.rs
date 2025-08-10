//! Wrestler API service module
//! 
//! This module provides frontend API wrappers for all wrestler-related
//! Tauri commands, including wrestler details, updates, and title management.

use crate::types::{invoke_tauri, Title, Show};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Enhanced Wrestler struct with additional fields for detailed wrestler management
/// TODO: This should be unified with the shared Wrestler type in src/types.rs
/// The shared type needs to be extended to include these additional fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrestlerDetails {
    pub id: i32,
    pub name: String,
    pub gender: String,
    pub wins: i32,
    pub losses: i32,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
    pub is_user_created: Option<bool>,
}

/// Title with current holder information
/// 
/// Combines a title with its current champion(s) and
/// calculates how long they've held the title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithHolders {
    pub title: Title,
    pub current_holders: Vec<TitleHolderInfo>,
    pub days_held: Option<i32>,
}

/// Title holder record representing a championship reign
/// 
/// Tracks when a wrestler held a title, including event details
/// and how the reign ended (if applicable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleHolder {
    pub id: i32,
    pub title_id: i32,
    pub wrestler_id: i32,
    pub held_since: String, // Using String for simplicity in frontend
    pub held_until: Option<String>,
    pub event_name: Option<String>,
    pub event_location: Option<String>,
    pub change_method: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Title holder information with wrestler details
/// 
/// Combines a TitleHolder record with the wrestler's name and gender
/// for display purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleHolderInfo {
    pub holder: TitleHolder,
    pub wrestler_name: String,
    pub wrestler_gender: String,
}

/// Fetches detailed wrestler information by ID
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler to fetch
/// 
/// # Returns
/// * `Ok(Some(WrestlerDetails))` - The wrestler's full details
/// * `Ok(None)` - No wrestler found with that ID
/// * `Err(String)` - Error message if the request fails
pub async fn get_wrestler_by_id(wrestler_id: i32) -> Result<Option<WrestlerDetails>, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    invoke_tauri("get_wrestler_by_id", args).await
}


/// Gets the shows that a wrestler is currently assigned to
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<Show>)` - List of shows the wrestler is assigned to
/// * `Err(String)` - Error message if the query fails
pub async fn get_wrestler_show_assignments(wrestler_id: i32) -> Result<Vec<Show>, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    invoke_tauri("get_shows_for_wrestler", args).await
}

/// Updates a wrestler's power ratings
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler to update
/// * `strength` - Physical strength rating (1-10)
/// * `speed` - Speed and quickness rating (1-10)
/// * `agility` - Agility and flexibility rating (1-10)
/// * `stamina` - Endurance rating (1-10)
/// * `charisma` - Crowd connection rating (1-10)
/// * `technique` - Technical ability rating (1-10)
/// 
/// # Returns
/// * `Ok(WrestlerDetails)` - Updated wrestler information
/// * `Err(String)` - Error message if the update fails
/// 
/// # Note
/// Pass None for any rating you don't want to change
pub async fn update_wrestler_power_ratings(
    wrestler_id: i32,
    strength: Option<i32>,
    speed: Option<i32>,
    agility: Option<i32>,
    stamina: Option<i32>,
    charisma: Option<i32>,
    technique: Option<i32>,
) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "strength": strength,
        "speed": speed,
        "agility": agility,
        "stamina": stamina,
        "charisma": charisma,
        "technique": technique
    });

    invoke_tauri("update_wrestler_power_ratings", args).await
}

/// Update wrestler's basic stats (height, weight, debut year, win/loss record)
pub async fn update_wrestler_basic_stats(
    wrestler_id: i32,
    height: Option<String>,
    weight: Option<String>,
    debut_year: Option<i32>,
    wins: i32,
    losses: i32,
) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "height": height,
        "weight": weight,
        "debutYear": debut_year,
        "wins": wins,
        "losses": losses
    });

    invoke_tauri("update_wrestler_basic_stats", args).await
}

/// Update wrestler's name and nickname
pub async fn update_wrestler_name(
    wrestler_id: i32,
    name: String,
    nickname: Option<String>,
) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "name": name,
        "nickname": nickname
    });

    invoke_tauri("update_wrestler_name", args).await
}

/// Update wrestler's real name
pub async fn update_wrestler_real_name(
    wrestler_id: i32,
    real_name: Option<String>,
) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "realName": real_name
    });

    invoke_tauri("update_wrestler_real_name", args).await
}

/// Update wrestler's biography
pub async fn update_wrestler_biography(
    wrestler_id: i32,
    biography: Option<String>,
) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "biography": biography
    });

    invoke_tauri("update_wrestler_biography", args).await
}

/// Deletes a wrestler from the system
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler to delete
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if deletion fails
/// 
/// # Note
/// Only user-created wrestlers can be deleted
pub async fn delete_wrestler(wrestler_id: i32) -> Result<String, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    invoke_tauri("delete_wrestler", args).await
}

/// Gets all titles available for a wrestler based on gender compatibility
/// 
/// # Arguments
/// * `wrestler_gender` - The wrestler's gender ("Male", "Female", or other)
/// 
/// # Returns
/// * `Ok(Vec<Title>)` - List of titles the wrestler can compete for
/// * `Err(String)` - Error message if the request fails
pub async fn get_titles_for_wrestler(wrestler_gender: String) -> Result<Vec<Title>, String> {
    let args = json!({
        "wrestlerGender": wrestler_gender
    });

    let titles_with_holders: Vec<TitleWithHolders> = invoke_tauri("get_titles_for_wrestler", args).await?;
    
    // Extract just the Title objects from TitleWithHolders
    Ok(titles_with_holders.into_iter().map(|twh| twh.title).collect())
}

/// Gets all championship titles currently held by a specific wrestler
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - List of titles with holder information
/// * `Err(String)` - Error message if the request fails
pub async fn get_current_titles_for_wrestler(wrestler_id: i32) -> Result<Vec<TitleWithHolders>, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    let all_titles: Vec<TitleWithHolders> = invoke_tauri("get_titles", args).await?;
    
    // Filter titles where the wrestler is the current holder
    Ok(all_titles.into_iter()
        .filter(|title_with_holders| {
            title_with_holders.current_holders.iter()
                .any(|holder| holder.holder.wrestler_id == wrestler_id)
        })
        .collect())
}

/// Vacates a championship title (removes current holder)
/// 
/// # Arguments
/// * `title_id` - The ID of the title to vacate
/// * `event_name` - Optional event where the vacancy occurred
/// * `event_location` - Optional location of the event
/// * `change_method` - Optional reason for vacancy (e.g., "Injury", "Retirement")
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if vacancy fails
pub async fn vacate_title(
    title_id: i32,
    event_name: Option<String>,
    event_location: Option<String>,
    change_method: Option<String>
) -> Result<String, String> {
    let args = json!({
        "titleId": title_id,
        "eventName": event_name,
        "eventLocation": event_location,
        "changeMethod": change_method
    });

    invoke_tauri("vacate_title", args).await
}

/// Assigns a championship title to a wrestler
/// 
/// # Arguments
/// * `title_id` - The ID of the title to assign
/// * `wrestler_id` - The ID of the new champion
/// * `event_name` - Optional event where the title changed hands
/// * `event_location` - Optional location of the event
/// * `change_method` - Optional method of victory (e.g., "Pinfall", "Submission")
/// 
/// # Returns
/// * `Ok(String)` - Success message
/// * `Err(String)` - Error message if assignment fails
pub async fn assign_title_to_wrestler(
    title_id: i32, 
    wrestler_id: i32,
    event_name: Option<String>,
    event_location: Option<String>,
    change_method: Option<String>
) -> Result<String, String> {
    let args = json!({
        "titleId": title_id,
        "newWrestlerId": wrestler_id,
        "eventName": event_name,
        "eventLocation": event_location,
        "changeMethod": change_method
    });

    invoke_tauri("update_title_holder", args).await
}

/// Gets titles that can be assigned to a wrestler
/// 
/// Filters titles based on gender compatibility and excludes
/// titles the wrestler already holds.
/// 
/// # Arguments
/// * `wrestler_id` - The ID of the wrestler
/// * `wrestler_gender` - The wrestler's gender
/// 
/// # Returns
/// * `Ok(Vec<TitleWithHolders>)` - List of assignable titles
/// * `Err(String)` - Error message if the request fails
pub async fn get_assignable_titles(wrestler_id: i32, wrestler_gender: String) -> Result<Vec<TitleWithHolders>, String> {
    // Get all titles available for the wrestler's gender
    let available_titles = get_titles_for_wrestler(wrestler_gender).await?;
    
    // Get titles currently held by this wrestler
    let current_titles = get_current_titles_for_wrestler(wrestler_id).await?;
    let current_title_ids: Vec<i32> = current_titles.iter().map(|t| t.title.id).collect();
    
    // Get all titles with holders to return the right format
    let all_titles: Vec<TitleWithHolders> = invoke_tauri("get_titles", json!({})).await?;
    
    // Filter titles: must be available for gender and not currently held by this wrestler
    Ok(all_titles.into_iter()
        .filter(|title_with_holders| {
            // Check if title is available for this gender
            let gender_compatible = available_titles.iter().any(|available| available.id == title_with_holders.title.id);
            
            // Check if title is not currently held by this wrestler
            let not_currently_held = !current_title_ids.contains(&title_with_holders.title.id);
            
            gender_compatible && not_currently_held
        })
        .collect())
}