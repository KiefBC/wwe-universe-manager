use crate::types::{Wrestler, Gender, Show};
use crate::utils::navigation::{WorkflowSteps, NavigationContext};
use crate::components::executive_layout::{ExecutivePageLayout, ExecutiveContentSection};
use crate::components::form_enhancements::*;
use crate::components::enhanced_buttons::*;
use crate::components::animation_utils::*;
use crate::components::loading_states::*;
use crate::components::notification_system::*;
use leptos::prelude::*;
use leptos::ev;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
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

// Validation constants
const HEIGHT_FORMAT_ERROR: &str = r#"Height must be in format like 6'1" (feet'inches") or 185cm (metric)"#;
const HEIGHT_RANGE_ERROR: &str = "Height must be between 3'0\" and 8'0\" (91-244cm)";
const HEIGHT_INCHES_ERROR: &str = "Inches must be 0-11 (there are only 12 inches in a foot)";
const HEIGHT_BOUNDS_ERROR: &str = "Height must be between 3'0\" and 8'0\" for realistic wrestler proportions";
const HEIGHT_INVALID_ERROR: &str = r#"Height must contain valid numbers in format like 6'1" or 185cm"#;
const HEIGHT_METRIC_BOUNDS_ERROR: &str = "Height must be between 91cm and 244cm for realistic proportions";

/// Enhanced height validation with professional feedback
fn validate_height(input: &str) -> Option<String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return None;
    }
    
    // Check if input is metric (ends with cm)
    if trimmed.to_lowercase().ends_with("cm") {
        let cm_str = &trimmed[..trimmed.len()-2].trim();
        let cm_value = match cm_str.parse::<f64>() {
            Ok(v) => v,
            Err(_) => return Some(HEIGHT_INVALID_ERROR.to_string()),
        };
        
        // Realistic bounds: 91cm to 244cm (3'0" to 8'0")  
        if cm_value < 91.0 || cm_value > 244.0 {
            return Some(HEIGHT_METRIC_BOUNDS_ERROR.to_string());
        }
        
        None
    } else {
        // Imperial format processing
        let apostrophe_pos = match trimmed.find('\'') {
            Some(pos) => pos,
            None => return Some(HEIGHT_FORMAT_ERROR.to_string()),
        };
        
        let feet_str = &trimmed[..apostrophe_pos];
        let remaining = &trimmed[apostrophe_pos + 1..];
        
        let inches_str = if remaining.ends_with('"') {
            &remaining[..remaining.len()-1]
        } else {
            remaining
        };
        
        // Parse and validate bounds
        let feet = match feet_str.parse::<u32>() {
            Ok(f) => f,
            Err(_) => return Some(HEIGHT_INVALID_ERROR.to_string()),
        };
        let inches = match inches_str.parse::<u32>() {
            Ok(i) => i,
            Err(_) => return Some(HEIGHT_INVALID_ERROR.to_string()),
        };
        
        // Realistic bounds: 3'0" to 8'0" (36 to 96 inches total)
        if feet < 3 || feet > 8 {
            return Some(HEIGHT_RANGE_ERROR.to_string());
        }
        if inches > 11 {
            return Some(HEIGHT_INCHES_ERROR.to_string());
        }
        
        // Additional total height validation
        let total_inches = feet * 12 + inches;
        if total_inches < 36 || total_inches > 96 {
            return Some(HEIGHT_BOUNDS_ERROR.to_string());
        }
        
        None
    }
}

/// Enhanced weight validation with professional feedback
fn validate_weight(input: &str) -> Option<String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return None;
    }
    
    // Parse weight (support both lbs and kg)
    let is_metric = trimmed.to_lowercase().ends_with("kg");
    let is_imperial = trimmed.to_lowercase().ends_with("lbs");
    
    if !is_metric && !is_imperial {
        return Some("Weight must end with 'lbs' or 'kg' (e.g., '185lbs' or '84kg')".to_string());
    }
    
    let suffix_len = if is_metric { 2 } else { 3 };
    let weight_str = &trimmed[..trimmed.len()-suffix_len].trim();
    let weight_value = match weight_str.parse::<f64>() {
        Ok(w) => w,
        Err(_) => return Some("Weight must be a valid number".to_string()),
    };
    
    // Realistic bounds
    if is_metric {
        if weight_value < 45.0 || weight_value > 200.0 {
            return Some("Weight must be between 45kg and 200kg for realistic proportions".to_string());
        }
    } else {
        if weight_value < 100.0 || weight_value > 440.0 {
            return Some("Weight must be between 100lbs and 440lbs for realistic proportions".to_string());
        }
    }
    
    None
}

/// Enhanced name validation with professional feedback
fn validate_name(input: &String) -> Option<String> {
    let trimmed = input.trim();
    
    if trimmed.is_empty() {
        return Some("Name is required".to_string());
    }
    
    if trimmed.len() < 2 {
        return Some("Name must be at least 2 characters".to_string());
    }
    
    if trimmed.len() > 50 {
        return Some("Name must be 50 characters or less".to_string());
    }
    
    // Check for basic professionalism
    if trimmed.chars().all(|c| c.is_numeric()) {
        return Some("Name cannot be all numbers".to_string());
    }
    
    None
}

/// Enhanced debut year validation
fn validate_debut_year(input: &String) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    }
    
    let year = match input.trim().parse::<i32>() {
        Ok(y) => y,
        Err(_) => return Some("Debut year must be a valid year".to_string()),
    };
    
    let current_year = js_sys::Date::new_0().get_full_year() as i32;
    
    if year < 1950 {
        return Some("Debut year cannot be before 1950".to_string());
    }
    
    if year > current_year + 5 {
        return Some("Debut year cannot be more than 5 years in the future".to_string());
    }
    
    None
}

/// Enhanced power rating validation
fn validate_power_rating(input: &String) -> Option<String> {
    if input.trim().is_empty() {
        return None;
    }
    
    let rating = match input.trim().parse::<i32>() {
        Ok(r) => r,
        Err(_) => return Some("Power rating must be a number".to_string()),
    };
    
    if rating < 1 || rating > 100 {
        return Some("Power rating must be between 1 and 100".to_string());
    }
    
    None
}

/// Enhanced Create Wrestler Component with Phase 4.5 Micro-Interactions
/// 
/// Professional wrestler creation form featuring:
/// - Executive-quality form with real-time validation
/// - Smooth animations and professional feedback
/// - Progress tracking with visual indicators
/// - Enhanced error handling and recovery
/// - Professional notification system
/// - Staggered form field entrance animations
#[component]
pub fn CreateWrestlerEnhanced(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    // Form state with enhanced tracking
    let (name, set_name) = signal(String::new());
    let (real_name, set_real_name) = signal(String::new());
    let (nickname, set_nickname) = signal(String::new());
    let (gender, set_gender) = signal("Male".to_string());
    let (height, set_height) = signal(String::new());
    let (weight, set_weight) = signal(String::new());
    let (debut_year, set_debut_year) = signal(String::new());
    let (strength, set_strength) = signal(String::new());
    let (speed, set_speed) = signal(String::new());
    let (agility, set_agility) = signal(String::new());
    let (stamina, set_stamina) = signal(String::new());
    let (charisma, set_charisma) = signal(String::new());
    let (technique, set_technique) = signal(String::new());
    let (biography, set_biography) = signal(String::new());
    
    // Enhanced form state management
    let (loading, set_loading) = signal(false);
    let (success, set_success) = signal(false);
    let (error, set_error) = signal(None::<String>());
    let (form_step, set_form_step) = signal(0); // 0: Basic, 1: Details, 2: Stats, 3: Bio
    let (form_progress, set_form_progress) = signal(0.0);

    // Notification system
    let notification_manager = NotificationManager::new();

    // Calculate form completion progress
    Effect::new(move || {
        let mut completed_fields = 0;
        let total_fields = 13;
        
        if !name.get().trim().is_empty() { completed_fields += 1; }
        if !real_name.get().trim().is_empty() { completed_fields += 1; }
        if !nickname.get().trim().is_empty() { completed_fields += 1; }
        if !height.get().trim().is_empty() { completed_fields += 1; }
        if !weight.get().trim().is_empty() { completed_fields += 1; }
        if !debut_year.get().trim().is_empty() { completed_fields += 1; }
        if !strength.get().trim().is_empty() { completed_fields += 1; }
        if !speed.get().trim().is_empty() { completed_fields += 1; }
        if !agility.get().trim().is_empty() { completed_fields += 1; }
        if !stamina.get().trim().is_empty() { completed_fields += 1; }
        if !charisma.get().trim().is_empty() { completed_fields += 1; }
        if !technique.get().trim().is_empty() { completed_fields += 1; }
        if !biography.get().trim().is_empty() { completed_fields += 1; }
        
        let progress = (completed_fields as f32 / total_fields as f32) * 100.0;
        set_form_progress.set(progress);
    });

    // Enhanced form submission with professional feedback
    let handle_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        
        // Validate required fields
        if name.get().trim().is_empty() {
            notification_manager.show_error("Validation Error", "Wrestler name is required");
            return;
        }

        set_loading.set(true);
        set_error.set(None);
        
        // Show loading notification
        let loading_id = notification_manager.show_loading("Creating Wrestler", "Processing talent profile...");
        
        spawn_local(async move {
            let wrestler_data = EnhancedWrestlerData {
                name: name.get_untracked().trim().to_string(),
                gender: match gender.get_untracked().as_str() {
                    "Female" => Gender::Female,
                    _ => Gender::Male,
                },
                real_name: if real_name.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    Some(real_name.get_untracked().trim().to_string()) 
                },
                nickname: if nickname.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    Some(nickname.get_untracked().trim().to_string()) 
                },
                height: if height.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    Some(height.get_untracked().trim().to_string()) 
                },
                weight: if weight.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    Some(weight.get_untracked().trim().to_string()) 
                },
                debut_year: if debut_year.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    debut_year.get_untracked().trim().parse().ok() 
                },
                strength: if strength.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    strength.get_untracked().trim().parse().ok() 
                },
                speed: if speed.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    speed.get_untracked().trim().parse().ok() 
                },
                agility: if agility.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    agility.get_untracked().trim().parse().ok() 
                },
                stamina: if stamina.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    stamina.get_untracked().trim().parse().ok() 
                },
                charisma: if charisma.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    charisma.get_untracked().trim().parse().ok() 
                },
                technique: if technique.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    technique.get_untracked().trim().parse().ok() 
                },
                biography: if biography.get_untracked().trim().is_empty() { 
                    None 
                } else { 
                    Some(biography.get_untracked().trim().to_string()) 
                },
            };
            
            let args = match serde_wasm_bindgen::to_value(&wrestler_data) {
                Ok(args) => args,
                Err(e) => {
                    notification_manager.dismiss(&loading_id);
                    notification_manager.show_error("Serialization Error", &e.to_string());
                    set_loading.set(false);
                    return;
                }
            };

            match invoke("create_user_wrestler", args).await {
                Ok(result) => {
                    notification_manager.dismiss(&loading_id);
                    
                    match serde_wasm_bindgen::from_value::<Wrestler>(result) {
                        Ok(created_wrestler) => {
                            set_success.set(true);
                            set_error.set(None);
                            
                            // Show success notification with wrestler name
                            notification_manager.show_success(
                                "Wrestler Created",
                                &format!("{} has been added to your roster!", created_wrestler.name)
                            );
                            
                            // Navigate back after delay
                            spawn_local(async move {
                                gloo_timers::future::TimeoutFuture::new(2000).await;
                                set_current_page.set("wrestlers".to_string());
                            });
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to parse response: {}", e);
                            set_error.set(Some(error_msg.clone()));
                            notification_manager.show_error("Creation Error", &error_msg);
                        }
                    }
                }
                Err(e) => {
                    notification_manager.dismiss(&loading_id);
                    let error_msg = format!("Failed to create wrestler: {:?}", e);
                    set_error.set(Some(error_msg.clone()));
                    notification_manager.show_error("Creation Failed", &error_msg);
                }
            }
            
            set_loading.set(false);
        });
    };

    // Step navigation handlers
    let next_step = move |_| {
        let current = form_step.get();
        if current < 3 {
            set_form_step.set(current + 1);
        }
    };

    let prev_step = move |_| {
        let current = form_step.get();
        if current > 0 {
            set_form_step.set(current - 1);
        }
    };

    // Calculate step progress
    let step_progress = move || ((form_step.get() + 1) as f32 / 4.0) * 100.0;

    view! {
        <ExecutivePageLayout
            title="Create Wrestler".to_string()
            subtitle=Some("Add new talent to your wrestling roster with comprehensive profile details".to_string())
        >
            // Progress tracking section
            <ExecutiveContentSection
                title="Creation Progress".to_string()
                class="mb-6".to_string()
            >
                <div class="card bg-base-100 border border-base-300/50 shadow-lg animate-card-entrance">
                    <div class="card-body p-6">
                        <div class="flex items-center justify-between mb-4">
                            <div>
                                <h3 class="text-lg font-bold text-base-content">Form Completion</h3>
                                <p class="text-sm text-base-content/70">Fill out wrestler details to build their profile</p>
                            </div>
                            <div class="text-right">
                                <div class="text-2xl font-bold text-primary animate-metric-counter">
                                    {move || format!("{:.0}%", form_progress.get())}
                                </div>
                                <div class="text-xs text-base-content/60">Complete</div>
                            </div>
                        </div>
                        
                        <AnimatedProgress 
                            value=Signal::derive(move || form_progress.get())
                            color="primary".to_string()
                            show_percentage=false
                            class="mb-4".to_string()
                        />
                        
                        // Step indicator
                        <div class="steps w-full">
                            <div class={move || if form_step.get() >= 0 { "step step-primary" } else { "step" }}>
                                "Basic Info"
                            </div>
                            <div class={move || if form_step.get() >= 1 { "step step-primary" } else { "step" }}>
                                "Personal Details"
                            </div>
                            <div class={move || if form_step.get() >= 2 { "step step-primary" } else { "step" }}>
                                "Power Stats"
                            </div>
                            <div class={move || if form_step.get() >= 3 { "step step-primary" } else { "step" }}>
                                "Biography"
                            </div>
                        </div>
                    </div>
                </div>
            </ExecutiveContentSection>

            // Main form with step-by-step interface
            <ExecutiveContentSection
                title=move || match form_step.get() {
                    0 => "Step 1: Basic Information".to_string(),
                    1 => "Step 2: Personal Details".to_string(), 
                    2 => "Step 3: Power Ratings".to_string(),
                    3 => "Step 4: Biography & Background".to_string(),
                    _ => "Wrestler Profile".to_string(),
                }
                class="".to_string()
            >
                <ExecutiveForm
                    title=move || match form_step.get() {
                        0 => "Essential Wrestler Information".to_string(),
                        1 => "Physical Characteristics".to_string(),
                        2 => "Athletic Performance Ratings".to_string(), 
                        3 => "Background & Story".to_string(),
                        _ => "Wrestler Profile".to_string(),
                    }
                    description=Some(move || match form_step.get() {
                        0 => "Provide the core identity details for your wrestler".to_string(),
                        1 => "Add physical stats and career information".to_string(),
                        2 => "Rate athletic abilities from 1-100 (optional but recommended)".to_string(),
                        3 => "Create a compelling backstory for your wrestler".to_string(),
                        _ => "Complete wrestler creation".to_string(),
                    })
                    on_submit=Box::new(handle_submit)
                    loading=loading.get()
                    success=success.get()
                    error=error.get()
                    class="max-w-4xl mx-auto".to_string()
                >
                    {move || match form_step.get() {
                        0 => view! {
                            // Step 1: Basic Info
                            <StaggeredContainer stagger_delay=100 class="space-y-6".to_string()>
                                <ExecutiveInput
                                    label="Wrestling Name".to_string()
                                    input_type="text".to_string()
                                    value=name.into()
                                    set_value=set_name
                                    placeholder="e.g., Stone Cold Steve Austin".to_string()
                                    required=true
                                    helper=Some("This is the name fans will know them by".to_string())
                                    validate=Some(Box::new(validate_name))
                                />
                                
                                <ExecutiveSelect
                                    label="Gender".to_string()
                                    value=gender.into()
                                    set_value=set_gender
                                    options=vec![
                                        ("Male".to_string(), "Male".to_string()),
                                        ("Female".to_string(), "Female".to_string()),
                                    ]
                                    required=true
                                    helper=Some("Determines division and championship eligibility".to_string())
                                />
                                
                                <ExecutiveInput
                                    label="Real Name".to_string()
                                    input_type="text".to_string()
                                    value=real_name.into()
                                    set_value=set_real_name
                                    placeholder="e.g., Steven James Anderson".to_string()
                                    helper=Some("The wrestler's actual legal name (optional)".to_string())
                                />
                                
                                <ExecutiveInput
                                    label="Nickname".to_string()
                                    input_type="text".to_string()
                                    value=nickname.into()
                                    set_value=set_nickname
                                    placeholder="e.g., The Texas Rattlesnake".to_string()
                                    helper=Some("A memorable moniker or catchphrase (optional)".to_string())
                                />
                            </StaggeredContainer>
                        }.into_any(),
                        
                        1 => view! {
                            // Step 2: Physical Details
                            <StaggeredContainer stagger_delay=100 class="space-y-6".to_string()>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <ExecutiveInput
                                        label="Height".to_string()
                                        input_type="text".to_string()
                                        value=height.into()
                                        set_value=set_height
                                        placeholder=r#"e.g., 6'1" or 185cm"#.to_string()
                                        helper=Some("Supports both imperial (6'1\") and metric (185cm) formats".to_string())
                                        validate=Some(Box::new(|h: &String| validate_height(h)))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Weight".to_string()
                                        input_type="text".to_string()
                                        value=weight.into()
                                        set_value=set_weight
                                        placeholder="e.g., 250lbs or 113kg".to_string()
                                        helper=Some("Include units (lbs or kg)".to_string())
                                        validate=Some(Box::new(|w: &String| validate_weight(w)))
                                    />
                                </div>
                                
                                <ExecutiveInput
                                    label="Debut Year".to_string()
                                    input_type="number".to_string()
                                    value=debut_year.into()
                                    set_value=set_debut_year
                                    placeholder="e.g., 1995".to_string()
                                    helper=Some("The year they started their wrestling career".to_string())
                                    validate=Some(Box::new(validate_debut_year))
                                />
                            </StaggeredContainer>
                        }.into_any(),
                        
                        2 => view! {
                            // Step 3: Power Ratings
                            <StaggeredContainer stagger_delay=100 class="space-y-6".to_string()>
                                <div class="alert alert-info mb-6 animate-card-entrance">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    <div>
                                        <h3 class="font-bold">Power Ratings Guide</h3>
                                        <div class="text-sm">Rate each attribute from 1-100 based on the wrestler abilities. All ratings are optional but help create a more detailed profile.</div>
                                    </div>
                                </div>
                                
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <ExecutiveInput
                                        label="Strength (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=strength.into()
                                        set_value=set_strength
                                        placeholder="e.g., 85".to_string()
                                        helper=Some("Physical power and lifting ability".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Speed (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=speed.into()
                                        set_value=set_speed
                                        placeholder="e.g., 75".to_string()
                                        helper=Some("Quickness and agility in the ring".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Agility (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=agility.into()
                                        set_value=set_agility
                                        placeholder="e.g., 90".to_string()
                                        helper=Some("Flexibility and acrobatic skills".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Stamina (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=stamina.into()
                                        set_value=set_stamina
                                        placeholder="e.g., 80".to_string()
                                        helper=Some("Endurance and match longevity".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Charisma (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=charisma.into()
                                        set_value=set_charisma
                                        placeholder="e.g., 95".to_string()
                                        helper=Some("Crowd connection and mic skills".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                    
                                    <ExecutiveInput
                                        label="Technique (1-100)".to_string()
                                        input_type="number".to_string()
                                        value=technique.into()
                                        set_value=set_technique
                                        placeholder="e.g., 70".to_string()
                                        helper=Some("Wrestling technical skill and execution".to_string())
                                        validate=Some(Box::new(validate_power_rating))
                                    />
                                </div>
                            </StaggeredContainer>
                        }.into_any(),
                        
                        3 => view! {
                            // Step 4: Biography
                            <StaggeredContainer stagger_delay=100 class="space-y-6".to_string()>
                                <ExecutiveTextArea
                                    label="Biography & Background".to_string()
                                    value=biography.into()
                                    set_value=set_biography
                                    placeholder="Tell the story of this wrestler's background, motivations, and character. What drives them? What's their journey been like? This helps create compelling storylines and character development.".to_string()
                                    rows=8
                                    max_length=Some(1000)
                                    helper=Some("Create a compelling backstory that can be used for storylines and character development".to_string())
                                />
                            </StaggeredContainer>
                        }.into_any(),
                        
                        _ => view! {}.into_any(),
                    }}
                    
                    // Custom form footer with step navigation
                    <div class="flex items-center justify-between pt-6 border-t border-base-300">
                        <div class="flex gap-2">
                            {move || if form_step.get() > 0 {
                                view! {
                                    <ExecutiveActionButton
                                        variant="ghost".to_string()
                                        size="md".to_string()
                                        icon=Some(view! {
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                                            </svg>
                                        }.into_any())
                                        on_click=Some(Box::new(prev_step))
                                        disabled=loading.get()
                                    >
                                        "Previous"
                                    </ExecutiveActionButton>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                        </div>
                        
                        <div class="flex gap-2">
                            {move || if form_step.get() < 3 {
                                view! {
                                    <ExecutiveActionButton
                                        variant="primary".to_string()
                                        size="md".to_string()
                                        icon_position="right".to_string()
                                        icon=Some(view! {
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                            </svg>
                                        }.into_any())
                                        on_click=Some(Box::new(next_step))
                                        disabled=loading.get()
                                    >
                                        "Next Step"
                                    </ExecutiveActionButton>
                                }.into_any()
                            } else {
                                view! {
                                    <ExecutiveActionButton
                                        variant="success".to_string()
                                        size="md".to_string()
                                        loading=loading.get()
                                        success=success.get()
                                        icon=Some(view! {
                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"/>
                                            </svg>
                                        }.into_any())
                                        on_click=Some(Box::new(move |e| {
                                            // Convert to submit event
                                            if let Some(form) = web_sys::window()
                                                .and_then(|w| w.document())
                                                .and_then(|d| d.query_selector("form").ok()?)
                                                .and_then(|f| f.dyn_into::<web_sys::HtmlFormElement>().ok())
                                            {
                                                let _ = form.request_submit();
                                            }
                                        }))
                                        disabled=name.get().trim().is_empty() || loading.get()
                                        ripple=true
                                    >
                                        if loading.get() {
                                            "Creating Wrestler..."
                                        } else if success.get() {
                                            "Wrestler Created!"
                                        } else {
                                            "Create Wrestler"
                                        }
                                    </ExecutiveActionButton>
                                }.into_any()
                            }}
                        </div>
                    </div>
                </ExecutiveForm>
            </ExecutiveContentSection>

            // Notification Container
            <ExecutiveNotificationContainer 
                manager=notification_manager
                position="top-right".to_string()
            />
        </ExecutivePageLayout>
    }
}