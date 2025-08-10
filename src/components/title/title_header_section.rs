use crate::types::Title;
use leptos::prelude::*;

/// Title header section component displaying title information and prestige styling
/// 
/// Shows title name, division, type, gender, prestige tier with appropriate styling
#[component]
pub fn TitleHeaderSection(
    /// The title to display
    title: Signal<Title>,
    /// Whether to show delete functionality
    show_delete_button: Signal<bool>,
    /// Callback when delete is requested
    on_delete_request: WriteSignal<bool>,
    /// Delete error message
    delete_error: ReadSignal<Option<String>>,
    /// Whether deletion is in progress
    deleting: ReadSignal<bool>,
) -> impl IntoView {
    let get_prestige_info = move |tier: i32| {
        match tier {
            1 => ("World Championship", "text-warning", "border-warning", "bg-warning/20"),
            2 => ("Secondary Championship", "text-base-content/70", "border-base-300", "bg-base-200"),
            3 => ("Tag Team Championship", "text-accent", "border-accent", "bg-accent/20"),
            _ => ("Specialty Championship", "text-secondary", "border-secondary", "bg-secondary/20"),
        }
    };

    view! {
        <div class={move || {
            let title_data = title.get();
            let (_, _, border_color, bg_color) = get_prestige_info(title_data.prestige_tier);
            format!("card border {} {}", border_color, bg_color)
        }}>
            <div class="card-body">
                <div class="flex items-start justify-between mb-6">
                    <div>
                        <h1 class="text-4xl font-bold text-base-content mb-2">
                            {move || title.get().name.clone()}
                        </h1>
                        <div class="flex items-center space-x-4 text-sm">
                            <span class={move || {
                                let title_data = title.get();
                                let (_, prestige_color, _, _) = get_prestige_info(title_data.prestige_tier);
                                format!("font-semibold {}", prestige_color)
                            }}>
                                {move || {
                                    let title_data = title.get();
                                    let (prestige_name, _, _, _) = get_prestige_info(title_data.prestige_tier);
                                    prestige_name
                                }}
                            </span>
                            <span class="text-base-content/40">"•"</span>
                            <span class="text-base-content/80">
                                {move || title.get().division.clone()}
                            </span>
                            <span class="text-base-content/40">"•"</span>
                            <span class="text-base-content/80">
                                {move || title.get().title_type.clone()}
                            </span>
                            <span class="text-base-content/40">"•"</span>
                            <span class="text-base-content/80">
                                {move || title.get().gender.clone()}
                            </span>
                        </div>
                    </div>
                    <div class="text-right space-y-2">
                        <div>
                            <div class="text-2xl font-bold text-base-content">
                                "Tier " {move || title.get().prestige_tier}
                            </div>
                            <div class="text-sm text-base-content/60">
                                "Prestige Level"
                            </div>
                        </div>
                        
                        // Delete button (only for user-created titles)
                        <Show when=move || show_delete_button.get() && title.get().is_user_created.unwrap_or(false)>
                            <div class="flex flex-col gap-2">
                                <Show when=move || delete_error.get().is_some()>
                                    <div class="alert alert-error">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-5 w-5" fill="none" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                                        </svg>
                                        <span class="text-xs">{move || delete_error.get().unwrap_or_default()}</span>
                                    </div>
                                </Show>
                                
                                <button
                                    class="btn btn-error btn-sm gap-1"
                                    disabled=move || deleting.get()
                                    on:click=move |_| on_delete_request.set(true)
                                >
                                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                                    </svg>
                                    "Delete Title"
                                </button>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}