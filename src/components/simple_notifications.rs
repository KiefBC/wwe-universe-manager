use leptos::prelude::*;

/// Simple Professional Toast Notification
#[component]
pub fn SimpleToast(
    /// Message to display
    message: String,
    /// Toast type
    #[prop(default = "info".to_string())]
    toast_type: String,
    /// Show state
    show: ReadSignal<bool>,
    /// Dismiss handler
    #[prop(optional)]
    on_dismiss: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let (alert_class, icon_svg) = match toast_type.as_str() {
        "success" => ("alert-success", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
            </svg>
        }),
        "error" => ("alert-error", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        }),
        "warning" => ("alert-warning", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.732 15c-.77.833.192 2.5 1.732 2.5z"></path>
            </svg>
        }),
        _ => ("alert-info", view! {
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
        }),
    };

    view! {
        <div class={move || if show.get() { 
            format!("toast toast-top toast-end animate-toast-slide-in z-50") 
        } else { 
            "hidden".to_string() 
        }}>
            <div class={format!("alert {} shadow-lg", alert_class)}>
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