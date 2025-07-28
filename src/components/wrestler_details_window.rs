use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::types::{Show, fetch_shows};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wrestler {
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_wrestler_by_id(wrestler_id: i32) -> Result<Option<Wrestler>, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("get_wrestler_by_id", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_promotion(wrestler_id: i32, promotion: String) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "promotion": promotion
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_promotion", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_power_ratings(
    wrestler_id: i32,
    strength: Option<i32>,
    speed: Option<i32>,
    agility: Option<i32>,
    stamina: Option<i32>,
    charisma: Option<i32>,
    technique: Option<i32>,
) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "strength": strength,
        "speed": speed,
        "agility": agility,
        "stamina": stamina,
        "charisma": charisma,
        "technique": technique
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_power_ratings", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_basic_stats(
    wrestler_id: i32,
    height: Option<String>,
    weight: Option<String>,
    debut_year: Option<i32>,
    wins: i32,
    losses: i32,
) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "height": height,
        "weight": weight,
        "debutYear": debut_year,
        "wins": wins,
        "losses": losses
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_basic_stats", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_name(
    wrestler_id: i32,
    name: String,
    nickname: Option<String>,
) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "name": name,
        "nickname": nickname
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_name", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_real_name(
    wrestler_id: i32,
    real_name: Option<String>,
) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "realName": real_name
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_real_name", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn update_wrestler_biography(
    wrestler_id: i32,
    biography: Option<String>,
) -> Result<Wrestler, String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id,
        "biography": biography
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("update_wrestler_biography", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}


fn extract_wrestler_id_from_url() -> Option<i32> {
    web_sys::window()?
        .location()
        .hash()
        .ok()?
        .strip_prefix("#wrestler?id=")?
        .parse()
        .ok()
}

#[component]
pub fn WrestlerDetailsWindow() -> impl IntoView {
    let (wrestler, set_wrestler) = signal(None::<Wrestler>);
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (editing_power_ratings, set_editing_power_ratings) = signal(false);
    
    // Signal to track current wrestler ID from URL
    let (current_wrestler_id, set_current_wrestler_id) = signal(extract_wrestler_id_from_url());
    
    // Temporary state for editing power ratings
    let (temp_strength, set_temp_strength) = signal(0i32);
    let (temp_speed, set_temp_speed) = signal(0i32);
    let (temp_agility, set_temp_agility) = signal(0i32);
    let (temp_stamina, set_temp_stamina) = signal(0i32);
    let (temp_charisma, set_temp_charisma) = signal(0i32);
    let (temp_technique, set_temp_technique) = signal(0i32);
    
    // Handler for promotion dropdown change
    let handle_promotion_change = move |new_promotion: String| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_promotion(w.id, new_promotion.clone()).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update promotion: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for basic stats change
    let handle_basic_stats_change = move |height: Option<String>, weight: Option<String>, debut_year: Option<i32>, wins: i32, losses: i32| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_basic_stats(w.id, height, weight, debut_year, wins, losses).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update basic stats: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for name change
    let handle_name_change = move |name: String, nickname: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_name(w.id, name, nickname).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update name: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for real name change
    let handle_real_name_change = move |real_name: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_real_name(w.id, real_name).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update real name: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for biography change
    let handle_biography_change = move |biography: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_biography(w.id, biography).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update biography: {}", e)));
                    }
                }
            });
        }
    };


    // Check for URL changes using web_sys setTimeout in a loop
    Effect::new(move |_| {
        use wasm_bindgen::JsCast;
        
        let check_url_change = {
            let set_wrestler_id_clone = set_current_wrestler_id;
            let current_wrestler_id_clone = current_wrestler_id;
            
            Closure::wrap(Box::new(move || {
                let current_id = extract_wrestler_id_from_url();
                if current_id != current_wrestler_id_clone.get() {
                    set_wrestler_id_clone.set(current_id);
                }
            }) as Box<dyn Fn()>)
        };
        
        // Set up recurring check every 500ms
        let check_function = check_url_change.as_ref().unchecked_ref();
        if let Some(window) = web_sys::window() {
            window.set_interval_with_callback_and_timeout_and_arguments_0(check_function, 500).ok();
        }
        
        // Don't forget the closure or it will be dropped
        check_url_change.forget();
    });

    // Load shows data once on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match fetch_shows().await {
                Ok(shows_data) => {
                    set_shows.set(shows_data);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load shows: {}", e)));
                }
            }
        });
    });

    // Load wrestler data when wrestler ID changes
    Effect::new(move |_| {
        let wrestler_id = current_wrestler_id.get();
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            if let Some(wrestler_id) = wrestler_id {
                match get_wrestler_by_id(wrestler_id).await {
                    Ok(Some(wrestler_data)) => {
                        set_wrestler.set(Some(wrestler_data));
                    }
                    Ok(None) => {
                        set_error.set(Some("Wrestler not found".to_string()));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load wrestler: {}", e)));
                    }
                }
            } else {
                set_error.set(Some("Invalid wrestler ID in URL".to_string()));
            }
            
            set_loading.set(false);
        });
    });

    view! {
        <div class="h-full p-4 overflow-auto">
            <div class="max-w-4xl mx-auto">
                <Show when=move || loading.get()>
                    <div class="flex justify-center items-center py-20">
                        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-red-600"></div>
                        <span class="ml-3 text-slate-400">"Loading wrestler details..."</span>
                    </div>
                </Show>
                
                <Show when=move || error.get().is_some()>
                    <div class="bg-red-900/50 border border-red-600 rounded-lg p-8 text-center">
                        <h3 class="text-red-400 text-lg font-semibold mb-2">"Error"</h3>
                        <p class="text-red-300">{move || error.get().unwrap_or_default()}</p>
                    </div>
                </Show>
                
                <Show when=move || !loading.get() && error.get().is_none() && wrestler.get().is_some()>
                    {move || {
                        wrestler.get().map(|w| {
                            view! {
                                <div class="card-modern rounded-xl relative overflow-hidden">
                                    // Header with sleek styling
                                    <HeaderSection wrestler=wrestler />

                                    <div class="grid md:grid-cols-2 gap-6 p-6">
                                        // Left side - Image and basic info
                                        <div class="space-y-4">
                                            // Wrestler image placeholder
                                            <PhotoSection />

                                            // Wrestler name banner
                                            <NameBannerSection 
                                                wrestler=wrestler
                                                on_name_change=handle_name_change
                                            />

                                            // Championship & Team Status
                                            <ChampionshipTeamSection 
                                                wrestler=wrestler
                                            />
                                        </div>

                                        // Right side - Stats and info
                                        <div class="space-y-4">
                                            // Real name section
                                            <RealNameSection 
                                                wrestler=wrestler
                                                on_real_name_change=handle_real_name_change
                                            />

                                            // Power ratings (only show if any exist)
                                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                                <Show
                                                    when=move || w.strength.is_some() || w.speed.is_some() || w.agility.is_some() || 
                                                        w.stamina.is_some() || w.charisma.is_some() || w.technique.is_some()
                                                    fallback=move || view! {
                                                        <div class="text-center text-slate-400 text-sm">
                                                            "No power ratings available"
                                                        </div>
                                                    }
                                                >
                                                    <div class="flex items-center justify-between mb-4 border-b border-slate-700 pb-2">
                                                        <h4 class="text-slate-100 font-bold text-lg">
                                                            "Power Ratings"
                                                        </h4>
                                                        <button
                                                            class="text-slate-400 hover:text-slate-200 text-sm font-medium flex items-center space-x-1"
                                                            on:click=move |_| {
                                                                if let Some(w) = wrestler.get() {
                                                                    // Initialize temp values with current wrestler stats
                                                                    set_temp_strength.set(w.strength.unwrap_or(0));
                                                                    set_temp_speed.set(w.speed.unwrap_or(0));
                                                                    set_temp_agility.set(w.agility.unwrap_or(0));
                                                                    set_temp_stamina.set(w.stamina.unwrap_or(0));
                                                                    set_temp_charisma.set(w.charisma.unwrap_or(0));
                                                                    set_temp_technique.set(w.technique.unwrap_or(0));
                                                                    set_editing_power_ratings.set(true);
                                                                }
                                                            }
                                                        >
                                                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                                                            </svg>
                                                            <span>"Edit"</span>
                                                        </button>
                                                    </div>
                                                    <Show 
                                                        when=move || !editing_power_ratings.get()
                                                        fallback=move || view! {
                                                            <div class="space-y-3">
                                                                <PowerBarEdit label="STRENGTH" value=temp_strength set_value=set_temp_strength _color="bg-red-500" />
                                                                <PowerBarEdit label="SPEED" value=temp_speed set_value=set_temp_speed _color="bg-blue-500" />
                                                                <PowerBarEdit label="AGILITY" value=temp_agility set_value=set_temp_agility _color="bg-green-500" />
                                                                <PowerBarEdit label="STAMINA" value=temp_stamina set_value=set_temp_stamina _color="bg-purple-500" />
                                                                <PowerBarEdit label="CHARISMA" value=temp_charisma set_value=set_temp_charisma _color="bg-indigo-500" />
                                                                <PowerBarEdit label="TECHNIQUE" value=temp_technique set_value=set_temp_technique _color="bg-cyan-500" />
                                                            </div>
                                                            <div class="flex space-x-2 mt-4">
                                                                <button
                                                                    class="flex-1 bg-green-600 hover:bg-green-700 text-white px-3 py-2 rounded text-sm font-medium"
                                                                    on:click=move |_| {
                                                                        if let Some(w) = wrestler.get() {
                                                                            spawn_local(async move {
                                                                                let strength = if temp_strength.get() > 0 { Some(temp_strength.get()) } else { None };
                                                                                let speed = if temp_speed.get() > 0 { Some(temp_speed.get()) } else { None };
                                                                                let agility = if temp_agility.get() > 0 { Some(temp_agility.get()) } else { None };
                                                                                let stamina = if temp_stamina.get() > 0 { Some(temp_stamina.get()) } else { None };
                                                                                let charisma = if temp_charisma.get() > 0 { Some(temp_charisma.get()) } else { None };
                                                                                let technique = if temp_technique.get() > 0 { Some(temp_technique.get()) } else { None };
                                                                                
                                                                                match update_wrestler_power_ratings(
                                                                                    w.id,
                                                                                    strength,
                                                                                    speed,
                                                                                    agility,
                                                                                    stamina,
                                                                                    charisma,
                                                                                    technique
                                                                                ).await {
                                                                                    Ok(updated_wrestler) => {
                                                                                        set_wrestler.set(Some(updated_wrestler));
                                                                                        set_editing_power_ratings.set(false);
                                                                                    }
                                                                                    Err(e) => {
                                                                                        set_error.set(Some(format!("Failed to update power ratings: {}", e)));
                                                                                    }
                                                                                }
                                                                            });
                                                                        }
                                                                    }
                                                                >
                                                                    "Save"
                                                                </button>
                                                                <button
                                                                    class="flex-1 bg-slate-600 hover:bg-slate-700 text-white px-3 py-2 rounded text-sm font-medium"
                                                                    on:click=move |_| {
                                                                        set_editing_power_ratings.set(false);
                                                                    }
                                                                >
                                                                    "Cancel"
                                                                </button>
                                                            </div>
                                                        }
                                                    >
                                                        <div class="space-y-3">
                                                            {w.strength.map(|val| view! { <PowerBar label="STRENGTH" value=val color="bg-red-500" /> })}
                                                            {w.speed.map(|val| view! { <PowerBar label="SPEED" value=val color="bg-blue-500" /> })}
                                                            {w.agility.map(|val| view! { <PowerBar label="AGILITY" value=val color="bg-green-500" /> })}
                                                            {w.stamina.map(|val| view! { <PowerBar label="STAMINA" value=val color="bg-purple-500" /> })}
                                                            {w.charisma.map(|val| view! { <PowerBar label="CHARISMA" value=val color="bg-indigo-500" /> })}
                                                            {w.technique.map(|val| view! { <PowerBar label="TECHNIQUE" value=val color="bg-cyan-500" /> })}
                                                        </div>
                                                    </Show>
                                                </Show>
                                            </div>

                                            // Promotion Section (separate component)
                                            <PromotionSection 
                                                wrestler=wrestler
                                                shows=shows
                                                on_promotion_change=handle_promotion_change
                                            />

                                            // Basic stats (separate component)
                                            <BasicStatsSection 
                                                wrestler=wrestler
                                                on_stats_change=handle_basic_stats_change
                                            />

                                            // Biography
                                            <BiographySection 
                                                wrestler=wrestler
                                                on_biography_change=handle_biography_change
                                            />
                                        </div>
                                    </div>

                                </div>
                            }
                        })
                    }}
                </Show>
            </div>
        </div>
    }
}

#[component]
fn PowerBar(
    #[prop(into)] label: String,
    #[prop(into)] value: i32,
    #[prop(into)] color: String,
) -> impl IntoView {
    let percentage = (value as f32 / 10.0 * 100.0) as i32;
    
    view! {
        <div class="flex items-center space-x-3">
            <span class="text-slate-300 font-medium text-sm w-20 text-right">{label}</span>
            <div class="flex-1 bg-slate-700/50 rounded-full h-3 border border-slate-600">
                <div 
                    class=format!("h-full rounded-full {} flex items-center justify-end pr-1", color)
                    style=format!("width: {}%", percentage)
                >
                    <span class="text-xs font-medium text-white">{value}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PowerBarEdit(
    #[prop(into)] label: String,
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
    #[prop(into)] _color: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center space-x-3">
            <span class="text-slate-300 font-medium text-sm w-20 text-right">{label}</span>
            <div class="flex-1 flex items-center space-x-2">
                <input
                    type="range"
                    min="0"
                    max="10"
                    class="flex-1 h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer slider"
                    prop:value=move || value.get().to_string()
                    on:input:target=move |ev| {
                        if let Ok(val) = ev.target().value().parse::<i32>() {
                            set_value.set(val.clamp(0, 10));
                        }
                    }
                />
                <div class="w-12 text-center">
                    <input
                        type="number"
                        min="0"
                        max="10"
                        class="w-full bg-slate-700/50 border border-slate-600 rounded px-2 py-1 text-slate-100 text-sm text-center"
                        prop:value=move || value.get().to_string()
                        on:input:target=move |ev| {
                            if let Ok(val) = ev.target().value().parse::<i32>() {
                                set_value.set(val.clamp(0, 10));
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn PromotionSection<F>(
    wrestler: ReadSignal<Option<Wrestler>>,
    shows: ReadSignal<Vec<Show>>,
    on_promotion_change: F,
) -> impl IntoView
where
    F: Fn(String) + 'static + Copy + Send + Sync,
{
    view! {
        <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4 mb-4">
            <div class="grid grid-cols-2 gap-4 text-sm">
                // Promotion section
                <div>
                    <span class="text-slate-400 font-medium">"Promotion: "</span>
                </div>
                <div>
                    {move || {
                        if let Some(current_wrestler) = wrestler.get() {
                            if let Some(promotion) = current_wrestler.promotion {
                                view! {
                                    <span class="text-slate-100">{promotion}</span>
                                }
                            } else {
                                view! {
                                    <span class="text-slate-400 italic">{"No promotion assigned".to_string()}</span>
                                }
                            }
                        } else {
                            view! {
                                <span class="text-slate-400 italic">{"Loading...".to_string()}</span>
                            }
                        }
                    }}
                </div>
                // Promotion dropdown
                <div class="col-span-2">
                    <select 
                        class="w-full bg-slate-700/50 border border-slate-600 rounded-lg px-3 py-2 text-slate-100 text-sm focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent"
                        prop:value=move || {
                            wrestler.get().and_then(|w| w.promotion).unwrap_or_default()
                        }
                        on:change:target=move |ev| {
                            on_promotion_change(ev.target().value());
                        }
                    >
                        <option value="" class="bg-slate-800">"Select a promotion..."</option>
                        {move || shows.get().into_iter().map(|show| {
                            view! {
                                <option 
                                    value=show.name.clone()
                                    class="bg-slate-800"
                                >
                                    {show.name.clone()}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BasicStatsSection<F>(
    wrestler: ReadSignal<Option<Wrestler>>,
    on_stats_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>, Option<String>, Option<i32>, i32, i32) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_height, set_temp_height) = signal(String::new());
    let (temp_weight, set_temp_weight) = signal(String::new());
    let (temp_debut_year, set_temp_debut_year) = signal(String::new());
    let (temp_wins, set_temp_wins) = signal(0i32);
    let (temp_losses, set_temp_losses) = signal(0i32);

    view! {
        <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
            <div class="flex items-center justify-between mb-4 border-b border-slate-700 pb-2">
                <h4 class="text-slate-100 font-bold text-lg">
                    "Basic Stats"
                </h4>
                <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                    <button
                        class="text-slate-400 hover:text-slate-200 text-sm font-medium flex items-center space-x-1"
                        on:click=move |_| {
                            if let Some(w) = wrestler.get() {
                                set_temp_height.set(w.height.unwrap_or_default());
                                set_temp_weight.set(w.weight.unwrap_or_default());
                                set_temp_debut_year.set(w.debut_year.map(|y| y.to_string()).unwrap_or_default());
                                set_temp_wins.set(w.wins);
                                set_temp_losses.set(w.losses);
                                set_editing.set(true);
                            }
                        }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        <span>"Edit"</span>
                    </button>
                </Show>
            </div>
            <Show 
                when=move || !editing.get()
                fallback=move || view! {
                    <div class="space-y-3">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Height"</label>
                                <input
                                    type="text"
                                    class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm"
                                    placeholder="e.g., 6'5\""
                                    prop:value=move || temp_height.get()
                                    on:input:target=move |ev| {
                                        set_temp_height.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Weight"</label>
                                <input
                                    type="text"
                                    class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm"
                                    placeholder="e.g., 250 lbs"
                                    prop:value=move || temp_weight.get()
                                    on:input:target=move |ev| {
                                        set_temp_weight.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Debut Year"</label>
                                <input
                                    type="number"
                                    class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm"
                                    placeholder="e.g., 2010"
                                    prop:value=move || temp_debut_year.get()
                                    on:input:target=move |ev| {
                                        set_temp_debut_year.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Gender"</label>
                                <div class="bg-slate-700/30 border border-slate-600 rounded px-3 py-2 text-slate-300 text-sm">
                                    {move || wrestler.get().map(|w| w.gender).unwrap_or_default()}
                                </div>
                            </div>
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Wins"</label>
                                <input
                                    type="number"
                                    min="0"
                                    class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm"
                                    prop:value=move || temp_wins.get().to_string()
                                    on:input:target=move |ev| {
                                        if let Ok(val) = ev.target().value().parse::<i32>() {
                                            set_temp_wins.set(val.max(0));
                                        }
                                    }
                                />
                            </div>
                            <div>
                                <label class="block text-slate-400 font-medium text-sm mb-1">"Losses"</label>
                                <input
                                    type="number"
                                    min="0"
                                    class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm"
                                    prop:value=move || temp_losses.get().to_string()
                                    on:input:target=move |ev| {
                                        if let Ok(val) = ev.target().value().parse::<i32>() {
                                            set_temp_losses.set(val.max(0));
                                        }
                                    }
                                />
                            </div>
                        </div>
                    </div>
                    <div class="flex space-x-2 mt-4">
                        <button
                            class="flex-1 bg-green-600 hover:bg-green-700 text-white px-3 py-2 rounded text-sm font-medium"
                            on:click=move |_| {
                                let height = if temp_height.get().is_empty() { None } else { Some(temp_height.get()) };
                                let weight = if temp_weight.get().is_empty() { None } else { Some(temp_weight.get()) };
                                let debut_year = if temp_debut_year.get().is_empty() { 
                                    None 
                                } else { 
                                    temp_debut_year.get().parse::<i32>().ok() 
                                };
                                
                                on_stats_change(height, weight, debut_year, temp_wins.get(), temp_losses.get());
                                set_editing.set(false);
                            }
                        >
                            "Save"
                        </button>
                        <button
                            class="flex-1 bg-slate-600 hover:bg-slate-700 text-white px-3 py-2 rounded text-sm font-medium"
                            on:click=move |_| {
                                set_editing.set(false);
                            }
                        >
                            "Cancel"
                        </button>
                    </div>
                }
            >
                <div class="grid grid-cols-2 gap-4 text-sm">
                    {move || wrestler.get().map(|w| view! {
                        {w.height.as_ref().map(|height| view! {
                            <div>
                                <span class="text-slate-400 font-medium">"Height: "</span>
                                <span class="text-slate-100">{height.clone()}</span>
                            </div>
                        })}
                        {w.weight.as_ref().map(|weight| view! {
                            <div>
                                <span class="text-slate-400 font-medium">"Weight: "</span>
                                <span class="text-slate-100">{weight.clone()}</span>
                            </div>
                        })}
                        {w.debut_year.map(|year| view! {
                            <div>
                                <span class="text-slate-400 font-medium">"Debut: "</span>
                                <span class="text-slate-100">{year.to_string()}</span>
                            </div>
                        })}
                        <div>
                            <span class="text-slate-400 font-medium">"Gender: "</span>
                            <span class="text-slate-100">{w.gender.clone()}</span>
                        </div>
                    })}
                </div>
            </Show>
        </div>
    }
}

#[component]
fn HeaderSection(
    wrestler: ReadSignal<Option<Wrestler>>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-800/80 backdrop-blur-sm border-b border-slate-700 p-6 text-center relative">
            <div class="absolute top-4 left-6 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                "WRESTLER"
            </div>
            <div class="absolute top-4 right-6 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                {move || wrestler.get().map(|w| format!("#{:03}", w.id)).unwrap_or_default()}
            </div>
            <h2 class="text-2xl font-bold text-slate-100">
                "Wrestler Profile"
            </h2>
            <p class="text-slate-400 text-sm mt-1">
                "Character Details & Statistics"
            </p>
        </div>
    }
}

#[component]
fn PhotoSection() -> impl IntoView {
    view! {
        <div class="bg-slate-800/60 border border-slate-700 rounded-lg aspect-[3/4] flex items-center justify-center relative overflow-hidden">
            <div class="absolute inset-4 bg-slate-700/50 backdrop-blur-sm rounded border border-slate-600 flex items-center justify-center">
                <div class="text-center text-slate-400">
                    <svg class="w-16 h-16 mx-auto mb-2" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                    </svg>
                    <p class="text-sm font-medium">"Photo Coming Soon"</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn NameBannerSection<F>(
    wrestler: ReadSignal<Option<Wrestler>>,
    on_name_change: F,
) -> impl IntoView
where
    F: Fn(String, Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_name, set_temp_name) = signal(String::new());
    let (temp_nickname, set_temp_nickname) = signal(String::new());

    view! {
        <div class="bg-slate-800/80 backdrop-blur-sm border border-slate-700 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
                <div class="flex-1"></div>
                <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                    <button
                        class="text-slate-400 hover:text-slate-200 text-xs font-medium flex items-center space-x-1"
                        on:click=move |_| {
                            if let Some(w) = wrestler.get() {
                                set_temp_name.set(w.name);
                                set_temp_nickname.set(w.nickname.unwrap_or_default());
                                set_editing.set(true);
                            }
                        }
                    >
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        <span>"Edit"</span>
                    </button>
                </Show>
            </div>
            <Show 
                when=move || !editing.get()
                fallback=move || view! {
                    <div class="space-y-3">
                        <div>
                            <label class="block text-slate-400 font-medium text-sm mb-1">"Name"</label>
                            <input
                                type="text"
                                class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-lg font-bold text-center"
                                prop:value=move || temp_name.get()
                                on:input:target=move |ev| {
                                    set_temp_name.set(ev.target().value());
                                }
                            />
                        </div>
                        <div>
                            <label class="block text-slate-400 font-medium text-sm mb-1">"Nickname"</label>
                            <input
                                type="text"
                                class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm text-center"
                                placeholder="Optional nickname"
                                prop:value=move || temp_nickname.get()
                                on:input:target=move |ev| {
                                    set_temp_nickname.set(ev.target().value());
                                }
                            />
                        </div>
                    </div>
                    <div class="flex space-x-2 mt-4">
                        <button
                            class="flex-1 bg-green-600 hover:bg-green-700 text-white px-3 py-2 rounded text-sm font-medium"
                            on:click=move |_| {
                                let nickname = if temp_nickname.get().is_empty() { None } else { Some(temp_nickname.get()) };
                                on_name_change(temp_name.get(), nickname);
                                set_editing.set(false);
                            }
                        >
                            "Save"
                        </button>
                        <button
                            class="flex-1 bg-slate-600 hover:bg-slate-700 text-white px-3 py-2 rounded text-sm font-medium"
                            on:click=move |_| {
                                set_editing.set(false);
                            }
                        >
                            "Cancel"
                        </button>
                    </div>
                }
            >
                {move || wrestler.get().map(|w| view! {
                    <h3 class="text-3xl font-bold text-slate-100 text-center">
                        {w.name}
                    </h3>
                    {w.nickname.as_ref().map(|nickname| view! {
                        <p class="text-center text-slate-400 text-sm mt-1">{nickname.clone()}</p>
                    })}
                })}
            </Show>
        </div>
    }
}

#[component]
fn RealNameSection<F>(
    wrestler: ReadSignal<Option<Wrestler>>,
    on_real_name_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_real_name, set_temp_real_name) = signal(String::new());

    view! {
        <Show when=move || wrestler.get().and_then(|w| w.real_name.clone()).is_some() || editing.get()>
            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                <div class="flex items-center justify-between mb-2 border-b border-slate-700 pb-2">
                    <div class="text-indigo-400 text-sm font-medium">
                        "Real Name"
                    </div>
                    <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                        <button
                            class="text-slate-400 hover:text-slate-200 text-xs font-medium flex items-center space-x-1"
                            on:click=move |_| {
                                if let Some(w) = wrestler.get() {
                                    set_temp_real_name.set(w.real_name.unwrap_or_default());
                                    set_editing.set(true);
                                }
                            }
                        >
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                            <span>"Edit"</span>
                        </button>
                    </Show>
                </div>
                <Show 
                    when=move || !editing.get()
                    fallback=move || view! {
                        <div class="space-y-3">
                            <input
                                type="text"
                                class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-lg"
                                placeholder="Enter real name"
                                prop:value=move || temp_real_name.get()
                                on:input:target=move |ev| {
                                    set_temp_real_name.set(ev.target().value());
                                }
                            />
                        </div>
                        <div class="flex space-x-2 mt-4">
                            <button
                                class="flex-1 bg-green-600 hover:bg-green-700 text-white px-3 py-2 rounded text-sm font-medium"
                                on:click=move |_| {
                                    let real_name = if temp_real_name.get().is_empty() { None } else { Some(temp_real_name.get()) };
                                    on_real_name_change(real_name);
                                    set_editing.set(false);
                                }
                            >
                                "Save"
                            </button>
                            <button
                                class="flex-1 bg-slate-600 hover:bg-slate-700 text-white px-3 py-2 rounded text-sm font-medium"
                                on:click=move |_| {
                                    set_editing.set(false);
                                }
                            >
                                "Cancel"
                            </button>
                        </div>
                    }
                >
                    {move || wrestler.get().and_then(|w| w.real_name).map(|real_name| view! {
                        <p class="text-slate-100 font-semibold text-lg">{real_name}</p>
                    })}
                </Show>
            </div>
        </Show>
    }
}

#[component]
fn BiographySection<F>(
    wrestler: ReadSignal<Option<Wrestler>>,
    on_biography_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_biography, set_temp_biography) = signal(String::new());

    view! {
        <Show when=move || wrestler.get().and_then(|w| w.biography.clone()).is_some() || editing.get()>
            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                <div class="flex items-center justify-between mb-3 border-b border-slate-700 pb-2">
                    <h4 class="text-slate-100 font-semibold text-lg">"Biography"</h4>
                    <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                        <button
                            class="text-slate-400 hover:text-slate-200 text-sm font-medium flex items-center space-x-1"
                            on:click=move |_| {
                                if let Some(w) = wrestler.get() {
                                    set_temp_biography.set(w.biography.unwrap_or_default());
                                    set_editing.set(true);
                                }
                            }
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                            <span>"Edit"</span>
                        </button>
                    </Show>
                </div>
                <Show 
                    when=move || !editing.get()
                    fallback=move || view! {
                        <div class="space-y-3">
                            <textarea
                                class="w-full bg-slate-700/50 border border-slate-600 rounded px-3 py-2 text-slate-100 text-sm resize-none"
                                rows="4"
                                placeholder="Enter biography..."
                                prop:value=move || temp_biography.get()
                                on:input:target=move |ev| {
                                    set_temp_biography.set(ev.target().value());
                                }
                            ></textarea>
                        </div>
                        <div class="flex space-x-2 mt-4">
                            <button
                                class="flex-1 bg-green-600 hover:bg-green-700 text-white px-3 py-2 rounded text-sm font-medium"
                                on:click=move |_| {
                                    let biography = if temp_biography.get().is_empty() { None } else { Some(temp_biography.get()) };
                                    on_biography_change(biography);
                                    set_editing.set(false);
                                }
                            >
                                "Save"
                            </button>
                            <button
                                class="flex-1 bg-slate-600 hover:bg-slate-700 text-white px-3 py-2 rounded text-sm font-medium"
                                on:click=move |_| {
                                    set_editing.set(false);
                                }
                            >
                                "Cancel"
                            </button>
                        </div>
                    }
                >
                    {move || wrestler.get().and_then(|w| w.biography).map(|bio| view! {
                        <p class="text-slate-300 text-sm leading-relaxed">
                            {bio}
                        </p>
                    })}
                </Show>
            </div>
        </Show>
    }
}


#[component]
fn ChampionshipTeamSection(
    wrestler: ReadSignal<Option<Wrestler>>,
) -> impl IntoView {
    view! {
        <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
            <h4 class="text-slate-100 font-bold text-lg mb-4 border-b border-slate-700 pb-2">
                "Championship & Team Status"
            </h4>
            
            <div class="space-y-4">
                // Record section
                <div class="flex items-center justify-between">
                    <span class="text-slate-400 font-medium text-sm">"Record:"</span>
                    <span class="text-slate-100 font-semibold">
                        {move || wrestler.get().map(|w| format!("{}-{}", w.wins, w.losses)).unwrap_or_default()}
                    </span>
                </div>
                
                // Current Belt section
                <div class="space-y-2">
                    <span class="text-slate-400 font-medium text-sm">"Current Belt:"</span>
                    <div class="bg-slate-700/50 border border-slate-600 rounded-lg p-3 flex items-center space-x-3">
                        <div class="w-10 h-10 bg-yellow-600/20 border border-yellow-600/50 rounded-lg flex items-center justify-center">
                            // Championship belt icon
                            <svg class="w-6 h-6 text-yellow-500" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M5 16L3 14l5.5-5.5L10 10l4-4 4 4 1.5-1.5L15 3l-4 4L7 3 2.5 8.5 5 11v5zm2.5 2.5L9 17l1.5 1.5L12 17l1.5 1.5L15 17l1.5 1.5L18 17v-2l-1.5-1.5L15 15l-1.5-1.5L12 15l-1.5-1.5L9 15l-1.5 1.5L6 17v2l1.5-1.5z"/>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <p class="text-slate-300 text-sm italic">"No championship held"</p>
                            <p class="text-slate-500 text-xs">"Belt management coming soon"</p>
                        </div>
                    </div>
                </div>
                
                // Tag Team section
                <div class="space-y-2">
                    <span class="text-slate-400 font-medium text-sm">"Tag Team:"</span>
                    <div class="bg-slate-700/50 border border-slate-600 rounded-lg p-3">
                        <div class="flex items-center space-x-2 mb-2">
                            // Partner placeholders
                            <div class="flex space-x-2">
                                <div class="w-8 h-8 bg-slate-600/50 border border-slate-500 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-slate-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                                <div class="w-8 h-8 bg-slate-600/50 border border-slate-500 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-slate-400" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                                <div class="w-8 h-8 bg-slate-600/30 border border-slate-500/50 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-slate-500" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <div>
                            <p class="text-slate-300 text-sm italic">"No tag team partners"</p>
                            <p class="text-slate-500 text-xs">"Tag team management coming soon"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}