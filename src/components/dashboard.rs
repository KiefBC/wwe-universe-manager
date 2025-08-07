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

/// GM Dashboard component 
/// 
/// Main dashboard for wrestling management:
/// - Create shows, wrestlers, and championships
/// - Access show rosters and match booking
/// - Central hub for all wrestling management activities
#[component]
pub fn PromotionDashboard(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
    /// Signal that triggers data refresh when incremented
    refresh_trigger: ReadSignal<u32>,
) -> impl IntoView {
    // Reactive resource that fetches all shows from the backend
    // Automatically refreshes when refresh_trigger signal changes
    let _shows_resource = LocalResource::new(move || {
        let _trigger = refresh_trigger.get(); // This makes the resource reactive to refresh_trigger
        fetch_shows()
    });
    
    // Event handler to navigate to the wrestlers list page
    // Arguments: _ (Mouse event - unused)
    let navigate_to_wrestlers = move |_| {
        set_current_page.set("wrestlers".to_string());
    };
    
    // Event handler to create comprehensive test data
    // Creates:
    // - 5 wrestlers with detailed profiles
    // - 2 shows (RAW and SmackDown)
    // - 15 championship titles
    // - Sample matches and rosters
    // Arguments: _ (Mouse event - unused)
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


    view! {
        <div class="space-y-8">
            <div class="text-center mb-8">
                <h2 class="text-3xl font-bold text-base-content mb-2">
                    "GM Dashboard"
                </h2>
                <p class="text-base-content/70">
                    "Manage shows, wrestlers, and championships"
                </p>
            </div>


            <div class="grid grid-cols-1 md:grid-cols-6 gap-4">
                <button
                    class="btn btn-primary gap-2"
                    on:click=move |_| set_current_page.set("create-show".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                    </svg>
                    "Create Show"
                </button>
                <button 
                    class="btn btn-secondary gap-2"
                    on:click=move |_| set_current_page.set("show-roster".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                    </svg>
                    "Show Rosters"
                </button>
                <button 
                    class="btn btn-secondary/80 gap-2"
                    on:click=navigate_to_wrestlers
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                    </svg>
                    "Wrestlers"
                </button>
                <button 
                    class="btn btn-accent gap-2"
                    on:click=move |_| set_current_page.set("titles".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z" />
                    </svg>
                    "Titles"
                </button>
                <button 
                    class="btn btn-info gap-2"
                    on:click=move |_| set_current_page.set("booker".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 0V3a2 2 0 00-2-2V1a2 2 0 00-2 2v2H9z M8 5h2v6H8V5z" />
                    </svg>
                    "Booker"
                </button>
                <button 
                    class="btn btn-warning gap-2"
                    on:click=create_test_data
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
                    </svg>
                    "Test Data"
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
