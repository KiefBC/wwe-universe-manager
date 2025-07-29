use crate::services::wrestler_api::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Championship section component for wrestler details
/// 
/// Displays current championships held by a wrestler with prestige-based styling
/// and duration information.
#[component]
pub fn ChampionshipSection(
    /// The wrestler whose championships to display
    wrestler: ReadSignal<Option<WrestlerDetails>>,
) -> impl IntoView
{
    let (current_titles, set_current_titles) = signal(Vec::<TitleWithHolders>::new());
    let (loading, set_loading) = signal(false);
    let (assignable_titles, set_assignable_titles) = signal(Vec::<TitleWithHolders>::new());
    let (loading_assignable, set_loading_assignable) = signal(false);
    let (operation_loading, set_operation_loading) = signal(false);
    let (error_message, set_error_message) = signal(None::<String>);

    // Load current titles when wrestler changes
    Effect::new(move |_| {
        if let Some(w) = wrestler.get() {
            let wrestler_id = w.id;
            spawn_local(async move {
                set_loading.set(true);
                
                match get_current_titles_for_wrestler(wrestler_id).await {
                    Ok(titles) => {
                        set_current_titles.set(titles);
                    }
                    Err(_) => {
                        set_current_titles.set(Vec::new());
                    }
                }
                
                set_loading.set(false);
            });
        }
    });

    // Load assignable titles when wrestler changes
    Effect::new(move |_| {
        if let Some(w) = wrestler.get() {
            let wrestler_id = w.id;
            let wrestler_gender = w.gender.clone();
            spawn_local(async move {
                set_loading_assignable.set(true);
                
                match get_assignable_titles(wrestler_id, wrestler_gender).await {
                    Ok(titles) => {
                        set_assignable_titles.set(titles);
                    }
                    Err(_) => {
                        set_assignable_titles.set(Vec::new());
                    }
                }
                
                set_loading_assignable.set(false);
            });
        }
    });

    // Handler for vacating a title
    let handle_vacate_title = move |title_id: i32| {
        if let Some(w) = wrestler.get() {
            let wrestler_id = w.id;
            set_operation_loading.set(true);
            set_error_message.set(None);
            
            spawn_local(async move {
                match vacate_title(
                    title_id, 
                    Some("Title Vacated".to_string()), 
                    None, 
                    Some("Vacated via UI".to_string())
                ).await {
                    Ok(_) => {
                        // Refresh title data after successful operation
                        match get_current_titles_for_wrestler(wrestler_id).await {
                            Ok(updated_titles) => {
                                set_current_titles.set(updated_titles);
                            }
                            Err(e) => {
                                set_error_message.set(Some(format!("Failed to refresh titles: {}", e)));
                            }
                        }
                        
                        // Also refresh assignable titles
                        if let Some(w) = wrestler.get() {
                            match get_assignable_titles(wrestler_id, w.gender).await {
                                Ok(assignable) => {
                                    set_assignable_titles.set(assignable);
                                }
                                Err(_) => {
                                    // Ignore errors for assignable titles refresh
                                }
                            }
                        }
                    }
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to vacate title: {}", e)));
                    }
                }
                set_operation_loading.set(false);
            });
        }
    };

    // Handler for assigning a new title
    let handle_assign_title = move |title_id: i32| {
        if let Some(w) = wrestler.get() {
            let wrestler_id = w.id;
            set_operation_loading.set(true);
            set_error_message.set(None);
            
            spawn_local(async move {
                match assign_title_to_wrestler(
                    title_id,
                    wrestler_id,
                    Some("Title Assigned".to_string()),
                    None,
                    Some("Assigned via UI".to_string())
                ).await {
                    Ok(_) => {
                        // Refresh title data after successful operation
                        match get_current_titles_for_wrestler(wrestler_id).await {
                            Ok(updated_titles) => {
                                set_current_titles.set(updated_titles);
                            }
                            Err(e) => {
                                set_error_message.set(Some(format!("Failed to refresh titles: {}", e)));
                            }
                        }
                        
                        // Also refresh assignable titles
                        if let Some(w) = wrestler.get() {
                            match get_assignable_titles(wrestler_id, w.gender).await {
                                Ok(assignable) => {
                                    set_assignable_titles.set(assignable);
                                }
                                Err(_) => {
                                    // Ignore errors for assignable titles refresh
                                }
                            }
                        }
                    }
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to assign title: {}", e)));
                    }
                }
                set_operation_loading.set(false);
            });
        }
    };

    // Helper function to get prestige styling based on title division
    let get_prestige_styling = |division: &str| -> (&str, &str, &str) {
        match division {
            // Tier 1 - World Championships (Gold)
            "World" | "WWE Championship" | "Women's World" | "WWE Women's Championship" => {
                ("bg-yellow-500/20 border-yellow-500/50", "bg-yellow-500/30 border-yellow-500", "text-yellow-600")
            },
            // Tier 2 - Secondary Championships (Silver) 
            "Intercontinental" | "United States" | "Women's Intercontinental" | "Women's United States" => {
                ("bg-slate-400/20 border-slate-400/50", "bg-slate-400/30 border-slate-400", "text-slate-600")
            },
            // Tier 3 - Tag Team Championships (Bronze)
            "World Tag Team" | "WWE Tag Team" | "Women's Tag Team" => {
                ("bg-orange-600/20 border-orange-600/50", "bg-orange-600/30 border-orange-600", "text-orange-700")
            },
            // Tier 4 - Specialty Championships (Purple)
            _ => {
                ("bg-purple-500/20 border-purple-500/50", "bg-purple-500/30 border-purple-500", "text-purple-600")
            }
        }
    };

    view! {
        <div class="space-y-2">
            <span class="text-base-content/70 font-medium text-sm">"Current Title:"</span>
            
            <Show when=move || loading.get()>
                <div class="bg-base-300 border border-base-content/20 rounded-lg p-4 flex items-center justify-center">
                    <span class="loading loading-spinner loading-sm"></span>
                    <span class="ml-2 text-base-content/70">"Loading titles..."</span>
                </div>
            </Show>

            <Show when=move || !loading.get() && current_titles.get().is_empty()>
                <div class="bg-base-300 border border-base-content/20 rounded-lg p-4 text-center">
                    <div class="w-12 h-12 bg-base-content/10 border border-base-content/20 rounded-lg flex items-center justify-center mx-auto mb-3">
                        <svg class="w-8 h-8 text-base-content/40" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M5 16L3 14l5.5-5.5L10 10l4-4 4 4 1.5-1.5L15 3l-4 4L7 3 2.5 8.5 5 11v5zm2.5 2.5L9 17l1.5 1.5L12 17l1.5 1.5L15 17l1.5 1.5L18 17v-2l-1.5-1.5L15 15l-1.5-1.5L12 15l-1.5-1.5L9 15l-1.5 1.5L6 17v2l1.5-1.5z"/>
                        </svg>
                    </div>
                    <p class="text-base-content/70 text-sm italic">"No championship held"</p>
                </div>
            </Show>

            <Show when=move || !loading.get() && !current_titles.get().is_empty()>
                <div class="space-y-3">
                    <For
                        each=move || current_titles.get()
                        key=|title_with_holders| title_with_holders.title.id
                        children=move |title_with_holders| {
                            let title = title_with_holders.title.clone();
                            let (bg_class, icon_class, text_class) = get_prestige_styling(&title.division);
                            
                            view! {
                                <div class=format!("rounded-lg p-4 border {}", bg_class)>
                                    // Title header with prestige-colored icon
                                    <div class="flex items-center space-x-3 mb-3">
                                        <div class=format!("w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0 {}", icon_class)>
                                            <svg class=format!("w-6 h-6 {}", text_class) fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M5 16L3 14l5.5-5.5L10 10l4-4 4 4 1.5-1.5L15 3l-4 4L7 3 2.5 8.5 5 11v5zm2.5 2.5L9 17l1.5 1.5L12 17l1.5 1.5L15 17l1.5 1.5L18 17v-2l-1.5-1.5L15 15l-1.5-1.5L12 15l-1.5-1.5L9 15l-1.5 1.5L6 17v2l1.5-1.5z"/>
                                            </svg>
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <h3 class="text-base-content font-bold text-lg truncate">{title.name.clone()}</h3>
                                            <p class="text-base-content/70 text-sm">{format!("{} {}", title.division, title.title_type)}</p>
                                        </div>
                                    </div>
                                    
                                    // Weeks held info and vacate button
                                    <div class="flex items-center justify-between">
                                        <div class="text-base-content/60 text-sm">
                                            {move || {
                                                if let Some(days) = title_with_holders.days_held {
                                                    let weeks = if days >= 7 { days / 7 } else { 0 };
                                                    let remaining_days = days % 7;
                                                    
                                                    if weeks > 0 {
                                                        if remaining_days > 0 {
                                                            format!("{} weeks, {} days", weeks, remaining_days)
                                                        } else {
                                                            format!("{} weeks", weeks)
                                                        }
                                                    } else {
                                                        format!("{} days", days)
                                                    }
                                                } else {
                                                    "New champion".to_string()
                                                }
                                            }}
                                        </div>
                                        <button
                                            class="btn btn-error btn-xs"
                                            disabled=move || operation_loading.get()
                                            on:click={
                                                let title_id = title.id;
                                                move |_| handle_vacate_title(title_id)
                                            }
                                        >
                                            <Show when=move || operation_loading.get()>
                                                <span class="loading loading-spinner loading-xs"></span>
                                            </Show>
                                            <Show when=move || !operation_loading.get()>
                                                "Vacate"
                                            </Show>
                                        </button>
                                    </div>
                                </div>
                            }
                        }
                    />
                </div>
            </Show>

            // Error message display
            <Show when=move || error_message.get().is_some()>
                <div class="alert alert-error text-sm">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-4 w-4" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>{move || error_message.get().unwrap_or_default()}</span>
                </div>
            </Show>
            
            // Assign new title section
            <div class="mt-4">
                <div class="flex items-center gap-3 mb-2">
                    <span class="text-base-content/70 font-medium text-sm">"Assign New Title:"</span>
                    <Show when=move || loading_assignable.get()>
                        <span class="loading loading-spinner loading-xs"></span>
                    </Show>
                </div>
                
                <Show when=move || !loading_assignable.get() && assignable_titles.get().is_empty()>
                    <div class="text-base-content/50 text-xs italic">
                        "No titles available to assign"
                    </div>
                </Show>
                
                <Show when=move || !loading_assignable.get() && !assignable_titles.get().is_empty()>
                    <select 
                        class="select select-bordered select-sm w-full"
                        disabled=move || operation_loading.get()
                        on:change:target=move |ev| {
                            let value = ev.target().value();
                            if !value.is_empty() && value != "0" {
                                if let Ok(title_id) = value.parse::<i32>() {
                                    handle_assign_title(title_id);
                                }
                                // Reset selection
                                ev.target().set_value("0");
                            }
                        }
                    >
                        <option value="0">"Select a title to assign..."</option>
                        <For
                            each=move || assignable_titles.get()
                            key=|title_with_holders| title_with_holders.title.id
                            children=move |title_with_holders| {
                                let title = title_with_holders.title.clone();
                                let current_holder = if title_with_holders.current_holders.is_empty() {
                                    "Vacant".to_string()
                                } else {
                                    format!("Held by {}", title_with_holders.current_holders[0].wrestler_name)
                                };
                                
                                view! {
                                    <option value=title.id.to_string()>
                                        {format!("{} - {} ({})", title.name, title.division, current_holder)}
                                    </option>
                                }
                            }
                        />
                    </select>
                </Show>
            </div>
        </div>
    }
}