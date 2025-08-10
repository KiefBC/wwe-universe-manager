use crate::components::show::match_creation_form::MatchCreationForm;
use crate::components::show::match_list_section::MatchListSection;
use crate::types::{
    add_wrestler_to_match, create_match, fetch_matches_for_show,
    fetch_shows, fetch_wrestlers_for_show, Match, MatchData,
    Show, Wrestler,
};
use leptos::prelude::*;
use wasm_bindgen::JsCast;
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
) -> impl IntoView {
    // State management
    let (selected_show, set_selected_show) = signal(None::<Show>);
    let (matches, set_matches) = signal(Vec::<Match>::new());
    let (show_wrestlers, set_show_wrestlers) = signal(Vec::<Wrestler>::new()); 
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (loading, set_loading) = signal(false);
    let (shows_loading, set_shows_loading) = signal(true);
    let (status_message, set_status_message) = signal(None::<String>);
    let (error_message, set_error_message) = signal(None::<String>);
    let (show_create_form, set_show_create_form) = signal(false);
    
    // Create match form state
    let (match_name, set_match_name) = signal(String::new());
    let (match_type, set_match_type) = signal("Singles".to_string());
    let (match_stipulation, set_match_stipulation) = signal("Standard".to_string());
    
    // Communication signals for sub-components
    let (create_match_trigger, set_create_match_trigger) = signal(false);
    let (add_wrestler_to_match_trigger, set_add_wrestler_to_match_trigger) = signal(None::<(i32, i32)>);
    
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
    let on_show_change = move |ev: web_sys::Event| {
        let target = ev.target().unwrap();
        let select = target.dyn_into::<web_sys::HtmlSelectElement>().unwrap();
        let value = select.value();
        if let Ok(show_id) = value.parse::<i32>() {
            let shows_list = shows.get();
            if let Some(show) = shows_list.iter().find(|s| s.id == show_id) {
                set_selected_show.set(Some(show.clone()));
                load_show_data(show_id);
            }
        } else {
            set_selected_show.set(None);
            set_matches.set(Vec::new());
            set_show_wrestlers.set(Vec::new());
        }
    };
    
    // Handle create match trigger
    Effect::new(move |_| {
        if create_match_trigger.get() {
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
            set_create_match_trigger.set(false); // Reset trigger
        }
    });
    
    // Handle add wrestler to match trigger
    Effect::new(move |_| {
        if let Some((match_id, wrestler_id)) = add_wrestler_to_match_trigger.get() {
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
            set_add_wrestler_to_match_trigger.set(None); // Reset trigger
        }
    });
    
    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-info/10 via-accent/10 to-primary/10 rounded-none border-b border-info/20 mb-6 sm:mb-8">
                <div class="hero-content text-center py-4 sm:py-6">
                    <div class="max-w-6xl w-full">
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-info via-accent to-primary bg-clip-text text-transparent mb-6">
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
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="px-4 sm:px-6 lg:px-8 pb-8 sm:pb-12">
                <div class="max-w-6xl mx-auto space-y-6">
            
                    // Show Selection Section
                    <section>
                        <div class="mb-6">
                            <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Show Selection"</h2>
                            <p class="text-base-content/70 text-sm sm:text-base">"Select a show to start booking matches and managing the card"</p>
                        </div>
                        
                        <div class="card bg-gradient-to-br from-info/5 to-info/2 border border-info/20 shadow-lg">
                            <div class="card-body p-4 sm:p-6">
                                <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 sm:gap-6">
                                    <div class="w-12 h-12 bg-info/20 rounded-xl flex items-center justify-center">
                                        <svg class="w-7 h-7 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                        </svg>
                                    </div>
                                    <div class="flex-1 w-full sm:w-auto">
                                        <h3 class="text-lg font-bold text-base-content mb-2">"Active Show"</h3>
                                        <div class="form-control w-full">
                                            <select 
                                                class="select select-bordered w-full bg-base-100 text-base focus:border-info focus:outline-none min-h-[44px]"
                                                on:change=on_show_change
                                            >
                                                <option value="" selected=move || selected_show.get().is_none()>
                                                    "Choose a show to book matches..."
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
                                    {move || {
                                        if let Some(show) = selected_show.get() {
                                            view! {
                                                <div class="flex flex-col items-end gap-2">
                                                    <div class="badge badge-info badge-lg gap-2">
                                                        <div class="w-2 h-2 rounded-full bg-current animate-pulse"></div>
                                                        "Active"
                                                    </div>
                                                    <div class="text-xs text-base-content/60 text-right">
                                                        {format!("Booking: {}", show.name)}
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <div class="badge badge-ghost">
                                                    "No Selection"
                                                </div>
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            </div>
                        </div>
                    </section>
            
                    // Status and Error Messages
                    <Show when=move || status_message.get().is_some() || error_message.get().is_some()>
                        <section>
                            <div class="space-y-3">
                                <Show when=move || status_message.get().is_some()>
                                    <div class="alert alert-success shadow-lg border border-success/20">
                                        <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <div>
                                            <h3 class="font-bold">"Booking Successful"</h3>
                                            <div class="text-sm opacity-80">{move || status_message.get().unwrap_or_default()}</div>
                                        </div>
                                    </div>
                                </Show>
                                <Show when=move || error_message.get().is_some()>
                                    <div class="alert alert-error shadow-lg border border-error/20">
                                        <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <div>
                                            <h3 class="font-bold">"Booking Failed"</h3>
                                            <div class="text-sm opacity-80">{move || error_message.get().unwrap_or_default()}</div>
                                        </div>
                                    </div>
                                </Show>
                            </div>
                        </section>
                    </Show>
                    
                    // Loading indicator
                    <Show when=move || loading.get()>
                        <section>
                            <div class="flex flex-col items-center justify-center py-8 sm:py-12">
                                <div class="loading loading-spinner loading-lg text-info mb-4"></div>
                                <div class="text-base-content/70 text-sm">"Processing booking changes..."</div>
                            </div>
                        </section>
                    </Show>
            
                    // Match Booking Interface (only show when a show is selected)
                    <Show when=move || selected_show.get().is_some()>
                        <section>
                            <div class="mb-6">
                                <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Match Card Management"</h2>
                                <p class="text-base-content/70 text-sm sm:text-base">"Professional match booking and creative control for your selected show"</p>
                            </div>
                            
                            // Create Match Section with Professional Styling
                            <div class="card bg-gradient-to-br from-info/5 to-info/2 border border-info/20 shadow-lg">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 mb-6">
                                        <div class="flex items-center gap-3">
                                            <div class="w-12 h-12 bg-info/20 rounded-xl flex items-center justify-center">
                                                <svg class="w-7 h-7 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3 class="text-lg font-bold text-base-content">
                                                    "Match Card: "
                                                    {move || selected_show.get().map(|s| s.name).unwrap_or_default()}
                                                </h3>
                                                <p class="text-sm text-base-content/60">"Professional match booking and talent coordination"</p>
                                            </div>
                                        </div>
                                        <button
                                            class="btn btn-info gap-2 w-full sm:w-auto min-h-[44px]"
                                            on:click=move |_| set_show_create_form.set(!show_create_form.get())
                                        >
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                            </svg>
                                            {move || if show_create_form.get() { "Cancel Match Creation" } else { "Create New Match" }}
                                        </button>
                                    </div>
                                    
                                    <MatchCreationForm 
                                        show_form=show_create_form.into()
                                        set_show_form=set_show_create_form
                                        match_name=match_name.into()
                                        set_match_name=set_match_name
                                        match_type=match_type.into()
                                        set_match_type=set_match_type
                                        match_stipulation=match_stipulation.into()
                                        set_match_stipulation=set_match_stipulation
                                        loading=loading.into()
                                        on_create_match=set_create_match_trigger
                                    />
                                    
                                    <MatchListSection 
                                        matches=matches.into()
                                        show_wrestlers=show_wrestlers.into()
                                        loading=loading.into()
                                        on_add_wrestler_to_match=set_add_wrestler_to_match_trigger
                                    />
                                </div>
                            </div>
                        </section>
                    </Show>
                </div>
            </div>
        </div>
    }
}