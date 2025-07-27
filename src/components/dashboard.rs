use crate::types::fetch_shows;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Main dashboard component with show selector and action buttons
/// 
/// Currently displays:
/// - Show selector dropdown (for future wrestler filtering by show)
/// - Action buttons for creating shows, viewing wrestlers, and managing championships
/// - Statistics cards showing counts
#[component]
pub fn Dashboard(
    set_current_page: WriteSignal<String>,
    refresh_trigger: ReadSignal<u32>,
) -> impl IntoView {
    let shows_resource = LocalResource::new(move || {
        let _trigger = refresh_trigger.get(); // This makes the resource reactive to refresh_trigger
        fetch_shows()
    });
    let (selected_show_name, set_selected_show_name) = signal(String::new());
    
    // PLANNED ARCHITECTURE:
    // - Show selector will become a FK in the Wrestler table for categorization
    // - Wrestler roster will be filtered based on selected show
    // - Currently shows example data, will connect to real wrestler database
    // Opens wrestler details window showing example wrestler data
    // TODO: In the future, this will open a wrestler roster filtered by selected show
    let open_wrestler_window = move |_| {
        spawn_local(async move {
            let args = serde_json::json!({
                "wrestlerId": "the-rock"
            });
            let _ = invoke("open_wrestler_window", serde_wasm_bindgen::to_value(&args).unwrap()).await;
        });
    };
    
    // Creates test data (5 wrestlers and 2 shows) if it doesn't exist
    let create_test_data = move |_| {
        web_sys::console::log_1(&"Test Data button clicked!".into());
        spawn_local(async move {
            web_sys::console::log_1(&"Starting test data creation...".into());
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
            let result = invoke("create_test_data", args).await;
            
            web_sys::console::log_1(&format!("Raw result: {:?}", result).into());
            
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(message) => {
                    web_sys::console::log_1(&format!("✅ Test data result: {}", message).into());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("❌ Failed to parse result: {:?}", e).into());
                }
            }
        });
    };

    let options_view = move || {
        shows_resource.get().map(|result| {
            let mut options = Vec::new();

            match &*result {
                Ok(shows) => {
                    // Set first show as default if none selected and shows exist
                    if !shows.is_empty() && selected_show_name.get().is_empty() {
                        set_selected_show_name.set(shows[0].name.clone());
                    }

                    if shows.is_empty() {
                        options.push(view! {
                            <option value={String::new()}>
                                {"-- No shows available --".to_string()}
                            </option>
                        });
                    } else {
                        for show in shows {
                            let show_name = show.name.clone();

                            options.push(view! {
                                <option value={show_name.clone()}>
                                    {show_name.clone()}
                                </option>
                            });
                        }
                    }
                }
                Err(e) => {
                    options.push(view! {
                        <option value={String::new()}>
                            {format!("Error: {}", e)}
                        </option>
                    });
                }
            }

            options
        })
    };

    view! {
        <div class="space-y-8">
            <div class="card-modern rounded-xl p-8">
                <div class="text-center mb-8">
                    <h2 class="text-3xl font-bold text-slate-100 mb-2">
                        "Show Management"
                    </h2>
                    <p class="text-slate-400">
                        "Select and manage your wrestling shows"
                    </p>
                </div>

                <div class="bg-slate-800/50 rounded-lg p-6 mb-8">
                    <Suspense fallback=move || {
                        view! {
                            <div class="flex items-center justify-center py-12">
                                <div class="flex items-center space-x-3">
                                    <div class="w-5 h-5 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
                                    <span class="text-slate-300">"Loading shows..."</span>
                                </div>
                            </div>
                        }
                    }>
                        <div class="space-y-6">
                            <div>
                                <label class="block text-sm font-medium text-slate-300 mb-3">
                                    "Select Show"
                                </label>
                                <select
                                    on:change:target=move |ev| {
                                        set_selected_show_name.set(ev.target().value());
                                    }
                                    class="w-full bg-slate-700 border border-slate-600 rounded-lg px-4 py-3 text-slate-100 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
                                >
                                    {options_view}
                                </select>
                            </div>

                            <div class="text-center p-4 bg-slate-800/30 rounded-lg border border-slate-700">
                                <span class="text-slate-400 text-sm">"Current Selection: "</span>
                                <span class="font-semibold text-slate-100">
                                    {move || {
                                        let selection = selected_show_name.get();
                                        if selection.is_empty() {
                                            "None Selected".to_string()
                                        } else {
                                            selection
                                        }
                                    }}
                                </span>
                            </div>
                        </div>
                    </Suspense>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                    <button
                        class="btn btn-primary bg-indigo-600 hover:bg-indigo-700 border-indigo-600 text-white px-6 py-3 rounded-lg flex items-center justify-center space-x-2"
                        on:click=move |_| set_current_page.set("create-show".to_string())
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                        </svg>
                        <span>"Create Show"</span>
                    </button>
                    <button 
                        class="btn bg-purple-600 hover:bg-purple-700 border-purple-600 text-white px-6 py-3 rounded-lg flex items-center justify-center space-x-2"
                        on:click=open_wrestler_window
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                        </svg>
                        <span>"Wrestlers"</span>
                    </button>
                    <button class="btn bg-cyan-600 hover:bg-cyan-700 border-cyan-600 text-white px-6 py-3 rounded-lg flex items-center justify-center space-x-2">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span>"Championships"</span>
                    </button>
                    <button 
                        class="btn bg-orange-600 hover:bg-orange-700 border-orange-600 text-white px-6 py-3 rounded-lg flex items-center justify-center space-x-2"
                        on:click=create_test_data
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
                        </svg>
                        <span>"Test Data"</span>
                    </button>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <div class="w-10 h-10 bg-indigo-500/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-indigo-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                            </svg>
                        </div>
                    </div>
                    <div class="space-y-1">
                        <p class="text-2xl font-bold text-slate-100">"0"</p>
                        <p class="text-sm text-slate-400">"Total Shows"</p>
                        <p class="text-xs text-slate-500">"Create your first show"</p>
                    </div>
                </div>

                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <div class="w-10 h-10 bg-purple-500/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                            </svg>
                        </div>
                    </div>
                    <div class="space-y-1">
                        <p class="text-2xl font-bold text-slate-100">"0"</p>
                        <p class="text-sm text-slate-400">"Wrestlers"</p>
                        <p class="text-xs text-slate-500">"Build your roster"</p>
                    </div>
                </div>

                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <div class="w-10 h-10 bg-cyan-500/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path>
                            </svg>
                        </div>
                    </div>
                    <div class="space-y-1">
                        <p class="text-2xl font-bold text-slate-100">"0"</p>
                        <p class="text-sm text-slate-400">"Championships"</p>
                        <p class="text-xs text-slate-500">"Create titles"</p>
                    </div>
                </div>
            </div>

        </div>
    }
}
