use leptos::prelude::*;

/// Navigation utilities for executive-level routing and breadcrumbs
/// 
/// Provides sophisticated navigation patterns for wrestling enterprise management:
/// - Context-aware breadcrumbs showing current location
/// - Professional navigation state management
/// - Executive quick navigation shortcuts

#[derive(Debug, Clone, PartialEq)]
pub struct BreadcrumbItem {
    pub label: String,
    pub route: Option<String>,
    pub icon: Option<String>,
    pub is_current: bool,
}

/// Executive navigation context for breadcrumb generation
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationContext {
    pub current_page: String,
    pub parent_pages: Vec<String>,
    pub show_id: Option<i32>,
    pub wrestler_id: Option<i32>,
    pub title_id: Option<i32>,
}

/// Page information for context-aware headers
#[derive(Debug, Clone, PartialEq)]
pub struct PageInfo {
    pub title: String,
    pub description: String,
    pub icon: String,
    pub category: String,
}

/// Get page information for context-aware headers
pub fn get_page_info(page: &str) -> PageInfo {
    match page {
        "promotion-dashboard" => PageInfo {
            title: "Executive Command Center".to_string(),
            description: "Strategic oversight and operational management for your wrestling entertainment empire".to_string(),
            icon: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4".to_string(),
            category: "Executive".to_string(),
        },
        "wrestlers" => PageInfo {
            title: "Talent Management".to_string(),
            description: "Wrestler profiles, performance analytics, and strategic talent development".to_string(),
            icon: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 515.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 919.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z".to_string(),
            category: "Human Resources".to_string(),
        },
        "create-wrestler" => PageInfo {
            title: "New Talent Acquisition".to_string(),
            description: "Add new wrestlers to your global talent pool with complete profiles and power ratings".to_string(),
            icon: "M12 6v6m0 0v6m0-6h6m-6 0H6".to_string(),
            category: "Human Resources".to_string(),
        },
        "titles" => PageInfo {
            title: "Championship Division".to_string(),
            description: "Title lineage management, holder tracking, and championship prestige metrics".to_string(),
            icon: "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z".to_string(),
            category: "Legacy Management".to_string(),
        },
        "create-title" => PageInfo {
            title: "New Championship Creation".to_string(),
            description: "Create and configure new championship titles with prestige tiers and divisions".to_string(),
            icon: "M12 6v6m0 0v6m0-6h6m-6 0H6".to_string(),
            category: "Legacy Management".to_string(),
        },
        "show-roster" => PageInfo {
            title: "Show Management".to_string(),
            description: "Program oversight, roster assignments, and content strategy for all active shows".to_string(),
            icon: "M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z".to_string(),
            category: "Content Strategy".to_string(),
        },
        "create-show" => PageInfo {
            title: "New Show Development".to_string(),
            description: "Launch new programming content with strategic planning and roster considerations".to_string(),
            icon: "M12 6v6m0 0v6m0-6h6m-6 0H6".to_string(),
            category: "Content Strategy".to_string(),
        },
        "booker" => PageInfo {
            title: "Event Booking System".to_string(),
            description: "Strategic match creation, storyline development, and creative direction".to_string(),
            icon: "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 712-2h2a2 2 0 712 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 712-2h2a2 2 0 712 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z".to_string(),
            category: "Creative Direction".to_string(),
        },
        "analytics" => PageInfo {
            title: "Business Intelligence Dashboard".to_string(),
            description: "Advanced analytics, performance metrics, and strategic planning with AI insights".to_string(),
            icon: "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z".to_string(),
            category: "Executive Analytics".to_string(),
        },
        "command-center" => PageInfo {
            title: "Executive Command Center".to_string(),
            description: "Real-time system monitoring, strategic alerts, and executive oversight with professional controls".to_string(),
            icon: "M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 714.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 713.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 713.138-3.138z".to_string(),
            category: "Mission Critical".to_string(),
        },
        "bulk-operations" => PageInfo {
            title: "Bulk Operations Center".to_string(),
            description: "Strategic multi-entity operations with impact analysis and professional rollback capabilities".to_string(),
            icon: "M19 11H5m14 0a2 2 0 712 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 712-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10".to_string(),
            category: "Mass Operations".to_string(),
        },
        "executive-reporting" => PageInfo {
            title: "Executive Reporting Suite".to_string(),
            description: "Comprehensive business intelligence reports with professional export and strategic insights".to_string(),
            icon: "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 712-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z".to_string(),
            category: "Business Intelligence".to_string(),
        },
        "design-system-showcase" => PageInfo {
            title: "Design System Showcase".to_string(),
            description: "Professional design system components and patterns for WWE enterprise applications".to_string(),
            icon: "M7 21a4 4 0 01-4-4V5a2 2 0 712-2h4a2 2 0 712 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z".to_string(),
            category: "Design System".to_string(),
        },
        _ => PageInfo {
            title: "WWE Universe Manager".to_string(),
            description: "Wrestling Management System (WMS)".to_string(),
            icon: "M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4".to_string(),
            category: "General".to_string(),
        },
    }
}

/// Generate professional breadcrumbs based on current navigation context
pub fn generate_breadcrumbs(context: &NavigationContext) -> Vec<BreadcrumbItem> {
    let mut breadcrumbs = vec![];

    // Always start with Executive Dashboard
    breadcrumbs.push(BreadcrumbItem {
        label: "Executive Command".to_string(),
        route: Some("promotion-dashboard".to_string()),
        icon: Some("M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4".to_string()),
        is_current: context.current_page == "promotion-dashboard",
    });

    // Add context-specific breadcrumbs
    match context.current_page.as_str() {
        "wrestlers" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Talent Management".to_string(),
                route: None,
                icon: Some("M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z".to_string()),
                is_current: true,
            });
        }
        "create-wrestler" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Talent Management".to_string(),
                route: Some("wrestlers".to_string()),
                icon: Some("M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z".to_string()),
                is_current: false,
            });
            breadcrumbs.push(BreadcrumbItem {
                label: "New Talent Acquisition".to_string(),
                route: None,
                icon: Some("M12 6v6m0 0v6m0-6h6m-6 0H6".to_string()),
                is_current: true,
            });
        }
        "titles" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Championship Division".to_string(),
                route: None,
                icon: Some("M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z".to_string()),
                is_current: true,
            });
        }
        "create-title" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Championship Division".to_string(),
                route: Some("titles".to_string()),
                icon: Some("M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z".to_string()),
                is_current: false,
            });
            breadcrumbs.push(BreadcrumbItem {
                label: "New Championship".to_string(),
                route: None,
                icon: Some("M12 6v6m0 0v6m0-6h6m-6 0H6".to_string()),
                is_current: true,
            });
        }
        "show-roster" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Show Management".to_string(),
                route: None,
                icon: Some("M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z".to_string()),
                is_current: true,
            });
        }
        "create-show" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Show Management".to_string(),
                route: Some("show-roster".to_string()),
                icon: Some("M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z".to_string()),
                is_current: false,
            });
            breadcrumbs.push(BreadcrumbItem {
                label: "New Show Development".to_string(),
                route: None,
                icon: Some("M12 6v6m0 0v6m0-6h6m-6 0H6".to_string()),
                is_current: true,
            });
        }
        "booker" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Event Booking".to_string(),
                route: None,
                icon: Some("M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z".to_string()),
                is_current: true,
            });
        }
        "analytics" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Business Intelligence".to_string(),
                route: None,
                icon: Some("M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z".to_string()),
                is_current: true,
            });
        }
        "command-center" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Command Center".to_string(),
                route: None,
                icon: Some("M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 713.138-3.138z".to_string()),
                is_current: true,
            });
        }
        "bulk-operations" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Bulk Operations".to_string(),
                route: None,
                icon: Some("M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10".to_string()),
                is_current: true,
            });
        }
        "executive-reporting" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Executive Reporting".to_string(),
                route: None,
                icon: Some("M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z".to_string()),
                is_current: true,
            });
        }
        "design-system-showcase" => {
            breadcrumbs.push(BreadcrumbItem {
                label: "Design System Showcase".to_string(),
                route: None,
                icon: Some("M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z".to_string()),
                is_current: true,
            });
        }
        _ => {}
    }

    breadcrumbs
}

/// Professional breadcrumb component with DaisyUI styling
#[component]
pub fn ExecutiveBreadcrumbs(
    /// Current navigation context
    navigation_context: NavigationContext,
    /// Navigation handler for breadcrumb clicks
    _set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let breadcrumbs = generate_breadcrumbs(&navigation_context);

    let breadcrumbs_len = breadcrumbs.len();
    
    view! {
        <div class="breadcrumbs text-sm font-medium bg-base-200/30 rounded-lg p-3 mb-4">
            <ul class="flex items-center space-x-1">
                {breadcrumbs.into_iter().enumerate().map(|(index, item)| {
                    let is_last = index == breadcrumbs_len - 1;
                    let label = item.label;
                    let is_current = item.is_current;
                    
                    view! {
                        <li class="flex items-center">
                            <div class={
                                if is_current {
                                    "flex items-center gap-2 px-3 py-2 rounded-md transition-colors duration-200 text-primary font-semibold bg-primary/10"
                                } else {
                                    "flex items-center gap-2 px-3 py-2 rounded-md transition-colors duration-200 text-base-content/70"
                                }
                            }>
                                <span>{label}</span>
                            </div>
                            
                            // Separator (except for last item)
                            {if !is_last {
                                view! {
                                    <svg class="w-4 h-4 mx-2 text-base-content/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                    </svg>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

/// Workflow steps component for complex processes
#[component]
pub fn WorkflowSteps(
    /// Current step (0-based index)
    current_step: ReadSignal<usize>,
    /// Step labels and descriptions
    steps: Vec<(String, String)>,
    /// Optional click handler for step navigation
    on_step_click: Option<Callback<usize>>,
) -> impl IntoView {
    view! {
        <div class="w-full">
            <ul class="steps w-full">
                // TODO: Fix For component parsing errors
                // For now, showing simple step indicators
                {steps.into_iter().enumerate().map(|(step_index, (step_label, step_description))| {
                    let is_current = current_step.get() == step_index;
                    let is_completed = current_step.get() > step_index;
                    let _is_clickable = on_step_click.is_some();

                    view! {
                        <li class={
                            let mut classes = vec!["step"];
                            if is_current {
                                classes.push("step-primary");
                            } else if is_completed {
                                classes.push("step-success");
                            }
                            classes.join(" ")
                        }>
                            <div class="text-center">
                                <div class="font-semibold">{step_label}</div>
                                <div class="text-xs text-base-content/70">{step_description}</div>
                            </div>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

/// Context-aware page header with executive styling
#[component]
pub fn ExecutivePageHeader(
    /// Current page identifier
    current_page: ReadSignal<String>,
    /// Navigation handler for breadcrumb clicks
    set_current_page: WriteSignal<String>,
    /// Optional additional actions for the header
    children: Children,
) -> impl IntoView {
    view! {
        <div class="mb-6">
            // Breadcrumbs
            <ExecutiveBreadcrumbs 
                navigation_context=NavigationContext {
                    current_page: current_page.get(),
                    parent_pages: vec![],
                    show_id: None,
                    wrestler_id: None,
                    title_id: None,
                }
                _set_current_page=set_current_page
            />
            
            // Page Header
            {
                let page_info = get_page_info(&current_page.get());
                view! {
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-4">
                            <div class="w-12 h-12 bg-gradient-to-br from-primary via-accent to-secondary rounded-xl flex items-center justify-center shadow-lg">
                                <svg class="w-7 h-7 text-base-100" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={page_info.icon} />
                                </svg>
                            </div>
                            <div>
                                <h1 class="text-3xl font-bold text-base-content mb-2 bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
                                    {page_info.title}
                                </h1>
                                <div class="flex items-center gap-3">
                                    <div class="badge badge-primary badge-sm">
                                        {page_info.category}
                                    </div>
                                    <p class="text-base-content/70 text-sm">
                                        {page_info.description}
                                    </p>
                                </div>
                            </div>
                        </div>
                        
                        // Actions section
                        <div class="flex items-center gap-3">
                            {children()}
                            <ExecutiveShortcuts set_current_page />
                        </div>
                    </div>
                }
            }
        </div>
    }
}

/// Quick navigation shortcuts for executive users
#[component]
pub fn ExecutiveShortcuts(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let shortcuts = vec![
        ("wrestlers", "Talent Management", "M17 20h5v-2a3 3 0 00-5.356-1.857"),
        ("titles", "Championships", "M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"),
        ("show-roster", "Show Management", "M15 10l4.553-2.276A1 1 0 0721 8.618v6.764a1 1 0 01-1.447.894L15 14"),
        ("booker", "Event Booking", "M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 712-2h2a2 2 0 712 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 712-2h2a2 2 0 712 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"),
        ("analytics", "Business Intelligence", "M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 712-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"),
        ("command-center", "Command Center", "M9 12l2 2 4-4"),
    ];

    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost btn-square tooltip tooltip-left" data-tip="Quick Navigation">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
            </div>
            <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-64 border border-base-300">
                <li class="menu-title">
                    <span class="text-base-content/60">"Quick Navigation"</span>
                </li>
                {shortcuts.into_iter().map(|(route, label, icon)| {
                    let route_clone = route.to_string();
                    view! {
                        <li>
                            <a 
                                class="gap-3"
                                on:click=move |_| set_current_page.set(route_clone.clone())
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={icon} />
                                </svg>
                                {label}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}