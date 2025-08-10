use leptos::prelude::*;

/// Theme Enhancement Component
/// 
/// Provides theme-aware enhancements for all executive components:
/// - Professional loading states with theme-aware styling
/// - Consistent gradient patterns across all theme variants
/// - Professional micro-animations and transitions
/// - Theme-specific enhancement overlays
#[component]
pub fn ThemeEnhancedContainer(
    /// The content to enhance with theme features
    children: Children,
    /// Theme enhancement variant
    #[prop(default = "professional".to_string())]
    enhancement_level: String,
    /// Whether to show loading state
    #[prop(default = false)]
    loading: bool,
    /// Loading message
    #[prop(default = "Loading...".to_string())]
    loading_message: String,
) -> impl IntoView {
    
    let enhancement_classes = match enhancement_level.as_str() {
        "executive" => "animate-fade-in-up",
        "premium" => "animate-slide-in-right",
        "professional" => "transition-all duration-200",
        "subtle" => "transition-opacity duration-150",
        _ => "",
    };
    
    view! {
        <div class={format!("relative {}", enhancement_classes)}>
            {if loading {
                view! {
                    <ThemeAwareLoader message={loading_message.clone()} />
                }.into_any()
            } else {
                view! {
                    {children()}
                }.into_any()
            }}
        </div>
    }
}

/// Theme-Aware Loading Component
/// 
/// Professional loading states that adapt to all theme variants:
/// - WWE Executive: Gold shimmer with dark slate backdrop
/// - AEW Modern: Cyan pulse with tech-style animation
/// - NJPW Premium: Purple gradient with luxury animation
/// - Corporate Dark: Blue professional spinner
#[component]
pub fn ThemeAwareLoader(
    /// Loading message to display
    #[prop(default = "Loading...".to_string())]
    message: String,
    /// Loader size variant
    #[prop(default = "normal".to_string())]
    size: String,
    /// Whether to show as overlay
    #[prop(default = false)]
    overlay: bool,
) -> impl IntoView {
    
    let size_classes = match size.as_str() {
        "sm" => "loading-sm",
        "lg" => "loading-lg", 
        "xl" => "loading-lg",
        _ => "",
    };
    
    let container_classes = if overlay {
        "absolute inset-0 bg-base-100/80 backdrop-blur-sm flex items-center justify-center z-50"
    } else {
        "flex items-center justify-center p-8"
    };
    
    view! {
        <div class={container_classes}>
            <div class="text-center space-y-4">
                <div class="flex justify-center">
                    <span class={format!("loading loading-spinner {} text-primary", size_classes)}></span>
                </div>
                <div class="text-base-content/70 text-sm font-medium">{message}</div>
                <div class="flex justify-center space-x-1">
                    <div class="w-2 h-2 bg-primary/60 rounded-full animate-bounce" style="animation-delay: 0ms"></div>
                    <div class="w-2 h-2 bg-primary/60 rounded-full animate-bounce" style="animation-delay: 150ms"></div>
                    <div class="w-2 h-2 bg-primary/60 rounded-full animate-bounce" style="animation-delay: 300ms"></div>
                </div>
            </div>
        </div>
    }
}

/// Professional Gradient Enhancement
/// 
/// Provides consistent gradient patterns that work across all theme variants.
/// Automatically adapts colors based on current theme while maintaining
/// professional appearance standards.
#[component] 
pub fn ThemeGradientWrapper(
    /// Gradient variant
    #[prop(default = "subtle".to_string())]
    variant: String,
    /// Gradient direction
    #[prop(default = "to-br".to_string())]
    direction: String,
    /// Content to wrap with gradient
    children: Children,
    /// Whether to add professional shadow
    #[prop(default = true)]
    shadow: bool,
    /// Border styling
    #[prop(default = "subtle".to_string())]
    border: String,
) -> impl IntoView {
    
    let gradient_classes = match variant.as_str() {
        "primary" => format!("bg-gradient-{} from-primary/10 to-primary/5", direction),
        "secondary" => format!("bg-gradient-{} from-secondary/10 to-secondary/5", direction), 
        "accent" => format!("bg-gradient-{} from-accent/10 to-accent/5", direction),
        "premium" => format!("bg-gradient-{} from-primary/10 via-accent/10 to-secondary/10", direction),
        "executive" => format!("bg-gradient-{} from-primary/15 to-primary/8", direction),
        "professional" => format!("bg-gradient-{} from-base-100 to-base-200/50", direction),
        "subtle" => format!("bg-gradient-{} from-base-100/50 to-transparent", direction),
        _ => "bg-base-100".to_string(),
    };
    
    let border_classes = match border.as_str() {
        "primary" => "border border-primary/20",
        "accent" => "border border-accent/20",
        "professional" => "border border-base-300/50",
        "subtle" => "border border-base-300/30",
        "none" => "",
        _ => "border border-base-300/30",
    };
    
    let shadow_classes = if shadow {
        match variant.as_str() {
            "executive" => "shadow-executive",
            "premium" => "shadow-premium", 
            "professional" => "shadow-professional",
            _ => "shadow-lg",
        }
    } else {
        ""
    };
    
    let classes = format!("{} {} {} rounded-lg transition-all duration-200", 
        gradient_classes, border_classes, shadow_classes);
    
    view! {
        <div class={classes}>
            {children()}
        </div>
    }
}

/// Professional Micro-Animation Component
/// 
/// Subtle, executive-level animations that enhance user experience
/// without being distracting. Adapts timing and style based on theme.
#[component]
pub fn ProfessionalAnimation(
    /// Animation type
    #[prop(default = "fade".to_string())]
    animation: String,
    /// Animation timing
    #[prop(default = "smooth".to_string())]
    timing: String,
    /// Whether animation is active
    #[prop(default = true)]
    active: bool,
    /// Content to animate
    children: Children,
) -> impl IntoView {
    
    let animation_classes = if active {
        match animation.as_str() {
            "fade" => match timing.as_str() {
                "fast" => "animate-fade-in-up duration-300",
                "slow" => "animate-fade-in-up duration-1000",
                _ => "animate-fade-in-up duration-600",
            },
            "slide" => match timing.as_str() {
                "fast" => "animate-slide-in-right duration-300",
                "slow" => "animate-slide-in-right duration-800", 
                _ => "animate-slide-in-right duration-500",
            },
            "glow" => "animate-executive-glow",
            "pulse" => "animate-professional-pulse",
            "hover" => "hover:scale-105 hover:shadow-lg transition-all duration-200",
            _ => "",
        }
    } else {
        ""
    };
    
    view! {
        <div class={animation_classes}>
            {children()}
        </div>
    }
}

/// Theme-Aware Status Indicator
/// 
/// Professional status indicators that adapt their colors and styling
/// based on the current theme while maintaining consistent meaning.
#[component]
pub fn ThemeStatusIndicator(
    /// Status type
    status: String,
    /// Size variant  
    #[prop(default = "normal".to_string())]
    size: String,
    /// Whether to show animation
    #[prop(default = false)]
    animate: bool,
    /// Optional text label
    #[prop(optional)]
    label: Option<String>,
) -> impl IntoView {
    
    let (color_classes, icon) = match status.as_str() {
        "live" => ("text-success bg-success/20", "●"),
        "active" => ("text-primary bg-primary/20", "◉"),
        "pending" => ("text-warning bg-warning/20", "◐"),
        "inactive" => ("text-base-content/50 bg-base-300/20", "○"),
        "error" => ("text-error bg-error/20", "✕"),
        "success" => ("text-success bg-success/20", "✓"),
        _ => ("text-base-content/60 bg-base-300/20", "◯"),
    };
    
    let size_classes = match size.as_str() {
        "sm" => "text-xs px-2 py-1",
        "lg" => "text-base px-3 py-2", 
        _ => "text-sm px-2 py-1",
    };
    
    let animation_class = if animate && status == "live" {
        "animate-pulse"
    } else {
        ""
    };
    
    view! {
        <div class={format!("inline-flex items-center gap-2 rounded-full {} {} {}", color_classes, size_classes, animation_class)}>
            <span class="font-bold">{icon}</span>
            {move || {
                if let Some(text) = label.clone() {
                    view! {
                        <span class="font-medium">{text}</span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}

/// Professional Theme Transition Component
/// 
/// Handles smooth transitions between themes with professional timing
/// and visual feedback. Ensures consistent user experience during theme changes.
#[component]
pub fn ThemeTransitionManager(
    /// Content that undergoes theme transitions
    children: Children,
    /// Whether transition is in progress
    #[prop(default = false)]
    transitioning: bool,
) -> impl IntoView {
    
    view! {
        <div class={format!("transition-all duration-300 ease-in-out {}", 
            if transitioning { "opacity-90" } else { "opacity-100" })}>
            <div class="transform transition-transform duration-200">
                {children()}
            </div>
        </div>
    }
}