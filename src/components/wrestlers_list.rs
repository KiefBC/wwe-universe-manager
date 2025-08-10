use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::types::Gender;

/// Wrestler data structure for frontend display
/// 
/// This mirrors the backend Wrestler model but is kept separate
/// for frontend-specific serialization needs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wrestler {
    pub id: i32,
    pub name: String,
    pub gender: Gender,
    pub wins: i32,
    pub losses: i32,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
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

/// Fetches all wrestlers from the backend via Tauri command
/// 
/// # Returns
/// * `Ok(Vec<Wrestler>)` - List of all wrestlers
/// * `Err(String)` - Error message if fetch fails
async fn get_wrestlers() -> Result<Vec<Wrestler>, String> {
    let result = invoke("get_wrestlers", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

/// Opens a separate window to display wrestler details
/// 
/// # Arguments
/// * `wrestler_id` - ID of the wrestler to display
/// 
/// # Returns
/// * `Ok(())` - Window opened successfully
/// * `Err(String)` - Error message if window creation fails
async fn open_wrestler_window(wrestler_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id
    }))
    .map_err(|e| e.to_string())?;

    let _result = invoke("open_wrestler_window", args).await;
    Ok(())
}

/// Wrestlers list component with search and filtering
/// 
/// Displays all wrestlers in a responsive grid with:
/// - Search functionality (debounced)
/// - Power rating displays
/// - User-created wrestler indicators
/// - Click to open detail windows
/// 
/// # Props
/// * `set_current_page` - Signal to change the current page/route
#[component]
pub fn WrestlersList(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (search_term, set_search_term) = signal(String::new());
    let (debounced_search_term, set_debounced_search_term) = signal(String::new());
    let (view_mode, set_view_mode) = signal("cards".to_string()); // "cards" or "rows"

    // Effect that loads wrestlers when component mounts
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

    // Effect that debounces search input with 300ms delay
    // Prevents excessive filtering while user is typing
    Effect::new(move |_| {
        let current_term = search_term.get();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            if search_term.get_untracked() == current_term {
                set_debounced_search_term.set(current_term);
            }
        });
    });

    // Computed function that filters wrestlers based on search term
    // Searches in name, nickname, and real name fields
    let filtered_wrestlers = move || {
        let term = debounced_search_term.get().to_lowercase();
        if term.is_empty() {
            wrestlers.get()
        } else {
            wrestlers.get().into_iter()
                .filter(|w| {
                    w.name.to_lowercase().contains(&term) || 
                    w.nickname.as_ref().map_or(false, |n| n.to_lowercase().contains(&term)) ||
                    w.real_name.as_ref().map_or(false, |r| r.to_lowercase().contains(&term))
                })
                .collect()
        }
    };


    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-primary/10 via-accent/10 to-secondary/10 rounded-none border-b border-primary/20 mb-6 sm:mb-8">
                <div class="hero-content text-center py-4 sm:py-6">
                    <div class="max-w-6xl w-full">
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent mb-6">
                            "Wrestling Management System (WMS)"
                        </h1>
                        
                        // Action buttons row - mobile responsive
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 px-4 mt-2">
                            <button
                                class="btn btn-primary gap-2 w-full sm:w-auto min-h-[44px]"
                                on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                </svg>
                                "Back to Command Hub"
                            </button>
                            <button
                                class="btn btn-secondary gap-2 w-full sm:w-auto min-h-[44px]"
                                on:click=move |_| set_current_page.set("create-wrestler".to_string())
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                </svg>
                                "Create New Talent"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="px-4 sm:px-6 lg:px-8 pb-8 sm:pb-12">

                // Professional Search & Analytics Section
                <section class="mb-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-6 gap-4">
                            <div>
                                <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Talent Management"</h2>
                                <p class="text-base-content/70 text-sm sm:text-base">"Search and manage your global wrestling roster"</p>
                            </div>
                            
                            <Show when=move || !loading.get() && error.get().is_none() && !wrestlers.get().is_empty()>
                                <div class="stats stats-horizontal bg-base-200 shadow-lg border border-base-300/50 w-full sm:w-auto">
                                    <div class="stat px-3 sm:px-6 py-2">
                                        <div class="stat-title text-xs">Total Roster</div>
                                        <div class="stat-value text-lg text-primary">{move || filtered_wrestlers().len()}</div>
                                        <div class="stat-desc text-xs">
                                            {move || {
                                                let search_value = search_term.get();
                                                if search_value.is_empty() {
                                                    "Global talent pool".to_string()
                                                } else {
                                                    format!("Matching \"{}\"", search_value)
                                                }
                                            }}
                                        </div>
                                    </div>
                                </div>
                            </Show>
                        </div>
                        
                        // Enhanced Search Interface
                        <Show when=move || !loading.get() && error.get().is_none() && !wrestlers.get().is_empty()>
                            <div class="card bg-base-200/50 border border-base-300/30 mb-8">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex items-center justify-between mb-4">
                                        <div class="flex items-center gap-3">
                                            <div class="w-10 h-10 bg-info/20 rounded-xl flex items-center justify-center">
                                                <svg class="w-6 h-6 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3 class="text-lg font-bold text-base-content">"Search Talent"</h3>
                                                <p class="text-base-content/70 text-sm">"Find wrestlers by name, nickname, or real name"</p>
                                            </div>
                                        </div>
                                        
                                        // View Toggle Buttons
                                        <div class="flex items-center gap-2">
                                            <div class="join">
                                                <button 
                                                    class=move || format!("btn btn-sm join-item min-h-[36px] {}", 
                                                        if view_mode.get() == "cards" { "btn-primary" } else { "btn-ghost" })
                                                    on:click=move |_| set_view_mode.set("cards".to_string())
                                                    title="Card View"
                                                >
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                                                    </svg>
                                                </button>
                                                <button 
                                                    class=move || format!("btn btn-sm join-item min-h-[36px] {}", 
                                                        if view_mode.get() == "rows" { "btn-primary" } else { "btn-ghost" })
                                                    on:click=move |_| set_view_mode.set("rows".to_string())
                                                    title="List View"
                                                >
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"/>
                                                    </svg>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    <div class="form-control">
                                        <div class="relative">
                                            <input 
                                                type="text"
                                                placeholder="Search wrestlers by name, nickname, or real name..."
                                                class="input input-bordered w-full pl-12 pr-12 min-h-[44px]"
                                                prop:value=move || search_term.get()
                                                on:input=move |ev| {
                                                    let value = event_target_value(&ev);
                                                    set_search_term.set(value);
                                                }
                                            />
                                            <svg class="w-5 h-5 absolute left-4 top-1/2 transform -translate-y-1/2 text-base-content/50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                            </svg>
                                            <Show when=move || !search_term.get().is_empty()>
                                                <button 
                                                    class="btn btn-ghost btn-sm absolute right-2 top-1/2 transform -translate-y-1/2 min-h-[32px] h-8 w-8 p-0"
                                                    on:click=move |_| set_search_term.set(String::new())
                                                >
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                                    </svg>
                                                </button>
                                            </Show>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </Show>
                    </div>
                </section>

                // Loading State
                <Show when=move || loading.get()>
                    <div class="max-w-4xl mx-auto">
                        <div class="card bg-base-200/50 border border-base-300/30">
                            <div class="card-body p-8 sm:p-12 text-center">
                                <div class="flex flex-col items-center gap-4">
                                    <span class="loading loading-spinner loading-lg text-primary"></span>
                                    <div>
                                        <h3 class="text-lg font-bold text-base-content mb-1">"Loading Global Roster"</h3>
                                        <p class="text-base-content/70 text-sm">"Gathering talent data from all promotions..."</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </Show>

                // Error State
                <Show when=move || error.get().is_some()>
                    <div class="max-w-4xl mx-auto">
                        <div class="card bg-error/10 border border-error/30">
                            <div class="card-body p-6 sm:p-8">
                                <div class="flex items-center gap-4 mb-4">
                                    <div class="w-12 h-12 bg-error/20 rounded-xl flex items-center justify-center">
                                        <svg class="w-7 h-7 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="text-lg font-bold text-error mb-1">"Error Loading Wrestlers"</h3>
                                        <p class="text-error/80 text-sm">{move || error.get().unwrap_or_default()}</p>
                                    </div>
                                </div>
                                <div class="flex justify-end">
                                    <button class="btn btn-error btn-sm" on:click=move |_| {
                                        // Trigger a reload - you could implement a reload function here
                                        window().unwrap().location().reload().unwrap();
                                    }>
                                        "Retry"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </Show>

                // Empty Database State  
                <Show when=move || !loading.get() && error.get().is_none() && wrestlers.get().is_empty()>
                    <div class="max-w-4xl mx-auto">
                        <div class="card bg-info/10 border border-info/30">
                            <div class="card-body p-8 sm:p-12 text-center">
                                <div class="flex flex-col items-center gap-6">
                                    <div class="w-20 h-20 bg-info/20 rounded-2xl flex items-center justify-center">
                                        <svg class="w-12 h-12 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-bold text-base-content mb-2">"No Wrestlers Found"</h3>
                                        <p class="text-base-content/70 max-w-md">"Your roster is empty. Use the Test Data button on the dashboard to add sample wrestlers, or create your own custom talent."</p>
                                    </div>
                                    <div class="flex flex-col sm:flex-row gap-3">
                                        <button 
                                            class="btn btn-primary gap-2"
                                            on:click=move |_| set_current_page.set("create-wrestler".to_string())
                                        >
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                            </svg>
                                            "Create First Wrestler"
                                        </button>
                                        <button 
                                            class="btn btn-info gap-2"
                                            on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                                        >
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"/>
                                            </svg>
                                            "Load Test Data"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </Show>

                // Empty Search Results
                <Show when=move || !loading.get() && error.get().is_none() && !wrestlers.get().is_empty() && filtered_wrestlers().is_empty()>
                    <div class="max-w-4xl mx-auto">
                        <div class="card bg-warning/10 border border-warning/30">
                            <div class="card-body p-6 sm:p-8 text-center">
                                <div class="flex flex-col items-center gap-4">
                                    <div class="w-16 h-16 bg-warning/20 rounded-xl flex items-center justify-center">
                                        <svg class="w-8 h-8 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="text-lg font-bold text-base-content mb-1">"No Results Found"</h3>
                                        <p class="text-base-content/70 text-sm">
                                            {format!("No wrestlers found matching \"{}\"", search_term.get())}
                                        </p>
                                    </div>
                                    <button 
                                        class="btn btn-warning btn-sm gap-2"
                                        on:click=move |_| set_search_term.set(String::new())
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                        </svg>
                                        "Clear Search"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </Show>

                // Professional Wrestler Cards Grid
                <Show when=move || !loading.get() && error.get().is_none() && !filtered_wrestlers().is_empty()>
                    <section>
                        <div class="max-w-6xl mx-auto">
                            <div class="mb-6">
                                <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Active Roster"</h2>
                                <p class="text-base-content/70 text-sm sm:text-base">"Click any wrestler to view their detailed profile and analytics"</p>
                            </div>
                            
                            // Dynamic View Layout - Cards or Rows
                            {move || {
                                if view_mode.get() == "rows" {
                                    // Compact Row View
                                    view! {
                                        <div class="space-y-2">
                                            <div class="hidden sm:grid sm:grid-cols-12 gap-4 px-4 py-2 text-sm font-medium text-base-content/60 border-b border-base-300/30">
                                                <div class="col-span-1">"Type"</div>
                                                <div class="col-span-4">"Name"</div>
                                                <div class="col-span-2">"Win-Loss"</div>
                                                <div class="col-span-2">"Win Rate"</div>
                                                <div class="col-span-2">"Gender"</div>
                                                <div class="col-span-1">"Action"</div>
                                            </div>
                                            <For
                                                each=move || filtered_wrestlers()
                                                key=|wrestler| wrestler.id
                                                children=move |wrestler| {
                                                    let wrestler_id = wrestler.id;
                                                    let is_user_created = wrestler.is_user_created.unwrap_or(false);
                                                    let total_matches = wrestler.wins + wrestler.losses;
                                                    let win_rate = if total_matches > 0 {
                                                        (wrestler.wins as f64 / total_matches as f64) * 100.0
                                                    } else {
                                                        0.0
                                                    };

                                                    view! {
                                                        <div 
                                                            class="grid grid-cols-1 sm:grid-cols-12 gap-2 sm:gap-4 px-3 sm:px-4 py-3 bg-base-100 hover:bg-base-200/70 rounded-lg border border-base-300/30 hover:border-primary/40 cursor-pointer transition-all duration-200 group"
                                                            on:click=move |_| {
                                                                let wrestler_id = wrestler_id;
                                                                spawn_local(async move {
                                                                    if let Err(e) = open_wrestler_window(wrestler_id.to_string()).await {
                                                                        web_sys::console::error_1(&format!("Failed to open wrestler window: {}", e).into());
                                                                    }
                                                                });
                                                            }
                                                        >
                                                            // Mobile layout - stacked
                                                            <div class="sm:hidden space-y-2">
                                                                <div class="flex items-center justify-between">
                                                                    <div class="flex items-center gap-2">
                                                                        <div class=format!("badge badge-xs {}", 
                                                                            if is_user_created { "badge-success" } else { "badge-neutral" })>
                                                                            {if is_user_created { "Custom" } else { "System" }}
                                                                        </div>
                                                                        <span class="font-medium text-base-content">{wrestler.name.clone()}</span>
                                                                    </div>
                                                                    <svg class="w-4 h-4 text-base-content/40 group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                                                    </svg>
                                                                </div>
                                                                <div class="flex items-center justify-between text-sm text-base-content/60">
                                                                    <span>{format!("{}-{} ({:.1}%)", wrestler.wins, wrestler.losses, win_rate)}</span>
                                                                    <span>{format!("{:?}", wrestler.gender)}</span>
                                                                </div>
                                                            </div>
                                                            
                                                            // Desktop layout - grid columns
                                                            <div class="hidden sm:contents">
                                                                <div class="col-span-1 flex items-center">
                                                                    <div class=format!("badge badge-xs {}", 
                                                                        if is_user_created { "badge-success" } else { "badge-neutral" })>
                                                                        {if is_user_created { "Custom" } else { "System" }}
                                                                    </div>
                                                                </div>
                                                                <div class="col-span-4 flex items-center">
                                                                    <span class="font-medium text-base-content group-hover:text-primary transition-colors">{wrestler.name.clone()}</span>
                                                                </div>
                                                                <div class="col-span-2 flex items-center text-sm text-base-content/70">
                                                                    <span>{format!("{}-{}", wrestler.wins, wrestler.losses)}</span>
                                                                </div>
                                                                <div class="col-span-2 flex items-center text-sm">
                                                                    <span class=format!("font-medium {}", 
                                                                        if win_rate >= 70.0 { "text-success" } 
                                                                        else if win_rate >= 50.0 { "text-primary" } 
                                                                        else { "text-base-content/60" })>
                                                                        {format!("{:.1}%", win_rate)}
                                                                    </span>
                                                                </div>
                                                                <div class="col-span-2 flex items-center text-sm text-base-content/70">
                                                                    <span>{format!("{:?}", wrestler.gender)}</span>
                                                                </div>
                                                                <div class="col-span-1 flex items-center justify-center">
                                                                    <svg class="w-4 h-4 text-base-content/40 group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                                                    </svg>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    }.into_any()
                                } else {
                                    // Original Card View
                                    view! {
                                        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4 sm:gap-6">
                                            <For
                                                each=move || filtered_wrestlers()
                                                key=|wrestler| wrestler.id
                                                children=move |wrestler| {
                                                    let wrestler_id = wrestler.id;
                                                    let is_user_created = wrestler.is_user_created.unwrap_or(false);

                                                    view! {
                                                        <div class="card bg-gradient-to-br from-base-100 to-base-200/50 hover:from-primary/5 hover:to-accent/5 border border-base-300/30 hover:border-primary/40 shadow-lg hover:shadow-xl cursor-pointer group transition-all duration-200 min-h-[120px] sm:min-h-[140px]"
                                                             on:click=move |_| {
                                                                let wrestler_id = wrestler_id;
                                                                spawn_local(async move {
                                                                    if let Err(e) = open_wrestler_window(wrestler_id.to_string()).await {
                                                                        web_sys::console::error_1(&format!("Failed to open wrestler window: {}", e).into());
                                                                    }
                                                                });
                                                             }>
                                                <div class="card-body p-3 sm:p-4 flex flex-col justify-between h-full">
                                                    // Header with badge
                                                    <div class="flex items-start justify-between mb-2">
                                                        <div class="flex-1 min-w-0">
                                                            <div class="badge badge-sm text-xs mb-2 {move || if is_user_created { \"badge-success\" } else { \"badge-neutral\" }}">
                                                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                                    {move || {
                                                                        if is_user_created {
                                                                            view! {
                                                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                                            }.into_any()
                                                                        } else {
                                                                            view! {
                                                                                <path fill-rule="evenodd" d="M18 8a6 6 0 01-7.743 5.743L10 14l-1 1-1 1H6v2H2v-4l4.257-4.257A6 6 0 1118 8zm-6-4a1 1 0 100 2 2 2 0 012 2 1 1 0 102 0 4 4 0 00-4-4z" clip-rule="evenodd" />
                                                                            }.into_any()
                                                                        }
                                                                    }}
                                                                </svg>
                                                                <span class="text-xs">{move || if is_user_created { "Custom" } else { "System" }}</span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    // Wrestler avatar/icon
                                                    <div class="flex-1 flex items-center justify-center mb-3">
                                                        <div class="w-12 h-12 sm:w-16 sm:h-16 bg-primary/20 rounded-2xl flex items-center justify-center group-hover:bg-primary/30 transition-colors">
                                                            <svg class="w-6 h-6 sm:w-8 sm:h-8 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                                            </svg>
                                                        </div>
                                                    </div>
                                                    
                                                    // Wrestler name and info
                                                    <div class="text-center">
                                                        <h3 class="text-sm sm:text-base font-bold text-base-content group-hover:text-primary transition-colors mb-1 line-clamp-2 leading-tight">
                                                            {wrestler.name.clone()}
                                                        </h3>
                                                        
                                                        // Win-loss record
                                                        <div class="text-xs text-base-content/60 group-hover:text-primary/70 transition-colors">
                                                            {format!("{}-{}", wrestler.wins, wrestler.losses)}
                                                        </div>
                                                    </div>
                                                    
                                                    // Bottom action indicator
                                                    <div class="flex justify-center pt-2 border-t border-base-300/30 group-hover:border-primary/20 transition-colors mt-2">
                                                        <svg class="w-4 h-4 text-base-content/40 group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                                        </svg>
                                                    </div>
                                                </div>
                                            </div>
                                                        }
                                                    }
                                                />
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </section>
                </Show>
            </div>
        </div>
    }
}