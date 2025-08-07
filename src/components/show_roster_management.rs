use crate::components::show::roster_section::RosterSection;
use crate::components::show::wrestler_assignment_section::WrestlerAssignmentSection;
use crate::types::{assign_wrestler_to_show, fetch_shows, fetch_unassigned_wrestlers, fetch_wrestlers_for_show, fetch_shows_for_wrestler, remove_wrestler_from_show, Show, Wrestler};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Show Roster Management component for assigning wrestlers to shows
/// 
/// Features:
/// - Select show from promotion-specific dropdown
/// - View current roster for selected show
/// - Assign available wrestlers to show roster
/// - Remove wrestlers from show roster
/// - Supports cross-promotion wrestler usage (global wrestler pool)
#[component]
pub fn ShowRosterManagement(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    // State management
    let (selected_show, set_selected_show) = signal(None::<Show>);
    let (current_roster, set_current_roster) = signal(Vec::<Wrestler>::new());
    let (available_wrestlers, set_available_wrestlers) = signal(Vec::<Wrestler>::new());
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (loading, set_loading) = signal(false);
    let (shows_loading, set_shows_loading) = signal(true);
    let (status_message, set_status_message) = signal(None::<String>);
    let (error_message, set_error_message) = signal(None::<String>);
    
    // Transfer confirmation state
    let (pending_transfer, set_pending_transfer) = signal(None::<(i32, String, String)>); // (wrestler_id, wrestler_name, current_show_name)
    let (show_confirmation, set_show_confirmation) = signal(false);
    
    // Communication signals for sub-components
    let (assign_wrestler_trigger, set_assign_wrestler_trigger) = signal(None::<i32>);
    let (remove_wrestler_trigger, set_remove_wrestler_trigger) = signal(None::<i32>);
    
    // Load shows on component mount using Effect like working components
    Effect::new(move |_| {
        spawn_local(async move {
            set_shows_loading.set(true);
            match fetch_shows().await {
                Ok(data) => {
                    set_shows.set(data);
                    set_error_message.set(None);
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to load shows: {}", e)));
                }
            }
            set_shows_loading.set(false);
        });
    });
    
    // Load roster data when show selection changes
    let load_roster_data = move |show_id: i32| {
        set_loading.set(true);
        set_status_message.set(None);
        set_error_message.set(None);
        
        spawn_local(async move {
            // Fetch current roster and unassigned wrestlers concurrently
            let roster_result = fetch_wrestlers_for_show(show_id).await;
            let unassigned_result = fetch_unassigned_wrestlers().await;
            
            match (roster_result, unassigned_result) {
                (Ok(roster), Ok(unassigned_wrestlers)) => {
                    set_current_roster.set(roster);
                    set_available_wrestlers.set(unassigned_wrestlers);
                    set_loading.set(false);
                },
                (Err(e), _) | (_, Err(e)) => {
                    set_error_message.set(Some(format!("Failed to load roster data: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Handle show selection change
    let on_show_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
        let value = select.value();
        if let Ok(show_id) = value.parse::<i32>() {
            let shows_list = shows.get();
            if let Some(show) = shows_list.iter().find(|s| s.id == show_id) {
                set_selected_show.set(Some(show.clone()));
                load_roster_data(show_id);
            }
        } else {
            set_selected_show.set(None);
            set_current_roster.set(Vec::new());
            set_available_wrestlers.set(Vec::new());
        }
    };
    
    
    // Handle wrestler assignment trigger - check for transfers first
    Effect::new(move |_| {
        if let Some(wrestler_id) = assign_wrestler_trigger.get() {
            if let Some(target_show) = selected_show.get() {
                set_loading.set(true);
                set_status_message.set(None);
                set_error_message.set(None);
                
                spawn_local(async move {
                    // Get wrestler name
                    let wrestler_name = available_wrestlers.get()
                        .iter()
                        .find(|w| w.id == wrestler_id)
                        .map(|w| w.name.clone())
                        .unwrap_or_else(|| format!("Wrestler #{}", wrestler_id));
                    
                    // Check if wrestler is currently assigned to another show
                    match fetch_shows_for_wrestler(wrestler_id).await {
                        Ok(current_shows) => {
                            if let Some(current_show) = current_shows.first() {
                                if current_show.id != target_show.id {
                                    // Wrestler is on a different show - show confirmation dialog
                                    set_pending_transfer.set(Some((wrestler_id, wrestler_name.clone(), current_show.name.clone())));
                                    set_show_confirmation.set(true);
                                    set_loading.set(false);
                                    return;
                                }
                            }
                            
                            // No current assignment or already on target show - proceed with assignment
                            match assign_wrestler_to_show(target_show.id, wrestler_id).await {
                                Ok(_) => {
                                    let message = if current_shows.is_empty() {
                                        format!("{} assigned to {} successfully!", wrestler_name, target_show.name)
                                    } else {
                                        format!("{} transferred to {} successfully!", wrestler_name, target_show.name)
                                    };
                                    set_status_message.set(Some(message));
                                    load_roster_data(target_show.id); // Reload data
                                },
                                Err(e) => {
                                    set_error_message.set(Some(format!("Failed to assign {} to {}: {}", wrestler_name, target_show.name, e)));
                                    set_loading.set(false);
                                }
                            }
                        },
                        Err(e) => {
                            set_error_message.set(Some(format!("Failed to check wrestler's current assignment: {}", e)));
                            set_loading.set(false);
                        }
                    }
                });
            }
            set_assign_wrestler_trigger.set(None); // Reset trigger
        }
    });

    // Handle wrestler removal trigger
    Effect::new(move |_| {
        if let Some(wrestler_id) = remove_wrestler_trigger.get() {
            if let Some(show) = selected_show.get() {
                set_loading.set(true);
                set_status_message.set(None);
                set_error_message.set(None);
                
                spawn_local(async move {
                    // Get wrestler name for better error messages
                    let wrestler_name = current_roster.get()
                        .iter()
                        .find(|w| w.id == wrestler_id)
                        .map(|w| w.name.clone())
                        .unwrap_or_else(|| format!("Wrestler #{}", wrestler_id));
                    
                    match remove_wrestler_from_show(show.id, wrestler_id).await {
                        Ok(_) => {
                            set_status_message.set(Some(format!("{} removed from {} successfully!", wrestler_name, show.name)));
                            load_roster_data(show.id); // Reload data
                        },
                        Err(e) => {
                            set_error_message.set(Some(format!("Failed to remove {} from {}: {}", wrestler_name, show.name, e)));
                            set_loading.set(false);
                        }
                    }
                });
            }
            set_remove_wrestler_trigger.set(None); // Reset trigger
        }
    });
    
    // Handle confirmation dialog actions
    let confirm_transfer = move |_| {
        if let (Some((wrestler_id, wrestler_name, from_show)), Some(to_show)) = (pending_transfer.get(), selected_show.get()) {
            set_show_confirmation.set(false);
            set_pending_transfer.set(None);
            set_loading.set(true);
            set_status_message.set(None);
            set_error_message.set(None);
            
            spawn_local(async move {
                match assign_wrestler_to_show(to_show.id, wrestler_id).await {
                    Ok(_) => {
                        set_status_message.set(Some(format!("{} transferred from {} to {} successfully!", wrestler_name, from_show, to_show.name)));
                        load_roster_data(to_show.id); // Reload data
                    },
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to transfer {} from {} to {}: {}", wrestler_name, from_show, to_show.name, e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    };
    
    let cancel_transfer = move |_| {
        set_show_confirmation.set(false);
        set_pending_transfer.set(None);
        set_loading.set(false);
    };
    
    view! {
        <div class="space-y-8">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-3xl font-bold text-base-content mb-2">
                        "Show Roster Management"
                    </h2>
                    <p class="text-base-content/70">
                        "Manage wrestler assignments for shows"
                    </p>
                </div>
                <button
                    class="btn btn-ghost gap-2"
                    on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                    </svg>
                    "Back to Dashboard"
                </button>
            </div>
            
            // Show Selection
            <div class="card bg-base-200 shadow-xl">
                <div class="card-body">
                    <h3 class="card-title text-xl mb-4">"Select Show"</h3>
                    <div class="form-control w-full max-w-xs">
                        <label class="label">
                            <span class="label-text">"Show:"</span>
                        </label>
                        <select 
                            class="select select-bordered w-full max-w-xs"
                            on:change=on_show_change
                        >
                            <option value="" selected=move || selected_show.get().is_none()>
                                "Choose a show..."
                            </option>
                            {move || {
                                let shows_list = shows.get();
                                if shows_loading.get() {
                                    vec![view! { <option value="".to_string() selected=false>{"Loading shows...".to_string()}</option> }]
                                } else if shows_list.is_empty() {
                                    vec![view! { <option value="".to_string() selected=false>{"No shows available".to_string()}</option> }]
                                } else {
                                    shows_list.iter().map(|show| {
                                        let is_selected = selected_show.get()
                                            .map(|s| s.id == show.id)
                                            .unwrap_or(false);
                                        let id_str = show.id.to_string();
                                        let name_str = show.name.clone();
                                        
                                        view! {
                                            <option value=id_str selected=is_selected>
                                                {name_str}
                                            </option>
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            }}
                        </select>
                    </div>
                </div>
            </div>
            
            // Status and Error Messages
            <Show when=move || status_message.get().is_some() || error_message.get().is_some()>
                <div class="space-y-2">
                    <Show when=move || status_message.get().is_some()>
                        <div class="alert alert-success">
                            <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>{move || status_message.get().unwrap_or_default()}</span>
                        </div>
                    </Show>
                    <Show when=move || error_message.get().is_some()>
                        <div class="alert alert-error">
                            <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>{move || error_message.get().unwrap_or_default()}</span>
                        </div>
                    </Show>
                </div>
            </Show>
            
            // Transfer Confirmation Dialog
            <Show when=move || show_confirmation.get()>
                <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
                    <div class="card bg-base-100 shadow-2xl w-full max-w-md mx-4">
                        <div class="card-body">
                            <h3 class="card-title text-warning">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.728-.833-2.498 0L4.316 15.5c-.77.833.192 2.5 1.732 2.5z"></path>
                                </svg>
                                "Transfer Wrestler?"
                            </h3>
                            {move || {
                                if let (Some((_, wrestler_name, from_show)), Some(to_show)) = (pending_transfer.get(), selected_show.get()) {
                                    view! {
                                        <p class="py-4">
                                            {format!("{} is currently assigned to {}.", wrestler_name, from_show)}
                                            <br/>
                                            <strong>{format!("Transfer to {}?", to_show.name)}</strong>
                                        </p>
                                    }.into_any()
                                } else {
                                    view! { <p>"Loading transfer details..."</p> }.into_any()
                                }
                            }}
                            <div class="card-actions justify-end gap-2">
                                <button 
                                    class="btn btn-ghost"
                                    on:click=cancel_transfer
                                >
                                    "Cancel"
                                </button>
                                <button 
                                    class="btn btn-warning"
                                    on:click=confirm_transfer
                                >
                                    "Transfer"
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
            
            // Loading indicator
            <Show when=move || loading.get()>
                <div class="flex justify-center">
                    <span class="loading loading-spinner loading-lg"></span>
                </div>
            </Show>
            
            // Roster Management (only show when a show is selected)
            <Show when=move || selected_show.get().is_some()>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <RosterSection 
                        current_roster=current_roster.into()
                        loading=loading.into()
                        on_remove_wrestler=set_remove_wrestler_trigger
                    />
                    <WrestlerAssignmentSection 
                        available_wrestlers=available_wrestlers.into()
                        loading=loading.into()
                        on_assign_wrestler=set_assign_wrestler_trigger
                    />
                </div>
            </Show>
        </div>
    }
}