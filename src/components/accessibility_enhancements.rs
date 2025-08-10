use leptos::prelude::*;

/// Minimal Accessibility Enhancement Components
/// 
/// Simple, working accessibility components that enhance the existing executive design
/// without complex web-sys integrations

/// Simple Skip Navigation Link
#[component]
pub fn SimpleSkipNavigation() -> impl IntoView {
    view! {
        <nav class="sr-only focus-within:not-sr-only">
            <a 
                href="#main-content"
                class="absolute top-0 left-0 z-50 px-4 py-2 bg-primary text-primary-content
                       transform -translate-y-full focus:translate-y-0
                       transition-transform duration-200 rounded-br-md
                       font-medium hover:bg-primary-focus
                       focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 focus:outline-none"
            >
                "Skip to main content"
            </a>
        </nav>
    }
}

/// Professional Focus Ring Mixin for Any Component
#[component]
pub fn FocusEnhanced(
    /// Child content
    children: ChildrenFn,
    /// Custom focus ring color
    #[prop(default = "primary".to_string())]
    focus_color: String,
    /// Focus ring thickness
    #[prop(default = "4".to_string())]
    focus_width: String,
) -> impl IntoView {
    let focus_classes = format!(
        "focus:ring-{} focus:ring-{}/50 focus:ring-offset-2 focus:ring-offset-base-100 focus:outline-none",
        focus_width, focus_color
    );
    
    view! {
        <div class=focus_classes>
            {children()}
        </div>
    }
}

/// Screen Reader Only Text
#[component]
pub fn ScreenReaderOnly(
    /// Text content for screen readers
    text: String,
) -> impl IntoView {
    view! {
        <span class="sr-only">{text}</span>
    }
}

/// Live Region for Screen Reader Announcements
#[component]
pub fn SimpleLiveRegion(
    /// Message to announce
    message: ReadSignal<String>,
    /// Politeness level
    #[prop(default = "polite".to_string())]
    aria_live: String,
) -> impl IntoView {
    view! {
        <div 
            class="sr-only"
            aria-live=aria_live
            aria-atomic="true"
            role="status"
        >
            {move || message.get()}
        </div>
    }
}

/// Accessible Executive Button with proper ARIA
#[component]
pub fn AccessibleButton(
    /// Button text
    text: String,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// Button variant
    #[prop(default = "primary".to_string())]
    variant: String,
    /// Whether button is disabled
    #[prop(default = false)]
    disabled: bool,
    /// ARIA label (if different from text)
    #[prop(optional)]
    aria_label: Option<String>,
    /// ARIA description
    #[prop(optional)]
    aria_description: Option<String>,
    /// Whether this controls expanded content
    #[prop(optional)]
    aria_expanded: Option<bool>,
    /// ID of element this button controls
    #[prop(optional)]
    aria_controls: Option<String>,
    /// Custom CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let variant_class = format!("btn-{}", variant);
    let button_classes = format!(
        "btn {} {} min-h-11 min-w-24 
         focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 
         focus:ring-offset-base-100 focus:outline-none 
         transition-all duration-200",
        variant_class, class
    );
    
    view! {
        <button
            type="button"
            class=button_classes
            disabled=disabled
            aria-label=aria_label.clone().unwrap_or_else(|| text.clone())
            aria-description=aria_description
            aria-expanded=aria_expanded.map(|b| b.to_string())
            aria-controls=aria_controls
            tabindex="0"
            on:click=move |_| {
                if !disabled {
                    if let Some(handler) = on_click {
                        handler.run(());
                    }
                }
            }
        >
            {text.clone()}
        </button>
    }
}

/// Accessible Card with proper semantic structure
#[component]
pub fn AccessibleCard(
    /// Card title
    title: String,
    /// Card content
    children: ChildrenFn,
    /// Whether this is clickable
    #[prop(default = false)]
    clickable: bool,
    /// Click handler for clickable cards
    #[prop(optional)]
    on_click: Option<Callback<()>>,
    /// ARIA label for the card
    #[prop(optional)]
    aria_label: Option<String>,
    /// Custom CSS classes
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let card_classes = format!(
        "card bg-base-100 border border-base-300/50 shadow-professional 
         transition-all duration-professional {}
         {}",
        if clickable { 
            "cursor-pointer hover:shadow-executive hover:-translate-y-1 
             focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 focus:outline-none" 
        } else { "" },
        class
    );
    
    if clickable {
        view! {
            <article 
                class=card_classes
                role="button"
                tabindex="0"
                aria-label=aria_label.clone().unwrap_or_else(|| format!("Navigate to {}", title))
                on:click=move |_| {
                    if let Some(handler) = on_click {
                        handler.run(());
                    }
                }
                on:keydown=move |e: web_sys::KeyboardEvent| {
                    if e.key() == "Enter" || e.key() == " " {
                        e.prevent_default();
                        if let Some(handler) = on_click {
                            handler.run(());
                        }
                    }
                }
            >
                <div class="card-body">
                    <h3 class="card-title text-xl mb-4">{title.clone()}</h3>
                    {children()}
                </div>
            </article>
        }.into_any()
    } else {
        view! {
            <article 
                class=card_classes
                aria-label=aria_label
            >
                <div class="card-body">
                    <h3 class="card-title text-xl mb-4">{title.clone()}</h3>
                    {children()}
                </div>
            </article>
        }.into_any()
    }
}

/// Keyboard Shortcut Hint
#[component]
pub fn KeyboardHint(
    /// Shortcut keys
    keys: Vec<String>,
    /// Description
    description: String,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-2 rounded bg-base-200/50">
            <span class="text-sm font-medium">{description}</span>
            <div class="flex gap-1">
                {keys.into_iter().map(|key| {
                    view! {
                        <kbd class="kbd kbd-sm">{key}</kbd>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

/// Professional Loading State with Screen Reader Support
#[component]
pub fn AccessibleLoadingState(
    /// Loading message
    #[prop(default = "Loading...".to_string())]
    message: String,
    /// Whether currently loading
    loading: bool,
) -> impl IntoView {
    view! {
        {if loading {
            view! {
                <div 
                    class="flex items-center justify-center p-8"
                    role="status"
                    aria-live="polite"
                    aria-busy="true"
                >
                    <div class="loading loading-spinner loading-lg text-primary mr-4"></div>
                    <span class="text-base-content">{message}</span>
                    <span class="sr-only">"Please wait, content is loading"</span>
                </div>
            }.into_any()
        } else {
            view! {}.into_any()
        }}
    }
}

/// Error State with Recovery Options
#[component]
pub fn AccessibleErrorState(
    /// Error title
    title: String,
    /// Error message
    message: String,
    /// Retry handler
    #[prop(optional)]
    on_retry: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div 
            class="card bg-error/10 border border-error/20 text-error shadow-lg"
            role="alert"
            aria-live="assertive"
        >
            <div class="card-body text-center">
                <svg class="w-12 h-12 mx-auto mb-4 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                          d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                <h2 class="text-xl font-bold mb-2">{title}</h2>
                <p class="mb-4">{message}</p>
                {if let Some(retry_handler) = on_retry {
                    view! {
                        <AccessibleButton 
                            text="Try Again".to_string()
                            variant="error".to_string()
                            on_click=Some(retry_handler)
                            aria_description="Attempt to reload the content".to_string()
                        />
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        </div>
    }
}

/// Simple High Contrast Detection
pub fn prefers_high_contrast() -> bool {
    // Simple implementation - could be enhanced with media queries
    false
}

/// Simple Reduced Motion Detection  
pub fn prefers_reduced_motion() -> bool {
    // Simple implementation - could be enhanced with media queries
    false
}

/// Get appropriate animation class based on user preferences
pub fn get_animation_class(animation: &str) -> String {
    if prefers_reduced_motion() {
        // Provide reduced motion alternative
        match animation {
            "animate-fade-in-up" => "opacity-100".to_string(),
            "animate-slide-in-right" => "opacity-100".to_string(),
            "hover:-translate-y-1" => "hover:opacity-90".to_string(),
            _ => "opacity-100".to_string(),
        }
    } else {
        animation.to_string()
    }
}