use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::types::Title;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
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
    
    // Title holder change form
    let (selected_wrestler_id, set_selected_wrestler_id) = signal(None::<i32>);
    let (event_name, set_event_name) = signal(String::new());
    let (event_location, set_event_location) = signal(String::new());
    let (change_method, set_change_method) = signal("won".to_string());
    let (updating, set_updating) = signal(false);
    let (update_success, set_update_success) = signal(None::<String>);
    
    // Delete functionality
    let (show_delete_confirmation, set_show_delete_confirmation) = signal(false);
    let (deleting, set_deleting) = signal(false);
    let (delete_error, set_delete_error) = signal(None::<String>);

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

    let get_prestige_info = move |tier: i32| {
        match tier {
            1 => ("World Championship", "text-warning", "border-warning", "bg-warning/20"),
            2 => ("Secondary Championship", "text-base-content/70", "border-base-300", "bg-base-200"),
            3 => ("Tag Team Championship", "text-accent", "border-accent", "bg-accent/20"),
            _ => ("Specialty Championship", "text-secondary", "border-secondary", "bg-secondary/20"),
        }
    };

    let handle_holder_change = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        if let (Some(title), Some(new_wrestler_id)) = (title_data.get(), selected_wrestler_id.get()) {
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
                
                // Validate tag team holder count
                let title_type = title.title.title_type.clone();
                let current_holder_count = title.current_holders.len();
                
                match title_type.as_str() {
                    "Singles" => {
                        if current_holder_count > 1 {
                            set_error.set(Some("Singles titles can only have one champion at a time".to_string()));
                            return;
                        }
                    },
                    "Tag Team" => {
                        // For now, we allow single holder changes for tag teams
                        // In the future, this should be enhanced to require both members
                        // TODO: Implement proper tag team holder assignment
                    },
                    "Triple Tag Team" => {
                        // Similar to tag team - future enhancement needed
                        // TODO: Implement proper triple tag team holder assignment
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
                
                let event_name_val = if event_name.get().trim().is_empty() { 
                    None 
                } else { 
                    Some(event_name.get().trim().to_string()) 
                };
                
                let event_location_val = if event_location.get().trim().is_empty() { 
                    None 
                } else { 
                    Some(event_location.get().trim().to_string()) 
                };
                
                match update_title_holder(
                    title.title.id,
                    new_wrestler_id,
                    event_name_val,
                    event_location_val,
                    Some(change_method.get()),
                ).await {
                    Ok(message) => {
                        set_update_success.set(Some(message));
                        // Reload title data to show the update
                        // This will trigger the Effect to reload
                        set_loading.set(true);
                        
                        // Reset form
                        set_selected_wrestler_id.set(None);
                        set_event_name.set(String::new());
                        set_event_location.set(String::new());
                        set_change_method.set("won".to_string());
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update title holder: {}", e)));
                    }
                }
                
                set_updating.set(false);
            });
        }
    };

    // Delete handlers
    let handle_delete_click = move |_| {
        set_show_delete_confirmation.set(true);
        set_delete_error.set(None);
    };

    let handle_confirm_delete = move |_| {
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

    let handle_cancel_delete = move |_| {
        set_show_delete_confirmation.set(false);
        set_delete_error.set(None);
    };

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
                    {move || {
                        let title_with_holders = title_data.get().unwrap();
                        let title = title_with_holders.title.clone();
                        let current_holders = title_with_holders.current_holders.clone();
                        let days_held = title_with_holders.days_held;
                        
                        let (prestige_name, prestige_color, border_color, bg_color) = get_prestige_info(title.prestige_tier);
                        
                        let holders_display = if current_holders.is_empty() {
                            "Vacant".to_string()
                        } else if current_holders.len() == 1 {
                            current_holders[0].wrestler_name.clone()
                        } else {
                            format!("{} & {}", 
                                current_holders[0].wrestler_name,
                                current_holders.get(1).map(|h| h.wrestler_name.as_str()).unwrap_or(""))
                        };
                        
                        let days_display = days_held
                            .map(|days| {
                                if days == 1 {
                                    "1 day".to_string()
                                } else {
                                    format!("{} days", days)
                                }
                            })
                            .unwrap_or_else(|| "No current holder".to_string());

                        view! {
                            <div class="space-y-8">
                                // Title Header
                                <div class={format!("card border {} {}", border_color, bg_color)}>
                                    <div class="card-body">
                                        <div class="flex items-start justify-between mb-6">
                                            <div>
                                                <h1 class="text-4xl font-bold text-base-content mb-2">
                                                    {title.name.clone()}
                                                </h1>
                                                <div class="flex items-center space-x-4 text-sm">
                                                    <span class={format!("font-semibold {}", prestige_color)}>
                                                        {prestige_name}
                                                    </span>
                                                    <span class="text-base-content/40">"•"</span>
                                                    <span class="text-base-content/80">
                                                        {title.division.clone()}
                                                    </span>
                                                    <span class="text-base-content/40">"•"</span>
                                                    <span class="text-base-content/80">
                                                        {title.title_type.clone()}
                                                    </span>
                                                    <span class="text-base-content/40">"•"</span>
                                                    <span class="text-base-content/80">
                                                        {title.gender.clone()}
                                                    </span>
                                                </div>
                                            </div>
                                            <div class="text-right space-y-2">
                                                <div>
                                                    <div class="text-2xl font-bold text-base-content">
                                                        "Tier " {title.prestige_tier}
                                                    </div>
                                                    <div class="text-sm text-base-content/60">
                                                        "Prestige Level"
                                                    </div>
                                                </div>
                                                
                                                // Delete button (only for user-created titles)
                                                <Show when=move || title.is_user_created.unwrap_or(false)>
                                                    <div class="flex flex-col gap-2">
                                                        <Show when=move || delete_error.get().is_some()>
                                                            <div class="alert alert-error">
                                                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-5 w-5" fill="none" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                                </svg>
                                                                <span class="text-xs">{move || delete_error.get().unwrap_or_default()}</span>
                                                            </div>
                                                        </Show>
                                                        
                                                        <Show when=move || !show_delete_confirmation.get()>
                                                            <button
                                                                class="btn btn-error btn-sm gap-1"
                                                                disabled=move || deleting.get()
                                                                on:click=handle_delete_click
                                                            >
                                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                                                </svg>
                                                                "Delete Title"
                                                            </button>
                                                        </Show>

                                                        <Show when=move || show_delete_confirmation.get()>
                                                            <div class="space-y-2">
                                                                <div class="bg-error/20 border border-error/30 rounded-lg p-3">
                                                                    <div class="flex items-center gap-2 mb-2">
                                                                        <svg class="w-4 h-4 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                                                                        </svg>
                                                                        <h5 class="text-error font-bold text-sm">"Confirm Deletion"</h5>
                                                                    </div>
                                                                    <p class="text-error/90 text-xs mb-2">
                                                                        "Are you sure you want to delete "
                                                                        <strong>{move || title_data.get().map(|t| t.title.name.clone()).unwrap_or_default()}</strong>
                                                                        "? This action cannot be undone."
                                                                    </p>
                                                                    <p class="text-error/70 text-xs">
                                                                        "This will also remove all title history and holder records."
                                                                    </p>
                                                                </div>
                                                                <div class="flex gap-1">
                                                                    <button
                                                                        class="btn btn-error btn-xs flex-1 gap-1"
                                                                        disabled=move || deleting.get()
                                                                        on:click=handle_confirm_delete
                                                                    >
                                                                        <Show when=move || deleting.get()>
                                                                            <span class="loading loading-spinner loading-xs"></span>
                                                                        </Show>
                                                                        <Show when=move || !deleting.get()>
                                                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                                                            </svg>
                                                                        </Show>
                                                                        {move || if deleting.get() { "Deleting..." } else { "Delete Forever" }}
                                                                    </button>
                                                                    <button
                                                                        class="btn btn-ghost btn-xs flex-1"
                                                                        disabled=move || deleting.get()
                                                                        on:click=handle_cancel_delete
                                                                    >
                                                                        "Cancel"
                                                                    </button>
                                                                </div>
                                                            </div>
                                                        </Show>
                                                    </div>
                                                </Show>
                                            </div>
                                        </div>

                                        // Current Champion Info
                                        <div class="card bg-base-200">
                                            <div class="card-body">
                                                <h3 class="card-title text-xl mb-4">
                                                    "Current Champion"
                                                </h3>
                                                <div class="flex items-center justify-between">
                                                    <div>
                                                        <div class="text-2xl font-bold text-base-content mb-1">
                                                            {holders_display}
                                                        </div>
                                                        <div class="text-base-content/60">
                                                            {if current_holders.is_empty() {
                                                                "Title is currently vacant".to_string()
                                                            } else {
                                                                format!("Held for {}", days_display)
                                                            }}
                                                        </div>
                                                    </div>
                                                    <Show when=move || !current_holders.is_empty()>
                                                        <div class="text-right">
                                                            <div class="text-lg font-semibold text-accent">
                                                                {days_display.clone()}
                                                            </div>
                                                            <div class="text-sm text-base-content/60">
                                                                "Championship Reign"
                                                            </div>
                                                        </div>
                                                    </Show>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Championship History Placeholder
                                <div class="card bg-base-200 border border-base-300">
                                    <div class="card-body">
                                        <h3 class="card-title text-xl mb-4">
                                            "Championship History"
                                        </h3>
                                        <div class="text-center py-8">
                                            <div class="text-base-content/60 mb-2">
                                                "Championship history tracking coming soon"
                                            </div>
                                            <div class="text-sm text-base-content/50">
                                                "This will show all previous champions and reign details"
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Change Champion Form
                                <div class="card bg-base-200 border border-base-300">
                                    <div class="card-body">
                                        <h3 class="card-title text-xl mb-6">
                                            "Change Champion"
                                        </h3>
                                    
                                        <Show when=move || update_success.get().is_some()>
                                            <div class="alert alert-success mb-6">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                                                <span>{move || update_success.get().unwrap_or_default()}</span>
                                            </div>
                                        </Show>

                                    <form on:submit=handle_holder_change>
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                            <div class="form-control">
                                                <label class="label">
                                                    <span class="label-text">"New Champion" <span class="text-error">"*"</span></span>
                                                </label>
                                                <select
                                                    class="select select-bordered w-full"
                                                    on:change=move |ev| {
                                                        let value = event_target_value(&ev);
                                                        if value.is_empty() || value == "none" {
                                                            set_selected_wrestler_id.set(None);
                                                        } else if let Ok(id) = value.parse::<i32>() {
                                                            set_selected_wrestler_id.set(Some(id));
                                                        }
                                                    }
                                                >
                                                    <option value="none">"Select a wrestler..."</option>
                                                    <For
                                                        each=move || filtered_wrestlers.get()
                                                        key=|wrestler| wrestler.id
                                                        children=move |wrestler| {
                                                            let wrestler_id_val = wrestler.id;
                                                            view! {
                                                                <option value={wrestler_id_val.to_string()}>
                                                                    {wrestler.name.clone()}
                                                                </option>
                                                            }
                                                        }
                                                    />
                                                </select>
                                                <div class="label">
                                                    <span class="label-text-alt text-base-content/60">
                                                        {format!("Showing {} division wrestlers", title.gender)}
                                                    </span>
                                                </div>
                                                <div class="label">
                                                    <span class="label-text-alt text-warning">"TODO: Filter by selected show in the future"</span>
                                                </div>
                                            </div>

                                            <div class="form-control">
                                                <label class="label">
                                                    <span class="label-text">"Change Method"</span>
                                                </label>
                                                <select
                                                    class="select select-bordered w-full"
                                                    prop:value=change_method
                                                    on:change=move |ev| set_change_method.set(event_target_value(&ev))
                                                >
                                                    <option value="won">"Won"</option>
                                                    <option value="awarded">"Awarded"</option>
                                                    <option value="stripped">"Previous champion stripped"</option>
                                                    <option value="vacated">"Previous champion vacated"</option>
                                                </select>
                                            </div>

                                            <div class="form-control">
                                                <label class="label">
                                                    <span class="label-text">"Event/Show Name"</span>
                                                </label>
                                                <input
                                                    type="text"
                                                    class="input input-bordered w-full"
                                                    placeholder="e.g., WrestleMania, Monday Night RAW"
                                                    prop:value=event_name
                                                    on:input=move |ev| set_event_name.set(event_target_value(&ev))
                                                />
                                            </div>

                                            <div class="form-control">
                                                <label class="label">
                                                    <span class="label-text">"Event Location"</span>
                                                </label>
                                                <input
                                                    type="text"
                                                    class="input input-bordered w-full"
                                                    placeholder="e.g., Madison Square Garden"
                                                    prop:value=event_location
                                                    on:input=move |ev| set_event_location.set(event_target_value(&ev))
                                                />
                                            </div>
                                        </div>

                                        <div class="card-actions justify-end mt-6">
                                            <button
                                                type="submit"
                                                class="btn btn-accent gap-2"
                                                disabled=move || updating.get() || selected_wrestler_id.get().is_none()
                                            >
                                                <Show when=move || updating.get()>
                                                    <span class="loading loading-spinner loading-sm"></span>
                                                </Show>
                                                {move || if updating.get() { "Updating..." } else { "Change Champion" }}
                                            </button>
                                        </div>
                                    </form>
                                    </div>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </Show>
        </div>
    }
}