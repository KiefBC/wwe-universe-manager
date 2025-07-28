use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

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
            1 => ("World Championship", "text-yellow-400", "border-yellow-500", "bg-yellow-600/20"),
            2 => ("Secondary Championship", "text-slate-300", "border-slate-400", "bg-slate-600/20"),
            3 => ("Tag Team Championship", "text-orange-400", "border-orange-500", "bg-orange-600/20"),
            _ => ("Specialty Championship", "text-purple-400", "border-purple-500", "bg-purple-600/20"),
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

    view! {
        <div class="container mx-auto p-6 bg-slate-900 min-h-screen">
            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-cyan-600"></div>
                    <span class="ml-3 text-slate-400">"Loading title details..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="bg-red-900/50 border border-red-600 rounded-lg p-6 text-center">
                    <h3 class="text-red-400 text-lg font-semibold mb-2">"Error"</h3>
                    <p class="text-red-300">{move || error.get().unwrap_or_default()}</p>
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
                                <div class={format!("border-2 {} {} rounded-xl p-8", border_color, bg_color)}>
                                    <div class="flex items-start justify-between mb-6">
                                        <div>
                                            <h1 class="text-4xl font-bold text-white mb-2">
                                                {title.name.clone()}
                                            </h1>
                                            <div class="flex items-center space-x-4 text-sm">
                                                <span class={format!("font-semibold {}", prestige_color)}>
                                                    {prestige_name}
                                                </span>
                                                <span class="text-slate-400">"•"</span>
                                                <span class="text-slate-300">
                                                    {title.division.clone()}
                                                </span>
                                                <span class="text-slate-400">"•"</span>
                                                <span class="text-slate-300">
                                                    {title.title_type.clone()}
                                                </span>
                                                <span class="text-slate-400">"•"</span>
                                                <span class="text-slate-300">
                                                    {title.gender.clone()}
                                                </span>
                                            </div>
                                        </div>
                                        <div class="text-right">
                                            <div class="text-2xl font-bold text-slate-100">
                                                "Tier " {title.prestige_tier}
                                            </div>
                                            <div class="text-sm text-slate-400">
                                                "Prestige Level"
                                            </div>
                                        </div>
                                    </div>

                                    // Current Champion Info
                                    <div class="bg-slate-800/50 rounded-lg p-6">
                                        <h3 class="text-xl font-semibold text-white mb-4">
                                            "Current Champion"
                                        </h3>
                                        <div class="flex items-center justify-between">
                                            <div>
                                                <div class="text-2xl font-bold text-white mb-1">
                                                    {holders_display}
                                                </div>
                                                <div class="text-slate-400">
                                                    {if current_holders.is_empty() {
                                                        "Title is currently vacant".to_string()
                                                    } else {
                                                        format!("Held for {}", days_display)
                                                    }}
                                                </div>
                                            </div>
                                            <Show when=move || !current_holders.is_empty()>
                                                <div class="text-right">
                                                    <div class="text-lg font-semibold text-cyan-400">
                                                        {days_display.clone()}
                                                    </div>
                                                    <div class="text-sm text-slate-400">
                                                        "Championship Reign"
                                                    </div>
                                                </div>
                                            </Show>
                                        </div>
                                    </div>
                                </div>

                                // Championship History Placeholder
                                <div class="bg-slate-800 border border-slate-600 rounded-lg p-8">
                                    <h3 class="text-xl font-semibold text-white mb-4">
                                        "Championship History"
                                    </h3>
                                    <div class="text-center py-8">
                                        <div class="text-slate-400 mb-2">
                                            "Championship history tracking coming soon"
                                        </div>
                                        <div class="text-sm text-slate-500">
                                            "This will show all previous champions and reign details"
                                        </div>
                                    </div>
                                </div>

                                // Change Champion Form
                                <div class="bg-slate-800 border border-slate-600 rounded-lg p-8">
                                    <h3 class="text-xl font-semibold text-white mb-6">
                                        "Change Champion"
                                    </h3>
                                    
                                    <Show when=move || update_success.get().is_some()>
                                        <div class="bg-green-900/50 border border-green-600 rounded-lg p-4 mb-6">
                                            <p class="text-green-300">
                                                {move || update_success.get().unwrap_or_default()}
                                            </p>
                                        </div>
                                    </Show>

                                    <form on:submit=handle_holder_change>
                                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                            <div>
                                                <label class="block text-sm font-medium text-slate-300 mb-2">
                                                    "New Champion" <span class="text-red-400">"*"</span>
                                                </label>
                                                <select
                                                    class="w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white focus:ring-2 focus:ring-cyan-500 focus:border-transparent"
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
                                                <p class="text-xs text-slate-500 mt-1">
                                                    {format!("Showing {} division wrestlers", title.gender)}
                                                    <br/>
                                                    <span class="text-yellow-500">"TODO: Filter by selected show in the future"</span>
                                                </p>
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-slate-300 mb-2">
                                                    "Change Method"
                                                </label>
                                                <select
                                                    class="w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white focus:ring-2 focus:ring-cyan-500 focus:border-transparent"
                                                    prop:value=change_method
                                                    on:change=move |ev| set_change_method.set(event_target_value(&ev))
                                                >
                                                    <option value="won">"Won"</option>
                                                    <option value="awarded">"Awarded"</option>
                                                    <option value="stripped">"Previous champion stripped"</option>
                                                    <option value="vacated">"Previous champion vacated"</option>
                                                </select>
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-slate-300 mb-2">
                                                    "Event/Show Name"
                                                </label>
                                                <input
                                                    type="text"
                                                    class="w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:ring-2 focus:ring-cyan-500 focus:border-transparent"
                                                    placeholder="e.g., WrestleMania, Monday Night RAW"
                                                    prop:value=event_name
                                                    on:input=move |ev| set_event_name.set(event_target_value(&ev))
                                                />
                                            </div>

                                            <div>
                                                <label class="block text-sm font-medium text-slate-300 mb-2">
                                                    "Event Location"
                                                </label>
                                                <input
                                                    type="text"
                                                    class="w-full px-4 py-3 bg-slate-700 border border-slate-600 rounded-lg text-white placeholder-slate-400 focus:ring-2 focus:ring-cyan-500 focus:border-transparent"
                                                    placeholder="e.g., Madison Square Garden"
                                                    prop:value=event_location
                                                    on:input=move |ev| set_event_location.set(event_target_value(&ev))
                                                />
                                            </div>
                                        </div>

                                        <div class="mt-6 flex justify-end">
                                            <button
                                                type="submit"
                                                class="px-6 py-3 bg-cyan-600 hover:bg-cyan-700 border border-cyan-500 text-white rounded-lg font-semibold transition-colors flex items-center space-x-2"
                                                disabled=move || updating.get() || selected_wrestler_id.get().is_none()
                                            >
                                                <Show when=move || updating.get()>
                                                    <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                                                </Show>
                                                <span>{move || if updating.get() { "Updating..." } else { "Change Champion" }}</span>
                                            </button>
                                        </div>
                                    </form>
                                </div>
                            </div>
                        }
                    }}
                </Show>
            </Show>
        </div>
    }
}