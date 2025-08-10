use leptos::prelude::*;

/// Champion information section component
/// 
/// Displays current champion information with days held
#[component]
pub fn ChampionInfoSection(
    /// Current holders display text
    holders_display: Signal<String>,
    /// Days held display text  
    days_display: Signal<String>,
    /// Whether there are current holders
    has_current_holders: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200">
            <div class="card-body">
                <h3 class="card-title text-xl mb-4">
                    "Current Champion"
                </h3>
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-2xl font-bold text-base-content mb-1">
                            {move || holders_display.get()}
                        </div>
                        <div class="text-base-content/60">
                            {move || {
                                if has_current_holders.get() {
                                    format!("Held for {}", days_display.get())
                                } else {
                                    "Title is currently vacant".to_string()
                                }
                            }}
                        </div>
                    </div>
                    <Show when=move || has_current_holders.get()>
                        <div class="text-right">
                            <div class="text-lg font-semibold text-accent">
                                {move || days_display.get()}
                            </div>
                            <div class="text-sm text-base-content/60">
                                "Championship Reign"
                            </div>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}