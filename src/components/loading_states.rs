use leptos::prelude::*;

/// Professional Loading States for WWE Universe Manager
/// 
/// Provides consistent, executive-quality loading experiences across
/// all components with proper skeleton screens, spinners, and progress indicators

/// Executive-level loading skeleton for wrestler cards
#[component]
pub fn WrestlerCardSkeleton() -> impl IntoView {
    view! {
        <div class="card bg-base-100 border border-base-300/50 shadow-lg animate-card-entrance">
            <div class="card-body p-4">
                // Header skeleton
                <div class="flex items-center gap-3 mb-4">
                    <div class="skeleton w-12 h-12 rounded-full shrink-0"></div>
                    <div class="flex-1">
                        <div class="skeleton h-4 w-3/4 mb-2"></div>
                        <div class="skeleton h-3 w-1/2"></div>
                    </div>
                    <div class="skeleton w-8 h-8 rounded"></div>
                </div>
                
                // Stats skeleton
                <div class="grid grid-cols-2 gap-2 mb-4">
                    <div class="skeleton h-8 w-full rounded"></div>
                    <div class="skeleton h-8 w-full rounded"></div>
                </div>
                
                // Power ratings skeleton
                <div class="space-y-2 mb-4">
                    <div class="skeleton h-3 w-full rounded"></div>
                    <div class="skeleton h-3 w-5/6 rounded"></div>
                    <div class="skeleton h-3 w-4/5 rounded"></div>
                </div>
                
                // Actions skeleton
                <div class="flex gap-2">
                    <div class="skeleton h-9 w-20 rounded"></div>
                    <div class="skeleton h-9 w-16 rounded"></div>
                </div>
            </div>
        </div>
    }
}

/// Executive-level loading skeleton for show cards
#[component]
pub fn ShowCardSkeleton() -> impl IntoView {
    view! {
        <div class="card bg-base-100 border border-base-300/50 shadow-lg animate-card-entrance">
            <div class="card-body p-6">
                // Header with title and status
                <div class="flex items-center justify-between mb-4">
                    <div class="flex items-center gap-3">
                        <div class="skeleton w-10 h-10 rounded-xl"></div>
                        <div>
                            <div class="skeleton h-5 w-32 mb-1"></div>
                            <div class="skeleton h-3 w-24"></div>
                        </div>
                    </div>
                    <div class="skeleton w-16 h-6 rounded-full"></div>
                </div>
                
                // Metrics grid
                <div class="grid grid-cols-3 gap-4 mb-4">
                    <div class="text-center">
                        <div class="skeleton h-8 w-12 mx-auto mb-1"></div>
                        <div class="skeleton h-3 w-16 mx-auto"></div>
                    </div>
                    <div class="text-center">
                        <div class="skeleton h-8 w-12 mx-auto mb-1"></div>
                        <div class="skeleton h-3 w-16 mx-auto"></div>
                    </div>
                    <div class="text-center">
                        <div class="skeleton h-8 w-12 mx-auto mb-1"></div>
                        <div class="skeleton h-3 w-16 mx-auto"></div>
                    </div>
                </div>
                
                // Action button
                <div class="skeleton h-10 w-full rounded"></div>
            </div>
        </div>
    }
}

/// Executive-level loading skeleton for title cards
#[component]
pub fn TitleCardSkeleton() -> impl IntoView {
    view! {
        <div class="card bg-base-100 border border-base-300/50 shadow-lg animate-card-entrance">
            <div class="card-body p-4">
                // Title header
                <div class="flex items-center gap-3 mb-3">
                    <div class="skeleton w-8 h-8 rounded-lg"></div>
                    <div class="flex-1">
                        <div class="skeleton h-4 w-3/4 mb-1"></div>
                        <div class="skeleton h-3 w-1/2"></div>
                    </div>
                    <div class="skeleton w-12 h-5 rounded"></div>
                </div>
                
                // Current holder
                <div class="flex items-center gap-3 mb-3">
                    <div class="skeleton w-10 h-10 rounded-full"></div>
                    <div class="flex-1">
                        <div class="skeleton h-3 w-16 mb-1"></div>
                        <div class="skeleton h-4 w-24"></div>
                    </div>
                </div>
                
                // Stats
                <div class="flex gap-4 mb-3">
                    <div class="flex-1">
                        <div class="skeleton h-3 w-16 mb-1"></div>
                        <div class="skeleton h-4 w-12"></div>
                    </div>
                    <div class="flex-1">
                        <div class="skeleton h-3 w-16 mb-1"></div>
                        <div class="skeleton h-4 w-12"></div>
                    </div>
                </div>
                
                // Action
                <div class="skeleton h-8 w-full rounded"></div>
            </div>
        </div>
    }
}

/// Professional data table loading state with skeleton rows
#[component]
pub fn DataTableSkeleton(
    /// Number of skeleton rows to show
    #[prop(default = 5)]
    rows: usize,
    /// Number of columns
    #[prop(default = 4)]
    columns: usize,
) -> impl IntoView {
    let skeleton_rows: Vec<_> = (0..rows).collect();
    let skeleton_cols: Vec<_> = (0..columns).collect();

    view! {
        <div class="overflow-x-auto">
            <table class="table">
                // Header skeleton
                <thead>
                    <tr>
                        {skeleton_cols.iter().map(|_| view! {
                            <th><div class="skeleton h-4 w-20"></div></th>
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                
                // Body skeleton
                <tbody>
                    {skeleton_rows.into_iter().map(|i| view! {
                        <tr class="animate-card-entrance" style=format!("animation-delay: {}ms", i * 50)>
                            {skeleton_cols.iter().map(|j| view! {
                                <td>
                                    <div class=format!("skeleton h-4 w-{}", if *j == 0 { "32" } else { "16" })></div>
                                </td>
                            }).collect::<Vec<_>>()}
                        </tr>
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}

/// Professional stats grid loading state
#[component]
pub fn StatsGridSkeleton(
    /// Number of stat cards
    #[prop(default = 4)]
    stats: usize,
) -> impl IntoView {
    let skeleton_stats: Vec<_> = (0..stats).collect();

    view! {
        <div class="stats stats-vertical sm:stats-horizontal shadow-lg border border-base-300/50 bg-gradient-to-br from-base-100 to-base-200/50">
            {skeleton_stats.into_iter().map(|i| view! {
                <div class="stat animate-card-entrance" style=format!("animation-delay: {}ms", i * 100)>
                    <div class="stat-figure">
                        <div class="skeleton w-10 h-10 rounded-full"></div>
                    </div>
                    <div class="stat-title">
                        <div class="skeleton h-3 w-24 mb-1"></div>
                    </div>
                    <div class="stat-value">
                        <div class="skeleton h-8 w-16"></div>
                    </div>
                    <div class="stat-desc">
                        <div class="skeleton h-3 w-32"></div>
                    </div>
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Professional grid loading state for cards
#[component]
pub fn CardGridSkeleton(
    /// Type of card skeleton
    #[prop(default = "wrestler".to_string())]
    card_type: String,
    /// Number of skeleton cards
    #[prop(default = 6)]
    count: usize,
    /// Grid columns class
    #[prop(default = "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3".to_string())]
    grid_cols: String,
) -> impl IntoView {
    let skeleton_cards: Vec<_> = (0..count).collect();

    view! {
        <div class=format!("grid {} gap-4", grid_cols)>
            {skeleton_cards.into_iter().map(|i| {
                let delay_style = format!("animation-delay: {}ms", i * 100);
                match card_type.as_str() {
                    "show" => view! {
                        <div style=delay_style>
                            <ShowCardSkeleton />
                        </div>
                    }.into_any(),
                    "title" => view! {
                        <div style=delay_style>
                            <TitleCardSkeleton />
                        </div>
                    }.into_any(),
                    _ => view! {
                        <div style=delay_style>
                            <WrestlerCardSkeleton />
                        </div>
                    }.into_any(),
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Professional loading page with executive branding
#[component]
pub fn ExecutiveLoadingPage(
    /// Loading message
    #[prop(default = "Loading Executive Dashboard...".to_string())]
    message: String,
    /// Show skeleton content preview
    #[prop(default = true)]
    show_skeleton: bool,
) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-100 flex flex-col">
            // Executive header skeleton
            <div class="text-center py-12 px-4">
                <div class="flex items-center justify-center gap-3 mb-6">
                    <div class="skeleton w-12 h-12 rounded-xl"></div>
                </div>
                <div class="skeleton h-10 w-80 mx-auto mb-4"></div>
                <div class="skeleton h-4 w-96 mx-auto"></div>
            </div>
            
            // Loading indicator
            <div class="flex flex-col items-center gap-4 mb-12">
                <span class="loading loading-spinner loading-lg text-primary animate-professional-pulse"></span>
                <p class="text-base-content/70 text-lg animate-professional-pulse">{message}</p>
            </div>
            
            // Skeleton preview
            {if show_skeleton {
                view! {
                    <div class="flex-1 px-4 pb-8">
                        <div class="max-w-7xl mx-auto space-y-8">
                            // Stats skeleton
                            <StatsGridSkeleton stats=3 />
                            
                            // Content grid skeleton
                            <CardGridSkeleton card_type="wrestler".to_string() count=6 />
                        </div>
                    </div>
                }
            } else {
                view! {}
            }}
        </div>
    }
}

/// Professional error state with retry animation
#[component]
pub fn ExecutiveErrorState(
    /// Error title
    #[prop(default = "Connection Issue".to_string())]
    title: String,
    /// Error message
    message: String,
    /// Retry handler
    #[prop(optional)]
    on_retry: Option<Box<dyn Fn() + 'static>>,
    /// Show support information
    #[prop(default = true)]
    show_support: bool,
) -> impl IntoView {
    view! {
        <div class="card bg-base-100 border border-error/20 shadow-lg max-w-md mx-auto animate-card-entrance">
            <div class="card-body text-center p-8">
                // Error icon
                <div class="mx-auto w-16 h-16 bg-error/10 rounded-full flex items-center justify-center mb-4 animate-error-shake">
                    <svg class="w-8 h-8 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                </div>
                
                // Error message
                <h3 class="text-xl font-bold text-error mb-2">{title}</h3>
                <p class="text-base-content/70 mb-6">{message}</p>
                
                // Actions
                <div class="flex flex-col gap-3">
                    {on_retry.map(|retry_fn| {
                        let handle_retry = move |_| retry_fn();
                        view! {
                            <button class="btn btn-error gap-2 animate-button-press" on:click=handle_retry>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                </svg>
                                "Try Again"
                            </button>
                        }
                    })}
                    
                    {if show_support {
                        view! {
                            <button class="btn btn-ghost btn-sm">
                                "Contact Support"
                            </button>
                        }
                    } else {
                        view! {}
                    }}
                </div>
            </div>
        </div>
    }
}

/// Professional empty state with action suggestions
#[component]
pub fn ExecutiveEmptyState(
    /// Title
    title: String,
    /// Description
    description: String,
    /// Primary action text
    #[prop(default = "Get Started".to_string())]
    action_text: String,
    /// Action handler
    #[prop(optional)]
    on_action: Option<Box<dyn Fn() + 'static>>,
    /// Icon SVG content
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Show suggestions
    #[prop(default = true)]
    show_suggestions: bool,
) -> impl IntoView {
    let default_icon = view! {
        <svg class="w-12 h-12 text-base-content/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
        </svg>
    };

    view! {
        <div class="text-center py-12 px-4 max-w-md mx-auto animate-card-entrance">
            // Icon
            <div class="mb-6 animate-professional-pulse">
                {icon.unwrap_or(default_icon)}
            </div>
            
            // Content
            <h3 class="text-xl font-bold text-base-content mb-2">{title}</h3>
            <p class="text-base-content/70 mb-6">{description}</p>
            
            // Action
            {on_action.map(|action_fn| {
                let handle_action = move |_| action_fn();
                view! {
                    <button class="btn btn-primary gap-2 mb-4 animate-button-press" on:click=handle_action>
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                        </svg>
                        {action_text}
                    </button>
                }
            })}
            
            // Suggestions
            {if show_suggestions {
                view! {
                    <div class="text-sm text-base-content/50 space-y-1">
                        <p>"💡 Try using the 'Test Data' button to populate sample content"</p>
                        <p>"📊 Check the Analytics dashboard for insights"</p>
                    </div>
                }
            } else {
                view! {}
            }}
        </div>
    }
}