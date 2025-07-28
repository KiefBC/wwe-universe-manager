use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_titles() -> Result<Vec<TitleWithHolders>, String> {
    let result = invoke("get_titles", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn open_title_window(title_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "titleId": title_id
    }))
    .map_err(|e| e.to_string())?;

    let _result = invoke("open_title_window", args).await;
    Ok(())
}

#[component]
pub fn TitlesList(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (titles, set_titles) = signal(Vec::<TitleWithHolders>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Load titles on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match get_titles().await {
                Ok(data) => {
                    set_titles.set(data);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    let handle_title_click = move |title_id: i32| {
        spawn_local(async move {
            if let Err(e) = open_title_window(title_id.to_string()).await {
                web_sys::console::error_1(&format!("Failed to open title window: {}", e).into());
            }
        });
    };

    // Create signals for each tier
    let (tier_1_titles, set_tier_1_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_2_titles, set_tier_2_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_3_titles, set_tier_3_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_4_titles, set_tier_4_titles) = signal(Vec::<TitleWithHolders>::new());

    // Update tier signals when titles change
    Effect::new(move |_| {
        let all_titles = titles.get();
        let mut tier_1 = Vec::new();
        let mut tier_2 = Vec::new();
        let mut tier_3 = Vec::new();
        let mut tier_4 = Vec::new();

        for title in all_titles {
            match title.title.prestige_tier {
                1 => tier_1.push(title),
                2 => tier_2.push(title),
                3 => tier_3.push(title),
                _ => tier_4.push(title),
            }
        }

        set_tier_1_titles.set(tier_1);
        set_tier_2_titles.set(tier_2);
        set_tier_3_titles.set(tier_3);
        set_tier_4_titles.set(tier_4);
    });

    view! {
        <div class="container mx-auto p-6 bg-slate-900 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn bg-slate-700 hover:bg-slate-600 border-slate-600 text-white px-4 py-2 rounded-lg flex items-center space-x-2"
                        on:click=move |_| set_current_page.set("home".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        <span>"Back to Dashboard"</span>
                    </button>
                    <button
                        class="btn bg-cyan-600 hover:bg-cyan-700 border-cyan-500 text-white px-6 py-2 rounded-lg flex items-center space-x-2 font-semibold"
                        on:click=move |_| set_current_page.set("create-title".to_string())
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                        </svg>
                        <span>"Create Title"</span>
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-white mb-2">
                    "Championship Titles"
                </h1>
                <p class="text-slate-400">
                    "Select a title to view detailed information and championship history."
                </p>
            </div>

            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-cyan-600"></div>
                    <span class="ml-3 text-slate-400">"Loading titles..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="bg-red-900/50 border border-red-600 rounded-lg p-6 text-center">
                    <h3 class="text-red-400 text-lg font-semibold mb-2">"Error Loading Titles"</h3>
                    <p class="text-red-300">{move || error.get().unwrap_or_default()}</p>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && titles.get().is_empty()>
                <div class="bg-slate-800 border border-slate-600 rounded-lg p-8 text-center">
                    <h3 class="text-slate-400 text-lg font-semibold mb-2">"No Titles Found"</h3>
                    <p class="text-slate-500 mb-4">"No championship titles are currently in the database."</p>
                    <p class="text-slate-600 text-sm">"Use the Create Title button to add championships to your universe."</p>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty()>
                <div class="space-y-12">
                    // Tier 1 - World Championships (1 per row, gold styling)
                    <div>
                        <Show when=move || !tier_1_titles.get().is_empty()>
                            <div class="mb-8">
                                <h2 class="text-2xl font-bold text-yellow-400 mb-6 flex items-center">
                                    <svg class="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                    </svg>
                                    "World Championships"
                                </h2>
                                <div class="space-y-4">
                                    <For
                                        each=move || tier_1_titles.get()
                                        key=|title| title.title.id
                                        children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else if title.current_holders.len() == 1 {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    } else {
                                                        format!("{} & {}", 
                                                            title.current_holders[0].wrestler_name,
                                                            title.current_holders[1].wrestler_name)
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="bg-gradient-to-r from-yellow-600/20 to-yellow-500/20 border-2 border-yellow-500 rounded-xl p-6 hover:border-yellow-400 hover:shadow-lg hover:shadow-yellow-500/30 transition-all duration-300 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="flex items-center justify-between">
                                                                <div class="flex-1">
                                                                    <h3 class="text-lg font-bold text-yellow-400 group-hover:text-yellow-300 transition-colors mb-2">
                                                                        {title.title.name.clone()}
                                                                    </h3>
                                                                    <p class="text-yellow-200 text-sm mb-2">
                                                                        {title.title.division.clone()}
                                                                    </p>
                                                                    <div class="flex items-center space-x-4 text-slate-300">
                                                                        <span class="font-semibold">
                                                                            "Champion: " <span class="text-yellow-400">{holders_text}</span>
                                                                        </span>
                                                                        <span class="text-slate-400">
                                                                            "•"
                                                                        </span>
                                                                        <span>
                                                                            {days_text}
                                                                        </span>
                                                                    </div>
                                                                </div>
                                                                <div class="text-yellow-500 group-hover:text-yellow-400 transition-colors">
                                                                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                                                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                                    </svg>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                                // Tier 2 - Secondary Championships (2 per row, silver styling)
                        <Show when=move || !tier_2_titles.get().is_empty()>
                            <div class="mb-8">
                                <h2 class="text-2xl font-bold text-slate-300 mb-6 flex items-center">
                                    <svg class="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd" />
                                    </svg>
                                    "Secondary Championships"
                                </h2>
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || tier_2_titles.get()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="bg-slate-800/60 border border-slate-400 rounded-lg p-6 hover:border-slate-300 hover:shadow-lg hover:shadow-slate-500/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <h3 class="text-lg font-bold text-slate-300 group-hover:text-slate-100 transition-colors mb-2">
                                                                {title.title.name.clone()}
                                                            </h3>
                                                            <p class="text-slate-400 text-sm mb-3">
                                                                {title.title.division.clone()}
                                                            </p>
                                                            <div class="space-y-1 text-sm text-slate-400">
                                                                <div>
                                                                    "Champion: " <span class="text-slate-300">{holders_text}</span>
                                                                </div>
                                                                <div>
                                                                    {days_text}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                        // Tier 3 - Tag Team Championships (3 per row, bronze styling)
                        <Show when=move || !tier_3_titles.get().is_empty()>
                            <div class="mb-8">
                                <h2 class="text-2xl font-bold text-orange-400 mb-6 flex items-center">
                                    <svg class="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                        <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                    </svg>
                                    "Tag Team Championships"
                                </h2>
                                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                    <For
                                        each=move || tier_3_titles.get()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else if title.current_holders.len() >= 2 {
                                                        format!("{} & {}", 
                                                            title.current_holders[0].wrestler_name,
                                                            title.current_holders[1].wrestler_name)
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="bg-slate-800/40 border border-orange-600/50 rounded-lg p-4 hover:border-orange-500 hover:shadow-lg hover:shadow-orange-500/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <h3 class="text-lg font-bold text-orange-400 group-hover:text-orange-300 transition-colors mb-2">
                                                                {title.title.name.clone()}
                                                            </h3>
                                                            <p class="text-orange-300 text-sm mb-2">
                                                                {title.title.division.clone()}
                                                            </p>
                                                            <div class="space-y-1 text-sm text-slate-400">
                                                                <div>
                                                                    "Champions: " <span class="text-orange-300">{holders_text}</span>
                                                                </div>
                                                                <div>
                                                                    {days_text}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                        // Tier 4 - Specialty Championships (4 per row, themed colors)
                        <Show when=move || !tier_4_titles.get().is_empty()>
                            <div class="mb-8">
                                <h2 class="text-xl font-bold text-purple-400 mb-6 flex items-center">
                                    <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd" />
                                    </svg>
                                    "Specialty Championships"
                                </h2>
                                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
                                    <For
                                        each=move || tier_4_titles.get()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="bg-slate-800/30 border border-purple-600/40 rounded-lg p-3 hover:border-purple-500 hover:shadow-lg hover:shadow-purple-500/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <h3 class="text-lg font-bold text-purple-400 group-hover:text-purple-300 transition-colors mb-2">
                                                                {title.title.name.clone()}
                                                            </h3>
                                                            <p class="text-purple-300 text-sm mb-2">
                                                                {title.title.division.clone()}
                                                            </p>
                                                            <div class="space-y-1 text-sm text-slate-500">
                                                                <div>
                                                                    <span class="text-purple-300">{holders_text}</span>
                                                                </div>
                                                                <div>
                                                                    {days_text}
                                                                </div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        </Show>
                    </div>
                </div>
            </Show>
        </div>
    }
}