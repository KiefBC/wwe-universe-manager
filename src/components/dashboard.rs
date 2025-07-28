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
    // Navigate to wrestlers list page
    let navigate_to_wrestlers = move |_| {
        set_current_page.set("wrestlers".to_string());
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

    let _options_view = move || {
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
            <div class="text-center mb-8">
                <h2 class="text-3xl font-bold text-base-content mb-2">
                    "General Manager Dashboard"
                </h2>
                <p class="text-base-content/70">
                    "Manage your WWE Universe with executive control"
                </p>
            </div>


            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
                <button
                    class="btn btn-primary gap-2"
                    on:click=move |_| set_current_page.set("create-show".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                    </svg>
                    "Book Show"
                </button>
                <button 
                    class="btn btn-secondary gap-2"
                    on:click=navigate_to_wrestlers
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                    "Manage Roster"
                </button>
                <button 
                    class="btn btn-accent gap-2"
                    on:click=move |_| set_current_page.set("titles".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                    </svg>
                    "Title Management"
                </button>
                <button 
                    class="btn btn-warning gap-2"
                    on:click=create_test_data
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
                    </svg>
                    "Populate Roster"
                </button>
            </div>

            <div class="stats stats-vertical lg:stats-horizontal shadow">
                <div class="stat">
                    <div class="stat-figure text-primary">
                        <svg class="inline-block w-8 h-8 stroke-current" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                        </svg>
                    </div>
                    <div class="stat-title">"Total Shows"</div>
                    <div class="stat-value text-primary">"0"</div>
                    <div class="stat-desc">"Create your first show"</div>
                </div>

                <div class="stat">
                    <div class="stat-figure text-secondary">
                        <svg class="inline-block w-8 h-8 stroke-current" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
                        </svg>
                    </div>
                    <div class="stat-title">"Wrestlers"</div>
                    <div class="stat-value text-secondary">"0"</div>
                    <div class="stat-desc">"Build your roster"</div>
                </div>

                <div class="stat">
                    <div class="stat-figure text-accent">
                        <svg class="inline-block w-8 h-8 stroke-current" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path>
                        </svg>
                    </div>
                    <div class="stat-title">"Championships"</div>
                    <div class="stat-value text-accent">"0"</div>
                    <div class="stat-desc">"Create titles"</div>
                </div>
            </div>

        </div>
    }
}
