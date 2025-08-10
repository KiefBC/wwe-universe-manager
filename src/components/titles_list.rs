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
    let (view_mode, set_view_mode) = signal("cards".to_string());

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

    // Get all filtered titles
    let get_filtered_titles = move || -> Vec<TitleWithHolders> {
        filter_titles_by_search(titles.get())
    };

    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-accent/10 via-secondary/10 to-primary/10 rounded-none border-b border-accent/20 mb-6 sm:mb-8">
                <div class="hero-content text-center py-4 sm:py-6">
                    <div class="max-w-6xl w-full">
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-accent via-secondary to-primary bg-clip-text text-transparent mb-6">
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
                                class="btn btn-accent gap-2 w-full sm:w-auto min-h-[44px]"
                                on:click=move |_| set_current_page.set("create-title".to_string())
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                </svg>
                                "Create Championship"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="px-4 sm:px-6 lg:px-8 pb-8 sm:pb-12">
                <div class="max-w-6xl mx-auto space-y-6">
                    
                    // Search and View Controls Section
                    <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty()>
                        <section>
                            <div class="mb-6">
                                <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Championship Search"</h2>
                                <p class="text-base-content/70 text-sm sm:text-base">"Find championships by name, division, or current champion"</p>
                            </div>
                            
                            <div class="card bg-gradient-to-r from-accent/5 to-accent/2 border border-accent/20 shadow-lg">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex flex-col lg:flex-row gap-4 lg:gap-6">
                                        // Search Input
                                        <div class="flex-1">
                                            <div class="form-control">
                                                <div class="input-group">
                                                    <span class="bg-accent/10 border-accent/20">
                                                        <svg class="w-5 h-5 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                                        </svg>
                                                    </span>
                                                    <input 
                                                        type="text"
                                                        placeholder="Search championships, divisions, or champions..."
                                                        class="input input-bordered flex-1 bg-base-100 focus:border-accent focus:outline-none min-h-[44px]"
                                                        prop:value=move || search_term.get()
                                                        on:input=move |ev| {
                                                            let value = event_target_value(&ev);
                                                            set_search_term.set(value);
                                                        }
                                                    />
                                                    <Show when=move || !search_term.get().is_empty()>
                                                        <button 
                                                            class="btn btn-ghost min-h-[44px]"
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
                                        
                                        // View Toggle
                                        <div class="flex items-center gap-4">
                                            <span class="text-sm font-medium text-base-content/70 hidden sm:inline">"View:"</span>
                                            <div class="join">
                                                <button class=move || format!("btn btn-sm join-item min-h-[36px] {}", 
                                                    if view_mode.get() == "cards" { "btn-primary" } else { "btn-ghost" })
                                                    on:click=move |_| set_view_mode.set("cards".to_string())>
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                                                    </svg>
                                                    <span class="hidden sm:inline ml-1">"Cards"</span>
                                                </button>
                                                <button class=move || format!("btn btn-sm join-item min-h-[36px] {}", 
                                                    if view_mode.get() == "rows" { "btn-primary" } else { "btn-ghost" })
                                                    on:click=move |_| set_view_mode.set("rows".to_string())>
                                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"/>
                                                    </svg>
                                                    <span class="hidden sm:inline ml-1">"List"</span>
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                    
                                    // Results Summary
                                    <div class="mt-4 flex items-center justify-between">
                                        <div class="text-sm text-base-content/60">
                                            {move || {
                                                let filtered_count = get_filtered_titles().len();
                                                let search_value = search_term.get();
                                                if search_value.is_empty() {
                                                    format!("{} championships total", filtered_count)
                                                } else {
                                                    format!("{} championships found", filtered_count)
                                                }
                                            }}
                                        </div>
                                        <div class="flex items-center gap-2">
                                            <div class="badge badge-accent badge-sm">{move || view_mode.get().to_uppercase()}</div>
                                            <div class="badge badge-ghost badge-sm">"VIEW"</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>
                    </Show>

                    // Loading State
                    <Show when=move || loading.get()>
                        <section>
                            <div class="flex flex-col items-center justify-center py-8 sm:py-12">
                                <div class="loading loading-spinner loading-lg text-accent mb-4"></div>
                                <div class="text-base-content/70 text-sm">"Loading championship portfolio..."</div>
                            </div>
                        </section>
                    </Show>

                    // Error State
                    <Show when=move || error.get().is_some()>
                        <section>
                            <div class="alert alert-error shadow-lg border border-error/20">
                                <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                                <div>
                                    <h3 class="font-bold">"Error Loading Championships"</h3>
                                    <div class="text-sm opacity-80">{move || error.get().unwrap_or_default()}</div>
                                </div>
                            </div>
                        </section>
                    </Show>

                    // Empty State
                    <Show when=move || !loading.get() && error.get().is_none() && titles.get().is_empty()>
                        <section>
                            <div class="alert alert-warning shadow-lg border border-warning/20">
                                <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                                </svg>
                                <div>
                                    <h3 class="font-bold">"No Championships Available"</h3>
                                    <div class="text-sm opacity-80">"No championship titles are currently in the database. Use the Create Championship button to add titles to your universe."</div>
                                </div>
                            </div>
                        </section>
                    </Show>

                    // Empty Search Results
                    <Show when=move || !loading.get() && error.get().is_none() && !titles.get().is_empty() && get_filtered_titles().is_empty()>
                        <section>
                            <div class="alert alert-info shadow-lg border border-info/20">
                                <svg class="w-6 h-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                                <div>
                                    <h3 class="font-bold">"No Championships Match Your Search"</h3>
                                    <div class="text-sm opacity-80">{format!("No championships found matching \"{}\". Try a different search term.", search_term.get())}</div>
                                </div>
                            </div>
                        </section>
                    </Show>

                    // Championships Display
                    <Show when=move || !loading.get() && error.get().is_none() && !get_filtered_titles().is_empty()>
                        <section>
                            <div class="mb-6">
                                <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Active Championships"</h2>
                                <p class="text-base-content/70 text-sm sm:text-base">"Professional championship portfolio with executive management features"</p>
                            </div>
                            
                            {move || {
                                let filtered_titles = get_filtered_titles();
                                
                                if view_mode.get() == "rows" {
                                    // Row View Layout
                                    view! {
                                        <div class="card bg-base-100 shadow-lg border border-base-300/50">
                                            // Desktop Header
                                            <div class="hidden lg:block">
                                                <div class="grid grid-cols-12 gap-4 p-4 border-b border-base-300/50 bg-base-200/50 font-semibold text-sm text-base-content/80">
                                                    <div class="col-span-1">"Tier"</div>
                                                    <div class="col-span-3">"Championship"</div>
                                                    <div class="col-span-2">"Division"</div>
                                                    <div class="col-span-3">"Current Champion"</div>
                                                    <div class="col-span-2">"Reign Length"</div>
                                                    <div class="col-span-1 text-center">"Action"</div>
                                                </div>
                                            </div>
                                            
                                            // Title Rows
                                            <div class="divide-y divide-base-300/30">
                                                <For
                                                    each=move || filtered_titles.clone()
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
                                                                title.current_holders.get(1).map(|h| h.wrestler_name.as_str()).unwrap_or(""))
                                                        };
                                                        
                                                        let days_text = title.days_held
                                                            .map(|days| format!("{} days", days))
                                                            .unwrap_or_else(|| "--".to_string());
                                                            
                                                        // Clone strings for reuse in mobile view
                                                        let holders_text_mobile = holders_text.clone();
                                                        let days_text_mobile = days_text.clone();
                                                            
                                                        let (tier_color, tier_bg) = match title.title.prestige_tier {
                                                            1 => ("text-warning", "bg-warning/10"),
                                                            2 => ("text-info", "bg-info/10"),
                                                            3 => ("text-accent", "bg-accent/10"),
                                                            _ => ("text-secondary", "bg-secondary/10")
                                                        };
                                                        
                                                        view! {
                                                            // Desktop Row
                                                            <div class="hidden lg:grid lg:grid-cols-12 gap-4 p-4 hover:bg-base-200/50 transition-colors cursor-pointer group"
                                                                 on:click=move |_| handle_title_click(title_id)>
                                                                <div class="col-span-1 flex items-center">
                                                                    <div class=format!("badge badge-sm {} {}", tier_color, tier_bg)>
                                                                        {format!("T{}", title.title.prestige_tier)}
                                                                    </div>
                                                                </div>
                                                                <div class="col-span-3 flex items-center font-medium text-base-content truncate">
                                                                    {title.title.name.clone()}
                                                                </div>
                                                                <div class="col-span-2 flex items-center text-sm text-base-content/70 truncate">
                                                                    {title.title.division.clone()}
                                                                </div>
                                                                <div class="col-span-3 flex items-center text-sm truncate">
                                                                    <span class=if title.current_holders.is_empty() { "text-base-content/40 italic" } else { "text-base-content" }>
                                                                        {holders_text}
                                                                    </span>
                                                                </div>
                                                                <div class="col-span-2 flex items-center text-xs text-base-content/60">
                                                                    {days_text}
                                                                </div>
                                                                <div class="col-span-1 flex items-center justify-center">
                                                                    <button class="btn btn-ghost btn-xs group-hover:btn-primary">
                                                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                                                                        </svg>
                                                                    </button>
                                                                </div>
                                                            </div>
                                                            
                                                            // Mobile Row
                                                            <div class="lg:hidden p-4 hover:bg-base-200/50 transition-colors cursor-pointer"
                                                                 on:click=move |_| handle_title_click(title_id)>
                                                                <div class="flex items-start justify-between mb-2">
                                                                    <div class="flex-1 min-w-0">
                                                                        <h3 class="font-semibold text-base-content truncate">{title.title.name.clone()}</h3>
                                                                        <p class="text-sm text-base-content/70 truncate">{title.title.division.clone()}</p>
                                                                    </div>
                                                                    <div class=format!("badge badge-sm {} {}", tier_color, tier_bg)>
                                                                        {format!("Tier {}", title.title.prestige_tier)}
                                                                    </div>
                                                                </div>
                                                                <div class="flex items-center justify-between text-sm">
                                                                    <span class=if title.current_holders.is_empty() { "text-base-content/40 italic" } else { "text-base-content/80" }>
                                                                        {if title.current_holders.is_empty() { "Vacant" } else { "Champion" }}: <span class="font-medium">{holders_text_mobile}</span>
                                                                    </span>
                                                                    <span class="text-xs text-base-content/60">{days_text_mobile}</span>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </div>
                                    }.into_any()
                                } else {
                                    // Card View Layout
                                    view! {
                                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
                                            <For
                                                each=move || filtered_titles.clone()
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
                                                            title.current_holders.get(1).map(|h| h.wrestler_name.as_str()).unwrap_or(""))
                                                    };
                                                    
                                                    let days_text = title.days_held
                                                        .map(|days| format!("{} days", days))
                                                        .unwrap_or_else(|| "No current holder".to_string());
                                                        
                                                    let (tier_color, tier_bg, tier_label) = match title.title.prestige_tier {
                                                        1 => ("text-warning", "bg-warning/10", "WORLD"),
                                                        2 => ("text-info", "bg-info/10", "SECONDARY"),
                                                        3 => ("text-accent", "bg-accent/10", "TAG TEAM"),
                                                        _ => ("text-secondary", "bg-secondary/10", "SPECIALTY")
                                                    };

                                                    view! {
                                                        <div class=format!("card {} border border-base-300/50 hover:border-accent/30 hover:shadow-lg hover:shadow-accent/10 transition-all duration-200 cursor-pointer group", tier_bg)
                                                             on:click=move |_| handle_title_click(title_id)>
                                                            <div class="card-body p-4 sm:p-6">
                                                                <div class="flex items-start gap-3 mb-4">
                                                                    <div class=format!("w-12 h-12 {} rounded-xl flex items-center justify-center", tier_bg)>
                                                                        <svg class=format!("w-7 h-7 {}", tier_color) fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                                                        </svg>
                                                                    </div>
                                                                    <div class=format!("badge badge-sm {}", tier_color)>{tier_label}</div>
                                                                </div>
                                                                
                                                                <h3 class="text-lg font-bold text-base-content mb-2 group-hover:text-accent transition-colors">
                                                                    {title.title.name.clone()}
                                                                </h3>
                                                                <p class="text-base-content/70 text-sm mb-3">
                                                                    {title.title.division.clone()}
                                                                </p>
                                                                
                                                                <div class="space-y-2 text-sm">
                                                                    <div class="flex items-center justify-between">
                                                                        <span class="text-base-content/60">"Champion:"</span>
                                                                        <span class=if title.current_holders.is_empty() { "text-base-content/40 italic" } else { "text-accent font-medium" }>
                                                                            {holders_text}
                                                                        </span>
                                                                    </div>
                                                                    <div class="flex items-center justify-between">
                                                                        <span class="text-base-content/60">"Reign:"</span>
                                                                        <span class="text-base-content/80">{days_text}</span>
                                                                    </div>
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
                        </section>
                    </Show>
                </div>
            </div>
        </div>
    }
}