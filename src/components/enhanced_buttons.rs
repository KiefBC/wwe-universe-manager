use leptos::prelude::*;
use leptos::ev;
use leptos::html;
use web_sys::{Element, HtmlElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Professional Enhanced Buttons for WWE Universe Manager
/// 
/// Provides executive-quality button interactions with micro-animations,
/// loading states, and professional feedback

/// Executive action button with enhanced micro-interactions
#[component]
pub fn ExecutiveActionButton(
    /// Button content
    children: ChildrenFn,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    /// Button variant
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Button size
    #[prop(default = "md".to_string())]
    size: String,
    /// Loading state
    #[prop(default = false)]
    loading: bool,
    /// Success state (shows checkmark temporarily)
    #[prop(default = false)]
    success: bool,
    /// Error state
    #[prop(default = false)]
    error: bool,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Icon position
    #[prop(default = "left".to_string())]
    icon_position: String,
    /// Icon SVG
    #[prop(optional)]
    icon: Option<AnyView>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
    /// Show ripple effect on click
    #[prop(default = true)]
    ripple: bool,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();
    let (show_success, set_show_success) = signal(false);

    // Watch for success state changes
    Effect::new(move || {
        if success {
            set_show_success.set(true);
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                set_show_success.set(false);
            });
        }
    });

    let handle_click = move |e: ev::MouseEvent| {
        if loading || disabled {
            return;
        }

        if let Some(button) = button_ref.get() {
            // Add press animation
            let _ = button.class_list().add_1("animate-button-press");
            
            // Create ripple effect if enabled
            if ripple {
                create_ripple_effect(&button, &e);
            }
            
            // Remove press animation
            let button_clone = button.clone();
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(150).await;
                let _ = button_clone.class_list().remove_1("animate-button-press");
            });
        }

        if let Some(handler) = &on_click {
            handler(e);
        }
    };

    let button_variant = match variant.as_str() {
        "secondary" => "btn-secondary",
        "accent" => "btn-accent",
        "success" => "btn-success",
        "warning" => "btn-warning",
        "error" => "btn-error",
        "ghost" => "btn-ghost",
        "outline" => "btn-outline",
        _ => "btn-primary",
    };

    let button_size = match size.as_str() {
        "xs" => "btn-xs",
        "sm" => "btn-sm",
        "lg" => "btn-lg",
        "xl" => "btn-xl",
        _ => "",
    };

    let button_state = if loading {
        "btn-disabled opacity-70"
    } else if error {
        "btn-error animate-error-shake"
    } else if disabled {
        "btn-disabled"
    } else {
        ""
    };

    // Success checkmark icon
    let success_icon = view! {
        <svg class="w-5 h-5 animate-success-bounce" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
        </svg>
    };

    // Loading spinner
    let loading_icon = view! {
        <span class="loading loading-spinner loading-sm"></span>
    };

    // Content with proper icon positioning
    let content = move || {
        if show_success.get() {
            view! {
                <div class="flex items-center gap-2">
                    {success_icon.clone()}
                    "Success!"
                </div>
            }.into_any()
        } else if loading {
            view! {
                <div class="flex items-center gap-2">
                    {loading_icon.clone()}
                    "Loading..."
                </div>
            }.into_any()
        } else {
            view! {
                <div class="flex items-center gap-2">
                    {if icon_position == "left" {
                        icon.clone().unwrap_or_else(|| view! {}.into_any())
                    } else {
                        view! {}.into_any()
                    }}
                    <span>{children()}</span>
                    {if icon_position == "right" {
                        icon.clone().unwrap_or_else(|| view! {}.into_any())
                    } else {
                        view! {}.into_any()
                    }}
                </div>
            }.into_any()
        }
    };

    view! {
        <button 
            node_ref=button_ref
            class=format!("btn {} {} {} {} transition-all duration-professional hover:animate-executive-hover relative overflow-hidden", 
                         button_variant, button_size, button_state, class)
            on:click=handle_click
            disabled=disabled || loading
        >
            {content}
        </button>
    }
}

/// Professional floating action button with enhanced animations
#[component]
pub fn ExecutiveFAB(
    /// Button content/icon
    children: ChildrenFn,
    /// Click handler
    on_click: Box<dyn Fn(ev::MouseEvent) + 'static>,
    /// FAB color
    #[prop(default = "primary".to_string())]
    color: String,
    /// FAB size
    #[prop(default = "md".to_string())]
    size: String,
    /// Position
    #[prop(default = "bottom-right".to_string())]
    position: String,
    /// Show tooltip
    #[prop(optional)]
    tooltip: Option<String>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let fab_ref = NodeRef::<html::Button>::new();

    let handle_click = move |e: ev::MouseEvent| {
        if let Some(fab) = fab_ref.get() {
            // Enhanced FAB animation
            let _ = fab.class_list().add_1("animate-success-bounce");
            
            let fab_clone = fab.clone();
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(600).await;
                let _ = fab_clone.class_list().remove_1("animate-success-bounce");
            });
        }
        
        on_click(e);
    };

    let color_class = match color.as_str() {
        "secondary" => "btn-secondary",
        "accent" => "btn-accent",
        "success" => "btn-success",
        "warning" => "btn-warning",
        "error" => "btn-error",
        _ => "btn-primary",
    };

    let size_class = match size.as_str() {
        "sm" => "w-12 h-12",
        "lg" => "w-20 h-20",
        "xl" => "w-24 h-24",
        _ => "w-16 h-16",
    };

    let position_class = match position.as_str() {
        "bottom-left" => "fixed bottom-6 left-6",
        "top-right" => "fixed top-20 right-6",
        "top-left" => "fixed top-20 left-6",
        _ => "fixed bottom-6 right-6",
    };

    let tooltip_attrs = if let Some(tip) = tooltip {
        format!("tooltip tooltip-left tooltip-primary", )
    } else {
        String::new()
    };

    view! {
        <div 
            class=format!("{} z-50 {}", position_class, tooltip_attrs)
            data-tip=tooltip
        >
            <button 
                node_ref=fab_ref
                class=format!("btn btn-circle {} {} shadow-executive hover:shadow-premium transition-all duration-professional hover:animate-executive-hover animate-card-entrance {}", 
                             color_class, size_class, class)
                on:click=handle_click
            >
                {children()}
            </button>
        </div>
    }
}

/// Professional button group with enhanced styling
#[component]
pub fn ExecutiveButtonGroup(
    /// Group orientation
    #[prop(default = "horizontal".to_string())]
    orientation: String,
    /// Button content
    children: ChildrenFn,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let group_class = if orientation == "vertical" {
        "btn-group btn-group-vertical"
    } else {
        "btn-group"
    };

    view! {
        <div class=format!("{} animate-card-entrance {}", group_class, class)>
            {children()}
        </div>
    }
}

/// Professional toggle button with state animations
#[component]
pub fn ExecutiveToggleButton(
    /// Button label
    label: String,
    /// Current state
    active: ReadSignal<bool>,
    /// State setter
    set_active: WriteSignal<bool>,
    /// Active variant
    #[prop(default = "primary".to_string())]
    active_variant: String,
    /// Inactive variant
    #[prop(default = "ghost".to_string())]
    inactive_variant: String,
    /// Button size
    #[prop(default = "md".to_string())]
    size: String,
    /// Icon for active state
    #[prop(optional)]
    active_icon: Option<AnyView>,
    /// Icon for inactive state
    #[prop(optional)]
    inactive_icon: Option<AnyView>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();

    let handle_click = move |_| {
        let new_state = !active.get();
        set_active.set(new_state);
        
        if let Some(button) = button_ref.get() {
            if new_state {
                let _ = button.class_list().add_1("animate-success-bounce");
            } else {
                let _ = button.class_list().add_1("animate-button-press");
            }
            
            // Clean up animation classes
            let button_clone = button.clone();
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(300).await;
                let _ = button_clone.class_list().remove_1("animate-success-bounce");
                let _ = button_clone.class_list().remove_1("animate-button-press");
            });
        }
    };

    let button_variant = move || {
        if active.get() {
            match active_variant.as_str() {
                "secondary" => "btn-secondary",
                "accent" => "btn-accent", 
                "success" => "btn-success",
                "warning" => "btn-warning",
                "error" => "btn-error",
                _ => "btn-primary",
            }
        } else {
            match inactive_variant.as_str() {
                "outline" => "btn-outline",
                "secondary" => "btn-secondary",
                _ => "btn-ghost",
            }
        }
    };

    let button_size = match size.as_str() {
        "xs" => "btn-xs",
        "sm" => "btn-sm", 
        "lg" => "btn-lg",
        "xl" => "btn-xl",
        _ => "",
    };

    let current_icon = move || {
        if active.get() {
            active_icon.clone()
        } else {
            inactive_icon.clone()
        }
    };

    view! {
        <button 
            node_ref=button_ref
            class=format!("btn {} {} transition-all duration-professional hover:animate-executive-hover {}", 
                         button_variant(), button_size, class)
            on:click=handle_click
        >
            <div class="flex items-center gap-2">
                {move || current_icon().unwrap_or_else(|| view! {}.into_any())}
                <span>{label.clone()}</span>
            </div>
        </button>
    }
}

/// Professional progress button with dynamic states
#[component]
pub fn ExecutiveProgressButton(
    /// Button label
    label: String,
    /// Click handler that returns progress updates
    on_click: Box<dyn Fn() -> () + 'static>,
    /// Current progress (0-100)
    progress: ReadSignal<f32>,
    /// Button variant
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Show progress text
    #[prop(default = true)]
    show_progress: bool,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();
    let (is_active, set_is_active) = signal(false);

    let handle_click = move |_| {
        set_is_active.set(true);
        on_click();
    };

    // Reset active state when progress completes
    Effect::new(move || {
        if progress.get() >= 100.0 && is_active.get() {
            set_is_active.set(false);
        }
    });

    let button_variant = match variant.as_str() {
        "secondary" => "btn-secondary",
        "accent" => "btn-accent",
        "success" => "btn-success", 
        "warning" => "btn-warning",
        "error" => "btn-error",
        _ => "btn-primary",
    };

    let button_content = move || {
        if is_active.get() && progress.get() < 100.0 {
            if show_progress {
                view! {
                    <div class="flex items-center gap-3 w-full">
                        <span class="loading loading-spinner loading-sm"></span>
                        <div class="flex-1">
                            <div class="flex items-center gap-2">
                                <span class="text-sm">"Processing..."</span>
                                <span class="text-xs opacity-70">{move || format!("{:.0}%", progress.get())}</span>
                            </div>
                            <progress 
                                class="progress progress-primary w-full h-1 mt-1" 
                                value=move || progress.get()
                                max="100"
                            ></progress>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="flex items-center gap-2">
                        <span class="loading loading-spinner loading-sm"></span>
                        "Processing..."
                    </div>
                }.into_any()
            }
        } else if progress.get() >= 100.0 && is_active.get() {
            view! {
                <div class="flex items-center gap-2 animate-success-bounce">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    "Complete!"
                </div>
            }.into_any()
        } else {
            view! {
                <span>{label.clone()}</span>
            }.into_any()
        }
    };

    view! {
        <button 
            node_ref=button_ref
            class=format!("btn {} transition-all duration-professional hover:animate-executive-hover min-w-32 {}", 
                         button_variant, class)
            on:click=handle_click
            disabled=is_active.get()
        >
            {button_content}
        </button>
    }
}

/// Helper function to create ripple effect
fn create_ripple_effect(button: &HtmlElement, event: &ev::MouseEvent) {
    let rect = button.get_bounding_client_rect();
    let x = event.client_x() as f64 - rect.left();
    let y = event.client_y() as f64 - rect.top();
    
    // Create ripple element
    if let Ok(ripple) = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("span")
    {
        let _ = ripple.set_attribute("class", "absolute pointer-events-none rounded-full bg-white/30 transform scale-0 animate-ping");
        let _ = ripple.set_attribute("style", &format!(
            "left: {}px; top: {}px; width: 20px; height: 20px; margin-left: -10px; margin-top: -10px;",
            x, y
        ));
        
        // Add to button
        let _ = button.append_child(&ripple);
        
        // Remove after animation
        let ripple_clone = ripple.clone();
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(600).await;
            let _ = ripple_clone.remove();
        });
    }
}