use leptos::prelude::*;

/// Executive Button System Component
/// 
/// Professional button system providing:
/// - Unified button variants and states consistent with CEO Dashboard
/// - Professional hover and active states
/// - Loading states and disabled handling
/// - Icon support and proper accessibility
/// - Theme-responsive styling
#[component]
pub fn ExecutiveButton(
    /// Button text content
    text: String,
    /// Button variant
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Button size
    #[prop(default = "normal".to_string())]
    size: String,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Whether button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Whether button is in loading state
    #[prop(default = false)]
    loading: bool,
    /// Optional icon (placed before text)
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Optional icon (placed after text)
    #[prop(optional)]
    icon_end: Option<AnyView>,
    /// Button type for forms
    #[prop(default = "button".to_string())]
    button_type: String,
    /// Whether button should be full width
    #[prop(default = false)]
    full_width: bool,
    /// Custom classes to add
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    
    let variant_classes = match variant.as_str() {
        "primary" => "btn-primary",
        "secondary" => "btn-secondary", 
        "accent" => "btn-accent",
        "neutral" => "btn-neutral",
        "ghost" => "btn-ghost",
        "link" => "btn-link",
        "info" => "btn-info",
        "success" => "btn-success",
        "warning" => "btn-warning",
        "error" => "btn-error",
        "executive" => "btn-primary shadow-executive hover:shadow-premium",
        "professional" => "btn-neutral shadow-professional",
        "premium" => "bg-gradient-to-r from-primary to-accent border-none text-primary-content hover:shadow-executive",
        _ => "btn-primary",
    };
    
    let size_classes = match size.as_str() {
        "xs" => "btn-xs",
        "sm" => "btn-sm", 
        "lg" => "btn-lg",
        "xl" => "btn-lg px-8",
        "executive" => "btn-lg px-8 py-3",
        _ => "", // normal size
    };
    
    let width_class = if full_width { "btn-wide w-full" } else { "" };
    
    let base_classes = format!("btn {} {} {} {}", 
        variant_classes, size_classes, width_class, class);
    
    view! {
        <button 
            type={button_type}
            class={base_classes}
            disabled={disabled || loading}
            on:click=move |_| {
                if !disabled && !loading {
                    if let Some(click_handler) = on_click {
                        click_handler.run(());
                    }
                }
            }
        >
            {if loading {
                view! {
                    <span class="loading loading-spinner loading-sm"></span>
                }.into_any()
            } else if let Some(icon_view) = icon {
                icon_view.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
            
            <span>{text}</span>
            
            {
                if let Some(end_icon) = icon_end {
                    end_icon.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }
        </button>
    }
}

/// Executive Button Group Component
/// 
/// Professional button grouping with consistent spacing
#[component]
pub fn ExecutiveButtonGroup(
    /// Button group alignment
    #[prop(default = "start".to_string())]
    alignment: String,
    /// Spacing between buttons
    #[prop(default = "normal".to_string())]
    spacing: String,
    /// Whether to stack on mobile
    #[prop(default = false)]
    stack_mobile: bool,
    /// The buttons to display
    children: Children,
) -> impl IntoView {
    
    let alignment_class = match alignment.as_str() {
        "center" => "justify-center",
        "end" => "justify-end",
        "between" => "justify-between",
        "around" => "justify-around",
        _ => "justify-start",
    };
    
    let spacing_class = match spacing.as_str() {
        "tight" => "gap-2",
        "loose" => "gap-6",
        "executive" => "gap-4 sm:gap-6",
        _ => "gap-3",
    };
    
    let responsive_class = if stack_mobile {
        "flex flex-col sm:flex-row"
    } else {
        "flex flex-row flex-wrap"
    };
    
    let classes = format!("{} {} {}", responsive_class, alignment_class, spacing_class);
    
    view! {
        <div class={classes}>
            {children()}
        </div>
    }
}

/// Executive Icon Button Component
/// 
/// Professional icon-only button for actions and controls
#[component]
pub fn ExecutiveIconButton(
    /// Button icon
    icon: AnyView,
    /// Tooltip text
    #[prop(optional)]
    tooltip: Option<String>,
    /// Button variant
    #[prop(default = "ghost".to_string())]
    variant: String,
    /// Button size
    #[prop(default = "normal".to_string())]
    size: String,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Whether button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Shape variant
    #[prop(default = "square".to_string())]
    shape: String,
) -> impl IntoView {
    
    let variant_classes = match variant.as_str() {
        "primary" => "btn-primary",
        "secondary" => "btn-secondary",
        "accent" => "btn-accent",
        "ghost" => "btn-ghost",
        "neutral" => "btn-neutral",
        _ => "btn-ghost",
    };
    
    let size_classes = match size.as_str() {
        "xs" => "btn-xs",
        "sm" => "btn-sm",
        "lg" => "btn-lg",
        _ => "",
    };
    
    let shape_classes = match shape.as_str() {
        "circle" => "btn-circle",
        "square" => "btn-square",
        _ => "btn-square",
    };
    
    let classes = format!("btn {} {} {}", variant_classes, size_classes, shape_classes);
    
    view! {
        <div class={if tooltip.is_some() { "tooltip" } else { "" }} data-tip={tooltip.unwrap_or_default()}>
            <button 
                class={classes}
                disabled={disabled}
                on:click=move |_| {
                    if !disabled {
                        if let Some(click_handler) = on_click {
                            click_handler.run(());
                        }
                    }
                }
            >
                {icon}
            </button>
        </div>
    }
}

/// Executive Action Button Component
/// 
/// Specialized button for primary actions with enhanced styling
#[component]
pub fn ExecutiveActionButton(
    /// Action title
    title: String,
    /// Action description
    description: String,
    /// Action icon
    icon: AnyView,
    /// Click handler
    on_click: Callback<()>,
    /// Button variant
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Whether action is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Optional badge text
    #[prop(optional)]
    badge: Option<String>,
) -> impl IntoView {
    
    let variant_classes = match variant.as_str() {
        "primary" => "btn-primary shadow-executive hover:shadow-premium",
        "executive" => "bg-gradient-to-r from-primary to-accent border-none text-primary-content shadow-executive hover:shadow-premium",
        "professional" => "btn-neutral shadow-professional hover:shadow-executive",
        _ => "btn-primary",
    };
    
    view! {
        <button 
            class={format!("btn {} btn-lg justify-start text-left p-4 h-auto min-h-[4rem]", variant_classes)}
            disabled={disabled}
            on:click=move |_| on_click.run(())
        >
            <div class="flex items-center gap-4 w-full">
                <div class="w-12 h-12 bg-base-100/20 rounded-xl flex items-center justify-center flex-shrink-0">
                    {icon}
                </div>
                
                <div class="flex-1 text-left">
                    <div class="flex items-center gap-2 mb-1">
                        <h3 class="font-bold text-base">{title}</h3>
                        {move || {
                            if let Some(badge_text) = badge.clone() {
                                view! {
                                    <div class="badge badge-sm bg-base-100/20 text-current border-none">
                                        {badge_text}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                    <p class="text-sm opacity-80 leading-relaxed">
                        {description}
                    </p>
                </div>
                
                <svg class="w-5 h-5 opacity-60 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                </svg>
            </div>
        </button>
    }
}

/// Executive Toggle Button Component
/// 
/// Professional toggle button for settings and preferences
#[component]
pub fn ExecutiveToggleButton(
    /// Toggle label
    label: String,
    /// Toggle state
    checked: ReadSignal<bool>,
    /// Toggle state setter
    set_checked: WriteSignal<bool>,
    /// Optional description
    #[prop(optional)]
    description: Option<String>,
    /// Toggle size
    #[prop(default = "normal".to_string())]
    size: String,
    /// Whether toggle is disabled
    #[prop(default = false)]
    disabled: bool,
) -> impl IntoView {
    
    let size_classes = match size.as_str() {
        "sm" => "toggle-sm",
        "lg" => "toggle-lg",
        _ => "",
    };
    
    view! {
        <div class="form-control">
            <label class="label cursor-pointer justify-start gap-4">
                <input 
                    type="checkbox"
                    class={format!("toggle toggle-primary {}", size_classes)}
                    checked={move || checked.get()}
                    disabled={disabled}
                    on:change=move |_| set_checked.set(!checked.get_untracked())
                />
                <div class="flex-1">
                    <span class="label-text font-medium">{label}</span>
                    {move || {
                        if let Some(desc) = description.clone() {
                            view! {
                                <div class="label-text-alt text-base-content/60 mt-1">
                                    {desc}
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </div>
            </label>
        </div>
    }
}