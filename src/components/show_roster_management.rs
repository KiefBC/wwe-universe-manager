use crate::types::{assign_wrestler_to_show, fetch_shows, fetch_wrestlers, fetch_wrestlers_for_show, remove_wrestler_from_show, Promotion, Show, Wrestler};
use leptos::prelude::*;
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
    selected_promotion: ReadSignal<Option<Promotion>>,
) -> impl IntoView {
    // State management
    let (selected_show, set_selected_show) = signal(None::<Show>);
    let (current_roster, set_current_roster) = signal(Vec::<Wrestler>::new());
    let (available_wrestlers, set_available_wrestlers) = signal(Vec::<Wrestler>::new());
    let (loading, set_loading) = signal(false);
    let (status_message, set_status_message) = signal(None::<String>);
    let (error_message, set_error_message) = signal(None::<String>);
    
    // Fetch shows for the selected promotion
    let shows_resource = LocalResource::new(move || {
        async move {
            fetch_shows().await // TODO: Filter by promotion when backend supports it
        }
    });
    
    // Load roster data when show selection changes
    let load_roster_data = move |show_id: i32| {
        set_loading.set(true);
        set_status_message.set(None);
        set_error_message.set(None);
        
        spawn_local(async move {
            // Fetch current roster and all wrestlers concurrently
            let roster_result = fetch_wrestlers_for_show(show_id).await;
            let all_wrestlers_result = fetch_wrestlers().await;
            
            match (roster_result, all_wrestlers_result) {
                (Ok(roster), Ok(all_wrestlers)) => {
                    set_current_roster.set(roster.clone());
                    
                    // Filter out wrestlers already in the roster
                    let roster_ids: std::collections::HashSet<i32> = roster.iter().map(|w| w.id).collect();
                    let available: Vec<Wrestler> = all_wrestlers
                        .into_iter()
                        .filter(|w| !roster_ids.contains(&w.id))
                        .collect();
                    
                    set_available_wrestlers.set(available);
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
    let on_show_change = move |event| {
        let value = event_target_value(&event);
        if let Ok(show_id) = value.parse::<i32>() {
            if let Some(shows_result) = shows_resource.get() {
                if let Ok(shows) = shows_result.as_ref() {
                    if let Some(show) = shows.iter().find(|s| s.id == show_id) {
                        set_selected_show.set(Some(show.clone()));
                        load_roster_data(show_id);
                    }
                }
            }
        } else {
            set_selected_show.set(None);
            set_current_roster.set(Vec::new());
            set_available_wrestlers.set(Vec::new());
        }
    };
    
    // Assign wrestler to show
    let assign_wrestler = move |wrestler_id: i32| {
        if let Some(show) = selected_show.get() {
            set_loading.set(true);
            set_status_message.set(None);
            set_error_message.set(None);
            
            spawn_local(async move {
                match assign_wrestler_to_show(show.id, wrestler_id).await {
                    Ok(_) => {
                        set_status_message.set(Some("Wrestler assigned successfully!".to_string()));
                        load_roster_data(show.id); // Reload data
                    },
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to assign wrestler: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    };
    
    // Remove wrestler from show
    let remove_wrestler = move |wrestler_id: i32| {
        if let Some(show) = selected_show.get() {
            set_loading.set(true);
            set_status_message.set(None);
            set_error_message.set(None);
            
            spawn_local(async move {
                match remove_wrestler_from_show(show.id, wrestler_id).await {
                    Ok(_) => {
                        set_status_message.set(Some("Wrestler removed successfully!".to_string()));
                        load_roster_data(show.id); // Reload data
                    },
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to remove wrestler: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
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
                        {move || {
                            if let Some(promotion) = selected_promotion.get() {
                                format!("Manage wrestler assignments for {} shows", promotion.name)
                            } else {
                                "Manage wrestler assignments for shows".to_string()
                            }
                        }}
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
                            <option value="" selected=selected_show.get().is_none()>
                                "Choose a show..."
                            </option>
                            <Suspense fallback=move || view! { <option>"Loading shows..."</option> }>
                                {move || {
                                    if let Some(shows_result) = shows_resource.get() {
                                        if let Ok(shows) = shows_result.as_ref() {
                                            shows.iter().map(|show| {
                                                let is_selected = selected_show.get()
                                                    .map(|s| s.id == show.id)
                                                    .unwrap_or(false);
                                                let id_str = show.id.to_string();
                                                let name_str = show.name.clone();
                                                
                                                view! {
                                                    <option value=id_str selected=is_selected>
                                                        {name_str}
                                                    </option>
                                                }.into_any()
                                            }).collect::<Vec<_>>()
                                        } else {
                                            vec![view! { <option>"Error loading shows"</option> }.into_any()]
                                        }
                                    } else {
                                        vec![view! { <option>"Loading..."</option> }.into_any()]
                                    }
                                }}
                            </Suspense>
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
            
            // Loading indicator
            <Show when=move || loading.get()>
                <div class="flex justify-center">
                    <span class="loading loading-spinner loading-lg"></span>
                </div>
            </Show>
            
            // Roster Management (only show when a show is selected)
            <Show when=move || selected_show.get().is_some()>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    // Current Roster
                    <div class="card bg-base-200 shadow-xl">
                        <div class="card-body">
                            <h3 class="card-title text-xl mb-4">
                                "Current Roster"
                                <div class="badge badge-primary">{move || current_roster.get().len()}</div>
                            </h3>
                            <div class="space-y-2 max-h-96 overflow-y-auto">
                                <For
                                    each=move || current_roster.get()
                                    key=|wrestler| wrestler.id
                                    children=move |wrestler: Wrestler| {
                                        let wrestler_id = wrestler.id;
                                        view! {
                                            <div class="flex items-center justify-between p-3 bg-base-100 rounded-lg">
                                                <div class="flex items-center space-x-3">
                                                    <div class="avatar placeholder">
                                                        <div class="bg-neutral text-neutral-content rounded-full w-10">
                                                            <span class="text-sm">{wrestler.name.chars().next().unwrap_or('?').to_string()}</span>
                                                        </div>
                                                    </div>
                                                    <div>
                                                        <p class="font-medium">{wrestler.name}</p>
                                                        <p class="text-sm text-base-content/70">
                                                            {format!("{} • {}-{}", wrestler.gender, wrestler.wins, wrestler.losses)}
                                                        </p>
                                                    </div>
                                                </div>
                                                <button
                                                    class="btn btn-sm btn-error"
                                                    on:click=move |_| remove_wrestler(wrestler_id)
                                                    disabled=move || loading.get()
                                                >
                                                    "Remove"
                                                </button>
                                            </div>
                                        }
                                    }
                                />
                                <Show when=move || current_roster.get().is_empty() && !loading.get()>
                                    <div class="text-center py-8 text-base-content/50">
                                        <p>"No wrestlers assigned to this show"</p>
                                        <p class="text-sm">"Add wrestlers from the available pool"</p>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </div>
                    
                    // Available Wrestlers
                    <div class="card bg-base-200 shadow-xl">
                        <div class="card-body">
                            <h3 class="card-title text-xl mb-4">
                                "Available Wrestlers"
                                <div class="badge badge-secondary">{move || available_wrestlers.get().len()}</div>
                            </h3>
                            <div class="space-y-2 max-h-96 overflow-y-auto">
                                <For
                                    each=move || available_wrestlers.get()
                                    key=|wrestler| wrestler.id
                                    children=move |wrestler: Wrestler| {
                                        let wrestler_id = wrestler.id;
                                        view! {
                                            <div class="flex items-center justify-between p-3 bg-base-100 rounded-lg">
                                                <div class="flex items-center space-x-3">
                                                    <div class="avatar placeholder">
                                                        <div class="bg-neutral text-neutral-content rounded-full w-10">
                                                            <span class="text-sm">{wrestler.name.chars().next().unwrap_or('?').to_string()}</span>
                                                        </div>
                                                    </div>
                                                    <div>
                                                        <p class="font-medium">{wrestler.name}</p>
                                                        <p class="text-sm text-base-content/70">
                                                            {format!("{} • {}-{}", wrestler.gender, wrestler.wins, wrestler.losses)}
                                                        </p>
                                                    </div>
                                                </div>
                                                <button
                                                    class="btn btn-sm btn-success"
                                                    on:click=move |_| assign_wrestler(wrestler_id)
                                                    disabled=move || loading.get()
                                                >
                                                    "Assign"
                                                </button>
                                            </div>
                                        }
                                    }
                                />
                                <Show when=move || available_wrestlers.get().is_empty() && !loading.get()>
                                    <div class="text-center py-8 text-base-content/50">
                                        <p>"All wrestlers are assigned to this show"</p>
                                        <p class="text-sm">"Great job building your roster!"</p>
                                    </div>
                                </Show>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}