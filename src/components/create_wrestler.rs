use crate::types::{Wrestler, Gender, Promotion};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedWrestlerData {
    pub name: String,
    pub gender: Gender,
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
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// Static error messages to avoid allocations
const HEIGHT_FORMAT_ERROR: &str = r#"Height must be in format like 6'1" (feet'inches") or 185cm (metric)"#;
const HEIGHT_RANGE_ERROR: &str = "Height must be between 3'0\" and 8'0\" (91-244cm)";
const HEIGHT_INCHES_ERROR: &str = "Inches must be 0-11 (there are only 12 inches in a foot)";
const HEIGHT_BOUNDS_ERROR: &str = "Height must be between 3'0\" and 8'0\" for realistic wrestler proportions";
const HEIGHT_INVALID_ERROR: &str = r#"Height must contain valid numbers in format like 6'1" or 185cm"#;
const HEIGHT_METRIC_BOUNDS_ERROR: &str = "Height must be between 91cm and 244cm for realistic proportions";

/// Validates height input supporting both imperial (6'1") and metric (185cm) formats
/// Optimized to minimize string allocations
fn validate_height(input: &str) -> Result<Cow<str>, &'static str> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Ok(Cow::Borrowed("Unknown"));
    }
    
    // Check if input is metric (ends with cm)
    if trimmed.to_lowercase().ends_with("cm") {
        let cm_str = &trimmed[..trimmed.len()-2].trim();
        let cm_value = cm_str.parse::<f64>().map_err(|_| HEIGHT_INVALID_ERROR)?;
        
        // Realistic bounds: 91cm to 244cm (3'0" to 8'0")  
        if cm_value < 91.0 || cm_value > 244.0 {
            return Err(HEIGHT_METRIC_BOUNDS_ERROR);
        }
        
        // Return normalized format (no decimals for whole numbers)
        if cm_value.fract() == 0.0 {
            Ok(Cow::Owned(format!("{:.0}cm", cm_value)))
        } else {
            Ok(Cow::Owned(format!("{:.1}cm", cm_value)))
        }
    } else {
        // Imperial format processing
        let apostrophe_pos = match trimmed.find('\'') {
            Some(pos) => pos,
            None => return Err(HEIGHT_FORMAT_ERROR),
        };
        
        // Zero-copy string slicing instead of Vec allocation
        let feet_str = &trimmed[..apostrophe_pos];
        let remaining = &trimmed[apostrophe_pos + 1..];
        
        // Remove potential trailing quote for parsing
        let inches_str = if remaining.ends_with('"') {
            &remaining[..remaining.len()-1]
        } else {
            remaining
        };
        
        // Parse and validate bounds
        let feet = feet_str.parse::<u32>().map_err(|_| HEIGHT_INVALID_ERROR)?;
        let inches = inches_str.parse::<u32>().map_err(|_| HEIGHT_INVALID_ERROR)?;
        
        // Realistic bounds: 3'0" to 8'0" (36 to 96 inches total)
        if feet < 3 || feet > 8 {
            return Err(HEIGHT_RANGE_ERROR);
        }
        if inches > 11 {
            return Err(HEIGHT_INCHES_ERROR);
        }
        
        // Additional check: ensure total height is within bounds (with overflow protection)
        let total_inches = feet.saturating_mul(12).saturating_add(inches);
        if total_inches < 36 || total_inches > 96 {
            return Err(HEIGHT_BOUNDS_ERROR);
        }
        
        // Optimize return value - avoid allocation if possible
        if trimmed.ends_with('"') {
            Ok(Cow::Borrowed(trimmed))
        } else {
            Ok(Cow::Owned(format!(r#"{}""#, trimmed)))
        }
    }
}

// Static error messages for weight validation
const WEIGHT_MIN_ERROR: &str = "Weight must be at least 50 lbs (23kg) for realistic wrestlers";
const WEIGHT_MAX_ERROR: &str = "Weight must be no more than 500 lbs (227kg) for realistic wrestlers";
const WEIGHT_INVALID_ERROR: &str = "Weight must be a valid number (e.g., 260, 120kg)";
const WEIGHT_METRIC_MIN_ERROR: &str = "Weight must be at least 23kg (50 lbs) for realistic wrestlers";
const WEIGHT_METRIC_MAX_ERROR: &str = "Weight must be no more than 227kg (500 lbs) for realistic wrestlers";

/// Processes weight input supporting both lbs and kg units, with realistic bounds checking
/// Optimized to minimize string allocations
fn process_weight(input: &str) -> Result<Cow<str>, &'static str> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Ok(Cow::Borrowed("Unknown"));
    }
    
    let lower = trimmed.to_lowercase();
    
    // Check for metric (kg) units
    if lower.ends_with("kg") {
        let kg_str = trimmed[..trimmed.len()-2].trim();
        let weight_kg = kg_str.parse::<f64>().map_err(|_| WEIGHT_INVALID_ERROR)?;
        
        // Realistic bounds: 23-227 kg for wrestlers (50-500 lbs)
        if weight_kg < 23.0 {
            return Err(WEIGHT_METRIC_MIN_ERROR);
        }
        if weight_kg > 227.0 {
            return Err(WEIGHT_METRIC_MAX_ERROR);
        }
        
        // Format consistently - remove unnecessary decimals for whole numbers
        if weight_kg.fract() == 0.0 {
            Ok(Cow::Owned(format!("{:.0}kg", weight_kg)))
        } else {
            Ok(Cow::Owned(format!("{:.1}kg", weight_kg)))
        }
    } else {
        // Imperial (lbs) units or plain numeric
        let numeric_str = if let Some(lbs_pos) = lower.find("lbs") {
            // Extract numeric part before "lbs" without creating intermediate strings
            trimmed[..lbs_pos].trim()
        } else {
            trimmed
        };
        
        // Parse and validate bounds
        let weight = numeric_str.parse::<f64>().map_err(|_| WEIGHT_INVALID_ERROR)?;
        
        // Realistic bounds: 50-500 lbs for wrestlers
        if weight < 50.0 {
            return Err(WEIGHT_MIN_ERROR);
        }
        if weight > 500.0 {
            return Err(WEIGHT_MAX_ERROR);
        }
        
        // Optimize return value - check if input already has correct format
        if lower.ends_with("lbs") && weight.fract() == 0.0 {
            // Check if the format matches our expected output
            let expected = format!("{:.0} lbs", weight);
            if trimmed.eq_ignore_ascii_case(&expected) {
                return Ok(Cow::Borrowed(trimmed));
            }
        }
        
        // Format consistently - remove unnecessary decimals for whole numbers
        if weight.fract() == 0.0 {
            Ok(Cow::Owned(format!("{:.0} lbs", weight)))
        } else {
            Ok(Cow::Owned(format!("{:.1} lbs", weight)))
        }
    }
}

// Static error messages for debut year validation
const DEBUT_YEAR_TOO_EARLY: &str = "Debut year must be 1950 or later (modern wrestling era)";
const DEBUT_YEAR_FUTURE: &str = "Debut year cannot be in the future (after 2024)";
const DEBUT_YEAR_INVALID: &str = "Debut year must be a valid 4-digit year (e.g., 1996)";

/// Validates debut year to ensure it's within realistic bounds
/// Optimized to minimize string allocations
fn validate_debut_year(input: &str) -> Result<Option<i32>, &'static str> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Ok(None);
    }
    
    let year = trimmed.parse::<i32>().map_err(|_| DEBUT_YEAR_INVALID)?;
    
    // Get current year - using a reasonable approximation since we can't access system time in WASM easily
    const CURRENT_YEAR: i32 = 2024; // This should be updated as needed
    
    // Realistic bounds: 1950 to current year
    if year < 1950 {
        return Err(DEBUT_YEAR_TOO_EARLY);
    }
    if year > CURRENT_YEAR {
        return Err(DEBUT_YEAR_FUTURE);
    }
    
    Ok(Some(year))
}

// Static error message for biography validation
const BIOGRAPHY_TOO_LONG: &str = "Biography must be under 2000 characters for readability";

/// Sanitizes biography input to prevent potential XSS and ensure reasonable length  
/// Optimized to minimize string allocations when no sanitization is needed
fn sanitize_biography(input: &str) -> Result<Cow<str>, &'static str> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Ok(Cow::Borrowed("Unknown"));
    }
    
    // Check length - reasonable biography should be under 2000 characters
    if trimmed.len() > 2000 {
        return Err(BIOGRAPHY_TOO_LONG);
    }
    
    // Check if sanitization is needed (zero-copy optimization)
    let needs_sanitization = trimmed.chars().any(|c| matches!(c, '&' | '<' | '>' | '"' | '\''));
    
    if !needs_sanitization {
        // No sanitization needed, return borrowed string
        if trimmed.len() == input.len() {
            Ok(Cow::Borrowed(input))
        } else {
            Ok(Cow::Owned(trimmed.to_string()))
        }
    } else {
        // Sanitization needed, build the sanitized string efficiently
        let capacity = trimmed.len() + 32; // Estimate extra space needed for entities
        let mut sanitized = String::with_capacity(capacity);
        
        for c in trimmed.chars() {
            match c {
                '&' => sanitized.push_str("&amp;"),
                '<' => sanitized.push_str("&lt;"),
                '>' => sanitized.push_str("&gt;"),
                '"' => sanitized.push_str("&quot;"),
                '\'' => sanitized.push_str("&#x27;"),
                _ => sanitized.push(c),
            }
        }
        
        Ok(Cow::Owned(sanitized))
    }
}

/// Returns "Unknown" for empty string fields, otherwise returns the trimmed value with zero-copy optimization
fn default_unknown_or_value(input: &str) -> Cow<str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        Cow::Borrowed("Unknown")
    } else if trimmed.len() == input.len() {
        // No trimming was needed, can borrow original
        Cow::Borrowed(input)
    } else {
        // Trimming was needed, must own the string
        Cow::Owned(trimmed.to_string())
    }
}

/// Fetches promotions from the backend via Tauri
async fn fetch_promotions() -> Result<Vec<Promotion>, String> {
    let result = invoke("get_promotions", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn create_user_wrestler(wrestler_data: EnhancedWrestlerData) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerData": wrestler_data
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("create_user_wrestler", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

#[component]
pub fn CreateWrestler(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (gender, set_gender) = signal(Gender::Male);
    let (real_name, set_real_name) = signal(String::new());
    let (nickname, set_nickname) = signal(String::new());
    let (height, set_height) = signal(String::new());
    let (weight, set_weight) = signal(String::new());
    let (debut_year, set_debut_year) = signal(String::new());
    let (promotion, set_promotion) = signal("Free Agent".to_string());
    let (strength, set_strength) = signal(5);
    let (speed, set_speed) = signal(5);
    let (agility, set_agility) = signal(5);
    let (stamina, set_stamina) = signal(5);
    let (charisma, set_charisma) = signal(5);
    let (technique, set_technique) = signal(5);
    let (biography, set_biography) = signal(String::new());
    
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);
    
    // Promotion-related state
    let (promotions, set_promotions) = signal(Vec::<Promotion>::new());
    let (promotions_loading, set_promotions_loading) = signal(false);
    let (promotions_error, set_promotions_error) = signal(None::<String>);

    // Load promotions on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_promotions_loading.set(true);
            set_promotions_error.set(None);
            
            match fetch_promotions().await {
                Ok(data) => {
                    set_promotions.set(data);
                }
                Err(e) => {
                    set_promotions_error.set(Some(e));
                }
            }
            set_promotions_loading.set(false);
        });
    });

    let submit_wrestler = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        if name.get().trim().is_empty() {
            set_error.set(Some("Wrestler name is required".to_string()));
            return;
        }

        // Validate height format
        let validated_height = match validate_height(&height.get()) {
            Ok(h) => h.into_owned(),
            Err(e) => {
                set_error.set(Some(e.to_string()));
                return;
            }
        };

        // Validate and process weight input
        let processed_weight = match process_weight(&weight.get()) {
            Ok(w) => w.into_owned(),
            Err(e) => {
                set_error.set(Some(e.to_string()));
                return;
            }
        };

        // Validate debut year
        let validated_debut_year = match validate_debut_year(&debut_year.get()) {
            Ok(year) => year,
            Err(e) => {
                set_error.set(Some(e.to_string()));
                return;
            }
        };

        // Sanitize biography
        let sanitized_biography = match sanitize_biography(&biography.get()) {
            Ok(bio) => bio.into_owned(),
            Err(e) => {
                set_error.set(Some(e.to_string()));
                return;
            }
        };

        let wrestler_data = EnhancedWrestlerData {
            name: name.get().trim().to_string(),
            gender: gender.get(),
            real_name: Some(default_unknown_or_value(&real_name.get()).into_owned()),
            nickname: Some(default_unknown_or_value(&nickname.get()).into_owned()),
            height: Some(validated_height),
            weight: Some(processed_weight),
            debut_year: validated_debut_year,
            promotion: Some(default_unknown_or_value(&promotion.get()).into_owned()),
            strength: Some(strength.get()),
            speed: Some(speed.get()),
            agility: Some(agility.get()),
            stamina: Some(stamina.get()),
            charisma: Some(charisma.get()),
            technique: Some(technique.get()),
            biography: Some(sanitized_biography),
        };

        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_user_wrestler(wrestler_data).await {
                Ok(_) => {
                    set_success.set(true);
                    // Navigate back to wrestlers list after a brief delay
                    set_timeout(
                        move || {
                            set_current_page.set("wrestlers".to_string());
                        },
                        std::time::Duration::from_millis(1500),
                    );
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn btn-ghost gap-2"
                        on:click=move |_| set_current_page.set("wrestlers".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Wrestlers"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "Create New Wrestler"
                </h1>
                <p class="text-base-content/70 mb-2">
                    "Add a custom wrestler to your universe"
                </p>
                <p class="text-base-content/60 text-sm">
                    "Only Name and Gender are required. Other fields will default to \"Unknown\" if empty."
                </p>
            </div>

            <Show when=move || success.get()>
                <div class="alert alert-success mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Wrestler Created Successfully!"</h3>
                        <div class="text-xs">"Redirecting to wrestlers list..."</div>
                    </div>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error Creating Wrestler"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <div class="card bg-base-200 border border-base-300">
                <div class="card-body">
                <form on:submit=submit_wrestler>
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        // Basic Information
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Basic Information"</h3>
                            
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Wrestler Name" <span class="text-error">"*"</span></span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., The Rock"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                    aria-label="Wrestler name (required)"
                                    aria-required="true"
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Gender" <span class="text-error">"*"</span></span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    prop:value=move || gender.get().to_string()
                                    on:change=move |ev| set_gender.set(Gender::from(event_target_value(&ev)))
                                    aria-label="Gender (required)"
                                    aria-required="true"
                                >
                                    <option value="Male">"Male"</option>
                                    <option value="Female">"Female"</option>
                                    <option value="Other">"Other"</option>
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Real Name"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., Dwayne Johnson"
                                    prop:value=real_name
                                    on:input=move |ev| set_real_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Nickname"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., The People's Champion"
                                    prop:value=nickname
                                    on:input=move |ev| set_nickname.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        // Physical Stats & Career
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Physical Stats & Career"</h3>
                            
                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Height"</span>
                                    </label>
                                    <input
                                        type="text"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 6'1\" or 185cm"
                                        prop:value=height
                                        on:input=move |ev| set_height.set(event_target_value(&ev))
                                        aria-label="Height in feet and inches or centimeters"
                                        aria-describedby="height-help"
                                    />
                                    <div id="height-help" class="text-xs text-base-content/60 mt-1">
                                        "Supports both imperial (6'1\") and metric (185cm) formats"
                                    </div>
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Weight"</span>
                                    </label>
                                    <input
                                        type="text"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 260 lbs or 120kg"
                                        prop:value=weight
                                        on:input=move |ev| set_weight.set(event_target_value(&ev))
                                        aria-label="Weight in pounds or kilograms"
                                        aria-describedby="weight-help"
                                    />
                                    <div id="weight-help" class="text-xs text-base-content/60 mt-1">
                                        "Supports both pounds (260 lbs) and kilograms (120kg)"
                                    </div>
                                </div>
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Debut Year"</span>
                                    </label>
                                    <input
                                        type="number"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 1996"
                                        prop:value=debut_year
                                        on:input=move |ev| set_debut_year.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Promotion"</span>
                                    </label>
                                    
                                    <Show when=move || promotions_error.get().is_some()>
                                        <div class="alert alert-warning mb-2 text-xs">
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-4 h-4">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                                            </svg>
                                            <div>
                                                <span class="font-bold">"Could not load promotions:"</span>
                                                <span class="ml-1">{move || promotions_error.get().unwrap_or_default()}</span>
                                            </div>
                                        </div>
                                    </Show>
                                    <select
                                        class="select select-bordered w-full"
                                        prop:value=promotion
                                        on:change=move |ev| set_promotion.set(event_target_value(&ev))
                                        aria-label="Promotion selection - choose from available promotions or Free Agent"
                                        aria-describedby="promotion-help"
                                        disabled=move || promotions_loading.get()
                                    >
                                        <Show
                                            when=move || promotions_loading.get()
                                            fallback=move || view! {
                                                <option value="Free Agent">"Free Agent"</option>
                                                <Show
                                                    when=move || promotions_error.get().is_none()
                                                    fallback=|| {}
                                                >
                                                    <For
                                                        each=move || promotions.get()
                                                        key=|promo| promo.id
                                                        children=move |promo| {
                                                            view! {
                                                                <option value={promo.name.clone()}>{promo.name.clone()}</option>
                                                            }
                                                        }
                                                    />
                                                </Show>
                                            }
                                        >
                                            <option value="Free Agent">"Loading promotions..."</option>
                                        </Show>
                                    </select>
                                    <div id="promotion-help" class="text-xs text-base-content/60 mt-1">
                                        "Select from available promotions or choose Free Agent"
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Power Ratings
                    <div class="mt-8">
                        <h3 class="text-xl font-semibold text-base-content mb-6">"Power Ratings"</h3>
                        <div class="grid grid-cols-2 lg:grid-cols-3 gap-6">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Strength: " <span class="text-secondary">{strength}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=strength
                                    on:input=move |ev| set_strength.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Speed: " <span class="text-secondary">{speed}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=speed
                                    on:input=move |ev| set_speed.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Agility: " <span class="text-secondary">{agility}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=agility
                                    on:input=move |ev| set_agility.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Stamina: " <span class="text-secondary">{stamina}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=stamina
                                    on:input=move |ev| set_stamina.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Charisma: " <span class="text-secondary">{charisma}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=charisma
                                    on:input=move |ev| set_charisma.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Technique: " <span class="text-secondary">{technique}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=technique
                                    on:input=move |ev| set_technique.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                        </div>
                    </div>

                    // Biography
                    <div class="mt-8">
                        <h3 class="text-xl font-semibold text-base-content mb-6">"Biography"</h3>
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Biography"</span>
                                </label>
                                <textarea
                                    rows="6"
                                    class="textarea textarea-bordered w-full"
                                    placeholder="Tell the wrestler's story..."
                                    prop:value=biography
                                    on:input=move |ev| set_biography.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                        </div>
                    </div>

                    // Submit Button
                    <div class="mt-8 flex justify-end space-x-4">
                        <button
                            type="button"
                            class="btn btn-ghost"
                            on:click=move |_| set_current_page.set("wrestlers".to_string())
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="btn btn-secondary"
                            disabled=move || loading.get()
                        >
                            <Show when=move || loading.get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            <span>{move || if loading.get() { "Creating..." } else { "Create Wrestler" }}</span>
                        </button>
                    </div>
                </form>
                </div>
            </div>
        </div>
    }
}