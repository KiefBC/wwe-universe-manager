use crate::components::*;
use crate::types::{fetch_executive_metrics, ExecutiveMetrics};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_json;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Accessible Executive Dashboard Component
/// 
/// WCAG 2.1 AA compliant professional wrestling empire management platform:
/// - Full keyboard navigation and screen reader support
/// - High contrast mode compatibility
/// - 44px minimum touch targets for motor accessibility
/// - Professional interface designed for inclusive executive use
#[component]
pub fn AccessibleExecutiveDashboard(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
    /// Signal that triggers data refresh when incremented
    refresh_trigger: ReadSignal<u32>,
    /// Signal to set refresh trigger for data reload
    set_refresh_trigger: WriteSignal<u32>,
) -> impl IntoView {
    // Executive metrics state
    let metrics = RwSignal::new(None::<ExecutiveMetrics>);
    let metrics_loading = RwSignal::new(true);
    let metrics_error = RwSignal::new(None::<String>);
    
    // Accessibility state
    let screen_reader_announcements = RwSignal::new(String::new());
    let keyboard_shortcuts_visible = RwSignal::new(false);
    let focus_manager = RwSignal::new(FocusManager::new());
    
    // Command palette state for power users
    let command_palette_open = RwSignal::new(false);
    let command_search = RwSignal::new(String::new());
    let command_selection = RwSignal::new(0usize);
    
    // Available commands for power users
    let available_commands = Signal::derive(move || {
        vec![
            Command {
                id: "talent".to_string(),
                title: "Talent Management".to_string(),
                description: Some("Manage wrestlers and their profiles".to_string()),
                shortcut: Some("Ctrl+T".to_string()),
                action: "wrestlers".to_string(),
            },
            Command {
                id: "shows".to_string(),
                title: "Show Management".to_string(),
                description: Some("Manage show rosters and assignments".to_string()),
                shortcut: Some("Ctrl+S".to_string()),
                action: "show-roster".to_string(),
            },
            Command {
                id: "championships".to_string(),
                title: "Championship Management".to_string(),
                description: Some("Manage titles and champions".to_string()),
                shortcut: Some("Ctrl+C".to_string()),
                action: "titles".to_string(),
            },
            Command {
                id: "booker".to_string(),
                title: "Match Booking".to_string(),
                description: Some("Create and manage matches".to_string()),
                shortcut: Some("Ctrl+B".to_string()),
                action: "booker".to_string(),
            },
            Command {
                id: "analytics".to_string(),
                title: "Analytics Dashboard".to_string(),
                description: Some("View business intelligence and reports".to_string()),
                shortcut: Some("Ctrl+A".to_string()),
                action: "analytics".to_string(),
            },
            Command {
                id: "command-center".to_string(),
                title: "Executive Command Center".to_string(),
                description: Some("System monitoring and administration".to_string()),
                shortcut: Some("Ctrl+E".to_string()),
                action: "command-center".to_string(),
            },
        ]
    });
    
    // Available keyboard shortcuts
    let keyboard_shortcuts = vec![
        KeyboardShortcut {
            keys: vec!["Ctrl".to_string(), "K".to_string()],
            description: "Open command palette".to_string(),
        },
        KeyboardShortcut {
            keys: vec!["?".to_string()],
            description: "Show keyboard shortcuts".to_string(),
        },
        KeyboardShortcut {
            keys: vec!["Ctrl".to_string(), "T".to_string()],
            description: "Navigate to Talent Management".to_string(),
        },
        KeyboardShortcut {
            keys: vec!["Ctrl".to_string(), "S".to_string()],
            description: "Navigate to Show Management".to_string(),
        },
        KeyboardShortcut {
            keys: vec!["Ctrl".to_string(), "C".to_string()],
            description: "Navigate to Championships".to_string(),
        },
        KeyboardShortcut {
            keys: vec!["Ctrl".to_string(), "R".to_string()],
            description: "Refresh dashboard data".to_string(),
        },
    ];
    
    // Reactive resource for live business intelligence data
    let _metrics_resource = LocalResource::new(move || {
        let _trigger = refresh_trigger.get();
        async move {
            metrics_loading.set(true);
            metrics_error.set(None);
            screen_reader_announcements.set("Loading dashboard data...".to_string());
            
            match fetch_executive_metrics().await {
                Ok(data) => {
                    metrics.set(Some(data));
                    metrics_error.set(None);
                    screen_reader_announcements.set("Dashboard data loaded successfully.".to_string());
                },
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to fetch executive metrics: {}", e).into());
                    metrics_error.set(Some(e.clone()));
                    screen_reader_announcements.set(format!("Error loading dashboard data: {}", e));
                }
            }
            metrics_loading.set(false);
        }
    });
    
    // Global keyboard event handler
    Effect::new(move |_| {
        let handle_global_keydown = move |e: web_sys::KeyboardEvent| {
            match e.key().as_str() {
                "?" if !e.ctrl_key() && !e.alt_key() => {
                    e.prevent_default();
                    keyboard_shortcuts_visible.update(|visible| *visible = !*visible);
                    screen_reader_announcements.set(
                        if keyboard_shortcuts_visible.get() {
                            "Keyboard shortcuts shown".to_string()
                        } else {
                            "Keyboard shortcuts hidden".to_string()
                        }
                    );
                },
                "k" if e.ctrl_key() => {
                    e.prevent_default();
                    command_palette_open.set(true);
                    screen_reader_announcements.set("Command palette opened".to_string());
                },
                "r" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_refresh_trigger.update(|n| *n += 1);
                    screen_reader_announcements.set("Dashboard data refresh triggered".to_string());
                },
                "t" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("wrestlers".to_string());
                    screen_reader_announcements.set("Navigating to Talent Management".to_string());
                },
                "s" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("show-roster".to_string());
                    screen_reader_announcements.set("Navigating to Show Management".to_string());
                },
                "c" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("titles".to_string());
                    screen_reader_announcements.set("Navigating to Championship Management".to_string());
                },
                "b" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("booker".to_string());
                    screen_reader_announcements.set("Navigating to Match Booking".to_string());
                },
                "a" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("analytics".to_string());
                    screen_reader_announcements.set("Navigating to Analytics Dashboard".to_string());
                },
                "e" if e.ctrl_key() && !e.shift_key() => {
                    e.prevent_default();
                    set_current_page.set("command-center".to_string());
                    screen_reader_announcements.set("Navigating to Executive Command Center".to_string());
                },
                _ => {}
            }
        };
        
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let handler = wasm_bindgen::closure::Closure::wrap(Box::new(handle_global_keydown) as Box<dyn FnMut(_)>);
            let _ = document.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
            handler.forget();
        }
    });
    
    // Executive navigation handlers with accessibility announcements
    let navigate_to_talent_management = move |_| {
        set_current_page.set("wrestlers".to_string());
        screen_reader_announcements.set("Navigating to Talent Management".to_string());
    };
    
    let navigate_to_show_management = move |_| {
        set_current_page.set("show-roster".to_string());
        screen_reader_announcements.set("Navigating to Show Management".to_string());
    };
    
    let navigate_to_championships = move |_| {
        set_current_page.set("titles".to_string());
        screen_reader_announcements.set("Navigating to Championship Management".to_string());
    };
    
    let navigate_to_booker = move |_| {
        set_current_page.set("booker".to_string());
        screen_reader_announcements.set("Navigating to Match Booking Dashboard".to_string());
    };
    
    let navigate_to_analytics = move |_| {
        set_current_page.set("analytics".to_string());
        screen_reader_announcements.set("Navigating to Analytics Dashboard".to_string());
    };
    
    let navigate_to_command_center = move |_| {
        set_current_page.set("command-center".to_string());
        screen_reader_announcements.set("Navigating to Executive Command Center".to_string());
    };
    
    let navigate_to_bulk_operations = move |_| {
        set_current_page.set("bulk-operations".to_string());
        screen_reader_announcements.set("Navigating to Bulk Operations".to_string());
    };
    
    let initialize_development_data = move |_| {
        screen_reader_announcements.set("Initializing development data...".to_string());
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
            let result = invoke("create_test_data", args).await;
            
            match serde_wasm_bindgen::from_value::<String>(result) {
                Ok(message) => {
                    screen_reader_announcements.set(format!("Development data initialized: {}", message));
                    set_refresh_trigger.update(|n| *n += 1);
                },
                Err(e) => {
                    screen_reader_announcements.set(format!("Error initializing data: {:?}", e));
                }
            }
        });
    };
    
    let handle_command_execution = move |command: Command| {
        set_current_page.set(command.action.clone());
        screen_reader_announcements.set(format!("Executing command: {}", command.title));
    };
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-base-100 to-base-200">
            // Skip navigation for keyboard users
            <SkipNavigation />
            
            // Screen reader live region for announcements
            <AccessibilityLiveRegion 
                message=screen_reader_announcements.read_only()
                aria_live="polite".to_string()
            />
            
            // Loading state announcer
            <AccessibleLoadingState 
                message="Loading executive dashboard data...".to_string()
                loading=metrics_loading.read_only()
            />
            
            // Main content with proper landmark
            <main 
                id="main-content"
                class="container mx-auto px-4 py-8 space-y-8"
                role="main"
                aria-label="Executive Dashboard"
            >
                // Executive Header with branding
                <header class="text-center space-y-4 pb-8" role="banner">
                    <h1 class="text-4xl lg:text-6xl font-bold bg-gradient-to-r from-primary to-accent 
                               bg-clip-text text-transparent animate-executive-entrance">
                        "WWE Universe Manager"
                    </h1>
                    <p class="text-xl lg:text-2xl text-base-content/80 font-medium">
                        "Executive Command Center"
                    </p>
                    <div class="text-sm text-base-content/60">
                        "Professional wrestling empire management platform"
                        <span class="sr-only">
                            ". Navigate using keyboard shortcuts or click interface elements. 
                             Press question mark for keyboard shortcuts, or Control+K for command palette."
                        </span>
                    </div>
                </header>
                
                // Executive KPI Overview with accessibility enhancements
                {move || {
                    let loading = metrics_loading.get();
                    let error_msg = metrics_error.get();
                    let metrics_data = metrics.get();
                    
                    if loading {
                        view! {
                            <section 
                                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8"
                                aria-label="Performance metrics loading"
                                role="region"
                            >
                                {(0..4).map(|i| view! {
                                    <div 
                                        key=i
                                        class="card bg-base-100 border border-base-300/50 shadow-professional animate-loading-skeleton"
                                        role="article"
                                        aria-label=format!("Metric card {} loading", i + 1)
                                    >
                                        <div class="card-body">
                                            <div class="skeleton h-4 w-20 mb-2"></div>
                                            <div class="skeleton h-8 w-16 mb-2"></div>
                                            <div class="skeleton h-3 w-24"></div>
                                        </div>
                                    </div>
                                }).collect::<Vec<_>>()}
                            </section>
                        }.into_any()
                    } else if let Some(error) = error_msg {
                        view! {
                            <section 
                                class="mb-8"
                                role="alert"
                                aria-live="assertive"
                            >
                                <div class="card bg-error/10 border border-error/20 text-error shadow-lg">
                                    <div class="card-body text-center">
                                        <svg class="w-12 h-12 mx-auto mb-4 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                                  d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <h2 class="text-xl font-bold mb-2">"Dashboard Data Unavailable"</h2>
                                        <p class="mb-4">{error}</p>
                                        <AccessibleExecutiveButton 
                                            label="Retry Loading".to_string()
                                            variant="error".to_string()
                                            on_click=Some(Callback::new(move |_| {
                                                set_refresh_trigger.update(|n| *n += 1);
                                            }))
                                            aria_description=Some("Attempt to reload dashboard data".to_string())
                                            keyboard_hint=Some("Ctrl+R".to_string())
                                        />
                                    </div>
                                </div>
                            </section>
                        }.into_any()
                    } else if let Some(data) = metrics_data {
                        view! {
                            <section 
                                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8"
                                aria-label="Executive performance metrics"
                                role="region"
                            >
                                // Total Wrestlers KPI
                                <article 
                                    class="card bg-gradient-to-br from-primary/10 to-primary/5 
                                           border border-primary/20 shadow-professional 
                                           hover:shadow-executive transition-all duration-professional
                                           focus-within:ring-4 focus-within:ring-primary/50"
                                    role="article"
                                    aria-labelledby="total-wrestlers-heading"
                                >
                                    <div class="card-body">
                                        <h3 id="total-wrestlers-heading" class="text-sm font-semibold text-primary uppercase tracking-wider">
                                            "Total Talent"
                                        </h3>
                                        <div class="text-3xl font-bold text-base-content animate-metric-counter">
                                            {data.total_wrestlers}
                                        </div>
                                        <div class="text-xs text-base-content/70">
                                            "Active Wrestlers"
                                        </div>
                                    </div>
                                </article>
                                
                                // Total Shows KPI
                                <article 
                                    class="card bg-gradient-to-br from-secondary/10 to-secondary/5 
                                           border border-secondary/20 shadow-professional 
                                           hover:shadow-executive transition-all duration-professional
                                           focus-within:ring-4 focus-within:ring-secondary/50"
                                    role="article"
                                    aria-labelledby="total-shows-heading"
                                >
                                    <div class="card-body">
                                        <h3 id="total-shows-heading" class="text-sm font-semibold text-secondary uppercase tracking-wider">
                                            "Active Shows"
                                        </h3>
                                        <div class="text-3xl font-bold text-base-content animate-metric-counter">
                                            {data.total_shows}
                                        </div>
                                        <div class="text-xs text-base-content/70">
                                            "Broadcasting Programs"
                                        </div>
                                    </div>
                                </article>
                                
                                // Total Titles KPI
                                <article 
                                    class="card bg-gradient-to-br from-accent/10 to-accent/5 
                                           border border-accent/20 shadow-professional 
                                           hover:shadow-executive transition-all duration-professional
                                           focus-within:ring-4 focus-within:ring-accent/50"
                                    role="article"
                                    aria-labelledby="total-titles-heading"
                                >
                                    <div class="card-body">
                                        <h3 id="total-titles-heading" class="text-sm font-semibold text-accent uppercase tracking-wider">
                                            "Championships"
                                        </h3>
                                        <div class="text-3xl font-bold text-base-content animate-metric-counter">
                                            {data.total_titles}
                                        </div>
                                        <div class="text-xs text-base-content/70">
                                            "Active Titles"
                                        </div>
                                    </div>
                                </article>
                                
                                // Total Matches KPI
                                <article 
                                    class="card bg-gradient-to-br from-info/10 to-info/5 
                                           border border-info/20 shadow-professional 
                                           hover:shadow-executive transition-all duration-professional
                                           focus-within:ring-4 focus-within:ring-info/50"
                                    role="article"
                                    aria-labelledby="total-matches-heading"
                                >
                                    <div class="card-body">
                                        <h3 id="total-matches-heading" class="text-sm font-semibold text-info uppercase tracking-wider">
                                            "Total Matches"
                                        </h3>
                                        <div class="text-3xl font-bold text-base-content animate-metric-counter">
                                            {data.total_matches}
                                        </div>
                                        <div class="text-xs text-base-content/70">
                                            "Booked Contests"
                                        </div>
                                    </div>
                                </article>
                            </section>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center text-base-content/70 py-8">
                                "No dashboard data available"
                            </div>
                        }.into_any()
                    }
                }}
                
                // Executive Operations Hub with full accessibility
                <section 
                    class="space-y-8"
                    aria-label="Executive operations and navigation"
                    role="region"
                >
                    <h2 class="text-2xl lg:text-3xl font-bold text-center text-base-content mb-8">
                        "Executive Operations Hub"
                        <span class="sr-only">
                            ". Use Tab to navigate between operation cards, 
                             Enter or Space to activate, or use keyboard shortcuts."
                        </span>
                    </h2>
                    
                    // Core Operations Grid
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        // Talent Management Card
                        <article 
                            class="card bg-gradient-to-br from-primary/5 to-primary/10 
                                   border border-primary/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-primary/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Talent Management - Keyboard shortcut Control+T"
                            on:click=navigate_to_talent_management
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.prevent_default();
                                    navigate_to_talent_management(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-primary">
                                    "👨‍💼"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Talent Management"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Comprehensive wrestler profiles, performance tracking, and strategic talent development"
                                </p>
                                <div class="text-sm text-primary font-medium">
                                    "Shortcut: Ctrl+T"
                                </div>
                            </div>
                        </article>
                        
                        // Show Management Card
                        <article 
                            class="card bg-gradient-to-br from-secondary/5 to-secondary/10 
                                   border border-secondary/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-secondary/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Show Management - Keyboard shortcut Control+S"
                            on:click=navigate_to_show_management
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.prevent_default();
                                    navigate_to_show_management(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-secondary">
                                    "📺"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Show Management"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Strategic show roster assignments, talent allocation, and programming optimization"
                                </p>
                                <div class="text-sm text-secondary font-medium">
                                    "Shortcut: Ctrl+S"
                                </div>
                            </div>
                        </article>
                        
                        // Championship Management Card
                        <article 
                            class="card bg-gradient-to-br from-accent/5 to-accent/10 
                                   border border-accent/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-accent/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Championship Management - Keyboard shortcut Control+C"
                            on:click=navigate_to_championships
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.prevent_default();
                                    navigate_to_championships(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-accent">
                                    "🏆"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Championship Management"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Title lineage tracking, championship prestige management, and holder transitions"
                                </p>
                                <div class="text-sm text-accent font-medium">
                                    "Shortcut: Ctrl+C"
                                </div>
                            </div>
                        </article>
                        
                        // Match Booking Card
                        <article 
                            class="card bg-gradient-to-br from-info/5 to-info/10 
                                   border border-info/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-info/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Match Booking - Keyboard shortcut Control+B"
                            on:click=navigate_to_booker
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.prevent_default();
                                    navigate_to_booker(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-info">
                                    "🥊"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Match Booking"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Professional match creation, participant management, and strategic booking decisions"
                                </p>
                                <div class="text-sm text-info font-medium">
                                    "Shortcut: Ctrl+B"
                                </div>
                            </div>
                        </article>
                        
                        // Analytics Dashboard Card
                        <article 
                            class="card bg-gradient-to-br from-success/5 to-success/10 
                                   border border-success/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-success/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Analytics Dashboard - Keyboard shortcut Control+A"
                            on:click=navigate_to_analytics
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.preventDefault();
                                    navigate_to_analytics(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-success">
                                    "📊"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Analytics Dashboard"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Business intelligence, performance analytics, and strategic insights for data-driven decisions"
                                </p>
                                <div class="text-sm text-success font-medium">
                                    "Shortcut: Ctrl+A"
                                </div>
                            </div>
                        </article>
                        
                        // Executive Command Center Card
                        <article 
                            class="card bg-gradient-to-br from-warning/5 to-warning/10 
                                   border border-warning/20 shadow-professional 
                                   hover:shadow-executive hover:-translate-y-1 
                                   transition-all duration-professional cursor-pointer
                                   focus-within:ring-4 focus-within:ring-warning/50"
                            tabindex="0"
                            role="button"
                            aria-label="Navigate to Executive Command Center - Keyboard shortcut Control+E"
                            on:click=navigate_to_command_center
                            on:keydown=move |e: web_sys::KeyboardEvent| {
                                if e.key() == "Enter" || e.key() == " " {
                                    e.prevent_default();
                                    navigate_to_command_center(());
                                }
                            }
                        >
                            <div class="card-body text-center">
                                <div class="text-4xl mb-4 text-warning">
                                    "🎛️"
                                </div>
                                <h3 class="card-title justify-center text-xl mb-2">
                                    "Command Center"
                                </h3>
                                <p class="text-base-content/70 mb-4">
                                    "Real-time system monitoring, performance metrics, and administrative controls"
                                </p>
                                <div class="text-sm text-warning font-medium">
                                    "Shortcut: Ctrl+E"
                                </div>
                            </div>
                        </article>
                    </div>
                    
                    // Strategic Operations Section
                    <div class="border-t border-base-300 pt-8">
                        <h3 class="text-xl font-semibold text-center text-base-content mb-6">
                            "Strategic Operations"
                        </h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 max-w-2xl mx-auto">
                            <AccessibleExecutiveButton 
                                label="Bulk Operations".to_string()
                                variant="neutral".to_string()
                                size="executive".to_string()
                                full_width=true
                                on_click=Some(Callback::new(navigate_to_bulk_operations))
                                aria_description=Some("Mass operations for talent and show management".to_string())
                            />
                            
                            <AccessibleExecutiveButton 
                                label="Initialize Test Data".to_string()
                                variant="ghost".to_string()
                                size="executive".to_string()
                                full_width=true
                                on_click=Some(Callback::new(initialize_development_data))
                                aria_description=Some("Populate system with sample wrestling data for testing".to_string())
                                keyboard_hint=Some("For development testing".to_string())
                            />
                        </div>
                    </div>
                </section>
                
                // Accessibility tools section
                <section 
                    class="text-center pt-8 border-t border-base-300/50"
                    aria-label="Accessibility and keyboard navigation help"
                >
                    <div class="text-sm text-base-content/60 space-y-2">
                        <p>
                            "Press " 
                            <kbd class="kbd kbd-sm">Ctrl</kbd>
                            "+"
                            <kbd class="kbd kbd-sm">K</kbd>
                            " for command palette, or "
                            <kbd class="kbd kbd-sm">?</kbd>
                            " for keyboard shortcuts"
                        </p>
                        <p class="sr-only">
                            "This dashboard is fully accessible with screen readers and keyboard navigation. 
                             All interactive elements can be reached using Tab key, and activated with Enter or Space."
                        </p>
                    </div>
                </section>
            </main>
            
            // Command Palette
            <CommandPalette 
                is_open=command_palette_open
                commands=available_commands
                search_query=command_search
                selected_index=command_selection
                on_execute=Callback::new(handle_command_execution)
            />
            
            // Keyboard Shortcuts Helper
            <KeyboardShortcutsHelper 
                shortcuts=keyboard_shortcuts
                show=keyboard_shortcuts_visible.read_only()
            />
            
            // Accessibility debugger (development only)
            // <AccessibilityDebugger />
        </div>
    }
}