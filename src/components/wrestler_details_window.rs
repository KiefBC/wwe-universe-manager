use crate::components::wrestler::championship_section::ChampionshipSection;
use crate::components::wrestler::power_ratings_section::PowerRatingsSection;
use crate::services::wrestler_api::*;
use crate::types::{Show, fetch_shows};
use crate::utils::url_watcher::use_url_watcher;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// WrestlerDetails, TitleWithHolders, and other types are now imported from wrestler_api service

// All API functions are now imported from the wrestler_api service
// URL parsing functions are now imported from utils::url_parser

#[component]
pub fn WrestlerDetailsWindow() -> impl IntoView {
    let (wrestler, set_wrestler) = signal(None::<WrestlerDetails>);
    let (shows, set_shows) = signal(Vec::<Show>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    // Use URL watcher hook to track current wrestler ID from URL without polling
    let current_wrestler_id = use_url_watcher();
    
    // Handler for show roster assignment changes - replaced deprecated promotion logic
    let (wrestler_shows, set_wrestler_shows) = signal(Vec::<Show>::new());
    
    let handle_show_assignment_change = move |_show_id: Option<i32>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                // This would require implementing show roster assignment/removal logic
                // For now, we'll load the current assignments to display them properly
                match get_wrestler_show_assignments(w.id).await {
                    Ok(assigned_shows) => {
                        set_wrestler_shows.set(assigned_shows);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load show assignments: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for basic stats change
    let handle_basic_stats_change = move |height: Option<String>, weight: Option<String>, debut_year: Option<i32>, wins: i32, losses: i32| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_basic_stats(w.id, height, weight, debut_year, wins, losses).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update basic stats: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for name change
    let handle_name_change = move |name: String, nickname: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_name(w.id, name, nickname).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update name: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for real name change
    let handle_real_name_change = move |real_name: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_real_name(w.id, real_name).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update real name: {}", e)));
                    }
                }
            });
        }
    };

    // Handler for biography change
    let handle_biography_change = move |biography: Option<String>| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                match update_wrestler_biography(w.id, biography).await {
                    Ok(updated_wrestler) => {
                        set_wrestler.set(Some(updated_wrestler));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to update biography: {}", e)));
                    }
                }
            });
        }
    };


    // URL watching is now handled by the use_url_watcher hook - no more polling!

    // Load shows data once on mount
    Effect::new(move |_| {
        spawn_local(async move {
            match fetch_shows().await {
                Ok(shows_data) => {
                    set_shows.set(shows_data);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load shows: {}", e)));
                }
            }
        });
    });


    // Load wrestler data when wrestler ID changes
    Effect::new(move |_| {
        let wrestler_id = current_wrestler_id.get();
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            if let Some(wrestler_id) = wrestler_id {
                match get_wrestler_by_id(wrestler_id).await {
                    Ok(Some(wrestler_data)) => {
                        set_wrestler.set(Some(wrestler_data));
                        
                        // Load show assignments for this wrestler
                        match get_wrestler_show_assignments(wrestler_id).await {
                            Ok(assigned_shows) => {
                                set_wrestler_shows.set(assigned_shows);
                            }
                            Err(e) => {
                                // Don't set error for show assignments, just log it
                                web_sys::console::log_1(&format!("Failed to load show assignments: {}", e).into());
                            }
                        }
                    }
                    Ok(None) => {
                        set_error.set(Some("Wrestler not found".to_string()));
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load wrestler: {}", e)));
                    }
                }
            } else {
                set_error.set(Some("Invalid wrestler ID in URL".to_string()));
            }
            
            set_loading.set(false);
        });
    });


    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-primary/10 via-accent/10 to-secondary/10 rounded-none border-b border-primary/20 mb-6 sm:mb-8">
                <div class="hero-content text-center py-6 sm:py-8 px-4">
                    <div class="max-w-4xl w-full">
                        <div class="flex items-center justify-center gap-4 sm:gap-6 mb-4">
                            <div class="indicator">
                                <span class="indicator-item badge badge-info badge-sm animate-pulse">"PROFILE"</span>
                                <div class="w-16 h-16 sm:w-20 sm:h-20 bg-gradient-to-br from-primary via-accent to-secondary rounded-2xl flex items-center justify-center shadow-2xl">
                                    <svg class="w-8 h-8 sm:w-12 sm:h-12 text-base-100" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent mb-3">
                            "Wrestler Profile & Analytics"
                        </h1>
                        <p class="text-sm sm:text-base lg:text-lg text-base-content/80 leading-relaxed max-w-2xl mx-auto">
                            "Comprehensive talent management with performance metrics, biographical data, and career statistics."
                        </p>
                    </div>
                </div>
            </div>
            
            <div class="px-4 sm:px-6 lg:px-8 pb-8 sm:pb-12">
                <div class="max-w-6xl mx-auto">
                    
                    // Professional Back Button
                    <div class="mb-6">
                        <button
                            class="btn btn-ghost gap-2 hover:btn-primary transition-colors min-h-[44px]"
                            on:click=move |_| {
                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
                                    let _result = invoke("back_to_wrestlers_list", args).await;
                                });
                            }
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                            </svg>
                            <span class="hidden sm:inline">"Back to Talent Management"</span>
                            <span class="sm:hidden">"Back"</span>
                        </button>
                    </div>
                    // Loading State
                    <Show when=move || loading.get()>
                        <div class="card bg-base-200/50 border border-base-300/30">
                            <div class="card-body p-8 sm:p-12 text-center">
                                <div class="flex flex-col items-center gap-4">
                                    <span class="loading loading-spinner loading-lg text-primary"></span>
                                    <div>
                                        <h3 class="text-lg font-bold text-base-content mb-1">"Loading Wrestler Profile"</h3>
                                        <p class="text-base-content/70 text-sm">"Gathering detailed analytics and performance data..."</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </Show>
                    
                    // Error State
                    <Show when=move || error.get().is_some()>
                        <div class="card bg-error/10 border border-error/30">
                            <div class="card-body p-6 sm:p-8">
                                <div class="flex items-center gap-4 mb-4">
                                    <div class="w-12 h-12 bg-error/20 rounded-xl flex items-center justify-center">
                                        <svg class="w-7 h-7 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <h3 class="text-lg font-bold text-error mb-1">"Error Loading Profile"</h3>
                                        <p class="text-error/80 text-sm">{move || error.get().unwrap_or_default()}</p>
                                    </div>
                                </div>
                                <div class="flex justify-end">
                                    <button class="btn btn-error btn-sm" on:click=move |_| {
                                        if let Some(window) = web_sys::window() {
                                            let _ = window.location().reload();
                                        }
                                    }>
                                        "Retry"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </Show>
                    
                    // Profile Content
                    <Show when=move || !loading.get() && error.get().is_none() && wrestler.get().is_some()>
                        {move || {
                            wrestler.get().map(|_w| {
                                view! {
                                    <div class="card bg-gradient-to-br from-base-100 to-base-200/30 border border-base-300/50 shadow-xl rounded-2xl overflow-hidden">
                                        <div class="card-body p-0">
                                            // Enhanced Header Section
                                            <HeaderSection wrestler=wrestler />

                                            // Mobile-first responsive layout
                                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 p-4 sm:p-6">
                                                // Left Column - Profile & Identity
                                                <div class="space-y-6">
                                                    // Wrestler name banner
                                                    <NameBannerSection 
                                                        wrestler=wrestler
                                                        on_name_change=handle_name_change
                                                    />

                                                    // Real name section
                                                    <RealNameSection 
                                                        wrestler=wrestler
                                                        on_real_name_change=handle_real_name_change
                                                    />

                                                    // Championship & Team Status
                                                    <ChampionshipTeamSection 
                                                        wrestler=wrestler
                                                    />
                                                    
                                                    // Delete wrestler component (only for user-created wrestlers)
                                                    <DeleteWrestlerComponent wrestler=wrestler />
                                                </div>

                                                // Right Column - Statistics & Performance
                                                <div class="space-y-6">
                                                    // Power ratings section
                                                    <PowerRatingsSection 
                                                        wrestler=wrestler
                                                        on_wrestler_updated=set_wrestler
                                                        on_error=set_error
                                                    />

                                                    // Basic stats (separate component)
                                                    <BasicStatsSection 
                                                        wrestler=wrestler
                                                        on_stats_change=handle_basic_stats_change
                                                    />

                                                    // Show Assignment Section
                                                    <ShowAssignmentSection 
                                                        _wrestler=wrestler
                                                        wrestler_shows=wrestler_shows
                                                        _all_shows=shows
                                                        _on_assignment_change=handle_show_assignment_change
                                                    />

                                                    // Biography
                                                    <BiographySection 
                                                        wrestler=wrestler
                                                        on_biography_change=handle_biography_change
                                                    />
                                                </div>
                                            </div>

                                        </div>
                                    </div>
                                }
                            })
                        }}
                    </Show>
                </div>
            </div>
        </div>
    }
}

// PowerBar and PowerBarEdit components moved to power_ratings_section.rs

#[component]
fn ShowAssignmentSection<F>(
    _wrestler: ReadSignal<Option<WrestlerDetails>>,
    wrestler_shows: ReadSignal<Vec<Show>>,
    _all_shows: ReadSignal<Vec<Show>>,
    _on_assignment_change: F,
) -> impl IntoView
where
    F: Fn(Option<i32>) + 'static + Copy + Send + Sync,
{
    view! {
        <div class="card bg-base-200 border border-base-100 mb-4">
            <div class="card-body">
                <h3 class="text-lg font-semibold text-base-content mb-4">"Show Assignments"</h3>
                <div class="space-y-3 text-sm">
                    // Current show assignments
                    <div>
                        <span class="text-base-content/70 font-medium">"Currently assigned to: "</span>
                        <div class="mt-2">
                            {move || {
                                let shows = wrestler_shows.get();
                                if shows.is_empty() {
                                    view! {
                                        <div class="text-base-content/60 italic">
                                            "No show assignments"
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="flex flex-wrap gap-2">
                                            {shows.into_iter().map(|show| {
                                                view! {
                                                    <span class="badge badge-primary">
                                                        {show.name}
                                                    </span>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                    
                    // Note about show roster management
                    <div class="alert alert-info">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                        <div>
                            <div class="text-sm">
                                "To manage show assignments, use the Show Roster Management interface from the main dashboard."
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn BasicStatsSection<F>(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
    on_stats_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>, Option<String>, Option<i32>, i32, i32) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_height, set_temp_height) = signal(String::new());
    let (temp_weight, set_temp_weight) = signal(String::new());
    let (temp_debut_year, set_temp_debut_year) = signal(String::new());
    let (temp_wins, set_temp_wins) = signal(0i32);
    let (temp_losses, set_temp_losses) = signal(0i32);

    view! {
        <div class="bg-base-200 border border-base-100 rounded-lg p-4">
            <div class="flex items-center justify-between mb-4 border-b border-base-100 pb-2">
                <h4 class="text-base-content font-bold text-lg">
                    "Basic Stats"
                </h4>
                <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                    <button
                        class="btn btn-ghost btn-sm gap-1"
                        on:click=move |_| {
                            if let Some(w) = wrestler.get() {
                                set_temp_height.set(w.height.unwrap_or_default());
                                set_temp_weight.set(w.weight.unwrap_or_default());
                                set_temp_debut_year.set(w.debut_year.map(|y| y.to_string()).unwrap_or_default());
                                set_temp_wins.set(w.wins);
                                set_temp_losses.set(w.losses);
                                set_editing.set(true);
                            }
                        }
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        <span>"Edit"</span>
                    </button>
                </Show>
            </div>
            <Show 
                when=move || !editing.get()
                fallback=move || view! {
                    <div class="space-y-3">
                        <div class="grid grid-cols-2 gap-4">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Height"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered input-sm w-full"
                                    placeholder="e.g., 6'5\""
                                    prop:value=move || temp_height.get()
                                    on:input:target=move |ev| {
                                        set_temp_height.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Weight"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered input-sm w-full"
                                    placeholder="e.g., 250 lbs"
                                    prop:value=move || temp_weight.get()
                                    on:input:target=move |ev| {
                                        set_temp_weight.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Debut Year"</span>
                                </label>
                                <input
                                    type="number"
                                    class="input input-bordered input-sm w-full"
                                    placeholder="e.g., 2010"
                                    prop:value=move || temp_debut_year.get()
                                    on:input:target=move |ev| {
                                        set_temp_debut_year.set(ev.target().value());
                                    }
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Gender"</span>
                                </label>
                                <div class="input input-bordered input-sm text-base-content/70">
                                    {move || wrestler.get().map(|w| w.gender).unwrap_or_default()}
                                </div>
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Wins"</span>
                                </label>
                                <input
                                    type="number"
                                    min="0"
                                    class="input input-bordered input-sm w-full"
                                    prop:value=move || temp_wins.get().to_string()
                                    on:input:target=move |ev| {
                                        if let Ok(val) = ev.target().value().parse::<i32>() {
                                            set_temp_wins.set(val.max(0));
                                        }
                                    }
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Losses"</span>
                                </label>
                                <input
                                    type="number"
                                    min="0"
                                    class="input input-bordered input-sm w-full"
                                    prop:value=move || temp_losses.get().to_string()
                                    on:input:target=move |ev| {
                                        if let Ok(val) = ev.target().value().parse::<i32>() {
                                            set_temp_losses.set(val.max(0));
                                        }
                                    }
                                />
                            </div>
                        </div>
                    </div>
                    <div class="flex space-x-2 mt-4">
                        <button
                            class="btn btn-success btn-sm flex-1"
                            on:click=move |_| {
                                let height = if temp_height.get().is_empty() { None } else { Some(temp_height.get()) };
                                let weight = if temp_weight.get().is_empty() { None } else { Some(temp_weight.get()) };
                                let debut_year = if temp_debut_year.get().is_empty() { 
                                    None 
                                } else { 
                                    temp_debut_year.get().parse::<i32>().ok() 
                                };
                                
                                on_stats_change(height, weight, debut_year, temp_wins.get(), temp_losses.get());
                                set_editing.set(false);
                            }
                        >
                            "Save"
                        </button>
                        <button
                            class="btn btn-ghost btn-sm flex-1"
                            on:click=move |_| {
                                set_editing.set(false);
                            }
                        >
                            "Cancel"
                        </button>
                    </div>
                }
            >
                <div class="grid grid-cols-2 gap-4 text-sm">
                    {move || wrestler.get().map(|w| view! {
                        {w.height.as_ref().map(|height| view! {
                            <div>
                                <span class="text-base-content/70 font-medium">"Height: "</span>
                                <span class="text-base-content">{height.clone()}</span>
                            </div>
                        })}
                        {w.weight.as_ref().map(|weight| view! {
                            <div>
                                <span class="text-base-content/70 font-medium">"Weight: "</span>
                                <span class="text-base-content">{weight.clone()}</span>
                            </div>
                        })}
                        {w.debut_year.map(|year| view! {
                            <div>
                                <span class="text-base-content/70 font-medium">"Debut: "</span>
                                <span class="text-base-content">{year.to_string()}</span>
                            </div>
                        })}
                        <div>
                            <span class="text-base-content/70 font-medium">"Gender: "</span>
                            <span class="text-base-content">{w.gender.clone()}</span>
                        </div>
                    })}
                </div>
            </Show>
        </div>
    }
}

#[component]
fn HeaderSection(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
) -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-base-300/80 to-base-200/50 border-b border-base-content/10 p-4 sm:p-6 text-center relative">
            // Status badges - responsive positioning
            <div class="flex justify-between items-start mb-4 sm:mb-0 sm:absolute sm:inset-x-0 sm:top-4 sm:px-6">
                <div class="badge badge-primary badge-sm">
                    <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                    </svg>
                    "TALENT"
                </div>
                <div class="badge badge-info badge-sm">
                    {move || wrestler.get().map(|w| format!("ID #{:03}", w.id)).unwrap_or_default()}
                </div>
            </div>
            
            // Main header content
            <div class="pt-2">
                <h2 class="text-xl sm:text-2xl lg:text-3xl font-bold text-base-content mb-2">
                    "Professional Profile"
                </h2>
                <p class="text-base-content/70 text-xs sm:text-sm">
                    "Comprehensive talent analytics and career management"
                </p>
            </div>
        </div>
    }
}


#[component]
fn NameBannerSection<F>(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
    on_name_change: F,
) -> impl IntoView
where
    F: Fn(String, Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_name, set_temp_name) = signal(String::new());
    let (temp_nickname, set_temp_nickname) = signal(String::new());

    view! {
        <div class="bg-base-200 border border-base-100 p-4 rounded-lg">
            <div class="flex items-center justify-between mb-2">
                <div class="flex-1"></div>
                <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                    <button
                        class="btn btn-ghost btn-xs gap-1"
                        on:click=move |_| {
                            if let Some(w) = wrestler.get() {
                                set_temp_name.set(w.name);
                                set_temp_nickname.set(w.nickname.unwrap_or_default());
                                set_editing.set(true);
                            }
                        }
                    >
                        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        <span>"Edit"</span>
                    </button>
                </Show>
            </div>
            <Show 
                when=move || !editing.get()
                fallback=move || view! {
                    <div class="space-y-3">
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Name"</span>
                            </label>
                            <input
                                type="text"
                                class="input input-bordered text-base sm:text-lg font-bold text-center min-h-[44px]"
                                prop:value=move || temp_name.get()
                                on:input:target=move |ev| {
                                    set_temp_name.set(ev.target().value());
                                }
                            />
                        </div>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Nickname"</span>
                            </label>
                            <input
                                type="text"
                                class="input input-bordered input-sm text-center"
                                placeholder="Optional nickname"
                                prop:value=move || temp_nickname.get()
                                on:input:target=move |ev| {
                                    set_temp_nickname.set(ev.target().value());
                                }
                            />
                        </div>
                    </div>
                    <div class="flex space-x-2 mt-4">
                        <button
                            class="btn btn-success btn-sm flex-1"
                            on:click=move |_| {
                                let nickname = if temp_nickname.get().is_empty() { None } else { Some(temp_nickname.get()) };
                                on_name_change(temp_name.get(), nickname);
                                set_editing.set(false);
                            }
                        >
                            "Save"
                        </button>
                        <button
                            class="btn btn-ghost btn-sm flex-1"
                            on:click=move |_| {
                                set_editing.set(false);
                            }
                        >
                            "Cancel"
                        </button>
                    </div>
                }
            >
                {move || wrestler.get().map(|w| view! {
                    <h3 class="text-3xl font-bold text-base-content text-center">
                        {w.name}
                    </h3>
                    {w.nickname.as_ref().map(|nickname| view! {
                        <p class="text-center text-base-content/70 text-sm mt-1">{nickname.clone()}</p>
                    })}
                })}
            </Show>
        </div>
    }
}

#[component]
fn RealNameSection<F>(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
    on_real_name_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_real_name, set_temp_real_name) = signal(String::new());

    view! {
        <Show when=move || wrestler.get().and_then(|w| w.real_name.clone()).is_some() || editing.get()>
            <div class="card bg-base-200 border border-base-100">
                <div class="card-body">
                    <div class="flex items-center justify-between mb-2 border-b border-base-content/20 pb-2">
                        <div class="text-primary text-sm font-medium">
                            "Real Name"
                        </div>
                    <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                        <button
                            class="btn btn-ghost btn-xs gap-1"
                            on:click=move |_| {
                                if let Some(w) = wrestler.get() {
                                    set_temp_real_name.set(w.real_name.unwrap_or_default());
                                    set_editing.set(true);
                                }
                            }
                        >
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                            <span>"Edit"</span>
                        </button>
                    </Show>
                </div>
                <Show 
                    when=move || !editing.get()
                    fallback=move || view! {
                        <div class="space-y-3">
                            <input
                                type="text"
                                class="input input-bordered text-lg"
                                placeholder="Enter real name"
                                prop:value=move || temp_real_name.get()
                                on:input:target=move |ev| {
                                    set_temp_real_name.set(ev.target().value());
                                }
                            />
                        </div>
                        <div class="flex space-x-2 mt-4">
                            <button
                                class="btn btn-success btn-sm flex-1"
                                on:click=move |_| {
                                    let real_name = if temp_real_name.get().is_empty() { None } else { Some(temp_real_name.get()) };
                                    on_real_name_change(real_name);
                                    set_editing.set(false);
                                }
                            >
                                "Save"
                            </button>
                            <button
                                class="btn btn-ghost btn-sm flex-1"
                                on:click=move |_| {
                                    set_editing.set(false);
                                }
                            >
                                "Cancel"
                            </button>
                        </div>
                    }
                >
                    {move || wrestler.get().and_then(|w| w.real_name).map(|real_name| view! {
                        <p class="text-base-content font-semibold text-lg">{real_name}</p>
                    })}
                </Show>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn BiographySection<F>(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
    on_biography_change: F,
) -> impl IntoView
where
    F: Fn(Option<String>) + 'static + Copy + Send + Sync,
{
    let (editing, set_editing) = signal(false);
    let (temp_biography, set_temp_biography) = signal(String::new());

    view! {
        <Show when=move || wrestler.get().and_then(|w| w.biography.clone()).is_some() || editing.get()>
            <div class="card bg-base-200 border border-base-100">
                <div class="card-body">
                    <div class="flex items-center justify-between mb-3 border-b border-base-content/20 pb-2">
                        <h4 class="text-base-content font-semibold text-lg">"Biography"</h4>
                    <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
                        <button
                            class="btn btn-ghost btn-sm gap-1"
                            on:click=move |_| {
                                if let Some(w) = wrestler.get() {
                                    set_temp_biography.set(w.biography.unwrap_or_default());
                                    set_editing.set(true);
                                }
                            }
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                            </svg>
                            <span>"Edit"</span>
                        </button>
                    </Show>
                </div>
                <Show 
                    when=move || !editing.get()
                    fallback=move || view! {
                        <div class="space-y-3">
                            <textarea
                                class="textarea textarea-bordered resize-none"
                                rows="4"
                                placeholder="Enter biography..."
                                prop:value=move || temp_biography.get()
                                on:input:target=move |ev| {
                                    set_temp_biography.set(ev.target().value());
                                }
                            ></textarea>
                        </div>
                        <div class="flex space-x-2 mt-4">
                            <button
                                class="btn btn-success btn-sm flex-1"
                                on:click=move |_| {
                                    let biography = if temp_biography.get().is_empty() { None } else { Some(temp_biography.get()) };
                                    on_biography_change(biography);
                                    set_editing.set(false);
                                }
                            >
                                "Save"
                            </button>
                            <button
                                class="btn btn-ghost btn-sm flex-1"
                                on:click=move |_| {
                                    set_editing.set(false);
                                }
                            >
                                "Cancel"
                            </button>
                        </div>
                    }
                >
                    {move || wrestler.get().and_then(|w| w.biography).map(|bio| view! {
                        <p class="text-base-content/80 text-sm leading-relaxed">
                            {bio}
                        </p>
                    })}
                </Show>
                </div>
            </div>
        </Show>
    }
}


#[component]
fn ChampionshipTeamSection(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200 border border-base-100">
            <div class="card-body">
                <h4 class="text-base-content font-bold text-lg mb-4 border-b border-base-content/20 pb-2">
                    "Championship & Team Status"
                </h4>
            
            <div class="space-y-4">
                // Record section
                <div class="flex items-center justify-between">
                    <span class="text-base-content/70 font-medium text-sm">"Record:"</span>
                    <span class="text-base-content font-semibold">
                        {move || wrestler.get().map(|w| format!("{}-{}", w.wins, w.losses)).unwrap_or_default()}
                    </span>
                </div>
                
                // Championship section
                <ChampionshipSection 
                    wrestler=wrestler 
                />
                
                // Tag Team section
                <div class="space-y-2">
                    <span class="text-base-content/70 font-medium text-sm">"Tag Team:"</span>
                    <div class="bg-base-300 border border-base-content/20 rounded-lg p-3">
                        <div class="flex items-center space-x-2 mb-2">
                            // Partner placeholders
                            <div class="flex space-x-2">
                                <div class="w-8 h-8 bg-base-content/20 border border-base-content/30 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-base-content/60" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                                <div class="w-8 h-8 bg-base-content/20 border border-base-content/30 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-base-content/60" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                                <div class="w-8 h-8 bg-base-content/10 border border-base-content/20 rounded-full flex items-center justify-center">
                                    <svg class="w-4 h-4 text-base-content/40" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <div>
                            <p class="text-base-content/80 text-sm italic">"No tag team partners"</p>
                            <p class="text-base-content/50 text-xs">"Tag team management coming soon"</p>
                        </div>
                    </div>
                </div>
            </div>
            </div>
        </div>
    }
}

#[component]
fn DeleteWrestlerComponent(
    wrestler: ReadSignal<Option<WrestlerDetails>>,
) -> impl IntoView {
    let (show_confirmation, set_show_confirmation) = signal(false);
    let (deleting, set_deleting) = signal(false);
    let (error, set_error) = signal(None::<String>);

    let handle_delete_click = move |_| {
        set_show_confirmation.set(true);
        set_error.set(None);
    };

    let handle_confirm_delete = move |_| {
        if let Some(w) = wrestler.get() {
            let wrestler_id = w.id;
            set_deleting.set(true);
            set_error.set(None);
            
            spawn_local(async move {
                match delete_wrestler(wrestler_id).await {
                    Ok(_) => {
                        // Close the window after successful deletion
                        if let Some(window) = web_sys::window() {
                            let _ = window.close();
                        }
                    }
                    Err(e) => {
                        set_error.set(Some(e));
                        set_deleting.set(false);
                        set_show_confirmation.set(false);
                    }
                }
            });
        }
    };

    let handle_cancel_delete = move |_| {
        set_show_confirmation.set(false);
        set_error.set(None);
    };

    view! {
        <Show when=move || wrestler.get().and_then(|w| w.is_user_created).unwrap_or(false)>
            <div class="card bg-error/10 border border-error/30 relative overflow-hidden">
                <div class="card-body p-4">
                    <div class="flex items-center gap-3 mb-3">
                        <div class="w-10 h-10 bg-error/20 border border-error/50 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </div>
                        <div class="flex-1">
                            <h4 class="text-error font-bold text-lg">"Danger Zone"</h4>
                            <p class="text-error/80 text-sm">"Permanently delete this wrestler"</p>
                        </div>
                    </div>
                    
                    <Show when=move || error.get().is_some()>
                        <div class="alert alert-error mb-3">
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span>{move || error.get().unwrap_or_default()}</span>
                        </div>
                    </Show>

                    <Show when=move || !show_confirmation.get()>
                        <button
                            class="btn btn-error w-full gap-2"
                            disabled=move || deleting.get()
                            on:click=handle_delete_click
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                            "Delete Wrestler"
                        </button>
                    </Show>

                    <Show when=move || show_confirmation.get()>
                        <div class="space-y-4">
                            <div class="bg-error/20 border border-error/30 rounded-lg p-4">
                                <div class="flex items-center gap-2 mb-2">
                                    <svg class="w-5 h-5 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                                    </svg>
                                    <h5 class="text-error font-bold">"Confirm Deletion"</h5>
                                </div>
                                <p class="text-error/90 text-sm mb-3">
                                    "Are you sure you want to delete "
                                    <strong>{move || wrestler.get().map(|w| w.name).unwrap_or_default()}</strong>
                                    "? This action cannot be undone."
                                </p>
                                <p class="text-error/70 text-xs">
                                    "This will also remove the wrestler from all show rosters, matches, and title histories."
                                </p>
                            </div>
                            <div class="flex gap-2">
                                <button
                                    class="btn btn-error flex-1 gap-2"
                                    disabled=move || deleting.get()
                                    on:click=handle_confirm_delete
                                >
                                    <Show when=move || deleting.get()>
                                        <span class="loading loading-spinner loading-sm"></span>
                                    </Show>
                                    <Show when=move || !deleting.get()>
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                        </svg>
                                    </Show>
                                    {move || if deleting.get() { "Deleting..." } else { "Yes, Delete Forever" }}
                                </button>
                                <button
                                    class="btn btn-ghost flex-1"
                                    disabled=move || deleting.get()
                                    on:click=handle_cancel_delete
                                >
                                    "Cancel"
                                </button>
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </Show>
    }
}


// TitleComponent has been moved to championship_section.rs