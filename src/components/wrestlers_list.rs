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

async fn get_wrestlers() -> Result<Vec<Wrestler>, String> {
    let result = invoke("get_wrestlers", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn open_wrestler_window(wrestler_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id
    }))
    .map_err(|e| e.to_string())?;

    let _result = invoke("open_wrestler_window", args).await;
    Ok(())
}

#[component]
pub fn WrestlersList(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);

    // Load wrestlers on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match get_wrestlers().await {
                Ok(data) => {
                    set_wrestlers.set(data);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    let handle_wrestler_click = move |wrestler_id: i32| {
        spawn_local(async move {
            if let Err(e) = open_wrestler_window(wrestler_id.to_string()).await {
                web_sys::console::error_1(&format!("Failed to open wrestler window: {}", e).into());
            }
        });
    };

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
                </div>
                <h1 class="text-4xl font-bold text-white mb-2">
                    "WWE Universe Roster"
                </h1>
                <p class="text-slate-400">
                    "Select a wrestler to view their detailed profile"
                </p>
            </div>

            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-red-600"></div>
                    <span class="ml-3 text-slate-400">"Loading wrestlers..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="bg-red-900/50 border border-red-600 rounded-lg p-6 text-center">
                    <h3 class="text-red-400 text-lg font-semibold mb-2">"Error Loading Wrestlers"</h3>
                    <p class="text-red-300">{move || error.get().unwrap_or_default()}</p>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && wrestlers.get().is_empty()>
                <div class="bg-slate-800 border border-slate-600 rounded-lg p-8 text-center">
                    <h3 class="text-slate-400 text-lg font-semibold mb-2">"No Wrestlers Found"</h3>
                    <p class="text-slate-500 mb-4">"No wrestlers are currently in the database."</p>
                    <p class="text-slate-600 text-sm">"Use the Test Data button on the dashboard to add sample wrestlers."</p>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && !wrestlers.get().is_empty()>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
                    <For
                        each=move || wrestlers.get()
                        key=|wrestler| wrestler.id
                        children=move |wrestler| {
                            let wrestler_id = wrestler.id;

                            view! {
                                <div class="bg-slate-800 border border-slate-600 rounded-lg p-4 h-32 hover:border-purple-500 hover:shadow-lg hover:shadow-purple-500/20 transition-all duration-200 cursor-pointer group flex flex-col justify-center"
                                     on:click=move |_| handle_wrestler_click(wrestler_id)>
                                    
                                    <div class="text-center">
                                        <h3 class="text-lg font-bold text-white group-hover:text-purple-400 transition-colors mb-2">
                                            {wrestler.name.clone()}
                                        </h3>
                                        <div class="h-4 mb-2">
                                            {wrestler.nickname.as_ref().map(|nickname| view! {
                                                <p class="text-slate-400 text-xs italic">
                                                    {format!("\"{}\"", nickname)}
                                                </p>
                                            })}
                                        </div>
                                        <div class="text-xs text-slate-500 group-hover:text-slate-400 transition-colors">
                                            "Click for details"
                                        </div>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>
        </div>
    }
}