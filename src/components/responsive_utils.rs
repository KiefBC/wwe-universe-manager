use leptos::prelude::*;
use wasm_bindgen::prelude::*;

/// Responsive utilities for mobile, tablet, and desktop optimization
/// 
/// This module provides components and utilities for creating responsive
/// executive-quality interfaces across all device types and orientations.

// Note: MediaQueryList temporarily disabled due to web_sys compatibility
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window"])]
//     fn matchMedia(query: &str) -> web_sys::MediaQueryList;
// }

/// Mobile-first responsive container with executive styling
#[component]
pub fn ResponsiveContainer(
    /// Child components to render inside the responsive container
    children: ChildrenFn,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
    /// Enable mobile-optimized padding
    #[prop(default = true)]
    mobile_padding: bool,
    /// Enable safe area insets for notched devices
    #[prop(default = true)]
    safe_areas: bool,
    /// Container max width breakpoint
    #[prop(into, default = "max-w-7xl".to_string())]
    max_width: String,
) -> impl IntoView {
    let container_class = format!(
        "w-full mx-auto {} {} {} {}",
        max_width,
        if mobile_padding {
            "px-mobile-md sm:px-tablet-sm lg:px-6 xl:px-8"
        } else {
            ""
        },
        if safe_areas {
            "pt-safe-top pb-safe-bottom pl-safe-left pr-safe-right"
        } else {
            ""
        },
        class
    );

    view! {
        <div class=container_class>
            {children()}
        </div>
    }
}

/// Mobile-optimized executive card with touch-friendly interactions
#[component]
pub fn ResponsiveExecutiveCard(
    /// Card title
    title: String,
    /// Card content
    children: ChildrenFn,
    /// Card variant for styling
    #[prop(into, default = "default".to_string())]
    variant: String,
    /// Enable touch feedback animations
    #[prop(default = true)]
    touch_feedback: bool,
    /// Optional click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
) -> impl IntoView {
    let card_class = match variant.as_str() {
        "primary" => "card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20",
        "secondary" => "card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20",
        "success" => "card bg-gradient-to-br from-success/10 to-success/5 border border-success/20",
        "warning" => "card bg-gradient-to-br from-warning/10 to-warning/5 border border-warning/20",
        "error" => "card bg-gradient-to-br from-error/10 to-error/5 border border-error/20",
        _ => "card bg-base-100 border border-base-300/50",
    };

    let full_class = format!(
        "{} shadow-lg transition-all duration-professional {} {} {} {}",
        card_class,
        if touch_feedback {
            "active:animate-touch-feedback hover:shadow-executive"
        } else {
            "hover:shadow-executive"
        },
        if on_click.is_some() {
            "cursor-pointer min-h-touch-target"
        } else {
            ""
        },
        "rounded-professional sm:rounded-executive",
        class
    );

    view! {
        <div 
            class=full_class
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            <div class="card-body p-mobile-md sm:p-tablet-sm lg:p-6">
                <h3 class="card-title text-base sm:text-lg font-semibold mb-mobile-sm sm:mb-tablet-sm">
                    {title}
                </h3>
                <div class="space-y-mobile-sm sm:space-y-tablet-sm">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Responsive grid system with mobile-first approach
#[component]
pub fn ResponsiveGrid(
    /// Child components to render in grid
    children: ChildrenFn,
    /// Grid columns for different breakpoints (mobile, tablet, desktop)
    #[prop(into, default = "grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4".to_string())]
    columns: String,
    /// Gap between grid items for different breakpoints
    #[prop(into, default = "gap-mobile-md sm:gap-tablet-sm lg:gap-6".to_string())]
    gap: String,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
) -> impl IntoView {
    let grid_class = format!("grid {} {} {}", columns, gap, class);

    view! {
        <div class=grid_class>
            {children()}
        </div>
    }
}

/// Mobile-optimized navigation with executive styling
#[component]
pub fn ResponsiveNavigation(
    /// Navigation items
    items: Vec<(String, String, Option<AnyView>)>, // (label, href, icon)
    /// Current active page
    current_page: String,
    /// Navigation click handler
    on_navigate: Box<dyn Fn(String) + 'static>,
    /// Enable mobile hamburger menu
    #[prop(default = true)]
    mobile_menu: bool,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
) -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <nav class=format!("bg-base-100 border-b border-base-300/50 sticky top-0 z-50 {}", class)>
            <ResponsiveContainer>
                <div class="flex items-center justify-between h-16 sm:h-20">
                    // Logo/Brand
                    <div class="flex items-center space-x-4">
                        <div class="w-8 h-8 sm:w-10 sm:h-10 bg-primary rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 sm:w-6 sm:h-6 text-primary-content" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                            </svg>
                        </div>
                        <h1 class="text-lg sm:text-xl font-bold text-base-content">
                            "WWE Universe Manager"
                        </h1>
                    </div>

                    // Desktop Navigation
                    <div class="hidden md:flex items-center space-x-1">
                        {items.iter().map(|(label, href, icon)| {
                            let is_active = current_page == *href;
                            let nav_class = if is_active {
                                "btn btn-primary btn-sm"
                            } else {
                                "btn btn-ghost btn-sm"
                            };
                            let href_clone = href.clone();
                            let on_navigate_clone = on_navigate.clone();
                            
                            view! {
                                <button 
                                    class=nav_class
                                    on:click=move |_| {
                                        on_navigate_clone(href_clone.clone());
                                    }
                                >
                                    {icon.clone().unwrap_or_else(|| view!{}.into_any())}
                                    <span class="ml-1">{label}</span>
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // Mobile Menu Button
                    {if mobile_menu {
                        view! {
                            <button
                                class="md:hidden btn btn-ghost btn-square btn-sm"
                                on:click=move |_| set_menu_open.update(|open| *open = !*open)
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    {if menu_open.get() {
                                        view! {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                                        }.into_any()
                                    }}
                                </svg>
                            </button>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                </div>

                // Mobile Menu Dropdown
                {if mobile_menu {
                    view! {
                        <div class=format!(
                            "md:hidden overflow-hidden transition-all duration-300 {}",
                            if menu_open.get() { "max-h-96 opacity-100" } else { "max-h-0 opacity-0" }
                        )>
                            <div class="py-mobile-sm space-y-1">
                                {items.iter().map(|(label, href, icon)| {
                                    let is_active = current_page == *href;
                                    let nav_class = if is_active {
                                        "btn btn-primary btn-sm w-full justify-start"
                                    } else {
                                        "btn btn-ghost btn-sm w-full justify-start"
                                    };
                                    let href_clone = href.clone();
                                    let on_navigate_clone = on_navigate.clone();
                                    
                                    view! {
                                        <button 
                                            class=nav_class
                                            on:click=move |_| {
                                                on_navigate_clone(href_clone.clone());
                                                set_menu_open.set(false);
                                            }
                                        >
                                            {icon.clone().unwrap_or_else(|| view!{}.into_any())}
                                            <span class="ml-2">{label}</span>
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </ResponsiveContainer>
        </nav>
    }
}

/// Touch-optimized button with executive styling
#[component]
pub fn ResponsiveExecutiveButton(
    /// Button content/text
    children: ChildrenFn,
    /// Button variant
    #[prop(into, default = "primary".to_string())]
    variant: String,
    /// Button size (responsive)
    #[prop(into, default = "responsive".to_string())]
    size: String,
    /// Optional icon
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn() + 'static>>,
    /// Enable ripple effect
    #[prop(default = true)]
    ripple: bool,
    /// Make button full width on mobile
    #[prop(default = false)]
    mobile_full_width: bool,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    let button_class = format!(
        "btn btn-{} {} {} {} {} {} {}",
        variant,
        match size.as_str() {
            "responsive" => "btn-sm sm:btn-md",
            "xs" => "btn-xs",
            "sm" => "btn-sm", 
            "md" => "btn-md",
            "lg" => "btn-lg",
            _ => "btn-md",
        },
        if mobile_full_width { "w-full sm:w-auto" } else { "" },
        if ripple { "transition-all duration-professional active:animate-touch-feedback" } else { "" },
        "min-h-touch-target min-w-touch-target",
        if disabled { "btn-disabled" } else { "" },
        class
    );

    view! {
        <button 
            class=button_class
            disabled=disabled
            on:click=move |_| {
                if let Some(handler) = &on_click {
                    handler();
                }
            }
        >
            {if let Some(icon) = icon {
                view! {
                    <span class="mr-1 sm:mr-2">{icon}</span>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            {children()}
        </button>
    }
}

/// Responsive stats/metrics display with mobile optimization
#[component]
pub fn ResponsiveExecutiveStats(
    /// Stats data (title, value, description, variant)
    stats: Vec<(String, String, String, String)>,
    /// Layout orientation
    #[prop(into, default = "responsive".to_string())]
    layout: String, // "horizontal", "vertical", "responsive"
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
) -> impl IntoView {
    let stats_class = match layout.as_str() {
        "horizontal" => "stats stats-horizontal",
        "vertical" => "stats stats-vertical",
        "responsive" => "stats stats-vertical sm:stats-horizontal",
        _ => "stats stats-vertical sm:stats-horizontal",
    };

    let full_class = format!(
        "{} shadow-lg border border-base-300/50 bg-gradient-to-br from-base-100 to-base-200/50 {}",
        stats_class,
        class
    );

    view! {
        <div class=full_class>
            {stats.into_iter().enumerate().map(|(index, (title, value, desc, variant))| {
                let text_color = match variant.as_str() {
                    "primary" => "text-primary",
                    "secondary" => "text-secondary", 
                    "accent" => "text-accent",
                    "success" => "text-success",
                    "warning" => "text-warning",
                    "error" => "text-error",
                    _ => "text-base-content",
                };

                let delay_style = format!("animation-delay: {}ms", index * 100);

                view! {
                    <div class="stat animate-mobile-fade-in p-mobile-md sm:p-tablet-sm lg:p-6" style=delay_style>
                        <div class=format!("stat-figure {}", text_color)>
                            <div class=format!("w-8 h-8 sm:w-10 sm:h-10 bg-{}/20 rounded-full flex items-center justify-center", variant)>
                                <svg class="w-4 h-4 sm:w-5 sm:h-5 stroke-current" fill="none" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                </svg>
                            </div>
                        </div>
                        <div class=format!("stat-title {}/80 text-xs sm:text-sm", text_color)>{title}</div>
                        <div class=format!("stat-value {} text-lg sm:text-2xl font-bold", text_color)>{value}</div>
                        <div class=format!("stat-desc {}/60 text-xs", text_color)>{desc}</div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

/// Mobile-optimized modal with executive styling
#[component]
pub fn ResponsiveExecutiveModal(
    /// Modal content
    children: ChildrenFn,
    /// Modal visibility
    open: ReadSignal<bool>,
    /// Close handler
    on_close: Box<dyn Fn() + 'static>,
    /// Modal title
    #[prop(into)]
    title: String,
    /// Modal size
    #[prop(into, default = "responsive".to_string())]
    size: String,
    /// Enable backdrop blur
    #[prop(default = true)]
    backdrop_blur: bool,
) -> impl IntoView {
    let modal_class = if open.get() {
        "modal modal-open"
    } else {
        "modal"
    };

    let modal_box_class = match size.as_str() {
        "responsive" => "modal-box w-11/12 max-w-lg sm:max-w-2xl lg:max-w-4xl max-h-[90vh]",
        "small" => "modal-box w-11/12 max-w-md",
        "large" => "modal-box w-11/12 max-w-4xl",
        "fullscreen" => "modal-box w-full h-full max-w-none max-h-none rounded-none",
        _ => "modal-box w-11/12 max-w-2xl",
    };

    let backdrop_class = if backdrop_blur {
        "modal-backdrop backdrop-blur-sm bg-black/50"
    } else {
        "modal-backdrop bg-black/50"
    };

    view! {
        <div class=modal_class>
            <div 
                class=backdrop_class
                on:click=move |_| on_close()
            ></div>
            <div class=format!("{} animate-mobile-scale-in relative", modal_box_class)>
                // Mobile-optimized header with close button
                <div class="flex items-center justify-between p-mobile-md sm:p-tablet-sm border-b border-base-300">
                    <h2 class="text-lg sm:text-xl font-bold text-base-content">{title}</h2>
                    <button 
                        class="btn btn-ghost btn-square btn-sm min-h-touch-target min-w-touch-target"
                        on:click=move |_| on_close()
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                        </svg>
                    </button>
                </div>

                // Modal content with responsive padding
                <div class="p-mobile-md sm:p-tablet-sm lg:p-6 overflow-y-auto max-h-[calc(90vh-8rem)]">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Responsive data table with mobile card fallback
#[component]
pub fn ResponsiveExecutiveTable(
    /// Table headers
    headers: Vec<String>,
    /// Table data rows
    rows: Vec<Vec<String>>,
    /// Enable mobile card view
    #[prop(default = true)]
    mobile_cards: bool,
    /// Additional CSS classes
    #[prop(into, default = "".to_string())]
    class: String,
) -> impl IntoView {
    view! {
        <div class=format!("overflow-hidden rounded-professional border border-base-300 {}", class)>
            // Desktop table view
            <div class="hidden md:block overflow-x-auto">
                <table class="table table-zebra w-full">
                    <thead class="bg-base-200">
                        <tr>
                            {headers.iter().map(|header| view! {
                                <th class="font-semibold text-base-content py-tablet-sm px-tablet-sm">{header}</th>
                            }).collect::<Vec<_>>()}
                        </tr>
                    </thead>
                    <tbody>
                        {rows.iter().enumerate().map(|(index, row)| {
                            let delay_style = format!("animation-delay: {}ms", index * 50);
                            view! {
                                <tr class="hover:bg-base-200/50 animate-mobile-fade-in" style=delay_style>
                                    {row.iter().map(|cell| view! {
                                        <td class="py-tablet-sm px-tablet-sm">{cell}</td>
                                    }).collect::<Vec<_>>()}
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>

            // Mobile card view
            {if mobile_cards {
                view! {
                    <div class="md:hidden space-y-mobile-sm p-mobile-md">
                        {rows.iter().enumerate().map(|(index, row)| {
                            let delay_style = format!("animation-delay: {}ms", index * 100);
                            view! {
                                <div class="card bg-base-100 border border-base-300/50 shadow-sm animate-mobile-slide-up" style=delay_style>
                                    <div class="card-body p-mobile-md space-y-mobile-sm">
                                        {row.iter().enumerate().map(|(col_index, cell)| {
                                            let header = headers.get(col_index).unwrap_or(&"".to_string());
                                            view! {
                                                <div class="flex justify-between items-center">
                                                    <span class="text-sm font-medium text-base-content/70">{header}</span>
                                                    <span class="text-sm text-base-content">{cell}</span>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}