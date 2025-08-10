use leptos::prelude::*;
use leptos::{ev, html};
use web_sys::{console, Element, HtmlElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Professional Animation Utilities for WWE Universe Manager
/// 
/// Provides executive-quality micro-interactions and loading states
/// with consistent timing, easing, and accessibility support

/// Professional loading skeleton component with animation
#[component]
pub fn ProfessionalSkeleton(
    /// Width class (e.g., "w-full", "w-32")
    #[prop(default = "w-full".to_string())]
    width: String,
    /// Height class (e.g., "h-4", "h-8") 
    #[prop(default = "h-4".to_string())]
    height: String,
    /// Additional CSS classes
    #[prop(default = String::new())]
    class: String,
    /// Enable shimmer effect
    #[prop(default = true)]
    shimmer: bool,
    /// Border radius class
    #[prop(default = "rounded".to_string())]
    radius: String,
) -> impl IntoView {
    let shimmer_class = if shimmer {
        "bg-gradient-to-r from-base-300 via-base-200 to-base-300 bg-[length:200%_100%] animate-loading-skeleton"
    } else {
        "bg-base-300 animate-loading-skeleton"
    };

    view! {
        <div class=format!("{} {} {} {} {}", width, height, radius, shimmer_class, class)></div>
    }
}

/// Professional loading spinner with executive styling
#[component] 
pub fn ProfessionalSpinner(
    /// Size variant
    #[prop(default = "md".to_string())]
    size: String,
    /// Color variant
    #[prop(default = "primary".to_string())]
    color: String,
    /// Additional message
    #[prop(default = None)]
    message: Option<String>,
) -> impl IntoView {
    let size_class = match size.as_str() {
        "xs" => "loading-xs",
        "sm" => "loading-sm", 
        "lg" => "loading-lg",
        "xl" => "loading-xl",
        _ => "loading-md",
    };

    let color_class = match color.as_str() {
        "secondary" => "text-secondary",
        "accent" => "text-accent",
        "success" => "text-success",
        "warning" => "text-warning",
        "error" => "text-error",
        _ => "text-primary",
    };

    view! {
        <div class="flex flex-col items-center gap-3 py-4">
            <span class=format!("loading loading-spinner {} {}", size_class, color_class)></span>
            {message.map(|msg| view! {
                <p class="text-base-content/70 text-sm animate-professional-pulse">{msg}</p>
            })}
        </div>
    }
}

/// Professional card entrance animation
#[component]
pub fn AnimatedCard(
    /// Child content
    children: ChildrenFn,
    /// Delay for staggered entrance (in ms)
    #[prop(default = 0)]
    delay: u32,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let delay_style = if delay > 0 {
        format!("animation-delay: {}ms;", delay)
    } else {
        String::new()
    };

    view! {
        <div 
            class=format!("animate-card-entrance {}", class)
            style=delay_style
        >
            {children()}
        </div>
    }
}

/// Professional button with press animation
#[component]
pub fn AnimatedButton(
    /// Button content
    children: ChildrenFn,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Box<dyn Fn(ev::MouseEvent) + 'static>>,
    /// Button variant
    #[prop(default = "btn-primary".to_string())]
    variant: String,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
    /// Disabled state
    #[prop(default = false)]
    disabled: bool,
    /// Loading state
    #[prop(default = false)]
    loading: bool,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();

    let handle_click = move |e: ev::MouseEvent| {
        if let Some(button) = button_ref.get() {
            // Add press animation
            let _ = button.class_list().add_1("animate-button-press");
            
            // Remove animation after completion
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

    let disabled_class = if disabled || loading { "btn-disabled" } else { "" };
    let loading_content = if loading {
        view! {
            <span class="loading loading-spinner loading-sm mr-2"></span>
        }
    } else {
        view! {}.into_any()
    };

    view! {
        <button 
            node_ref=button_ref
            class=format!("btn {} {} {} transition-all duration-quick hover:animate-executive-hover", variant, disabled_class, class)
            on:click=handle_click
            disabled=disabled || loading
        >
            {loading_content}
            {children()}
        </button>
    }
}

/// Professional form field with focus animations
#[component]
pub fn AnimatedFormField(
    /// Field label
    label: String,
    /// Input type
    #[prop(default = "text".to_string())]
    input_type: String,
    /// Current value
    value: ReadSignal<String>,
    /// Value setter
    set_value: WriteSignal<String>,
    /// Placeholder text
    #[prop(default = String::new())]
    placeholder: String,
    /// Error message
    #[prop(default = None)]
    error: Option<String>,
    /// Success state
    #[prop(default = false)]
    success: bool,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
    /// Required field
    #[prop(default = false)]
    required: bool,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let (focused, set_focused) = signal(false);
    
    let handle_focus = move |_| {
        set_focused.set(true);
        if let Some(input) = input_ref.get() {
            let _ = input.class_list().add_1("animate-form-field-focus");
        }
    };

    let handle_blur = move |_| {
        set_focused.set(false);
        if let Some(input) = input_ref.get() {
            let _ = input.class_list().remove_1("animate-form-field-focus");
        }
    };

    let handle_input = move |e: ev::Event| {
        let target = e.target().unwrap();
        let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
        set_value.set(input.value());
    };

    let input_class = if error.is_some() {
        "input input-bordered input-error animate-validation-error"
    } else if success {
        "input input-bordered input-success animate-validation-success"
    } else if focused.get() {
        "input input-bordered input-primary"
    } else {
        "input input-bordered"
    };

    let label_class = if focused.get() || !value.get().is_empty() {
        "label-text text-primary transition-colors duration-quick"
    } else {
        "label-text transition-colors duration-quick"
    };

    view! {
        <div class=format!("form-control {}", class)>
            <label class="label">
                <span class=label_class>
                    {label}
                    {if required { view! { <span class="text-error ml-1">"*"</span> } } else { view! {} }}
                </span>
            </label>
            <input 
                node_ref=input_ref
                type=input_type
                class=input_class
                placeholder=placeholder
                value=value
                on:input=handle_input
                on:focus=handle_focus
                on:blur=handle_blur
                required=required
            />
            {error.map(|err| view! {
                <label class="label">
                    <span class="label-text-alt text-error animate-validation-error">{err}</span>
                </label>
            })}
            {if success && error.is_none() {
                view! {
                    <label class="label">
                        <span class="label-text-alt text-success animate-validation-success">
                            <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                            </svg>
                            "Looks good!"
                        </span>
                    </label>
                }
            } else {
                view! {}
            }}
        </div>
    }
}

/// Professional toast notification with slide-in animation
#[component]
pub fn AnimatedToast(
    /// Toast message
    message: String,
    /// Toast type
    #[prop(default = "info".to_string())]
    toast_type: String,
    /// Show state
    show: ReadSignal<bool>,
    /// Auto-dismiss after ms (0 = no auto dismiss)
    #[prop(default = 3000)]
    auto_dismiss: u32,
    /// Dismiss handler
    #[prop(optional)]
    on_dismiss: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let toast_ref = NodeRef::<html::Div>::new();

    // Auto-dismiss effect
    if auto_dismiss > 0 {
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(auto_dismiss).await;
            if let Some(handler) = &on_dismiss {
                handler();
            }
        });
    }

    let (alert_class, icon_svg) = match toast_type.as_str() {
        "success" => ("alert-success", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
        }),
        "warning" => ("alert-warning", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.732 15c-.77.833.192 2.5 1.732 2.5z"></path>
            </svg>
        }),
        "error" => ("alert-error", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        }),
        _ => ("alert-info", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        }),
    };

    view! {
        <div class=move || if show.get() { 
            format!("toast toast-top toast-end animate-toast-slide-in z-50") 
        } else { 
            "hidden".to_string() 
        }>
            <div class=format!("alert {} shadow-lg", alert_class)>
                {icon_svg}
                <span>{message}</span>
                {on_dismiss.map(|handler| {
                    let dismiss_handler = move |_| handler();
                    view! {
                        <button class="btn btn-sm btn-ghost" on:click=dismiss_handler>
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                            </svg>
                        </button>
                    }
                })}
            </div>
        </div>
    }
}

/// Professional modal with slide-up animation
#[component]
pub fn AnimatedModal(
    /// Modal title
    title: String,
    /// Show state
    show: ReadSignal<bool>,
    /// Close handler
    on_close: Box<dyn Fn() + 'static>,
    /// Modal content
    children: ChildrenFn,
    /// Modal size
    #[prop(default = "md".to_string())]
    size: String,
    /// Disable backdrop click to close
    #[prop(default = false)]
    disable_backdrop_close: bool,
) -> impl IntoView {
    let modal_ref = NodeRef::<html::Div>::new();

    let handle_backdrop_click = move |e: ev::MouseEvent| {
        if !disable_backdrop_close {
            if let Some(target) = e.target() {
                if let Some(modal) = modal_ref.get() {
                    if target == modal.clone().into() {
                        on_close();
                    }
                }
            }
        }
    };

    let size_class = match size.as_str() {
        "sm" => "modal-box w-11/12 max-w-md",
        "lg" => "modal-box w-11/12 max-w-4xl",
        "xl" => "modal-box w-11/12 max-w-6xl",
        "full" => "modal-box w-full max-w-none h-full max-h-none",
        _ => "modal-box w-11/12 max-w-2xl",
    };

    view! {
        <div 
            class=move || if show.get() { "modal modal-open" } else { "modal" }
            node_ref=modal_ref
            on:click=handle_backdrop_click
        >
            <div class=format!("{} animate-modal-slide-up", size_class)>
                <div class="flex items-center justify-between mb-4">
                    <h3 class="font-bold text-lg">{title}</h3>
                    <button class="btn btn-sm btn-circle btn-ghost" on:click=move |_| on_close()>
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                </div>
                <div>
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Professional progress bar with smooth fill animation
#[component]
pub fn AnimatedProgress(
    /// Progress value (0-100)
    value: ReadSignal<f32>,
    /// Progress color
    #[prop(default = "primary".to_string())]
    color: String,
    /// Show percentage text
    #[prop(default = true)]
    show_percentage: bool,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let color_class = match color.as_str() {
        "secondary" => "progress-secondary",
        "accent" => "progress-accent", 
        "success" => "progress-success",
        "warning" => "progress-warning",
        "error" => "progress-error",
        _ => "progress-primary",
    };

    view! {
        <div class=format!("flex items-center gap-3 {}", class)>
            <progress 
                class=format!("progress {} flex-1 animate-progress-fill", color_class)
                value=move || value.get()
                max="100"
                style=move || format!("--progress-value: {}%", value.get())
            ></progress>
            {if show_percentage {
                view! {
                    <span class="text-sm font-medium text-base-content/70 min-w-[3rem] text-right animate-metric-counter">
                        {move || format!("{:.0}%", value.get())}
                    </span>
                }
            } else {
                view! {}
            }}
        </div>
    }
}

/// Professional staggered children animation container
#[component]
pub fn StaggeredContainer(
    /// Child content
    children: ChildrenFn,
    /// Stagger delay between children (in ms)
    #[prop(default = 100)]
    stagger_delay: u32,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    view! {
        <div 
            class=format!("animate-stagger-children {}", class)
            style=format!("--stagger-delay: {}ms", stagger_delay)
        >
            {children()}
        </div>
    }
}