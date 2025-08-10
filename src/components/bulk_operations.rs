use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json;
use serde::{Deserialize, Serialize};
use crate::types::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Result structure for bulk operations with detailed error reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BulkOperationResult {
    operation_name: String,
    success_count: u32,
    error_count: u32,
    success_messages: Vec<String>,
    error_messages: Vec<String>,
    timestamp: String,
}

/// Advanced Bulk Operations Component
/// 
/// Executive-level bulk management capabilities:
/// - Multi-wrestler strategic selection and management
/// - Batch match creation with professional templates
/// - Roster reorganization with impact analysis
/// - Strategic data operations with rollback capabilities
#[component]
pub fn BulkOperations(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (selected_wrestlers, set_selected_wrestlers) = signal(Vec::<i32>::new());
    let (selected_shows, set_selected_shows) = signal(Vec::<i32>::new());
    let (_selected_titles, _set_selected_titles) = signal(Vec::<i32>::new());
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (_titles, _set_titles) = signal(Vec::<Title>::new());
    let (operation_status, set_operation_status) = signal("".to_string());
    let (operation_details, set_operation_details) = signal(None::<BulkOperationResult>);
    let (bulk_operation_type, set_bulk_operation_type) = signal("wrestler_management".to_string());
    let (loading, set_loading) = signal(false);
    
    // Load data on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            load_wrestlers(set_wrestlers).await;
            load_shows(set_shows).await;
        });
    });
    
    // Toggle wrestler selection
    let toggle_wrestler = move |wrestler_id: i32| {
        let mut current = selected_wrestlers.get();
        if current.contains(&wrestler_id) {
            current.retain(|&id| id != wrestler_id);
        } else {
            current.push(wrestler_id);
        }
        set_selected_wrestlers.set(current);
    };
    
    // Toggle show selection
    let toggle_show = move |show_id: i32| {
        let mut current = selected_shows.get();
        if current.contains(&show_id) {
            current.retain(|&id| id != show_id);
        } else {
            current.push(show_id);
        }
        set_selected_shows.set(current);
    };
    
    // Execute bulk wrestler assignment
    let execute_bulk_wrestler_assignment = move |_| {
        let wrestler_ids = selected_wrestlers.get();
        let show_ids = selected_shows.get();
        
        if wrestler_ids.is_empty() || show_ids.is_empty() {
            set_operation_status.set("Please select wrestlers and shows".to_string());
            return;
        }
        
        set_loading.set(true);
        spawn_local(async move {
            set_operation_status.set("Processing bulk wrestler assignments...".into());
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "wrestlerIds": wrestler_ids,
                "showIds": show_ids
            })).unwrap();
            
            let result = invoke("bulk_assign_wrestlers_to_shows", args).await;
            match serde_wasm_bindgen::from_value::<BulkOperationResult>(result) {
                Ok(bulk_result) => {
                    set_operation_details.set(Some(bulk_result.clone()));
                    
                    if bulk_result.error_count == 0 {
                        set_operation_status.set(format!(
                            "Bulk assignment complete: {} successful operations", 
                            bulk_result.success_count
                        ));
                    } else {
                        set_operation_status.set(format!(
                            "Bulk assignment complete: {} successful, {} failed - See details below", 
                            bulk_result.success_count, bulk_result.error_count
                        ));
                    }
                }
                Err(_) => {
                    set_operation_status.set("Assignment completed with unknown status".to_string());
                    set_operation_details.set(None);
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Execute bulk match creation
    let execute_bulk_match_creation = move |_| {
        let show_ids = selected_shows.get();
        
        if show_ids.is_empty() {
            set_operation_status.set("Please select shows for match creation".to_string());
            return;
        }
        
        set_loading.set(true);
        spawn_local(async move {
            set_operation_status.set("Creating bulk matches from templates...".into());
            
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({
                "showIds": show_ids,
                "templateType": "championship"
            })).unwrap();
            
            let result = invoke("bulk_create_template_matches", args).await;
            match serde_wasm_bindgen::from_value::<BulkOperationResult>(result) {
                Ok(bulk_result) => {
                    set_operation_details.set(Some(bulk_result.clone()));
                    
                    if bulk_result.error_count == 0 {
                        set_operation_status.set(format!(
                            "Created {} matches from templates successfully", 
                            bulk_result.success_count
                        ));
                    } else {
                        set_operation_status.set(format!(
                            "Match creation: {} successful, {} failed - See details below", 
                            bulk_result.success_count, bulk_result.error_count
                        ));
                    }
                }
                Err(_) => {
                    set_operation_status.set("Template matches created with unknown status".to_string());
                    set_operation_details.set(None);
                }
            }
            
            set_loading.set(false);
        });
    };
    
    // Clear all selections
    let clear_selections = move |_| {
        set_selected_wrestlers.set(Vec::new());
        set_selected_shows.set(Vec::new());
        _set_selected_titles.set(Vec::new());
        set_operation_status.set("Selections cleared".to_string());
        set_operation_details.set(None); // Clear detailed error reports
    };
    
    view! {
        <div class="space-y-8">
            // Executive Bulk Operations Header
            <div class="text-center">
                <div class="flex items-center justify-center gap-3 mb-4">
                    <div class="w-12 h-12 bg-gradient-to-br from-primary via-accent to-secondary rounded-xl flex items-center justify-center shadow-lg">
                        <svg class="w-7 h-7 text-base-100" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"/>
                        </svg>
                    </div>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-3 bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
                    "Executive Bulk Operations"
                </h1>
                <p class="text-lg text-base-content/80 max-w-3xl mx-auto leading-relaxed">
                    "Advanced bulk management capabilities for wrestling empire operations. 
                    Strategic multi-entity operations with impact analysis and rollback support."
                </p>
            </div>
            
            // Operation Type Selector
            <div class="card bg-base-100 shadow-xl border border-base-300/50">
                <div class="card-body">
                    <h2 class="text-2xl font-bold mb-4 flex items-center gap-3">
                        <div class="w-8 h-8 bg-primary/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
                            </svg>
                        </div>
                        "Strategic Operations"
                    </h2>
                    
                    <div class="tabs tabs-bordered mb-6">
                        <button 
                            class=move || if bulk_operation_type.get() == "wrestler_management" { "tab tab-active" } else { "tab" }
                            on:click=move |_| set_bulk_operation_type.set("wrestler_management".to_string())
                        >
                            "Talent Management"
                        </button>
                        <button 
                            class=move || if bulk_operation_type.get() == "match_creation" { "tab tab-active" } else { "tab" }
                            on:click=move |_| set_bulk_operation_type.set("match_creation".to_string())
                        >
                            "Match Templates"
                        </button>
                        <button 
                            class=move || if bulk_operation_type.get() == "roster_reorganization" { "tab tab-active" } else { "tab" }
                            on:click=move |_| set_bulk_operation_type.set("roster_reorganization".to_string())
                        >
                            "Roster Strategy"
                        </button>
                    </div>
                    
                    // Operation Status
                    {move || {
                        if !operation_status.get().is_empty() {
                            view! {
                                <div class="alert alert-info mb-4">
                                    <svg class="stroke-current shrink-0 w-6 h-6" fill="none" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    <span>{operation_status.get()}</span>
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                    
                    // Detailed Operation Results with Specific Error/Success Messages
                    {move || {
                        if let Some(details) = operation_details.get() {
                            view! {
                                <div class="space-y-4 mb-6">
                                    // Success Messages
                                    {if !details.success_messages.is_empty() {
                                        view! {
                                            <div class="collapse collapse-arrow bg-success/10 border border-success/20">
                                                <input type="checkbox" class="peer" />
                                                <div class="collapse-title text-success font-medium flex items-center gap-2">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                    </svg>
                                                    {format!("Successful Operations ({})", details.success_count)}
                                                </div>
                                                <div class="collapse-content">
                                                    <div class="space-y-1 pt-2">
                                                        <For
                                                            each=move || details.success_messages.clone()
                                                            key=|msg| msg.clone()
                                                            children=move |message| {
                                                                view! {
                                                                    <div class="text-sm text-success/80 bg-success/5 p-2 rounded border-l-4 border-success/30">
                                                                        "✓ "{message}
                                                                    </div>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                    
                                    // Error Messages
                                    {if !details.error_messages.is_empty() {
                                        view! {
                                            <div class="collapse collapse-arrow bg-error/10 border border-error/20">
                                                <input type="checkbox" class="peer" checked />
                                                <div class="collapse-title text-error font-medium flex items-center gap-2">
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                    </svg>
                                                    {format!("Failed Operations ({})", details.error_count)}
                                                </div>
                                                <div class="collapse-content">
                                                    <div class="space-y-1 pt-2">
                                                        <For
                                                            each=move || details.error_messages.clone()
                                                            key=|msg| msg.clone()
                                                            children=move |message| {
                                                                view! {
                                                                    <div class="text-sm text-error/80 bg-error/5 p-2 rounded border-l-4 border-error/30">
                                                                        "✗ "{message}
                                                                    </div>
                                                                }
                                                            }
                                                        />
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
            </div>
            
            // Dynamic Content Based on Selected Operation
            {move || {
                match bulk_operation_type.get().as_str() {
                    "wrestler_management" => view! {
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            // Wrestler Selection Panel
                            <div class="card bg-base-100 shadow-xl border border-base-300/50">
                                <div class="card-body">
                                    <div class="flex items-center justify-between mb-4">
                                        <h3 class="text-xl font-bold flex items-center gap-2">
                                            <svg class="w-5 h-5 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                            </svg>
                                            "Select Talent"
                                        </h3>
                                        <div class="badge badge-secondary">
                                            {move || selected_wrestlers.get().len()}
                                        </div>
                                    </div>
                                    
                                    <div class="space-y-2 max-h-80 overflow-y-auto">
                                        <For
                                            each=move || wrestlers.get()
                                            key=|wrestler| wrestler.id
                                            children=move |wrestler| {
                                                let wrestler_id = wrestler.id;
                                                let is_selected = move || selected_wrestlers.get().contains(&wrestler_id);
                                                
                                                view! {
                                                    <div class="form-control">
                                                        <label class="cursor-pointer label justify-start gap-3 hover:bg-base-200/50 rounded-lg p-2">
                                                            <input 
                                                                type="checkbox" 
                                                                class="checkbox checkbox-primary"
                                                                checked=is_selected
                                                                on:change=move |_| toggle_wrestler(wrestler_id)
                                                            />
                                                            <div class="flex items-center gap-3">
                                                                <div class="w-10 h-10 bg-secondary/20 rounded-lg flex items-center justify-center">
                                                                    <span class="text-secondary font-bold text-sm">
                                                                        {wrestler.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                                                                    </span>
                                                                </div>
                                                                <div>
                                                                    <div class="font-medium">{wrestler.name.clone()}</div>
                                                                    <div class="text-sm text-base-content/60">
                                                                        {format!("{}-{} record", wrestler.wins, wrestler.losses)}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </label>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </div>
                            
                            // Show Selection Panel
                            <div class="card bg-base-100 shadow-xl border border-base-300/50">
                                <div class="card-body">
                                    <div class="flex items-center justify-between mb-4">
                                        <h3 class="text-xl font-bold flex items-center gap-2">
                                            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                            </svg>
                                            "Select Shows"
                                        </h3>
                                        <div class="badge badge-primary">
                                            {move || selected_shows.get().len()}
                                        </div>
                                    </div>
                                    
                                    <div class="space-y-2 max-h-80 overflow-y-auto">
                                        <For
                                            each=move || shows.get()
                                            key=|show| show.id
                                            children=move |show| {
                                                let show_id = show.id;
                                                let is_selected = move || selected_shows.get().contains(&show_id);
                                                
                                                view! {
                                                    <div class="form-control">
                                                        <label class="cursor-pointer label justify-start gap-3 hover:bg-base-200/50 rounded-lg p-2">
                                                            <input 
                                                                type="checkbox" 
                                                                class="checkbox checkbox-primary"
                                                                checked=is_selected
                                                                on:change=move |_| toggle_show(show_id)
                                                            />
                                                            <div class="flex items-center gap-3">
                                                                <div class="w-10 h-10 bg-primary/20 rounded-lg flex items-center justify-center">
                                                                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                                                    </svg>
                                                                </div>
                                                                <div>
                                                                    <div class="font-medium">{show.name.clone()}</div>
                                                                    <div class="text-sm text-base-content/60">
                                                                        {show.description.clone()}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </label>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        // Action Buttons
                        <div class="card bg-gradient-to-br from-base-100 to-base-200/50 shadow-xl border border-base-300/50">
                            <div class="card-body">
                                <div class="flex items-center justify-between gap-4 flex-wrap">
                                    <div class="flex gap-3">
                                        <button 
                                            class="btn btn-primary gap-2"
                                            disabled=loading.get()
                                            on:click=execute_bulk_wrestler_assignment
                                        >
                                            {move || if loading.get() { 
                                                view! { <span class="loading loading-spinner loading-sm"></span> }.into_any()
                                            } else {
                                                view! { 
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/>
                                                    </svg>
                                                }.into_any()
                                            }}
                                            "Execute Bulk Assignment"
                                        </button>
                                        <button 
                                            class="btn btn-secondary gap-2"
                                            on:click=clear_selections
                                        >
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                            </svg>
                                            "Clear Selections"
                                        </button>
                                    </div>
                                    <div class="stats stats-vertical lg:stats-horizontal shadow">
                                        <div class="stat">
                                            <div class="stat-title text-xs">"Selected Wrestlers"</div>
                                            <div class="stat-value text-lg text-secondary">{move || selected_wrestlers.get().len()}</div>
                                        </div>
                                        <div class="stat">
                                            <div class="stat-title text-xs">"Target Shows"</div>
                                            <div class="stat-value text-lg text-primary">{move || selected_shows.get().len()}</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any(),
                    
                    "match_creation" => view! {
                        <div class="card bg-base-100 shadow-xl border border-base-300/50">
                            <div class="card-body">
                                <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                                    <div class="w-8 h-8 bg-info/20 rounded-lg flex items-center justify-center">
                                        <svg class="w-5 h-5 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                        </svg>
                                    </div>
                                    "Bulk Match Creation Templates"
                                </h3>
                                
                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                    <div class="card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20">
                                        <div class="card-body">
                                            <h4 class="font-bold flex items-center gap-2">
                                                <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                                </svg>
                                                "Championship Night"
                                            </h4>
                                            <p class="text-sm text-base-content/70 mb-4">
                                                "Create a full championship card with title matches"
                                            </p>
                                            <button 
                                                class="btn btn-primary btn-sm w-full"
                                                disabled=loading.get()
                                                on:click=execute_bulk_match_creation
                                            >
                                                "Generate Championship Card"
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20">
                                        <div class="card-body">
                                            <h4 class="font-bold flex items-center gap-2">
                                                <svg class="w-5 h-5 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                                </svg>
                                                "Tournament Bracket"
                                            </h4>
                                            <p class="text-sm text-base-content/70 mb-4">
                                                "Auto-generate tournament matches with brackets"
                                            </p>
                                            <button class="btn btn-secondary btn-sm w-full">
                                                "Create Tournament"
                                            </button>
                                        </div>
                                    </div>
                                    
                                    <div class="card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20">
                                        <div class="card-body">
                                            <h4 class="font-bold flex items-center gap-2">
                                                <svg class="w-5 h-5 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                                </svg>
                                                "Rivalry Showcase"
                                            </h4>
                                            <p class="text-sm text-base-content/70 mb-4">
                                                "Generate matches based on storyline rivalries"
                                            </p>
                                            <button class="btn btn-accent btn-sm w-full">
                                                "Build Rivalries"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any(),
                    
                    _ => view! {
                        <div class="card bg-base-100 shadow-xl border border-base-300/50">
                            <div class="card-body">
                                <h3 class="text-2xl font-bold mb-4">"Strategic Roster Reorganization"</h3>
                                <div class="alert alert-info">
                                    <svg class="stroke-current shrink-0 w-6 h-6" fill="none" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    <span>"Advanced roster reorganization tools coming soon..."</span>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }
            }}
            
            // Navigation
            <div class="text-center">
                <button 
                    class="btn btn-neutral gap-2"
                    on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
                    </svg>
                    "Return to Executive Dashboard"
                </button>
            </div>
        </div>
    }
}


// Helper functions for loading data
async fn load_wrestlers(set_wrestlers: WriteSignal<Vec<Wrestler>>) {
    let result = invoke("get_wrestlers", serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap()).await;
    if let Ok(wrestlers) = serde_wasm_bindgen::from_value::<Vec<Wrestler>>(result) {
        set_wrestlers.set(wrestlers);
    } else {
        web_sys::console::error_1(&"Failed to load wrestlers".into());
        set_wrestlers.set(Vec::new());
    }
}

async fn load_shows(set_shows: WriteSignal<Vec<Show>>) {
    let result = invoke("get_shows", serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap()).await;
    if let Ok(shows) = serde_wasm_bindgen::from_value::<Vec<Show>>(result) {
        set_shows.set(shows);
    } else {
        web_sys::console::error_1(&"Failed to load shows".into());
        set_shows.set(Vec::new());
    }
}

