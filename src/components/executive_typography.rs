use leptos::prelude::*;

/// Executive Typography Scale Component
/// 
/// Professional typography system providing:
/// - Consistent heading hierarchy (H1-H6) with executive styling
/// - Theme-aware colors and professional spacing
/// - Responsive font sizes and line heights
/// - Gradient text effects for premium branding
/// - Professional text variants for different contexts
#[component]
pub fn ExecutiveHeading(
    /// Heading level (1-6)
    #[prop(default = 1)]
    level: u8,
    /// Heading text content
    text: String,
    /// Optional subtitle text
    #[prop(optional)]
    subtitle: Option<String>,
    /// Style variant
    #[prop(default = "default".to_string())]
    variant: String,
    /// Whether to use gradient text effect
    #[prop(default = false)]
    gradient: bool,
    /// Custom gradient colors (if gradient is true)
    #[prop(default = "from-primary to-accent".to_string())]
    gradient_colors: String,
    /// Text alignment
    #[prop(default = "left".to_string())]
    align: String,
    /// Optional margin bottom override
    #[prop(optional)]
    margin_bottom: Option<String>,
) -> impl IntoView {
    
    let base_classes = match level {
        1 => "text-4xl xl:text-5xl font-bold",
        2 => "text-3xl xl:text-4xl font-bold", 
        3 => "text-2xl xl:text-3xl font-bold",
        4 => "text-xl xl:text-2xl font-semibold",
        5 => "text-lg xl:text-xl font-semibold",
        6 => "text-base xl:text-lg font-medium",
        _ => "text-2xl font-bold",
    };
    
    let margin_class = margin_bottom.unwrap_or_else(|| {
        match level {
            1 => "mb-4 xl:mb-6".to_string(),
            2 => "mb-4 xl:mb-5".to_string(),
            3 => "mb-3 xl:mb-4".to_string(),
            4 => "mb-3".to_string(),
            5 => "mb-2".to_string(),
            6 => "mb-2".to_string(),
            _ => "mb-3".to_string(),
        }
    });
    
    let align_class = match align.as_str() {
        "center" => "text-center",
        "right" => "text-right",
        _ => "text-left",
    };
    
    let text_classes = if gradient {
        format!("{} bg-gradient-to-r {} bg-clip-text text-transparent", base_classes, gradient_colors)
    } else {
        match variant.as_str() {
            "primary" => format!("{} text-primary", base_classes),
            "secondary" => format!("{} text-secondary", base_classes),
            "accent" => format!("{} text-accent", base_classes),
            "muted" => format!("{} text-base-content/70", base_classes),
            _ => format!("{} text-base-content", base_classes),
        }
    };
    
    let final_classes = format!("{} {} {}", text_classes, align_class, margin_class);
    
    view! {
        <div class={align_class}>
            {match level {
                1 => view! { <h1 class={final_classes.clone()}>{text.clone()}</h1> }.into_any(),
                2 => view! { <h2 class={final_classes.clone()}>{text.clone()}</h2> }.into_any(),
                3 => view! { <h3 class={final_classes.clone()}>{text.clone()}</h3> }.into_any(),
                4 => view! { <h4 class={final_classes.clone()}>{text.clone()}</h4> }.into_any(),
                5 => view! { <h5 class={final_classes.clone()}>{text.clone()}</h5> }.into_any(),
                6 => view! { <h6 class={final_classes.clone()}>{text.clone()}</h6> }.into_any(),
                _ => view! { <h2 class={final_classes.clone()}>{text.clone()}</h2> }.into_any(),
            }}
            
            {move || {
                if let Some(sub) = subtitle.clone() {
                    let subtitle_classes = match level {
                        1 => "text-lg text-base-content/80 max-w-3xl mx-auto leading-relaxed mt-2",
                        2 => "text-base text-base-content/70 max-w-2xl mx-auto leading-relaxed mt-2",
                        3 => "text-sm text-base-content/70 leading-relaxed mt-1",
                        _ => "text-sm text-base-content/60 leading-relaxed mt-1",
                    };
                    
                    let final_subtitle_classes = if align == "center" {
                        format!("{} {}", subtitle_classes, align_class)
                    } else {
                        subtitle_classes.to_string()
                    };
                    
                    view! {
                        <p class={final_subtitle_classes}>
                            {sub}
                        </p>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Executive Text Component
/// 
/// Professional text component with consistent styling
#[component]
pub fn ExecutiveText(
    /// Text content
    text: String,
    /// Text variant
    #[prop(default = "body".to_string())]
    variant: String,
    /// Text size
    #[prop(default = "normal".to_string())]
    size: String,
    /// Text color
    #[prop(default = "normal".to_string())]
    color: String,
    /// Font weight
    #[prop(default = "normal".to_string())]
    weight: String,
    /// Line height
    #[prop(default = "normal".to_string())]
    leading: String,
    /// Margin bottom
    #[prop(optional)]
    margin_bottom: Option<String>,
) -> impl IntoView {
    
    let size_classes = match size.as_str() {
        "micro" => "text-micro",
        "xs" => "text-xs",
        "sm" => "text-sm",
        "lg" => "text-lg",
        "xl" => "text-xl",
        "executive" => "text-executive",
        "professional" => "text-professional",
        "metric" => "text-metric",
        _ => "text-base",
    };
    
    let color_classes = match color.as_str() {
        "primary" => "text-primary",
        "secondary" => "text-secondary", 
        "accent" => "text-accent",
        "muted" => "text-base-content/70",
        "subtle" => "text-base-content/60",
        "faint" => "text-base-content/50",
        _ => "text-base-content",
    };
    
    let weight_classes = match weight.as_str() {
        "light" => "font-light",
        "medium" => "font-medium",
        "semibold" => "font-semibold",
        "bold" => "font-bold",
        "extrabold" => "font-extrabold",
        _ => "font-normal",
    };
    
    let leading_classes = match leading.as_str() {
        "tight" => "leading-tight",
        "snug" => "leading-snug",
        "relaxed" => "leading-relaxed",
        "loose" => "leading-loose",
        _ => "leading-normal",
    };
    
    let margin_class = margin_bottom.unwrap_or_else(|| {
        match variant.as_str() {
            "caption" => "mb-1".to_string(),
            "body" => "mb-3".to_string(),
            "lead" => "mb-4".to_string(),
            _ => "".to_string(),
        }
    });
    
    let classes = format!("{} {} {} {} {}", 
        size_classes, color_classes, weight_classes, leading_classes, margin_class);
    
    view! {
        <p class={classes}>
            {text}
        </p>
    }
}

/// Executive Label Component
/// 
/// Professional label component for forms and data display
#[component]
pub fn ExecutiveLabel(
    /// Label text
    text: String,
    /// Label variant
    #[prop(default = "default".to_string())]
    variant: String,
    /// Whether the field is required
    #[prop(default = false)]
    required: bool,
    /// Optional description text
    #[prop(optional)]
    description: Option<String>,
) -> impl IntoView {
    
    let label_classes = match variant.as_str() {
        "form" => "label-text text-base-content font-medium",
        "metric" => "stat-title text-primary/80 font-medium text-sm",
        "professional" => "text-professional text-base-content font-semibold",
        "executive" => "text-executive text-primary font-bold",
        _ => "text-sm font-medium text-base-content/80",
    };
    
    view! {
        <div class="space-y-1">
            <label class={label_classes}>
                {text}
                {if required {
                    view! {
                        <span class="text-error ml-1">"*"</span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </label>
            
            {move || {
                if let Some(desc) = description.clone() {
                    view! {
                        <p class="text-xs text-base-content/60 leading-relaxed">
                            {desc}
                        </p>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}

/// Executive Quote Component
/// 
/// Professional blockquote styling for testimonials and emphasis
#[component]
pub fn ExecutiveQuote(
    /// Quote text
    text: String,
    /// Optional author
    #[prop(optional)]
    author: Option<String>,
    /// Optional author title
    #[prop(optional)]
    author_title: Option<String>,
    /// Quote variant
    #[prop(default = "default".to_string())]
    variant: String,
) -> impl IntoView {
    
    let quote_classes = match variant.as_str() {
        "primary" => "border-l-4 border-primary bg-primary/5 p-4 rounded-r-lg",
        "accent" => "border-l-4 border-accent bg-accent/5 p-4 rounded-r-lg",
        "executive" => "border-l-4 border-primary bg-gradient-to-r from-primary/10 to-transparent p-6 rounded-r-xl",
        _ => "border-l-4 border-base-300 bg-base-200/50 p-4 rounded-r-lg",
    };
    
    view! {
        <blockquote class={quote_classes}>
            <div class="space-y-3">
                <p class="text-base-content leading-relaxed italic">
                    "\"" {text} "\""
                </p>
                
                {
                    let author_clone = author.clone();
                    let author_title_clone = author_title.clone();
                    move || {
                        if author_clone.is_some() || author_title_clone.is_some() {
                            view! {
                                <div class="text-sm text-base-content/70">
                                    {
                                        if let Some(auth) = author_clone.clone() {
                                            view! {
                                                <div class="font-medium">{auth}</div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }
                                    {
                                        if let Some(title) = author_title_clone.clone() {
                                            view! {
                                                <div class="text-xs">{title}</div>
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
                }
            </div>
        </blockquote>
    }
}