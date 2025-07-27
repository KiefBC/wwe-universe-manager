use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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
    pub trivia: Option<String>,
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
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Load wrestler data on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            if let Some(wrestler_id) = extract_wrestler_id_from_url() {
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
                                    <div class="bg-slate-800/80 backdrop-blur-sm border-b border-slate-700 p-6 text-center relative">
                                        <div class="absolute top-4 left-6 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                                            "WRESTLER"
                                        </div>
                                        <div class="absolute top-4 right-6 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                                            {format!("#{:03}", w.id)}
                                        </div>
                                        <h2 class="text-2xl font-bold text-slate-100">
                                            "Wrestler Profile"
                                        </h2>
                                        <p class="text-slate-400 text-sm mt-1">
                                            "Character Details & Statistics"
                                        </p>
                                    </div>

                                    <div class="grid md:grid-cols-2 gap-6 p-6">
                                        // Left side - Image and basic info
                                        <div class="space-y-4">
                                            // Wrestler image placeholder
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

                                            // Wrestler name banner
                                            <div class="bg-slate-800/80 backdrop-blur-sm border border-slate-700 p-4 rounded-lg">
                                                <h3 class="text-3xl font-bold text-slate-100 text-center">
                                                    {w.name.clone()}
                                                </h3>
                                                {w.nickname.as_ref().map(|nickname| view! {
                                                    <p class="text-center text-slate-400 text-sm mt-1">{nickname.clone()}</p>
                                                })}
                                            </div>
                                        </div>

                                        // Right side - Stats and info
                                        <div class="space-y-4">
                                            // Real name section
                                            {w.real_name.as_ref().map(|real_name| view! {
                                                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                                    <div class="text-indigo-400 text-sm font-medium mb-2">
                                                        "Real Name"
                                                    </div>
                                                    <p class="text-slate-100 font-semibold text-lg">{real_name.clone()}</p>
                                                </div>
                                            })}

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
                                                    <h4 class="text-slate-100 font-bold text-lg mb-4 text-center border-b border-slate-700 pb-2">
                                                        "Power Ratings"
                                                    </h4>
                                                    <div class="space-y-3">
                                                        {w.strength.map(|val| view! { <PowerBar label="STRENGTH" value=val color="bg-red-500" /> })}
                                                        {w.speed.map(|val| view! { <PowerBar label="SPEED" value=val color="bg-blue-500" /> })}
                                                        {w.agility.map(|val| view! { <PowerBar label="AGILITY" value=val color="bg-green-500" /> })}
                                                        {w.stamina.map(|val| view! { <PowerBar label="STAMINA" value=val color="bg-purple-500" /> })}
                                                        {w.charisma.map(|val| view! { <PowerBar label="CHARISMA" value=val color="bg-indigo-500" /> })}
                                                        {w.technique.map(|val| view! { <PowerBar label="TECHNIQUE" value=val color="bg-cyan-500" /> })}
                                                    </div>
                                                </Show>
                                            </div>

                                            // Basic stats
                                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                                <div class="grid grid-cols-2 gap-4 text-sm">
                                                    {w.promotion.as_ref().map(|promotion| view! {
                                                        <div>
                                                            <span class="text-slate-400 font-medium">"Promotion: "</span>
                                                            <span class="text-slate-100">{promotion.clone()}</span>
                                                        </div>
                                                    })}
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
                                                    <div>
                                                        <span class="text-slate-400 font-medium">"Record: "</span>
                                                        <span class="text-slate-100">{format!("{}-{}", w.wins, w.losses)}</span>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    // Bottom section
                                    <div class="px-6 pb-6 space-y-4">
                                        // Biography
                                        {w.biography.as_ref().map(|bio| view! {
                                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                                <h4 class="text-slate-100 font-semibold text-lg mb-3">"Biography"</h4>
                                                <p class="text-slate-300 text-sm leading-relaxed">
                                                    {bio.clone()}
                                                </p>
                                            </div>
                                        })}

                                        // Did you know section (trivia)
                                        {w.trivia.as_ref().map(|trivia| view! {
                                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                                <h4 class="text-slate-100 font-semibold text-lg mb-3">"Did You Know"</h4>
                                                <p class="text-slate-300 text-sm leading-relaxed">
                                                    {trivia.clone()}
                                                </p>
                                            </div>
                                        })}
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