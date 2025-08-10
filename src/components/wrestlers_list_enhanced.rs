use leptos::prelude::*;
use leptos::html;
use leptos::ev;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::types::Gender;
use crate::components::executive_layout::{ExecutivePageLayout, ExecutiveContentSection};
use crate::components::{ResponsiveGrid, ResponsiveExecutiveCard};
use crate::components::executive_cards::{ExecutiveCard, ExecutiveMetricCard};
use crate::components::executive_buttons::{ExecutiveButton, ExecutiveButtonGroup, ExecutiveIconButton};
use crate::components::executive_typography::ExecutiveHeading;
use crate::components::animation_utils::*;
use crate::components::loading_states::*;
use crate::components::form_enhancements::*;
use crate::components::enhanced_buttons::*;
use crate::components::notification_system::*;
use crate::utils::navigation::NavigationContext;

/// Wrestler data structure for frontend display
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

async fn get_wrestlers() -> Result<Vec<Wrestler>, String> {
    let result = invoke("get_wrestlers", JsValue::NULL).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

async fn open_wrestler_window(wrestler_id: String) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerId": wrestler_id
    }))
    .map_err(|e| e.to_string())?;

    let _result = invoke("open_wrestler_window", args).await;
    Ok(())
}

/// Enhanced Wrestlers List Component with Phase 4.5 Micro-Interactions
/// 
/// Professional talent management interface featuring:
/// - Professional loading states with skeleton screens
/// - Smooth animations and transitions
/// - Enhanced button interactions with ripple effects
/// - Real-time form validation and feedback
/// - Professional notification system
/// - Staggered entrance animations for cards
#[component]
pub fn WrestlersListEnhanced(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (wrestlers, set_wrestlers) = signal(Vec::<Wrestler>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (search_term, set_search_term) = signal(String::new());
    let (debounced_search_term, set_debounced_search_term) = signal(String::new());
    let (selected_wrestlers, set_selected_wrestlers) = signal(std::collections::HashSet::<i32>::new());
    let (view_mode, set_view_mode) = signal("grid".to_string());
    let (sort_field, set_sort_field) = signal("name".to_string());
    let (sort_direction, set_sort_direction) = signal("asc".to_string());
    let (filter_gender, set_filter_gender) = signal("all".to_string());
    let (filter_type, set_filter_type) = signal("all".to_string());
    let (show_success_toast, set_show_success_toast) = signal(false);
    let (success_message, set_success_message) = signal(String::new());

    // Initialize notification manager
    let notification_manager = NotificationManager::new();

    // Enhanced data loading with professional feedback
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            // Show loading notification
            let loading_id = notification_manager.show_loading("Loading Talent", "Fetching wrestler roster...");
            
            match get_wrestlers().await {
                Ok(data) => {
                    notification_manager.dismiss(&loading_id);
                    set_wrestlers.set(data.clone());
                    set_error.set(None);
                    
                    // Show success notification with count
                    notification_manager.show_success(
                        "Talent Loaded", 
                        &format!("Successfully loaded {} wrestlers", data.len())
                    );
                }
                Err(e) => {
                    notification_manager.dismiss(&loading_id);
                    set_error.set(Some(e.clone()));
                    
                    // Show error notification
                    notification_manager.show_error(
                        "Loading Failed",
                        &format!("Failed to load wrestlers: {}", e)
                    );
                }
            }
            set_loading.set(false);
        });
    });

    // Enhanced debounced search with validation
    Effect::new(move |_| {
        let current_term = search_term.get();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(300).await;
            if search_term.get_untracked() == current_term {
                set_debounced_search_term.set(current_term);
            }
        });
    });

    // Enhanced filtering and sorting with performance metrics
    let filtered_and_sorted_wrestlers = move || {
        let mut result = wrestlers.get();
        
        // Apply filters with smooth transitions
        let term = debounced_search_term.get().to_lowercase();
        if !term.is_empty() {
            result = result.into_iter()
                .filter(|w| {
                    w.name.to_lowercase().contains(&term) || 
                    w.nickname.as_ref().map_or(false, |n| n.to_lowercase().contains(&term)) ||
                    w.real_name.as_ref().map_or(false, |r| r.to_lowercase().contains(&term))
                })
                .collect();
        }
        
        let gender_filter = filter_gender.get();
        if gender_filter != "all" {
            result = result.into_iter()
                .filter(|w| {
                    match gender_filter.as_str() {
                        "male" => w.gender == Gender::Male,
                        "female" => w.gender == Gender::Female,
                        _ => true,
                    }
                })
                .collect();
        }
        
        let type_filter = filter_type.get();
        if type_filter != "all" {
            result = result.into_iter()
                .filter(|w| {
                    match type_filter.as_str() {
                        "user" => w.is_user_created.unwrap_or(false),
                        "system" => !w.is_user_created.unwrap_or(false),
                        _ => true,
                    }
                })
                .collect();
        }
        
        // Apply sorting with smooth state transitions
        let sort_by = sort_field.get();
        let sort_dir = sort_direction.get();
        result.sort_by(|a, b| {
            let ordering = match sort_by.as_str() {
                "name" => a.name.cmp(&b.name),
                "wins" => a.wins.cmp(&b.wins),
                "losses" => a.losses.cmp(&b.losses),
                "winrate" => {
                    let a_rate = if a.wins + a.losses == 0 { 0.0 } else { a.wins as f64 / (a.wins + a.losses) as f64 };
                    let b_rate = if b.wins + b.losses == 0 { 0.0 } else { b.wins as f64 / (b.wins + b.losses) as f64 };
                    a_rate.partial_cmp(&b_rate).unwrap_or(std::cmp::Ordering::Equal)
                },
                "gender" => a.gender.to_string().cmp(&b.gender.to_string()),
                _ => a.name.cmp(&b.name),
            };
            if sort_dir == "desc" { ordering.reverse() } else { ordering }
        });
        
        result
    };

    // Enhanced wrestler click handler with feedback
    let handle_wrestler_click = move |wrestler_id: i32, wrestler_name: String| {
        spawn_local(async move {
            match open_wrestler_window(wrestler_id.to_string()).await {
                Ok(_) => {
                    notification_manager.show_success(
                        "Opening Details",
                        &format!("Opening {} profile in new window", wrestler_name)
                    );
                }
                Err(e) => {
                    notification_manager.show_error(
                        "Window Error",
                        &format!("Failed to open wrestler window: {}", e)
                    );
                }
            }
        });
    };

    // Calculate display metrics
    let total_wrestlers = move || wrestlers.get().len();
    let filtered_count = move || filtered_and_sorted_wrestlers().len();
    let selected_count = move || selected_wrestlers.get().len();

    view! {
        <ExecutivePageLayout
            title="Talent Management".to_string()
            description=Some("Professional wrestler roster and talent development center".to_string())
            navigation_context=NavigationContext {
                current_section: "Wrestling Operations".to_string(),
                current_page: "Talent Management".to_string(),
                breadcrumbs: vec![
                    ("Dashboard".to_string(), "dashboard".to_string()),
                    ("Talent Management".to_string(), "wrestlers".to_string()),
                ],
            }
            actions=Some(view! {
                <ExecutiveButtonGroup>
                    <ExecutiveActionButton
                        variant="primary".to_string()
                        size="sm".to_string()
                        icon=Some(view! {
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                        }.into_any())
                        on_click=Some(Box::new(move |_| {
                            set_current_page.set("create-wrestler".to_string());
                        }))
                    >
                        "Add Talent"
                    </ExecutiveActionButton>
                    
                    <ExecutiveToggleButton
                        label="Grid View".to_string()
                        active=Signal::derive(move || view_mode.get() == "grid")
                        set_active=WriteSignal::derive(move |grid_mode| {
                            if grid_mode {
                                set_view_mode.set("grid".to_string());
                            } else {
                                set_view_mode.set("table".to_string());
                            }
                        })
                        active_icon=Some(view! {
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path>
                            </svg>
                        }.into_any())
                        inactive_icon=Some(view! {
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path>
                            </svg>
                        }.into_any())
                    />
                </ExecutiveButtonGroup>
            }.into_any())
        >
            // Executive Metrics Dashboard with animations
            <ExecutiveContentSection
                title="Talent Portfolio Metrics".to_string()
                class="mb-6".to_string()
            >
                <div class="stats stats-vertical sm:stats-horizontal shadow-lg border border-base-300/50 bg-gradient-to-br from-base-100 to-base-200/50 w-full">
                    <div class="stat animate-card-entrance">
                        <div class="stat-figure text-primary">
                            <div class="w-10 h-10 bg-primary/20 rounded-full flex items-center justify-center animate-metric-counter">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="stat-title text-primary/80">"Total Roster"</div>
                        <div class="stat-value text-primary text-2xl font-bold animate-metric-counter">{move || total_wrestlers()}</div>
                        <div class="stat-desc text-primary/60">
                            {move || if total_wrestlers() == 0 { 
                                "Build your talent pipeline" 
                            } else { 
                                "Active wrestling talent" 
                            }}
                        </div>
                    </div>

                    <div class="stat animate-card-entrance" style="animation-delay: 100ms">
                        <div class="stat-figure text-secondary">
                            <div class="w-10 h-10 bg-secondary/20 rounded-full flex items-center justify-center animate-metric-counter">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="stat-title text-secondary/80">"Filtered Results"</div>
                        <div class="stat-value text-secondary text-2xl font-bold animate-metric-counter">{move || filtered_count()}</div>
                        <div class="stat-desc text-secondary/60">
                            {move || if filtered_count() != total_wrestlers() { 
                                format!("of {} total wrestlers", total_wrestlers())
                            } else { 
                                "All wrestlers visible".to_string()
                            }}
                        </div>
                    </div>

                    <div class="stat animate-card-entrance" style="animation-delay: 200ms">
                        <div class="stat-figure text-accent">
                            <div class="w-10 h-10 bg-accent/20 rounded-full flex items-center justify-center animate-metric-counter">
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
                                </svg>
                            </div>
                        </div>
                        <div class="stat-title text-accent/80">"Selected"</div>
                        <div class="stat-value text-accent text-2xl font-bold animate-metric-counter">{move || selected_count()}</div>
                        <div class="stat-desc text-accent/60">
                            {move || if selected_count() > 0 { 
                                "Ready for bulk operations" 
                            } else { 
                                "Click wrestlers to select" 
                            }}
                        </div>
                    </div>
                </div>
            </ExecutiveContentSection>

            // Professional Search and Filter Controls with enhanced animations
            <ExecutiveContentSection
                title="Search & Filter Controls".to_string()
                class="mb-6".to_string()
            >
                <ResponsiveGrid
                    columns="grid-cols-1 sm:grid-cols-2 lg:grid-cols-4".to_string()
                    gap="gap-mobile-md sm:gap-tablet-sm lg:gap-4".to_string()
                    class="animate-stagger-children".to_string()
                >
                    <ExecutiveInput
                        label="Search Talent".to_string()
                        input_type="search".to_string()
                        value=search_term.into()
                        set_value=set_search_term
                        placeholder="Search by name, nickname, or real name...".to_string()
                        helper=Some("Live search with 300ms debounce".to_string())
                        class="animate-card-entrance".to_string()
                        validate=Some(Box::new(|term: &String| {
                            if term.len() > 50 {
                                Some("Search term too long".to_string())
                            } else {
                                None
                            }
                        }))
                    />
                    
                    <ExecutiveSelect
                        label="Gender Filter".to_string()
                        value=filter_gender.into()
                        set_value=set_filter_gender
                        options=vec![
                            ("all".to_string(), "All Genders".to_string()),
                            ("male".to_string(), "Male".to_string()),
                            ("female".to_string(), "Female".to_string()),
                        ]
                        class="animate-card-entrance".to_string()
                        helper=Some("Filter by wrestler gender".to_string())
                    />
                    
                    <ExecutiveSelect
                        label="Talent Type".to_string()
                        value=filter_type.into()
                        set_value=set_filter_type
                        options=vec![
                            ("all".to_string(), "All Types".to_string()),
                            ("user".to_string(), "User Created".to_string()),
                            ("system".to_string(), "System Generated".to_string()),
                        ]
                        class="animate-card-entrance".to_string()
                        helper=Some("Filter by creation source".to_string())
                    />
                    
                    <ExecutiveSelect
                        label="Sort By".to_string()
                        value=sort_field.into()
                        set_value=set_sort_field
                        options=vec![
                            ("name".to_string(), "Name".to_string()),
                            ("wins".to_string(), "Wins".to_string()),
                            ("losses".to_string(), "Losses".to_string()),
                            ("winrate".to_string(), "Win Rate".to_string()),
                            ("gender".to_string(), "Gender".to_string()),
                        ]
                        class="animate-card-entrance".to_string()
                        helper=Some("Sort wrestlers by field".to_string())
                    />
                </ResponsiveGrid>
            </ExecutiveContentSection>

            // Main Content Area with loading states and animations
            <ExecutiveContentSection
                title=format!("Talent Roster ({} wrestlers)", filtered_count())
                class="".to_string()
            >
                {move || {
                    if loading.get() {
                        // Professional loading state with skeleton cards
                        view! {
                            <CardGridSkeleton 
                                card_type="wrestler".to_string()
                                count=6
                                grid_cols="grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4".to_string()
                            />
                        }.into_any()
                    } else if let Some(error_msg) = error.get() {
                        // Professional error state with retry functionality
                        view! {
                            <ExecutiveErrorState
                                title="Data Loading Error".to_string()
                                message=error_msg
                                on_retry=Some(Box::new(move || {
                                    // Trigger data reload
                                    set_loading.set(true);
                                    spawn_local(async move {
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
                                }))
                                show_support=true
                            />
                        }.into_any()
                    } else {
                        let filtered_wrestlers = filtered_and_sorted_wrestlers();
                        
                        if filtered_wrestlers.is_empty() {
                            // Professional empty state with action suggestions
                            view! {
                                <ExecutiveEmptyState
                                    title="No Wrestlers Found".to_string()
                                    description="No wrestlers match your current filters. Try adjusting your search criteria or add some wrestlers to get started.".to_string()
                                    action_text="Add Wrestler".to_string()
                                    on_action=Some(Box::new(move || {
                                        set_current_page.set("create-wrestler".to_string());
                                    }))
                                    icon=Some(view! {
                                        <svg class="w-12 h-12 text-base-content/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                        </svg>
                                    }.into_any())
                                    show_suggestions=true
                                />
                            }.into_any()
                        } else {
                            // Wrestler cards with staggered entrance animations
                            view! {
                                <StaggeredContainer
                                    stagger_delay=50
                                    class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4".to_string()
                                >
                                    {filtered_wrestlers.into_iter().enumerate().map(|(index, wrestler)| {
                                        let wrestler_id = wrestler.id;
                                        let wrestler_name = wrestler.name.clone();
                                        let is_selected = selected_wrestlers.get().contains(&wrestler_id);
                                        
                                        view! {
                                            <AnimatedCard
                                                delay=(index as u32 * 50)
                                                class="h-full".to_string()
                                            >
                                                <WrestlerCardEnhanced 
                                                    wrestler=wrestler
                                                    selected=is_selected
                                                    on_click=Box::new(move || {
                                                        handle_wrestler_click(wrestler_id, wrestler_name.clone());
                                                    })
                                                    on_select=Box::new(move |selected| {
                                                        set_selected_wrestlers.update(|set| {
                                                            if selected {
                                                                set.insert(wrestler_id);
                                                            } else {
                                                                set.remove(&wrestler_id);
                                                            }
                                                        });
                                                    })
                                                />
                                            </AnimatedCard>
                                        }
                                    }).collect::<Vec<_>>()}
                                </StaggeredContainer>
                            }.into_any()
                        }
                    }
                }}
            </ExecutiveContentSection>
            
            // Notification Container
            <ExecutiveNotificationContainer 
                manager=notification_manager
                position="top-right".to_string()
            />
        </ExecutivePageLayout>
    }
}

/// Enhanced wrestler card component with micro-interactions
#[component]
fn WrestlerCardEnhanced(
    wrestler: Wrestler,
    selected: bool,
    on_click: Box<dyn Fn() + 'static>,
    on_select: Box<dyn Fn(bool) + 'static>,
) -> impl IntoView {
    let card_ref = NodeRef::<html::Div>::new();
    
    let handle_card_click = move |_| {
        // Add click animation
        if let Some(card) = card_ref.get() {
            let _ = card.class_list().add_1("animate-button-press");
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(150).await;
                let _ = card.class_list().remove_1("animate-button-press");
            });
        }
        on_click();
    };

    let handle_select = move |e: ev::Event| {
        e.stop_propagation();
        on_select(!selected);
    };

    let win_rate = if wrestler.wins + wrestler.losses == 0 {
        0.0
    } else {
        (wrestler.wins as f32 / (wrestler.wins + wrestler.losses) as f32) * 100.0
    };

    let card_class = if selected {
        "card bg-primary/10 border border-primary shadow-lg cursor-pointer transition-all duration-professional hover:shadow-premium hover:animate-executive-hover"
    } else {
        "card bg-base-100 border border-base-300/50 shadow-lg cursor-pointer transition-all duration-professional hover:shadow-executive hover:animate-executive-hover"
    };

    view! {
        <div 
            node_ref=card_ref
            class=card_class
            on:click=handle_card_click
        >
            <div class="card-body p-4">
                // Header with selection checkbox
                <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-3">
                        <div class=format!("w-10 h-10 rounded-full flex items-center justify-center font-bold text-white text-sm {}",
                            match wrestler.gender {
                                Gender::Male => "bg-blue-500",
                                Gender::Female => "bg-pink-500",
                            }
                        )>
                            {move || wrestler.name.chars().next().unwrap_or('?').to_uppercase().to_string()}
                        </div>
                        <div>
                            <h3 class="font-bold text-base-content leading-tight">{wrestler.name.clone()}</h3>
                            {wrestler.nickname.map(|nick| view! {
                                <p class="text-xs text-base-content/60 italic">"\"{nick}\""</p>
                            })}
                        </div>
                    </div>
                    
                    <input 
                        type="checkbox"
                        class="checkbox checkbox-primary transition-all duration-quick hover:animate-button-press"
                        checked=selected
                        on:click=handle_select
                    />
                </div>
                
                // Stats row with animations
                <div class="grid grid-cols-2 gap-2 mb-3">
                    <div class="bg-base-200/50 rounded-lg p-2 text-center transition-all duration-quick hover:bg-base-200">
                        <div class="text-lg font-bold text-success animate-metric-counter">{wrestler.wins}</div>
                        <div class="text-xs text-base-content/60">Wins</div>
                    </div>
                    <div class="bg-base-200/50 rounded-lg p-2 text-center transition-all duration-quick hover:bg-base-200">
                        <div class="text-lg font-bold text-error animate-metric-counter">{wrestler.losses}</div>
                        <div class="text-xs text-base-content/60">Losses</div>
                    </div>
                </div>
                
                // Win rate progress bar
                <div class="mb-3">
                    <div class="flex items-center justify-between text-xs mb-1">
                        <span class="text-base-content/60">Win Rate</span>
                        <span class="font-medium">{format!("{:.1}%", win_rate)}</span>
                    </div>
                    <AnimatedProgress 
                        value=win_rate.into()
                        color="success".to_string()
                        show_percentage=false
                        class="".to_string()
                    />
                </div>
                
                // Action buttons with enhanced interactions
                <div class="flex gap-2">
                    <ExecutiveActionButton
                        variant="primary".to_string()
                        size="xs".to_string()
                        on_click=Some(Box::new(move |e| {
                            e.stop_propagation();
                            on_click();
                        }))
                        icon=Some(view! {
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                            </svg>
                        }.into_any())
                        ripple=true
                    >
                        "View"
                    </ExecutiveActionButton>
                    
                    <ExecutiveActionButton
                        variant="ghost".to_string()
                        size="xs".to_string()
                        on_click=Some(Box::new(move |e| {
                            e.stop_propagation();
                            // Edit functionality would go here
                        }))
                        icon=Some(view! {
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                            </svg>
                        }.into_any())
                        ripple=true
                    >
                        "Edit"
                    </ExecutiveActionButton>
                </div>
            </div>
        </div>
    }
}