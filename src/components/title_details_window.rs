use crate::components::title::champion_info_section::ChampionInfoSection;
use crate::components::title::championship_history_section::ChampionshipHistorySection;
use crate::components::title::change_champion_form::{ChangeChampionForm, ChampionChangeFormState};
use crate::components::title::title_header_section::TitleHeaderSection;
use crate::components::title::title_management_actions::TitleManagementActions;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::types::{Title, Wrestler};
use crate::constants::ui_constants::prestige_tiers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleHolderInfo {
    pub wrestler_name: String,
    pub wrestler_gender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithHolders {
    pub title: Title,
    pub current_holders: Vec<TitleHolderInfo>,
    pub days_held: Option<i32>,
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_titles() -> Result<Vec<TitleWithHolders>, String> {
    let result = invoke("get_titles", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn get_wrestlers() -> Result<Vec<Wrestler>, String> {
    let result = invoke("get_wrestlers", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_title_holder(
    title_id: i32,
    new_wrestler_id: i32,
    event_name: Option<String>,
    event_location: Option<String>,
    change_method: Option<String>,
) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "titleId": title_id,
        "newWrestlerId": new_wrestler_id,
        "eventName": event_name,
        "eventLocation": event_location,
        "changeMethod": change_method
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_title_holder", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn delete_title(title_id: i32) -> Result<String, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "titleId": title_id
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("delete_title", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

#[component]
pub fn TitleDetailsWindow() -> impl IntoView {
    let (title_data, set_title_data) = signal(None::<TitleWithHolders>);
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (filtered_wrestlers, set_filtered_wrestlers) = signal(Vec::<Wrestler>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    
    // Title holder change form state  
    let form_state = RwSignal::new(ChampionChangeFormState::default());
    let (updating, set_updating) = signal(false);
    let (update_success, set_update_success) = signal(None::<String>);
    
    // Delete functionality
    let (show_delete_confirmation, set_show_delete_confirmation) = signal(false);
    let (deleting, set_deleting) = signal(false);
    let (delete_error, set_delete_error) = signal(None::<String>);
    
    // Communication signals for sub-components
    let (delete_request_trigger, set_delete_request_trigger) = signal(false);
    let (confirm_delete_trigger, set_confirm_delete_trigger) = signal(false);
    let (cancel_delete_trigger, set_cancel_delete_trigger) = signal(false);

    // Parse title ID from URL hash
    let title_id = move || {
        window()
            .and_then(|w| w.location().hash().ok())
            .and_then(|hash| {
                // Expected format: #title?id=123
                if hash.starts_with("#title?id=") {
                    hash.strip_prefix("#title?id=")
                        .and_then(|id_str| id_str.parse::<i32>().ok())
                } else {
                    Some(1) // Default title ID for testing
                }
            })
            .unwrap_or(1)
    };

    // Load title and wrestler data
    Effect::new(move |_| {
        let target_id = title_id();
        
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            // Load titles and find the target title
            match get_titles().await {
                Ok(titles) => {
                    if let Some(title) = titles.into_iter().find(|t| t.title.id == target_id) {
                        set_title_data.set(Some(title));
                    } else {
                        set_error.set(Some(format!("Title with ID {} not found", target_id)));
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load title: {}", e)));
                }
            }

            // Load wrestlers
            match get_wrestlers().await {
                Ok(wrestlers_data) => {
                    set_wrestlers.set(wrestlers_data);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load wrestlers: {}", e)));
                }
            }

            set_loading.set(false);
        });
    });

    // Filter wrestlers based on title gender division
    Effect::new(move |_| {
        if let Some(title) = title_data.get() {
            let all_wrestlers = wrestlers.get();
            let title_gender = title.title.gender;
            
            let filtered = match title_gender.as_str() {
                "Mixed" => all_wrestlers, // Show all wrestlers for mixed titles
                _ => all_wrestlers.into_iter()
                    .filter(|w| w.gender == title_gender)
                    .collect()
            };
            
            set_filtered_wrestlers.set(filtered);
        }
    });

    let _get_prestige_info = move |tier: i32| {
        let info = prestige_tiers::get_prestige_info(tier);
        (info.name, info.text_color, info.border_color, info.background_color)
    };


    // Delete handlers
    let handle_delete_click = move || {
        set_show_delete_confirmation.set(true);
        set_delete_error.set(None);
    };

    let handle_confirm_delete = move || {
        if let Some(title) = title_data.get() {
            spawn_local(async move {
                set_deleting.set(true);
                set_delete_error.set(None);
                
                match delete_title(title.title.id).await {
                    Ok(_) => {
                        // Close the window after successful deletion
                        if let Some(window) = web_sys::window() {
                            let _ = window.close();
                        }
                    }
                    Err(e) => {
                        set_delete_error.set(Some(format!("Failed to delete title: {}", e)));
                    }
                }
                
                set_deleting.set(false);
            });
        }
    };

    let handle_cancel_delete = move || {
        set_show_delete_confirmation.set(false);
        set_delete_error.set(None);
    };

    // Handle component communication triggers
    Effect::new(move |_| {
        if delete_request_trigger.get() {
            set_show_delete_confirmation.set(true);
            set_delete_error.set(None);
            set_delete_request_trigger.set(false);
        }
    });

    Effect::new(move |_| {
        if confirm_delete_trigger.get() {
            handle_confirm_delete();
            set_confirm_delete_trigger.set(false);
        }
    });

    Effect::new(move |_| {
        if cancel_delete_trigger.get() {
            handle_cancel_delete();
            set_cancel_delete_trigger.set(false);
        }
    });

    // Form submission handler using callback approach
    let handle_form_submit = move |form_data: ChampionChangeFormState| {
        if let (Some(title), Some(new_wrestler_id)) = (title_data.get(), form_data.selected_wrestler_id) {
            // Validate gender matching
            let selected_wrestler = filtered_wrestlers.get().iter()
                .find(|w| w.id == new_wrestler_id)
                .cloned();
                
            if let Some(wrestler) = selected_wrestler {
                let title_gender = title.title.gender.clone();
                if title_gender != "Mixed" && wrestler.gender != title_gender {
                    set_error.set(Some(format!(
                        "Cannot assign {} wrestler to {} division title",
                        wrestler.gender, title_gender
                    )));
                    return;
                }
                
                let title_type = title.title.title_type.clone();
                let current_holder_count = title.current_holders.len();
                
                match title_type.as_str() {
                    "Singles" => {
                        if current_holder_count > 1 {
                            set_error.set(Some("Singles titles can only have one champion at a time".to_string()));
                            return;
                        }
                    },
                    "Tag Team" | "Triple Tag Team" => {
                        // Allow for now - TODO: Implement proper tag team assignment
                    },
                    _ => {}
                }
            } else {
                set_error.set(Some("Selected wrestler not found".to_string()));
                return;
            }
            
            spawn_local(async move {
                set_updating.set(true);
                set_update_success.set(None);
                
                let event_name_val = if form_data.event_name.trim().is_empty() { 
                    None 
                } else { 
                    Some(form_data.event_name.trim().to_string()) 
                };
                
                let event_location_val = if form_data.event_location.trim().is_empty() { 
                    None 
                } else { 
                    Some(form_data.event_location.trim().to_string()) 
                };
                
                match update_title_holder(
                    title.title.id,
                    new_wrestler_id,
                    event_name_val,
                    event_location_val,
                    Some(form_data.change_method),
                ).await {
                    Ok(message) => {
                        set_update_success.set(Some(message));
                        set_loading.set(true);
                        
                        // Reset form
                        form_state.set(ChampionChangeFormState::default());
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update title holder: {}", e)));
                    }
                }
                
                set_updating.set(false);
            });
        }
    };

    // Derived signals for component data - using Signal::derive
    let title_signal = Signal::derive(move || {
        title_data.get().map(|t| t.title.clone()).unwrap_or_else(|| Title {
            id: 0,
            name: String::new(),
            current_holder_id: None,
            title_type: String::new(),
            division: String::new(),
            prestige_tier: 1,
            gender: String::new(),
            show_id: None,
            is_active: true,
            is_user_created: Some(false),
        })
    });

    let holders_display = Signal::derive(move || {
        title_data.get().map(|title_with_holders| {
            let current_holders = title_with_holders.current_holders.clone();
            if current_holders.is_empty() {
                "Vacant".to_string()
            } else if current_holders.len() == 1 {
                current_holders[0].wrestler_name.clone()
            } else {
                format!("{} & {}", 
                    current_holders[0].wrestler_name,
                    current_holders.get(1).map(|h| h.wrestler_name.as_str()).unwrap_or(""))
            }
        }).unwrap_or_default()
    });

    let days_display = Signal::derive(move || {
        title_data.get().and_then(|title_with_holders| {
            title_with_holders.days_held.map(|days| {
                if days == 1 {
                    "1 day".to_string()
                } else {
                    format!("{} days", days)
                }
            })
        }).unwrap_or_else(|| "No current holder".to_string())
    });

    let has_current_holders = Signal::derive(move || {
        title_data.get().map(|title_with_holders| {
            !title_with_holders.current_holders.is_empty()
        }).unwrap_or(false)
    });

    let title_name_for_delete = Signal::derive(move || {
        title_data.get().map(|t| t.title.name.clone()).unwrap_or_default()
    });
    
    let show_delete_button = Signal::derive(move || true);

    view! {
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <span class="loading loading-spinner loading-lg text-accent"></span>
                    <span class="ml-3 text-base-content/70">"Loading title details..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none()>
                <Show when=move || title_data.get().is_some()>
                    <div class="space-y-8">
                        // Title Header with delete functionality
                        <TitleHeaderSection 
                            title=title_signal
                            show_delete_button=show_delete_button
                            on_delete_request=set_delete_request_trigger
                            delete_error=delete_error.into()
                            deleting=deleting.into()
                        />
                        <TitleManagementActions 
                            title_name=title_name_for_delete
                            show_delete_confirmation=show_delete_confirmation.into()
                            _set_show_delete_confirmation=set_show_delete_confirmation
                            deleting=deleting.into()
                            delete_error=delete_error.into()
                            on_confirm_delete=set_confirm_delete_trigger
                            on_cancel_delete=set_cancel_delete_trigger
                        />
                        
                        // Champion Info Section  
                        <ChampionInfoSection 
                            holders_display=holders_display
                            days_display=days_display
                            has_current_holders=has_current_holders
                        />
                        
                        // Championship History Section
                        <ChampionshipHistorySection />
                        
                        // Change Champion Form
                        <ChangeChampionForm 
                            title=title_signal
                            filtered_wrestlers=filtered_wrestlers.into()
                            form_state=form_state
                            updating=updating.into()
                            update_success=update_success.into()
                            on_submit=handle_form_submit
                        />
                    </div>
                </Show>
            </Show>
        </div>
    }
}
