use leptos::prelude::*;

/// Executive Card System Component
/// 
/// Professional card variants providing:
/// - Unified design system across all executive interfaces
/// - Multiple variants (primary, secondary, info, gradient, metric)
/// - Professional shadows, borders, and hover effects
/// - Theme-responsive styling matching CEO Dashboard quality
/// - Executive-level animations and interactions
#[component]
pub fn ExecutiveCard(
    /// Card variant for different styling contexts
    #[prop(default = "default".to_string())]
    variant: String,
    /// Optional card title
    #[prop(optional)]
    title: Option<String>,
    /// Optional card subtitle or description
    #[prop(optional)]
    subtitle: Option<String>,
    /// Optional icon for the card header
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Optional badge text for status/category
    #[prop(optional)]
    badge: Option<String>,
    /// Optional badge variant
    #[prop(default = "neutral".to_string())]
    badge_variant: String,
    /// Whether the card is clickable
    #[prop(default = false)]
    clickable: bool,
    /// Click handler for clickable cards
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Optional footer actions
    #[prop(optional)]
    footer: Option<AnyView>,
    /// Card size variant
    #[prop(default = "normal".to_string())]
    size: String,
    /// The main card content
    children: Children,
) -> impl IntoView {
    
    // Determine card styling based on variant
    let card_classes = match variant.as_str() {
        "primary" => "card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-professional hover:shadow-executive",
        "secondary" => "card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20 shadow-professional hover:shadow-executive",
        "accent" => "card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20 shadow-professional hover:shadow-executive",
        "info" => "card bg-gradient-to-br from-info/10 to-info/5 border border-info/20 shadow-professional hover:shadow-executive",
        "success" => "card bg-gradient-to-br from-success/10 to-success/5 border border-success/20 shadow-professional hover:shadow-executive",
        "warning" => "card bg-gradient-to-br from-warning/10 to-warning/5 border border-warning/20 shadow-professional hover:shadow-executive",
        "error" => "card bg-gradient-to-br from-error/10 to-error/5 border border-error/20 shadow-professional hover:shadow-executive",
        "gradient" => "card bg-gradient-to-br from-primary/10 via-accent/10 to-secondary/10 border border-primary/20 shadow-executive",
        "premium" => "card bg-gradient-to-br from-base-100 to-base-200/50 shadow-premium border border-base-300/50 hover:shadow-executive",
        "metric" => "card bg-gradient-to-br from-primary/5 to-accent/5 border border-primary/10 shadow-professional hover:bg-primary/10",
        _ => "card bg-base-100 hover:bg-base-200/70 border border-base-300/30 shadow-professional hover:shadow-executive",
    };
    
    // Determine padding based on size
    let padding_classes = match size.as_str() {
        "compact" => "p-3 sm:p-4",
        "large" => "p-6 sm:p-8 lg:p-10",
        "executive" => "p-4 sm:p-6 lg:p-8 xl:p-10",
        _ => "p-3 sm:p-4 lg:p-6",
    };
    
    // Add cursor pointer if clickable
    let cursor_class = if clickable { " cursor-pointer" } else { "" };
    let transition_class = " transition-all duration-200";
    
    let final_classes = format!("{}{}{}", card_classes, cursor_class, transition_class);
    
    view! {
        <div 
            class={final_classes}
            on:click=move |_| {
                if clickable {
                    if let Some(click_handler) = on_click {
                        click_handler.run(());
                    }
                }
            }
        >
            <div class={format!("card-body {}", padding_classes)}>
                
                // Card Header Section
                {
                    let has_header = title.is_some() || icon.is_some() || badge.is_some();
                    let title_clone = title.clone();
                    let badge_clone = badge.clone();
                    let subtitle_clone = subtitle.clone();
                    
                    if has_header {
                        view! {
                            <div class="flex items-center justify-between mb-4">
                                <div class="flex items-center gap-3">
                                    // Optional Icon
                                    {
                                        if let Some(icon_view) = icon {
                                            view! {
                                                <div class="w-10 h-10 bg-primary/20 rounded-xl flex items-center justify-center group-hover:bg-primary/30 transition-colors">
                                                    {icon_view}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                    
                                    // Title and Subtitle
                                    <div>
                                        {
                                            if let Some(card_title) = title_clone.clone() {
                                                view! {
                                                    <h3 class="text-lg font-bold text-base-content mb-1 group-hover:text-primary transition-colors">
                                                        {card_title}
                                                    </h3>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }
                                        
                                        {
                                            if let Some(card_subtitle) = subtitle_clone.clone() {
                                                view! {
                                                    <p class="text-base-content/70 text-sm">
                                                        {card_subtitle}
                                                    </p>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }
                                    </div>
                                </div>
                                
                                // Optional Badge
                                {
                                    if let Some(badge_text) = badge_clone.clone() {
                                        let badge_class = format!("badge badge-{} badge-sm", badge_variant);
                                        view! {
                                            <div class={badge_class}>
                                                {badge_text}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
                
                // Main Content
                <div class="flex-1">
                    {children()}
                </div>
                
                // Optional Footer
                {
                    if let Some(footer_view) = footer {
                        view! {
                            <div class="card-actions justify-end pt-4 border-t border-base-300/30 mt-4">
                                {footer_view}
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
                
            </div>
        </div>
    }
}

/// Executive Metric Card
/// 
/// Specialized card for displaying KPI metrics and statistics
/// matching the professional stats pattern from the CEO Dashboard
#[component]
pub fn ExecutiveMetricCard(
    /// The metric title/label
    title: String,
    /// The primary metric value
    value: String,
    /// Optional metric change/trend indicator
    #[prop(optional)]
    change: Option<String>,
    /// Optional description or context
    #[prop(optional)]
    description: Option<String>,
    /// Metric color theme
    #[prop(default = "primary".to_string())]
    color: String,
    /// Optional icon for the metric
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Whether to show a trend indicator
    #[prop(default = false)]
    show_trend: bool,
    /// Trend direction (up, down, neutral)
    #[prop(default = "neutral".to_string())]
    trend: String,
) -> impl IntoView {
    
    let color_classes = match color.as_str() {
        "primary" => ("text-primary", "bg-primary/20", "text-primary/80", "text-primary/60"),
        "secondary" => ("text-secondary", "bg-secondary/20", "text-secondary/80", "text-secondary/60"),
        "accent" => ("text-accent", "bg-accent/20", "text-accent/80", "text-accent/60"),
        "info" => ("text-info", "bg-info/20", "text-info/80", "text-info/60"),
        "success" => ("text-success", "bg-success/20", "text-success/80", "text-success/60"),
        "warning" => ("text-warning", "bg-warning/20", "text-warning/80", "text-warning/60"),
        _ => ("text-primary", "bg-primary/20", "text-primary/80", "text-primary/60"),
    };
    
    view! {
        <div class="stat group hover:bg-base-300/30 transition-colors duration-200 rounded-lg xl:px-6">
            <div class={format!("stat-figure {}", color_classes.0)}>
                <div class="avatar">
                    <div class={format!("w-10 h-10 rounded-full {} flex items-center justify-center", color_classes.1)}>
                        {
                            if let Some(icon_view) = icon {
                                icon_view.into_any()
                            } else {
                                view! {
                                    <svg class="w-6 h-6 stroke-current" fill="none" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                    </svg>
                                }.into_any()
                            }
                        }
                    </div>
                </div>
            </div>
            
            <div class={format!("stat-title {} font-medium", color_classes.2)}>
                {title}
            </div>
            
            <div class={format!("stat-value {} text-2xl font-bold", color_classes.0)}>
                {value}
            </div>
            
            <div class={format!("stat-desc {}", color_classes.3)}>
                {move || {
                    if let Some(desc) = description.clone() {
                        view! { <span>{desc}</span> }.into_any()
                    } else if show_trend && change.is_some() {
                        let trend_icon = match trend.as_str() {
                            "up" => "↗",
                            "down" => "↘",
                            _ => "→",
                        };
                        let trend_color = match trend.as_str() {
                            "up" => "text-success",
                            "down" => "text-error",
                            _ => "text-base-content/60",
                        };
                        
                        view! {
                            <span class={trend_color}>
                                {trend_icon} " " {change.clone().unwrap_or_default()}
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

/// Executive Action Card
/// 
/// Interactive card for navigation and actions, matching the 
/// professional card pattern from the CEO Dashboard
#[component]
pub fn ExecutiveActionCard(
    /// Action title
    title: String,
    /// Action description
    description: String,
    /// Action category for badge
    category: String,
    /// Category color
    #[prop(default = "primary".to_string())]
    category_color: String,
    /// Icon for the action
    icon: AnyView,
    /// Click handler
    on_click: Callback<()>,
    /// Optional status indicator
    #[prop(optional)]
    status: Option<String>,
) -> impl IntoView {
    
    view! {
        <div class="card bg-base-100 hover:bg-base-200/70 border border-base-300/30 cursor-pointer transition-all duration-200 hover:shadow-lg group" 
             on:click=move |_| on_click.run(())>
            <div class="card-body p-3 sm:p-4 lg:p-6">
                <div class="flex items-center gap-3 mb-3">
                    <div class="w-10 h-10 bg-primary/20 rounded-xl flex items-center justify-center group-hover:bg-primary/30 transition-colors">
                        {icon}
                    </div>
                    <div class={format!("badge badge-{} badge-sm", category_color)}>
                        {category.clone()}
                    </div>
                    {move || {
                        if let Some(status_text) = status.clone() {
                            view! {
                                <div class="badge badge-success badge-sm animate-pulse">
                                    {status_text}
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
                
                <h3 class="text-lg font-bold text-base-content mb-2 group-hover:text-primary transition-colors">
                    {title}
                </h3>
                
                <p class="text-base-content/70 text-sm leading-relaxed mb-3">
                    {description}
                </p>
                
                <div class="flex items-center justify-between">
                    <span class="text-xs text-base-content/50">
                        {category}
                    </span>
                    <svg class="w-4 h-4 text-base-content/40 group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                    </svg>
                </div>
            </div>
        </div>
    }
}