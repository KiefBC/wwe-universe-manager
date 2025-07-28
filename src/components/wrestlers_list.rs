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
    pub is_user_created: Option<bool>,
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
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn btn-ghost gap-2"
                        on:click=move |_| set_current_page.set("home".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Dashboard"
                    </button>
                    <button
                        class="btn btn-secondary gap-2"
                        on:click=move |_| set_current_page.set("create-wrestler".to_string())
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                        </svg>
                        "Create Wrestler"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "WWE Universe Roster"
                </h1>
                <p class="text-base-content/70">
                    "Select a wrestler to view their detailed profile. Custom wrestlers can be edited, system wrestlers are read-only."
                </p>
            </div>

            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <span class="loading loading-spinner loading-lg"></span>
                    <span class="ml-3 text-base-content/70">"Loading wrestlers..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error Loading Wrestlers"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && wrestlers.get().is_empty()>
                <div class="alert alert-info">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    <div>
                        <h3 class="font-bold">"No Wrestlers Found"</h3>
                        <div class="text-xs">"No wrestlers are currently in the database. Use the Test Data button on the dashboard to add sample wrestlers."</div>
                    </div>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && !wrestlers.get().is_empty()>
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
                    <For
                        each=move || wrestlers.get()
                        key=|wrestler| wrestler.id
                        children=move |wrestler| {
                            let wrestler_id = wrestler.id;
                            let is_user_created = wrestler.is_user_created.unwrap_or(false);

                            view! {
                                <div class="card bg-base-200 hover:bg-base-300 border border-base-300 hover:border-secondary transition-all duration-200 cursor-pointer group h-32 relative"
                                     on:click=move |_| handle_wrestler_click(wrestler_id)>
                                    <div class="card-body p-4 flex flex-col justify-center">
                                        // System wrestler indicator
                                        <Show when=move || !is_user_created>
                                            <div class="absolute top-2 right-2">
                                                <div class="badge badge-neutral badge-sm gap-1">
                                                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                                        <path fill-rule="evenodd" d="M18 8a6 6 0 01-7.743 5.743L10 14l-1 1-1 1H6v2H2v-4l4.257-4.257A6 6 0 1118 8zm-6-4a1 1 0 100 2 2 2 0 012 2 1 1 0 102 0 4 4 0 00-4-4z" clip-rule="evenodd" />
                                                    </svg>
                                                    <span>"System"</span>
                                                </div>
                                            </div>
                                        </Show>

                                        // User-created wrestler indicator
                                        <Show when=move || is_user_created>
                                            <div class="absolute top-2 right-2">
                                                <div class="badge badge-success badge-sm gap-1">
                                                    <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                    </svg>
                                                    <span>"Custom"</span>
                                                </div>
                                            </div>
                                        </Show>
                                        
                                        <div class="text-center">
                                            <h3 class="text-lg font-bold text-base-content group-hover:text-secondary transition-colors mb-2">
                                                {wrestler.name.clone()}
                                            </h3>
                                            <div class="h-4 mb-2">
                                                {wrestler.nickname.as_ref().map(|nickname| view! {
                                                    <p class="text-base-content/60 text-xs italic">
                                                        {format!("\"{}\"", nickname)}
                                                    </p>
                                                })}
                                            </div>
                                            <div class="text-xs text-base-content/50 group-hover:text-base-content/70 transition-colors">
                                                "Click for details"
                                            </div>
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