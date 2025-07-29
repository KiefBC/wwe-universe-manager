use crate::types::{invoke_tauri, Title};
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
    pub promotion: Option<String>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
    pub is_user_created: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithHolders {
    pub title: Title,
    pub current_holders: Vec<TitleHolderInfo>,
    pub days_held: Option<i32>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleHolderInfo {
    pub holder: TitleHolder,
    pub wrestler_name: String,
    pub wrestler_gender: String,
}

/// Get wrestler details by ID
pub async fn get_wrestler_by_id(wrestler_id: i32) -> Result<Option<WrestlerDetails>, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    invoke_tauri("get_wrestler_by_id", args).await
}

/// Update wrestler's promotion
pub async fn update_wrestler_promotion(wrestler_id: i32, promotion: String) -> Result<WrestlerDetails, String> {
    let args = json!({
        "wrestlerId": wrestler_id,
        "promotion": promotion
    });

    invoke_tauri("update_wrestler_promotion", args).await
}

/// Update wrestler's power ratings
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

/// Delete a wrestler
pub async fn delete_wrestler(wrestler_id: i32) -> Result<String, String> {
    let args = json!({
        "wrestlerId": wrestler_id
    });

    invoke_tauri("delete_wrestler", args).await
}

/// Get titles available for a wrestler based on gender
pub async fn get_titles_for_wrestler(wrestler_gender: String) -> Result<Vec<Title>, String> {
    let args = json!({
        "wrestlerGender": wrestler_gender
    });

    let titles_with_holders: Vec<TitleWithHolders> = invoke_tauri("get_titles_for_wrestler", args).await?;
    
    // Extract just the Title objects from TitleWithHolders
    Ok(titles_with_holders.into_iter().map(|twh| twh.title).collect())
}

/// Get current titles held by a specific wrestler
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

/// Vacate a title (remove current holder)
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

/// Assign a title to a wrestler with event details
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

/// Get titles that can be assigned to a wrestler based on gender, excluding currently held titles
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