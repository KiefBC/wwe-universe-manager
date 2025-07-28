use crate::types::{
    add_wrestler_to_match, create_match, fetch_matches_for_show,
    fetch_shows, fetch_wrestlers_for_show, Match, MatchData,
    Promotion, Show, Wrestler,
};
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Booker Dashboard component for match booking and management
/// 
/// Features:
/// - Select show from promotion-specific dropdown
/// - View existing matches for selected show
/// - Create new matches with various types and stipulations
/// - Add wrestlers to matches (only wrestlers assigned to selected show)
/// - Set match winners and manage results
/// - Match card organization and management
#[component]
pub fn BookerDashboard(
    set_current_page: WriteSignal<String>,
    selected_promotion: ReadSignal<Option<Promotion>>,
) -> impl IntoView {
    // State management
    let (selected_show, set_selected_show) = signal(None::<Show>);
    let (matches, set_matches) = signal(Vec::<Match>::new());
    let (show_wrestlers, set_show_wrestlers) = signal(Vec::<Wrestler>::new()); 
    let (loading, set_loading) = signal(false);
    let (status_message, set_status_message) = signal(None::<String>);
    let (error_message, set_error_message) = signal(None::<String>);
    let (show_create_form, set_show_create_form) = signal(false);
    
    // Create match form state
    let (match_name, set_match_name) = signal(String::new());
    let (match_type, set_match_type) = signal("Singles".to_string());
    let (match_stipulation, set_match_stipulation) = signal("Standard".to_string());
    
    // Fetch shows for the selected promotion
    let shows_resource = LocalResource::new(move || {
        async move {
            fetch_shows().await // TODO: Filter by promotion when backend supports it
        }
    });
    
    // Load show data when show selection changes
    let load_show_data = move |show_id: i32| {
        set_loading.set(true);
        set_status_message.set(None);
        set_error_message.set(None);
        
        spawn_local(async move {
            // Fetch matches and wrestlers for the show concurrently
            let matches_result = fetch_matches_for_show(show_id).await;
            let wrestlers_result = fetch_wrestlers_for_show(show_id).await;
            
            match (matches_result, wrestlers_result) {
                (Ok(show_matches), Ok(wrestlers)) => {
                    set_matches.set(show_matches);
                    set_show_wrestlers.set(wrestlers);
                    set_loading.set(false);
                },
                (Err(e), _) | (_, Err(e)) => {
                    set_error_message.set(Some(format!("Failed to load show data: {}", e)));
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
                        load_show_data(show_id);
                    }
                }
            }
        } else {
            set_selected_show.set(None);
            set_matches.set(Vec::new());
            set_show_wrestlers.set(Vec::new());
        }
    };
    
    // Create new match
    let create_new_match = move || {
        if let Some(show) = selected_show.get() {
            set_loading.set(true);
            set_status_message.set(None);
            set_error_message.set(None);
            
            let match_data = MatchData {
                show_id: show.id,
                match_name: if match_name.get().trim().is_empty() { None } else { Some(match_name.get().trim().to_string()) },
                match_type: match_type.get(),
                match_stipulation: if match_stipulation.get() == "Standard" { None } else { Some(match_stipulation.get()) },
                scheduled_date: None,
                match_order: Some((matches.get().len() + 1) as i32),
                is_title_match: false,
                title_id: None,
            };
            
            spawn_local(async move {
                match create_match(match_data).await {
                    Ok(_) => {
                        set_status_message.set(Some("Match created successfully!".to_string()));
                        set_match_name.set(String::new());
                        set_match_type.set("Singles".to_string());
                        set_match_stipulation.set("Standard".to_string());
                        set_show_create_form.set(false);
                        load_show_data(show.id); // Reload data
                    },
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to create match: {}", e)));
                        set_loading.set(false);
                    }
                }
            });
        }
    };
    
    // Add wrestler to match
    let add_wrestler_to_match_handler = move |match_id: i32, wrestler_id: i32| {
        set_loading.set(true);
        set_status_message.set(None);
        set_error_message.set(None);
        
        spawn_local(async move {
            match add_wrestler_to_match(match_id, wrestler_id, None, None).await {
                Ok(_) => {
                    set_status_message.set(Some("Wrestler added to match!".to_string()));
                    if let Some(show) = selected_show.get() {
                        load_show_data(show.id); // Reload data
                    }
                },
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to add wrestler: {}", e)));
                    set_loading.set(false);
                }
            }
        });
    };
    
    view! {
        <div class="space-y-8">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-3xl font-bold text-base-content mb-2">
                        "Match Booking Dashboard"
                    </h2>
                    <p class="text-base-content/70">
                        {move || {
                            if let Some(promotion) = selected_promotion.get() {
                                format!("Book matches and manage match cards for {} shows", promotion.name)
                            } else {
                                "Book matches and manage match cards".to_string()
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
                                "Choose a show to book matches..."
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
            
            // Match Booking Interface (only show when a show is selected)
            <Show when=move || selected_show.get().is_some()>
                <div class="space-y-6">
                    // Create Match Section
                    <div class="card bg-base-200 shadow-xl">
                        <div class="card-body">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="card-title text-xl">
                                    "Match Card for "
                                    {move || selected_show.get().map(|s| s.name).unwrap_or_default()}
                                </h3>
                                <button
                                    class="btn btn-primary gap-2"
                                    on:click=move |_| set_show_create_form.set(!show_create_form.get())
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                    </svg>
                                    "Create Match"
                                </button>
                            </div>
                            
                            // Create Match Form
                            <Show when=move || show_create_form.get()>
                                <div class="bg-base-100 p-4 rounded-lg mb-6">
                                    <h4 class="text-lg font-semibold mb-4">"Create New Match"</h4>
                                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                                        <div class="form-control">
                                            <label class="label">
                                                <span class="label-text">"Match Name (Optional)"</span>
                                            </label>
                                            <input
                                                type="text"
                                                placeholder="e.g., Main Event"
                                                class="input input-bordered"
                                                prop:value=match_name
                                                on:input=move |ev| set_match_name.set(event_target_value(&ev))
                                            />
                                        </div>
                                        
                                        <div class="form-control">
                                            <label class="label">
                                                <span class="label-text">"Match Type"</span>
                                            </label>
                                            <select 
                                                class="select select-bordered"
                                                prop:value=match_type
                                                on:change=move |ev| set_match_type.set(event_target_value(&ev))
                                            >
                                                <option value="Singles">"Singles"</option>
                                                <option value="Tag Team">"Tag Team"</option>
                                                <option value="Triple Threat">"Triple Threat"</option>
                                                <option value="Fatal 4-Way">"Fatal 4-Way"</option>
                                                <option value="Battle Royal">"Battle Royal"</option>
                                                <option value="Ladder Match">"Ladder Match"</option>
                                                <option value="Steel Cage">"Steel Cage"</option>
                                            </select>
                                        </div>
                                        
                                        <div class="form-control">
                                            <label class="label">
                                                <span class="label-text">"Stipulation"</span>
                                            </label>
                                            <select 
                                                class="select select-bordered"
                                                prop:value=match_stipulation
                                                on:change=move |ev| set_match_stipulation.set(event_target_value(&ev))
                                            >
                                                <option value="Standard">"Standard"</option>
                                                <option value="No DQ">"No Disqualification"</option>
                                                <option value="No Holds Barred">"No Holds Barred"</option>
                                                <option value="Last Man Standing">"Last Man Standing"</option>
                                                <option value="Submission Match">"Submission Match"</option>
                                                <option value="Hardcore">"Hardcore"</option>
                                            </select>
                                        </div>
                                    </div>
                                    
                                    <div class="flex justify-end space-x-2 mt-4">
                                        <button
                                            class="btn btn-ghost"
                                            on:click=move |_| set_show_create_form.set(false)
                                        >
                                            "Cancel"
                                        </button>
                                        <button
                                            class="btn btn-primary"
                                            on:click=move |_| create_new_match()
                                            disabled=move || loading.get()
                                        >
                                            "Create Match"
                                        </button>
                                    </div>
                                </div>
                            </Show>
                            
                            // Existing Matches List
                            <div class="space-y-4">
                                <For
                                    each=move || matches.get()
                                    key=|match_item| match_item.id
                                    children=move |match_item: Match| {
                                        let match_id = match_item.id;
                                        let match_name_display = match_item.match_name.clone()
                                            .unwrap_or_else(|| format!("{} Match", match_item.match_type));
                                        
                                        view! {
                                            <div class="card bg-base-100 border border-base-300">
                                                <div class="card-body">
                                                    <div class="flex items-center justify-between">
                                                        <div>
                                                            <h4 class="text-lg font-semibold">{match_name_display}</h4>
                                                            <p class="text-base-content/70">
                                                                {match_item.match_type.clone()}
                                                                {match_item.match_stipulation.as_ref().map(|s| format!(" - {}", s)).unwrap_or_default()}
                                                            </p>
                                                            <Show when=move || match_item.winner_id.is_some()>
                                                                <div class="badge badge-success gap-2 mt-2">
                                                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                                                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                                    </svg>
                                                                    "Match Complete"
                                                                </div>
                                                            </Show>
                                                        </div>
                                                        <div class="flex space-x-2">
                                                            <div class="dropdown dropdown-end">
                                                                <div tabindex="0" role="button" class="btn btn-sm btn-outline">
                                                                    "Add Wrestler"
                                                                </div>
                                                                <ul tabindex="0" class="dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow max-h-48 overflow-y-auto">
                                                                    <For
                                                                        each=move || show_wrestlers.get()
                                                                        key=|wrestler| wrestler.id
                                                                        children=move |wrestler: Wrestler| {
                                                                            let wrestler_id = wrestler.id;
                                                                            view! {
                                                                                <li>
                                                                                    <a on:click=move |_| add_wrestler_to_match_handler(match_id, wrestler_id)>
                                                                                        {wrestler.name}
                                                                                    </a>
                                                                                </li>
                                                                            }
                                                                        }
                                                                    />
                                                                </ul>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }
                                />
                                
                                <Show when=move || matches.get().is_empty() && !loading.get()>
                                    <div class="text-center py-8 text-base-content/50">
                                        <svg class="w-16 h-16 mx-auto mb-4 text-base-content/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 0V3a2 2 0 00-2-2V1a2 2 0 00-2 2v2H9z" />
                                        </svg>
                                        <p class="text-lg mb-2">"No matches scheduled"</p>
                                        <p class="text-sm">"Create your first match to start building the card"</p>
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