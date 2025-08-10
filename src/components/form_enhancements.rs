use leptos::prelude::*;
use leptos::ev;
use leptos::html;
use web_sys::{Element, HtmlElement, HtmlInputElement, HtmlSelectElement};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Professional Form Enhancements for WWE Universe Manager
/// 
/// Provides executive-quality form interactions with real-time validation,
/// smooth animations, and enhanced user feedback

/// Professional form with executive styling and validation
#[component]
pub fn ExecutiveForm(
    /// Form title
    title: String,
    /// Form description
    #[prop(optional)]
    description: Option<String>,
    /// Form content
    children: ChildrenFn,
    /// Submit handler
    on_submit: Box<dyn Fn(ev::SubmitEvent) + 'static>,
    /// Loading state
    #[prop(default = false)]
    loading: bool,
    /// Success state
    #[prop(default = false)]
    success: bool,
    /// Error message
    #[prop(optional)]
    error: Option<String>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let form_ref = NodeRef::<html::Form>::new();

    let handle_submit = move |e: ev::SubmitEvent| {
        e.prevent_default();
        
        // Add submit animation
        if let Some(form) = form_ref.get() {
            let _ = form.class_list().add_1("animate-form-field-focus");
        }
        
        on_submit(e);
    };

    view! {
        <div class=format!("card bg-base-100 border border-base-300/50 shadow-lg animate-card-entrance {}", class)>
            <div class="card-body p-6 lg:p-8">
                // Form header
                <div class="text-center mb-6">
                    <h2 class="text-2xl font-bold text-base-content mb-2">{title}</h2>
                    {description.map(|desc| view! {
                        <p class="text-base-content/70">{desc}</p>
                    })}
                </div>
                
                // Success/Error alerts
                {if success {
                    view! {
                        <div class="alert alert-success mb-4 animate-validation-success">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                            </svg>
                            <span>"Form submitted successfully!"</span>
                        </div>
                    }.into_any()
                } else if let Some(err) = error {
                    view! {
                        <div class="alert alert-error mb-4 animate-validation-error">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>{err}</span>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }}
                
                // Form content
                <form 
                    node_ref=form_ref
                    on:submit=handle_submit
                    class="space-y-6"
                >
                    {children()}
                    
                    // Submit button with loading state
                    <div class="pt-4">
                        <button 
                            type="submit"
                            class="btn btn-primary w-full gap-2 transition-all duration-professional hover:animate-executive-hover"
                            disabled=loading
                        >
                            {if loading {
                                view! {
                                    <>
                                        <span class="loading loading-spinner loading-sm"></span>
                                        "Processing..."
                                    </>
                                }.into_any()
                            } else {
                                view! {
                                    <>
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                        </svg>
                                        "Submit"
                                    </>
                                }.into_any()
                            }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}

/// Enhanced input field with professional styling and validation
#[component]
pub fn ExecutiveInput(
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
    #[prop(optional)]
    error: Option<String>,
    /// Success state
    #[prop(default = false)]
    success: bool,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Helper text
    #[prop(optional)]
    helper: Option<String>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
    /// Validation function
    #[prop(optional)]
    validate: Option<Box<dyn Fn(&String) -> Option<String> + 'static>>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let (focused, set_focused) = signal(false);
    let (local_error, set_local_error) = signal(None::<String>);
    
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
        
        // Validate on blur
        if let Some(validator) = &validate {
            let validation_result = validator(&value.get_untracked());
            set_local_error.set(validation_result);
        }
    };

    let handle_input = move |e: ev::Event| {
        let target = e.target().unwrap();
        let input = target.dyn_into::<HtmlInputElement>().unwrap();
        let new_value = input.value();
        set_value.set(new_value.clone());
        
        // Real-time validation
        if let Some(validator) = &validate {
            let validation_result = validator(&new_value);
            set_local_error.set(validation_result);
        }
    };

    let current_error = move || error.clone().or_else(|| local_error.get());
    let has_error = move || current_error().is_some();
    let is_valid = move || !has_error() && !value.get().is_empty() && (success || local_error.get().is_none());

    let input_class = move || {
        if has_error() {
            "input input-bordered input-error transition-all duration-quick animate-validation-error"
        } else if is_valid() {
            "input input-bordered input-success transition-all duration-quick"
        } else if focused.get() {
            "input input-bordered input-primary transition-all duration-quick"
        } else {
            "input input-bordered transition-all duration-quick hover:border-primary/50"
        }
    };

    let label_class = move || {
        if has_error() {
            "label-text text-error font-medium transition-colors duration-quick"
        } else if focused.get() || !value.get().is_empty() {
            "label-text text-primary font-medium transition-colors duration-quick"
        } else {
            "label-text transition-colors duration-quick"
        }
    };

    view! {
        <div class=format!("form-control {}", class)>
            <label class="label">
                <span class=label_class>
                    {label}
                    {if required { 
                        view! { <span class="text-error ml-1 animate-professional-pulse">"*"</span> } 
                    } else { 
                        view! {} 
                    }}
                </span>
            </label>
            
            <div class="relative">
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
                
                // Success/Error icons
                {move || {
                    if has_error() {
                        view! {
                            <div class="absolute inset-y-0 right-0 flex items-center pr-3 animate-validation-error">
                                <svg class="w-5 h-5 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                            </div>
                        }.into_any()
                    } else if is_valid() {
                        view! {
                            <div class="absolute inset-y-0 right-0 flex items-center pr-3 animate-validation-success">
                                <svg class="w-5 h-5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                </svg>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </div>
            
            // Helper/Error text
            <label class="label">
                {move || {
                    if let Some(err) = current_error() {
                        view! {
                            <span class="label-text-alt text-error animate-validation-error">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                {err}
                            </span>
                        }.into_any()
                    } else if is_valid() {
                        view! {
                            <span class="label-text-alt text-success animate-validation-success">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                </svg>
                                "Looks good!"
                            </span>
                        }.into_any()
                    } else if let Some(help) = helper {
                        view! {
                            <span class="label-text-alt text-base-content/60">{help}</span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </label>
        </div>
    }
}

/// Executive dropdown select with enhanced styling
#[component]
pub fn ExecutiveSelect(
    /// Field label
    label: String,
    /// Current value
    value: ReadSignal<String>,
    /// Value setter
    set_value: WriteSignal<String>,
    /// Options as (value, label) pairs
    options: Vec<(String, String)>,
    /// Placeholder option
    #[prop(default = "Select an option...".to_string())]
    placeholder: String,
    /// Error message
    #[prop(optional)]
    error: Option<String>,
    /// Success state
    #[prop(default = false)]
    success: bool,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Helper text
    #[prop(optional)]
    helper: Option<String>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let select_ref = NodeRef::<html::Select>::new();
    let (focused, set_focused) = signal(false);
    
    let handle_focus = move |_| {
        set_focused.set(true);
        if let Some(select) = select_ref.get() {
            let _ = select.class_list().add_1("animate-form-field-focus");
        }
    };

    let handle_blur = move |_| {
        set_focused.set(false);
        if let Some(select) = select_ref.get() {
            let _ = select.class_list().remove_1("animate-form-field-focus");
        }
    };

    let handle_change = move |e: ev::Event| {
        let target = e.target().unwrap();
        let select = target.dyn_into::<HtmlSelectElement>().unwrap();
        set_value.set(select.value());
    };

    let has_error = move || error.is_some();
    let is_valid = move || !has_error() && !value.get().is_empty() && success;

    let select_class = move || {
        if has_error() {
            "select select-bordered select-error w-full transition-all duration-quick"
        } else if is_valid() {
            "select select-bordered select-success w-full transition-all duration-quick"
        } else if focused.get() {
            "select select-bordered select-primary w-full transition-all duration-quick"
        } else {
            "select select-bordered w-full transition-all duration-quick hover:border-primary/50"
        }
    };

    let label_class = move || {
        if has_error() {
            "label-text text-error font-medium transition-colors duration-quick"
        } else if focused.get() || !value.get().is_empty() {
            "label-text text-primary font-medium transition-colors duration-quick"
        } else {
            "label-text transition-colors duration-quick"
        }
    };

    view! {
        <div class=format!("form-control {}", class)>
            <label class="label">
                <span class=label_class>
                    {label}
                    {if required { 
                        view! { <span class="text-error ml-1 animate-professional-pulse">"*"</span> } 
                    } else { 
                        view! {} 
                    }}
                </span>
            </label>
            
            <select 
                node_ref=select_ref
                class=select_class
                value=value
                on:change=handle_change
                on:focus=handle_focus
                on:blur=handle_blur
                required=required
            >
                <option value="" disabled=true>{placeholder}</option>
                {options.into_iter().map(|(val, label)| view! {
                    <option value=val>{label}</option>
                }).collect::<Vec<_>>()}
            </select>
            
            // Helper/Error text
            <label class="label">
                {move || {
                    if let Some(err) = error.clone() {
                        view! {
                            <span class="label-text-alt text-error animate-validation-error">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                {err}
                            </span>
                        }.into_any()
                    } else if is_valid() {
                        view! {
                            <span class="label-text-alt text-success animate-validation-success">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                </svg>
                                "Great choice!"
                            </span>
                        }.into_any()
                    } else if let Some(help) = helper {
                        view! {
                            <span class="label-text-alt text-base-content/60">{help}</span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </label>
        </div>
    }
}

/// Executive textarea with enhanced styling
#[component]
pub fn ExecutiveTextArea(
    /// Field label
    label: String,
    /// Current value
    value: ReadSignal<String>,
    /// Value setter
    set_value: WriteSignal<String>,
    /// Placeholder text
    #[prop(default = String::new())]
    placeholder: String,
    /// Number of rows
    #[prop(default = 4)]
    rows: u32,
    /// Error message
    #[prop(optional)]
    error: Option<String>,
    /// Success state
    #[prop(default = false)]
    success: bool,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Character limit
    #[prop(optional)]
    max_length: Option<u32>,
    /// Helper text
    #[prop(optional)]
    helper: Option<String>,
    /// Additional classes
    #[prop(default = String::new())]
    class: String,
) -> impl IntoView {
    let textarea_ref = NodeRef::<html::Textarea>::new();
    let (focused, set_focused) = signal(false);
    
    let handle_focus = move |_| {
        set_focused.set(true);
        if let Some(textarea) = textarea_ref.get() {
            let _ = textarea.class_list().add_1("animate-form-field-focus");
        }
    };

    let handle_blur = move |_| {
        set_focused.set(false);
        if let Some(textarea) = textarea_ref.get() {
            let _ = textarea.class_list().remove_1("animate-form-field-focus");
        }
    };

    let handle_input = move |e: ev::Event| {
        let target = e.target().unwrap();
        let textarea = target.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
        let new_value = textarea.value();
        
        // Apply character limit
        if let Some(max) = max_length {
            if new_value.len() <= max as usize {
                set_value.set(new_value);
            }
        } else {
            set_value.set(new_value);
        }
    };

    let has_error = move || error.is_some();
    let is_valid = move || !has_error() && !value.get().is_empty() && success;
    let char_count = move || value.get().len();
    let is_near_limit = move || {
        if let Some(max) = max_length {
            char_count() as f32 / max as f32 > 0.8
        } else {
            false
        }
    };

    let textarea_class = move || {
        if has_error() {
            "textarea textarea-bordered textarea-error resize-none transition-all duration-quick"
        } else if is_valid() {
            "textarea textarea-bordered textarea-success resize-none transition-all duration-quick"
        } else if focused.get() {
            "textarea textarea-bordered textarea-primary resize-none transition-all duration-quick"
        } else {
            "textarea textarea-bordered resize-none transition-all duration-quick hover:border-primary/50"
        }
    };

    let label_class = move || {
        if has_error() {
            "label-text text-error font-medium transition-colors duration-quick"
        } else if focused.get() || !value.get().is_empty() {
            "label-text text-primary font-medium transition-colors duration-quick"
        } else {
            "label-text transition-colors duration-quick"
        }
    };

    view! {
        <div class=format!("form-control {}", class)>
            <label class="label">
                <span class=label_class>
                    {label}
                    {if required { 
                        view! { <span class="text-error ml-1 animate-professional-pulse">"*"</span> } 
                    } else { 
                        view! {} 
                    }}
                </span>
                {max_length.map(|max| view! {
                    <span class=move || {
                        if is_near_limit() {
                            "label-text-alt text-warning animate-professional-pulse"
                        } else {
                            "label-text-alt text-base-content/60"
                        }
                    }>
                        {move || format!("{}/{}", char_count(), max)}
                    </span>
                })}
            </label>
            
            <textarea 
                node_ref=textarea_ref
                class=textarea_class
                placeholder=placeholder
                rows=rows as i32
                value=value
                on:input=handle_input
                on:focus=handle_focus
                on:blur=handle_blur
                required=required
                maxlength=max_length.map(|m| m.to_string())
            ></textarea>
            
            // Helper/Error text
            <label class="label">
                {move || {
                    if let Some(err) = error.clone() {
                        view! {
                            <span class="label-text-alt text-error animate-validation-error">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                {err}
                            </span>
                        }.into_any()
                    } else if is_valid() {
                        view! {
                            <span class="label-text-alt text-success animate-validation-success">
                                <svg class="w-4 h-4 inline mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                </svg>
                                "Perfect!"
                            </span>
                        }.into_any()
                    } else if let Some(help) = helper {
                        view! {
                            <span class="label-text-alt text-base-content/60">{help}</span>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}
            </label>
        </div>
    }
}