use leptos::prelude::*;

/// Executive Page Layout Component
/// 
/// Professional standard container providing:
/// - Executive header with professional styling
/// - Theme-aware gradient backgrounds
/// - Professional spacing and responsive design
/// - Consistent footer with corporate branding
/// 
/// This component ensures all pages follow executive-level design standards
/// matching the professional quality of the CEO Dashboard.
/// Note: Breadcrumb navigation is handled at the app level via ExecutivePageHeader.
#[component]
pub fn ExecutivePageLayout(
    /// Page title for header display
    title: String,
    /// Optional subtitle for additional context
    subtitle: Option<String>,
    /// The main content to display
    children: Children,
) -> impl IntoView {
    
    view! {
        <div class="min-h-screen bg-gradient-to-br from-base-100 via-base-100 to-base-200/30">
            <div class="container mx-auto px-4 sm:px-6 lg:px-8 py-6 sm:py-8">
                
                // Executive Page Header
                <div class="mb-8 xl:mb-12">
                    // Executive Page Title Section
                    <div class="text-center">
                        <h1 class="text-4xl xl:text-5xl font-bold text-base-content mb-3 bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
                            {title}
                        </h1>
                        {move || {
                            if let Some(sub) = subtitle.clone() {
                                view! {
                                    <p class="text-lg text-base-content/80 max-w-3xl mx-auto leading-relaxed">
                                        {sub}
                                    </p>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                </div>
                
                // Main Content Area with Professional Spacing
                <div class="max-w-full mx-auto">
                    <div class="space-y-8 xl:space-y-12">
                        {children()}
                    </div>
                </div>
                
                // Executive Footer Spacer
                <div class="mt-16 xl:mt-24"></div>
                
            </div>
        </div>
    }
}

/// Executive Content Section
/// 
/// Professional content container with:
/// - Gradient background and subtle borders
/// - Executive-level shadows and spacing
/// - Theme-aware styling
/// - Consistent with CEO Dashboard patterns
#[component]
pub fn ExecutiveContentSection(
    /// Optional section title
    #[prop(optional)]
    title: Option<String>,
    /// Optional section description
    #[prop(optional)]
    description: Option<String>,
    /// Optional icon for section header
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Content variant for different styling
    #[prop(default = "default".to_string())]
    variant: String,
    /// The content to display
    children: Children,
) -> impl IntoView {
    
    let section_classes = match variant.as_str() {
        "primary" => "card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-executive",
        "secondary" => "card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20 shadow-professional",
        "accent" => "card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20 shadow-professional",
        "info" => "card bg-gradient-to-br from-info/10 to-info/5 border border-info/20 shadow-professional",
        "success" => "card bg-gradient-to-br from-success/10 to-success/5 border border-success/20 shadow-professional",
        "warning" => "card bg-gradient-to-br from-warning/10 to-warning/5 border border-warning/20 shadow-professional",
        _ => "card bg-gradient-to-br from-base-100 to-base-200/50 shadow-xl border border-base-300/50",
    };
    
    view! {
        <div class={section_classes}>
            <div class="card-body p-4 sm:p-6 lg:p-8">
                
                // Section Header (if title or description provided)
                {
                    let has_header = title.is_some() || description.is_some() || icon.is_some();
                    let title_clone = title.clone();
                    let description_clone = description.clone();
                    
                    if has_header {
                        view! {
                            <div class="flex items-center gap-3 mb-6">
                                // Optional Icon
                                {
                                    if let Some(icon_view) = icon {
                                        view! {
                                            <div class="w-8 h-8 bg-primary/20 rounded-lg flex items-center justify-center">
                                                {icon_view}
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }
                                
                                <div class="flex-1">
                                    // Optional Title
                                    {
                                        if let Some(section_title) = title_clone.clone() {
                                            view! {
                                                <h2 class="text-2xl font-bold text-base-content">
                                                    {section_title}
                                                </h2>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                    
                                    // Optional Description
                                    {
                                        if let Some(desc) = description_clone.clone() {
                                            view! {
                                                <p class="text-base-content/70 text-sm sm:text-base">
                                                    {desc}
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }
                
                // Main Content
                {children()}
            </div>
        </div>
    }
}

/// Executive Navigation Back Button
/// 
/// Consistent back button for returning to main dashboard
#[component]
pub fn ExecutiveBackButton(
    /// Text for the back button
    #[prop(default = "Return to Executive Dashboard".to_string())]
    text: String,
    /// Route to navigate back to
    #[prop(default = "promotion-dashboard".to_string())]
    route: String,
    /// Navigation callback
    on_navigate: Callback<String>,
) -> impl IntoView {
    
    view! {
        <div class="text-center pt-8">
            <button 
                class="btn btn-neutral gap-2 btn-wide"
                on:click={
                    let route = route.clone();
                    move |_| {
                        on_navigate.run(route.clone());
                    }
                }
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
                </svg>
                {text}
            </button>
        </div>
    }
}