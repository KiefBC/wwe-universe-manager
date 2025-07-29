use leptos::prelude::*;

/// Title management actions component
/// 
/// Handles title deletion with confirmation dialog
#[component]
pub fn TitleManagementActions(
    /// Title name for confirmation display
    title_name: Signal<String>,
    /// Whether to show delete confirmation dialog
    show_delete_confirmation: ReadSignal<bool>,
    /// Set show delete confirmation
    _set_show_delete_confirmation: WriteSignal<bool>,
    /// Whether deletion is in progress
    deleting: ReadSignal<bool>,
    /// Delete error message
    delete_error: ReadSignal<Option<String>>,
    /// Callback when delete is confirmed
    on_confirm_delete: WriteSignal<bool>,
    /// Callback when delete is cancelled
    on_cancel_delete: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || show_delete_confirmation.get()>
            <div class="space-y-2">
                <Show when=move || delete_error.get().is_some()>
                    <div class="alert alert-error">
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-5 w-5" fill="none" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                        <span class="text-xs">{move || delete_error.get().unwrap_or_default()}</span>
                    </div>
                </Show>
                
                <div class="bg-error/20 border border-error/30 rounded-lg p-3">
                    <div class="flex items-center gap-2 mb-2">
                        <svg class="w-4 h-4 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
                        </svg>
                        <h5 class="text-error font-bold text-sm">"Confirm Deletion"</h5>
                    </div>
                    <p class="text-error/90 text-xs mb-2">
                        "Are you sure you want to delete "
                        <strong>{move || title_name.get()}</strong>
                        "? This action cannot be undone."
                    </p>
                    <p class="text-error/70 text-xs">
                        "This will also remove all title history and holder records."
                    </p>
                </div>
                <div class="flex gap-1">
                    <button
                        class="btn btn-error btn-xs flex-1 gap-1"
                        disabled=move || deleting.get()
                        on:click=move |_| on_confirm_delete.set(true)
                    >
                        <Show when=move || deleting.get()>
                            <span class="loading loading-spinner loading-xs"></span>
                        </Show>
                        <Show when=move || !deleting.get()>
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                            </svg>
                        </Show>
                        {move || if deleting.get() { "Deleting..." } else { "Delete Forever" }}
                    </button>
                    <button
                        class="btn btn-ghost btn-xs flex-1"
                        disabled=move || deleting.get()
                        on:click=move |_| on_cancel_delete.set(true)
                    >
                        "Cancel"
                    </button>
                </div>
            </div>
        </Show>
    }
}