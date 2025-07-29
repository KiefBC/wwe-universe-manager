use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::types::Title;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleHolderInfo {
    pub wrestler_name: String,
    pub wrestler_gender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleWithHolders {
    pub title: Title,
    pub current_holders: Vec<TitleHolderInfo>,
    pub days_held: Option<i32>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_titles() -> Result<Vec<TitleWithHolders>, String> {
    let result = invoke("get_titles", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn open_title_window(title_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "titleId": title_id
    }))
    .map_err(|e| e.to_string())?;

    let _result = invoke("open_title_window", args).await;
    Ok(())
}

#[component]
pub fn TitlesList(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (titles, set_titles) = signal(Vec::<TitleWithHolders>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (search_term, set_search_term) = signal(String::new());
    let (debounced_search_term, set_debounced_search_term) = signal(String::new());

    // Load titles on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match get_titles().await {
                Ok(data) => {
                    set_titles.set(data);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    // Debounce search term (300ms delay)
    Effect::new(move |_| {
        let current_term = search_term.get();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            if search_term.get_untracked() == current_term {
                set_debounced_search_term.set(current_term);
            }
        });
    });

    // Helper function to filter titles by search term
    let filter_titles_by_search = move |titles: Vec<TitleWithHolders>| -> Vec<TitleWithHolders> {
        let term = debounced_search_term.get().to_lowercase();
        if term.is_empty() {
            titles
        } else {
            titles.into_iter()
                .filter(|title| {
                    title.title.name.to_lowercase().contains(&term) ||
                    title.title.division.to_lowercase().contains(&term) ||
                    title.current_holders.iter().any(|holder| 
                        holder.wrestler_name.to_lowercase().contains(&term))
                })
                .collect()
        }
    };

    let handle_title_click = move |title_id: i32| {
        spawn_local(async move {
            if let Err(e) = open_title_window(title_id.to_string()).await {
                web_sys::console::error_1(&format!("Failed to open title window: {}", e).into());
            }
        });
    };

    // Create signals for each tier
    let (tier_1_titles, set_tier_1_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_2_titles, set_tier_2_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_3_titles, set_tier_3_titles) = signal(Vec::<TitleWithHolders>::new());
    let (tier_4_titles, set_tier_4_titles) = signal(Vec::<TitleWithHolders>::new());

    // Update tier signals when titles change
    Effect::new(move |_| {
        let all_titles = titles.get();
        let mut tier_1 = Vec::new();
        let mut tier_2 = Vec::new();
        let mut tier_3 = Vec::new();
        let mut tier_4 = Vec::new();

        for title in all_titles {
            match title.title.prestige_tier {
                1 => tier_1.push(title),
                2 => tier_2.push(title),
                3 => tier_3.push(title),
                _ => tier_4.push(title),
            }
        }

        set_tier_1_titles.set(tier_1);
        set_tier_2_titles.set(tier_2);
        set_tier_3_titles.set(tier_3);
        set_tier_4_titles.set(tier_4);
    });

    // Filtered tier signals based on search term
    let filtered_tier_1_titles = move || filter_titles_by_search(tier_1_titles.get());
    let filtered_tier_2_titles = move || filter_titles_by_search(tier_2_titles.get());
    let filtered_tier_3_titles = move || filter_titles_by_search(tier_3_titles.get());
    let filtered_tier_4_titles = move || filter_titles_by_search(tier_4_titles.get());

    view! {
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn btn-ghost gap-2"
                        on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Dashboard"
                    </button>
                    <button
                        class="btn btn-accent gap-2"
                        on:click=move |_| set_current_page.set("create-title".to_string())
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                        </svg>
                        "Create Title"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "Championship Titles"
                </h1>
                <p class="text-base-content/70">
                    "Select a title to view detailed information and championship history."
                </p>
            </div>

            // Search Input
            <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty()>
                <div class="mb-6">
                    <div class="form-control">
                        <div class="input-group">
                            <input 
                                type="text"
                                placeholder="Search titles by name, division, or champion..."
                                class="input input-bordered w-full"
                                prop:value=move || search_term.get()
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_search_term.set(value);
                                }
                            />
                            <Show when=move || !search_term.get().is_empty()>
                                <button 
                                    class="btn btn-ghost"
                                    on:click=move |_| set_search_term.set(String::new())
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </Show>
                        </div>
                    </div>
                    <div class="text-sm text-base-content/60 mt-2">
                        {move || {
                            let all_titles = titles.get();
                            let filtered_count = filter_titles_by_search(all_titles).len();
                            let search_value = search_term.get();
                            if search_value.is_empty() {
                                format!("{} titles total", filtered_count)
                            } else {
                                format!("{} titles found", filtered_count)
                            }
                        }}
                    </div>
                </div>
            </Show>

            <Show when=move || loading.get()>
                <div class="flex justify-center items-center py-12">
                    <span class="loading loading-spinner loading-lg text-accent"></span>
                    <span class="ml-3 text-base-content/70">"Loading titles..."</span>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error Loading Titles"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && titles.get().is_empty()>
                <div class="alert alert-warning">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" /></svg>
                    <div>
                        <h3 class="font-bold">"No Titles Found"</h3>
                        <div class="text-xs">"No championship titles are currently in the database. Use the Create Title button to add championships to your universe."</div>
                    </div>
                </div>
            </Show>

            // Empty search results
            <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty() && {
                let filtered_1 = filtered_tier_1_titles();
                let filtered_2 = filtered_tier_2_titles();
                let filtered_3 = filtered_tier_3_titles();
                let filtered_4 = filtered_tier_4_titles();
                filtered_1.is_empty() && filtered_2.is_empty() && filtered_3.is_empty() && filtered_4.is_empty()
            }>
                <div class="alert alert-warning">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" /></svg>
                    <div>
                        <h3 class="font-bold">"No Titles Match Your Search"</h3>
                        <div class="text-xs">{format!("No titles found matching \"{}\"", search_term.get())}</div>
                    </div>
                </div>
            </Show>

            <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty()>
                <div class="space-y-12">
                    // Tier 1 - World Championships (2 per row, gold styling)
                    <div>
                        <Show when=move || !filtered_tier_1_titles().is_empty()>
                            <div class="mb-8">
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || filtered_tier_1_titles()
                                        key=|title| title.title.id
                                        children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else if title.current_holders.len() == 1 {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    } else {
                                                        format!("{} & {}", 
                                                            title.current_holders[0].wrestler_name,
                                                            title.current_holders[1].wrestler_name)
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="card bg-warning/20 border border-warning hover:border-warning-focus hover:shadow-lg hover:shadow-warning/30 transition-all duration-300 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="card-body">
                                                                <div class="flex items-center justify-between">
                                                                    <div class="flex-1">
                                                                        <h2 class="card-title text-lg text-warning group-hover:text-warning-focus transition-colors mb-2">
                                                                            {title.title.name.clone()}
                                                                        </h2>
                                                                        <p class="text-warning-content text-sm mb-2">
                                                                            {title.title.division.clone()}
                                                                        </p>
                                                                        <div class="flex items-center space-x-4 text-base-content/70">
                                                                            <span class="font-semibold">
                                                                                "Champion: " <span class="text-warning">{holders_text}</span>
                                                                            </span>
                                                                            <span class="text-base-content/40">
                                                                                "•"
                                                                            </span>
                                                                            <span>
                                                                                {days_text}
                                                                            </span>
                                                                        </div>
                                                                    </div>
                                                                    <div class="text-warning group-hover:text-warning-focus transition-colors">
                                                                        <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                                                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                                        </svg>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                                // Tier 2 - Secondary Championships (2 per row, silver styling)
                        <Show when=move || !filtered_tier_2_titles().is_empty()>
                            <div class="mb-8">
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || filtered_tier_2_titles()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="card bg-base-200 border border-base-300 hover:border-base-content/30 hover:shadow-lg hover:shadow-base-content/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="card-body">
                                                                <h2 class="card-title text-lg text-base-content/70 group-hover:text-base-content transition-colors mb-2">
                                                                    {title.title.name.clone()}
                                                                </h2>
                                                                <p class="text-base-content/50 text-sm mb-3">
                                                                    {title.title.division.clone()}
                                                                </p>
                                                                <div class="space-y-1 text-sm text-base-content/50">
                                                                    <div>
                                                                        "Champion: " <span class="text-base-content/70">{holders_text}</span>
                                                                    </div>
                                                                    <div>
                                                                        {days_text}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                        // Tier 3 - Tag Team Championships (2 per row, bronze styling)
                        <Show when=move || !filtered_tier_3_titles().is_empty()>
                            <div class="mb-8">
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || filtered_tier_3_titles()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else if title.current_holders.len() >= 2 {
                                                        format!("{} & {}", 
                                                            title.current_holders[0].wrestler_name,
                                                            title.current_holders[1].wrestler_name)
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="card bg-base-200 border border-accent/50 hover:border-accent hover:shadow-lg hover:shadow-accent/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="card-body card-compact">
                                                                <h2 class="card-title text-lg text-accent group-hover:text-accent-focus transition-colors mb-2">
                                                                    {title.title.name.clone()}
                                                                </h2>
                                                                <p class="text-accent-content text-sm mb-2">
                                                                    {title.title.division.clone()}
                                                                </p>
                                                                <div class="space-y-1 text-sm text-base-content/60">
                                                                    <div>
                                                                        "Champions: " <span class="text-accent">{holders_text}</span>
                                                                    </div>
                                                                    <div>
                                                                        {days_text}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>

                        // Tier 4 - Specialty Championships (2 per row, themed colors)
                        <Show when=move || !filtered_tier_4_titles().is_empty()>
                            <div class="mb-8">
                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                    <For
                                        each=move || filtered_tier_4_titles()
                                        key=|title| title.title.id
                                                children=move |title| {
                                                    let title_id = title.title.id;
                                                    let holders_text = if title.current_holders.is_empty() {
                                                        "Vacant".to_string()
                                                    } else {
                                                        title.current_holders[0].wrestler_name.clone()
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());

                                                    view! {
                                                        <div class="card bg-base-200 border border-secondary/40 hover:border-secondary hover:shadow-lg hover:shadow-secondary/20 transition-all duration-200 cursor-pointer group"
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="card-body card-compact">
                                                                <h2 class="card-title text-lg text-secondary group-hover:text-secondary-focus transition-colors mb-2">
                                                                    {title.title.name.clone()}
                                                                </h2>
                                                                <p class="text-secondary-content text-sm mb-2">
                                                                    {title.title.division.clone()}
                                                                </p>
                                                                <div class="space-y-1 text-sm text-base-content/50">
                                                                    <div>
                                                                        <span class="text-secondary">{holders_text}</span>
                                                                    </div>
                                                                    <div>
                                                                        {days_text}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </Show>
                    </div>
                </div>
            </Show>
        </div>
    }
}