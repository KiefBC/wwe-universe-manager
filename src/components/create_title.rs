use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct TitleData {
    pub name: String,
    pub title_type: String,
    pub division: String,
    pub gender: String,
    pub show_id: Option<i32>,
    pub current_holder_id: Option<i32>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_shows() -> Result<Vec<Show>, String> {
    let result = invoke("get_shows", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn get_wrestlers() -> Result<Vec<Wrestler>, String> {
    let result = invoke("get_wrestlers", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn create_title(title_data: TitleData) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "titleData": title_data
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("create_belt", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

#[component]
pub fn CreateTitle(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (title_type, set_title_type) = signal("Singles".to_string());
    let (division, set_division) = signal("World".to_string());
    let (gender, set_gender) = signal("Male".to_string());
    let (show_id, set_show_id) = signal(None::<i32>);
    let (current_holder_id, set_current_holder_id) = signal(None::<i32>);
    
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);

    // Load shows and wrestlers
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (filtered_wrestlers, set_filtered_wrestlers) = signal(Vec::<Wrestler>::new());

    // Load data on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            // Load shows
            if let Ok(shows_data) = get_shows().await {
                set_shows.set(shows_data);
            }
            
            // Load wrestlers
            if let Ok(wrestlers_data) = get_wrestlers().await {
                set_wrestlers.set(wrestlers_data);
            }
        });
    });

    // Filter wrestlers based on selected gender
    Effect::new(move |_| {
        let all_wrestlers = wrestlers.get();
        let selected_gender = gender.get();
        
        let filtered = match selected_gender.as_str() {
            "Mixed" => all_wrestlers, // Show all wrestlers for mixed titles
            _ => all_wrestlers.into_iter()
                .filter(|w| w.gender == selected_gender)
                .collect()
        };
        
        set_filtered_wrestlers.set(filtered);
        
        // Reset current holder if it's no longer valid
        if let Some(holder_id) = current_holder_id.get() {
            let still_valid = filtered_wrestlers.get().iter()
                .any(|w| w.id == holder_id);
            if !still_valid {
                set_current_holder_id.set(None);
            }
        }
    });

    // Auto-calculate prestige tier based on division
    let prestige_tier = move || {
        match division.get().as_str() {
            "World" | "WWE Championship" | "Women's World" | "WWE Women's Championship" => 1,
            "Intercontinental" | "United States" | "Women's Intercontinental" | "Women's United States" => 2,
            "World Tag Team" | "WWE Tag Team" | "Women's Tag Team" => 3,
            _ => 4, // Specialty titles
        }
    };

    let submit_title = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        if name.get().trim().is_empty() {
            set_error.set(Some("Title name is required".to_string()));
            return;
        }

        // Validate tag team holder count
        let title_type_val = title_type.get();
        let expected_holders = match title_type_val.as_str() {
            "Singles" => 1,
            "Tag Team" => 2,
            "Triple Tag Team" => 3,
            _ => 1,
        };

        // For now, we only validate that Singles titles don't conflict with tag team logic
        // Future enhancement: Validate exact holder count for tag teams
        if title_type_val == "Tag Team" || title_type_val == "Triple Tag Team" {
            if current_holder_id.get().is_some() {
                set_error.set(Some(format!(
                    "{} titles require {} holders. Individual holder assignment not supported yet - please leave vacant and assign holders after creation.",
                    title_type_val, expected_holders
                )));
                return;
            }
        }

        // Validate gender matching for initial holder
        if let Some(holder_id) = current_holder_id.get() {
            let selected_gender = gender.get();
            let holder_valid = filtered_wrestlers.get().iter()
                .find(|w| w.id == holder_id)
                .map(|w| selected_gender == "Mixed" || w.gender == selected_gender)
                .unwrap_or(false);
                
            if !holder_valid {
                set_error.set(Some(format!(
                    "Selected wrestler cannot hold this {} division title",
                    selected_gender
                )));
                return;
            }
        }

        let title_data = TitleData {
            name: name.get().trim().to_string(),
            title_type: title_type.get(),
            division: division.get(),
            gender: gender.get(),
            show_id: show_id.get(),
            current_holder_id: current_holder_id.get(),
        };

        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_title(title_data).await {
                Ok(_) => {
                    set_success.set(true);
                    // Navigate back to titles list after a brief delay
                    set_timeout(
                        move || {
                            set_current_page.set("titles".to_string());
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
                        on:click=move |_| set_current_page.set("titles".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Titles"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "Create New Championship"
                </h1>
                <p class="text-base-content/70">
                    "Add a new championship title to your universe"
                </p>
            </div>

            <Show when=move || success.get()>
                <div class="alert alert-success mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Title Created Successfully!"</h3>
                        <div class="text-xs">"Redirecting to titles list..."</div>
                    </div>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error Creating Title"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <div class="card bg-base-200 border border-base-300">
                <div class="card-body">
                <form on:submit=submit_title>
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        // Basic Information
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Basic Information"</h3>
                            
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Title Name" <span class="text-error">"*"</span></span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., WWE Championship"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Title Type" <span class="text-error">"*"</span></span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    prop:value=title_type
                                    on:change=move |ev| set_title_type.set(event_target_value(&ev))
                                >
                                    <option value="Singles">"Singles"</option>
                                    <option value="Tag Team">"Tag Team"</option>
                                    <option value="Triple Tag Team">"Triple Tag Team"</option>
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Division" <span class="text-error">"*"</span></span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    prop:value=division
                                    on:change=move |ev| set_division.set(event_target_value(&ev))
                                >
                                    // Tier 1 - World Championships
                                    <optgroup label="World Championships (Tier 1)">
                                        <option value="World">"World Heavyweight"</option>
                                        <option value="WWE Championship">"WWE Championship"</option>  
                                        <option value="Women's World">"Women's World"</option>
                                        <option value="WWE Women's Championship">"WWE Women's Championship"</option>
                                    </optgroup>
                                    
                                    // Tier 2 - Secondary
                                    <optgroup label="Secondary Championships (Tier 2)">
                                        <option value="Intercontinental">"Intercontinental"</option>
                                        <option value="United States">"United States"</option>
                                        <option value="Women's Intercontinental">"Women's Intercontinental"</option>
                                        <option value="Women's United States">"Women's United States"</option>
                                    </optgroup>

                                    // Tier 3 - Tag Team
                                    <optgroup label="Tag Team Championships (Tier 3)">
                                        <option value="World Tag Team">"World Tag Team"</option>
                                        <option value="WWE Tag Team">"WWE Tag Team"</option>
                                        <option value="Women's Tag Team">"Women's Tag Team"</option>
                                    </optgroup>

                                    // Tier 4 - Specialty
                                    <optgroup label="Specialty Championships (Tier 4)">
                                        <option value="Money in the Bank">"Money in the Bank"</option>
                                        <option value="Hardcore">"Hardcore"</option>
                                        <option value="Speed">"Speed"</option>
                                        <option value="24/7">"24/7"</option>
                                        <option value="Cruiserweight">"Cruiserweight"</option>
                                    </optgroup>
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Prestige Tier (Auto-calculated)"</span>
                                </label>
                                <div class="bg-base-200 border border-base-300 rounded-lg px-4 py-3 text-base-content">
                                    {move || {
                                        let tier = prestige_tier();
                                        let (tier_name, tier_color) = match tier {
                                            1 => ("Tier 1 - World Championship", "text-warning"),
                                            2 => ("Tier 2 - Secondary Championship", "text-base-content"),
                                            3 => ("Tier 3 - Tag Team Championship", "text-accent"),
                                            _ => ("Tier 4 - Specialty Championship", "text-secondary"),
                                        };
                                        view! {
                                            <span class={tier_color}>{tier_name}</span>
                                        }
                                    }}
                                </div>
                            </div>
                        </div>

                        // Championship Settings
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Championship Settings"</h3>
                            
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Gender Division" <span class="text-error">"*"</span></span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    prop:value=gender
                                    on:change=move |ev| set_gender.set(event_target_value(&ev))
                                >
                                    <option value="Male">"Male"</option>
                                    <option value="Female">"Female"</option>
                                    <option value="Mixed">"Mixed (Any Gender)"</option>
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Assigned Show"</span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        if value.is_empty() || value == "none" {
                                            set_show_id.set(None);
                                        } else if let Ok(id) = value.parse::<i32>() {
                                            set_show_id.set(Some(id));
                                        }
                                    }
                                >
                                    <option value="none">"Cross-Brand (No specific show)"</option>
                                    <For
                                        each=move || shows.get()
                                        key=|show| show.id
                                        children=move |show| {
                                            let show_id_val = show.id;
                                            view! {
                                                <option value={show_id_val.to_string()}>
                                                    {show.name.clone()}
                                                </option>
                                            }
                                        }
                                    />
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Initial Champion (Optional)"</span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    on:change=move |ev| {
                                        let value = event_target_value(&ev);
                                        if value.is_empty() || value == "none" {
                                            set_current_holder_id.set(None);
                                        } else if let Ok(id) = value.parse::<i32>() {
                                            set_current_holder_id.set(Some(id));
                                        }
                                    }
                                >
                                    <option value="none">"Vacant Title"</option>
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
                                    <span class="label-text-alt">
                                        {move || {
                                            match gender.get().as_str() {
                                                "Mixed" => "Showing all wrestlers (Mixed division)".to_string(),
                                                gender => format!("Showing {} wrestlers only", gender)
                                            }
                                        }}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Submit Button
                    <div class="mt-8 flex justify-end space-x-4">
                        <button
                            type="button"
                            class="btn btn-ghost"
                            on:click=move |_| set_current_page.set("titles".to_string())
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="btn btn-primary"
                            disabled=move || loading.get()
                        >
                            <Show when=move || loading.get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            <span>{move || if loading.get() { "Creating..." } else { "Create Championship" }}</span>
                        </button>
                    </div>
                </form>
                </div>
            </div>
        </div>
    }
}