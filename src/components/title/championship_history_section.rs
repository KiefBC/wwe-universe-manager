use leptos::prelude::*;

/// Championship history section component
/// 
/// Placeholder for championship history tracking functionality
#[component]
pub fn ChampionshipHistorySection() -> impl IntoView {
    view! {
        <div class="card bg-base-200 border border-base-300">
            <div class="card-body">
                <h3 class="card-title text-xl mb-4">
                    "Championship History"
                </h3>
                <div class="text-center py-8">
                    <div class="text-base-content/60 mb-2">
                        "Championship history tracking coming soon"
                    </div>
                    <div class="text-sm text-base-content/50">
                        "This will show all previous champions and reign details"
                    </div>
                </div>
            </div>
        </div>
    }
}