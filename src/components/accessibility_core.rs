use leptos::prelude::*;
use leptos::html;
use web_sys::{Element, HtmlElement, HtmlInputElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Comprehensive WCAG 2.1 AA Accessibility Core Components
/// 
/// Professional accessibility implementation that enhances the beloved executive design
/// without compromising visual quality. Provides screen reader support, keyboard navigation,
/// color contrast compliance, and motor/cognitive accessibility features.

/// Executive Accessible Button with full WCAG compliance
#[component]
pub fn AccessibleExecutiveButton(
    /// Button label for screen readers
    label: String,
    /// Button variant matching executive design
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
    /// ARIA description for complex actions
    #[prop(optional)]
    aria_description: Option<String>,
    /// ARIA label when different from visible text
    #[prop(optional)]
    aria_label: Option<String>,
    /// Whether this controls expanded content
    #[prop(optional)]
    aria_expanded: Option<bool>,
    /// ID of element this button controls
    #[prop(optional)]
    aria_controls: Option<String>,
    /// Whether button is pressed (for toggles)
    #[prop(optional)]
    aria_pressed: Option<bool>,
    /// Keyboard shortcut hint
    #[prop(optional)]
    keyboard_hint: Option<String>,
    /// Custom classes to add
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let button_ref = NodeRef::<html::Button>::new();
    
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
        "xs" => "btn-xs min-h-8 min-w-16", // Ensure minimum touch target
        "sm" => "btn-sm min-h-10 min-w-20", 
        "lg" => "btn-lg min-h-12 min-w-24",
        "xl" => "btn-lg px-8 min-h-12 min-w-32",
        "executive" => "btn-lg px-8 py-3 min-h-12 min-w-32",
        _ => "min-h-11 min-w-24", // WCAG 44px minimum touch target
    };
    
    // High contrast focus ring for accessibility
    let focus_classes = "focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-base-100 focus:outline-none";
    
    let base_classes = format!("btn {} {} {} {} transition-all duration-200 relative", 
        variant_classes, size_classes, focus_classes, class);
    
    // Keyboard handler with enhanced navigation
    let handle_keydown = move |e: ev::KeyboardEvent| {
        match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if !disabled && !loading {
                    if let Some(click_handler) = on_click {
                        click_handler.run(());
                    }
                }
            },
            _ => {}
        }
    };
    
    // Build ARIA attributes
    let aria_label_attr = aria_label.unwrap_or_else(|| label.clone());
    let aria_describedby = keyboard_hint.as_ref().map(|_| format!("{}-hint", label.replace(" ", "-").to_lowercase()));
    
    view! {
        <div class="relative inline-block">
            <button 
                node_ref=button_ref
                type="button"
                class=base_classes
                disabled={disabled || loading}
                aria-label=aria_label_attr
                aria-description=aria_description
                aria-expanded=aria_expanded
                aria-controls=aria_controls
                aria-pressed=aria_pressed
                aria-describedby=aria_describedby.clone()
                role="button"
                tabindex="0"
                on:click=move |_| {
                    if !disabled && !loading {
                        if let Some(click_handler) = on_click {
                            click_handler.run(());
                        }
                    }
                }
                on:keydown=handle_keydown
            >
                // Loading state with screen reader announcement
                {if loading {
                    view! {
                        <>
                            <span class="loading loading-spinner loading-sm mr-2" aria-hidden="true"></span>
                            <span class="sr-only">"Loading..."</span>
                            <span aria-hidden="true">{label}</span>
                        </>
                    }.into_any()
                } else {
                    view! {
                        <span>{label}</span>
                    }.into_any()
                }}
                
                // High contrast indicator for disabled state
                {if disabled && !loading {
                    view! {
                        <span class="absolute inset-0 bg-base-content/10 rounded-btn pointer-events-none" aria-hidden="true"></span>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </button>
            
            // Keyboard hint tooltip
            {if let Some(hint) = keyboard_hint {
                view! {
                    <div 
                        id=aria_describedby.unwrap_or_default()
                        class="absolute -bottom-8 left-1/2 transform -translate-x-1/2 
                               px-2 py-1 text-xs bg-base-200 text-base-content rounded
                               opacity-0 pointer-events-none transition-opacity duration-200
                               group-focus:opacity-100 group-hover:opacity-100"
                        role="tooltip"
                    >
                        {hint}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}

/// Accessible Executive Form Field with comprehensive validation
#[component]
pub fn AccessibleExecutiveField(
    /// Field label
    label: String,
    /// Field type (text, email, password, etc.)
    #[prop(default = "text".to_string())]
    field_type: String,
    /// Field value signal
    value: ReadWriteSignal<String>,
    /// Field placeholder
    #[prop(optional)]
    placeholder: Option<String>,
    /// Whether field is required
    #[prop(default = false)]
    required: bool,
    /// Whether field is disabled
    #[prop(default = false)]
    disabled: bool,
    /// Validation error message
    #[prop(optional)]
    error: Option<String>,
    /// Help text for field
    #[prop(optional)]
    help_text: Option<String>,
    /// Success message
    #[prop(optional)]
    success_message: Option<String>,
    /// Input validation pattern
    #[prop(optional)]
    pattern: Option<String>,
    /// Minimum length
    #[prop(optional)]
    min_length: Option<u32>,
    /// Maximum length
    #[prop(optional)]
    max_length: Option<u32>,
    /// Autocomplete attribute
    #[prop(default = "off".to_string())]
    autocomplete: String,
    /// Custom classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let field_id = format!("field-{}", label.replace(" ", "-").to_lowercase());
    let error_id = format!("{}-error", field_id);
    let help_id = format!("{}-help", field_id);
    let input_ref = NodeRef::<html::Input>::new();
    
    // Validation state
    let is_valid = Signal::derive(move || {
        let val = value.get();
        if required && val.is_empty() {
            false
        } else if let Some(min) = min_length {
            val.len() >= min as usize
        } else {
            true
        }
    });
    
    let has_error = error.is_some();
    let has_success = success_message.is_some() && is_valid.get();
    
    // Field classes with high contrast focus
    let input_classes = format!(
        "input input-bordered w-full transition-all duration-200 
         focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 
         focus:ring-offset-base-100 focus:outline-none {}
         {}",
        if has_error { 
            "input-error border-error focus:ring-error/50" 
        } else if has_success { 
            "input-success border-success focus:ring-success/50" 
        } else { 
            "focus:ring-primary/50" 
        },
        class
    );
    
    // ARIA describedby builder
    let mut aria_describedby_parts = Vec::new();
    if help_text.is_some() {
        aria_describedby_parts.push(help_id.clone());
    }
    if has_error {
        aria_describedby_parts.push(error_id.clone());
    }
    let aria_describedby = if aria_describedby_parts.is_empty() {
        None
    } else {
        Some(aria_describedby_parts.join(" "))
    };
    
    view! {
        <div class="form-control w-full space-y-2">
            // Label with visual focus indicator
            <label 
                for=field_id.clone()
                class="label cursor-pointer group"
            >
                <span class="label-text font-medium text-base-content 
                           group-focus-within:text-primary transition-colors duration-200">
                    {label}
                    {if required {
                        view! {
                            <span 
                                class="text-error ml-1" 
                                aria-label="required"
                                title="This field is required"
                            >
                                "*"
                            </span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                </span>
            </label>
            
            // Input field
            <input 
                node_ref=input_ref
                type=field_type
                id=field_id
                class=input_classes
                placeholder=placeholder.unwrap_or_default()
                required=required
                disabled=disabled
                pattern=pattern
                minlength=min_length.map(|n| n.to_string())
                maxlength=max_length.map(|n| n.to_string())
                autocomplete=autocomplete
                aria-required=required.to_string()
                aria-invalid=has_error.to_string()
                aria-describedby=aria_describedby
                prop:value=move || value.get()
                on:input=move |e| {
                    value.set(event_target_value(&e));
                }
                on:focus=move |_| {
                    // Add focus ring animation
                    if let Some(input) = input_ref.get() {
                        let _ = input.class_list().add_1("animate-form-field-focus");
                    }
                }
            />
            
            // Help text
            {if let Some(help) = help_text {
                view! {
                    <div 
                        id=help_id
                        class="text-sm text-base-content/70 flex items-center gap-1"
                    >
                        <svg 
                            class="w-4 h-4 text-info" 
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                        >
                            <path 
                                stroke-linecap="round" 
                                stroke-linejoin="round" 
                                stroke-width="2" 
                                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z">
                            </path>
                        </svg>
                        {help}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            
            // Error message with animation
            {if let Some(err) = error {
                view! {
                    <div 
                        id=error_id
                        class="text-sm text-error flex items-center gap-1 animate-validation-error"
                        role="alert"
                        aria-live="polite"
                    >
                        <svg 
                            class="w-4 h-4 text-error flex-shrink-0" 
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                        >
                            <path 
                                stroke-linecap="round" 
                                stroke-linejoin="round" 
                                stroke-width="2" 
                                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z">
                            </path>
                        </svg>
                        {err}
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            
            // Success message
            {if let Some(success) = success_message {
                if has_success {
                    view! {
                        <div class="text-sm text-success flex items-center gap-1 animate-validation-success">
                            <svg 
                                class="w-4 h-4 text-success" 
                                fill="none" 
                                stroke="currentColor" 
                                viewBox="0 0 24 24"
                                aria-hidden="true"
                            >
                                <path 
                                    stroke-linecap="round" 
                                    stroke-linejoin="round" 
                                    stroke-width="2" 
                                    d="M5 13l4 4L19 7">
                                </path>
                            </svg>
                            {success}
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}

/// Professional Skip Link for keyboard navigation
#[component]
pub fn SkipNavigation() -> impl IntoView {
    view! {
        <nav class="sr-only focus-within:not-sr-only">
            <a 
                href="#main-content"
                class="absolute top-0 left-0 z-50 px-4 py-2 bg-primary text-primary-content
                       transform -translate-y-full focus:translate-y-0
                       transition-transform duration-200 rounded-br-md
                       font-medium hover:bg-primary-focus"
            >
                "Skip to main content"
            </a>
        </nav>
    }
}

/// Executive Live Region for screen reader announcements
#[component]
pub fn AccessibilityLiveRegion(
    /// Message to announce
    message: ReadSignal<String>,
    /// Politeness level
    #[prop(default = "polite".to_string())]
    aria_live: String,
    /// Whether the message should be atomic
    #[prop(default = false)]
    aria_atomic: bool,
) -> impl IntoView {
    view! {
        <div 
            class="sr-only"
            aria-live=aria_live
            aria-atomic=aria_atomic.to_string()
            role="status"
        >
            {move || message.get()}
        </div>
    }
}

/// Professional Loading Announcements
#[component]
pub fn AccessibleLoadingState(
    /// Loading message
    #[prop(default = "Loading content...".to_string())]
    message: String,
    /// Whether currently loading
    loading: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div 
            class={move || if loading.get() { "sr-only" } else { "hidden" }}
            aria-live="polite"
            aria-busy={move || loading.get().to_string()}
            role="status"
        >
            {move || if loading.get() { message.clone() } else { String::new() }}
        </div>
    }
}

/// Executive Focus Management
#[derive(Clone)]
pub struct FocusManager {
    previous_focus: Option<Element>,
}

impl FocusManager {
    pub fn new() -> Self {
        Self { previous_focus: None }
    }
    
    pub fn store_focus(&mut self) {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            self.previous_focus = document.active_element();
        }
    }
    
    pub fn restore_focus(&self) {
        if let Some(element) = &self.previous_focus {
            if let Ok(html_element) = element.dyn_ref::<HtmlElement>() {
                let _ = html_element.focus();
            }
        }
    }
    
    pub fn focus_first_interactive(&self, container_selector: &str) {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Ok(container) = document.query_selector(container_selector) {
                if let Some(container) = container {
                    if let Ok(first_interactive) = container.query_selector(
                        "button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])"
                    ) {
                        if let Some(element) = first_interactive {
                            if let Ok(html_element) = element.dyn_ref::<HtmlElement>() {
                                let _ = html_element.focus();
                            }
                        }
                    }
                }
            }
        }
    }
}