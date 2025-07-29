/// URL parsing utilities for wrestler details window
/// 
/// This module provides functions for extracting wrestler IDs from URLs
/// and managing URL-based navigation state.

/// Extract wrestler ID from the current URL hash
/// 
/// Looks for URLs in the format: #wrestler?id=123
/// Returns Some(123) if found, None if not found or invalid
pub fn extract_wrestler_id_from_url() -> Option<i32> {
    web_sys::window()?
        .location()
        .hash()
        .ok()?
        .strip_prefix("#wrestler?id=")?
        .parse()
        .ok()
}

/// Check if the current URL is a wrestler details page
/// 
/// Returns true if the URL hash starts with "#wrestler"
pub fn is_wrestler_details_page() -> bool {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .map(|hash| hash.starts_with("#wrestler"))
        .unwrap_or(false)
}

/// Generate a wrestler details URL for the given ID
/// 
/// Returns a URL hash in the format: #wrestler?id=123
pub fn generate_wrestler_url(wrestler_id: i32) -> String {
    format!("#wrestler?id={}", wrestler_id)
}

/// Extract any query parameter from the current URL hash
/// 
/// For a URL like #wrestler?id=123&tab=stats, this function can extract
/// the value of any parameter by name
pub fn extract_url_param(param_name: &str) -> Option<String> {
    let window = web_sys::window()?;
    let hash = window.location().hash().ok()?;
    
    // Find the parameter in the hash
    let param_start = hash.find(&format!("{}=", param_name))?;
    let value_start = param_start + param_name.len() + 1;
    
    // Extract the value (until the next & or end of string)
    let remaining = &hash[value_start..];
    let value_end = remaining.find('&').unwrap_or(remaining.len());
    
    Some(remaining[..value_end].to_string())
}