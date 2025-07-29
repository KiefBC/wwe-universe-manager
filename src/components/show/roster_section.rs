use crate::types::Wrestler;
use leptos::prelude::*;

/// Current roster display component for show roster management
/// 
/// Displays the current wrestlers assigned to a show with the ability to remove them
#[component]
pub fn RosterSection(
    /// Current roster of wrestlers for the selected show
    current_roster: ReadSignal<Vec<Wrestler>>,
    /// Loading state
    loading: ReadSignal<bool>,
    /// Callback when a wrestler should be removed from the roster
    on_remove_wrestler: WriteSignal<Option<i32>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body">
                <h3 class="card-title text-xl mb-4">
                    "Current Roster"
                    <div class="badge badge-primary">{move || current_roster.get().len()}</div>
                </h3>
                <div class="space-y-2 max-h-96 overflow-y-auto">
                    <For
                        each=move || current_roster.get()
                        key=|wrestler| wrestler.id
                        children=move |wrestler: Wrestler| {
                            let wrestler_id = wrestler.id;
                            view! {
                                <div class="flex items-center justify-between p-3 bg-base-100 rounded-lg">
                                    <div class="flex items-center space-x-3">
                                        <div class="avatar placeholder">
                                            <div class="bg-neutral text-neutral-content rounded-full w-10">
                                                <span class="text-sm">{wrestler.name.chars().next().unwrap_or('?').to_string()}</span>
                                            </div>
                                        </div>
                                        <div>
                                            <p class="font-medium">{wrestler.name}</p>
                                            <p class="text-sm text-base-content/70">
                                                {format!("{} • {}-{}", wrestler.gender, wrestler.wins, wrestler.losses)}
                                            </p>
                                        </div>
                                    </div>
                                    <button
                                        class="btn btn-sm btn-error"
                                        on:click=move |_| on_remove_wrestler.set(Some(wrestler_id))
                                        disabled=move || loading.get()
                                    >
                                        "Remove"
                                    </button>
                                </div>
                            }
                        }
                    />
                    <Show when=move || current_roster.get().is_empty() && !loading.get()>
                        <div class="text-center py-8 text-base-content/50">
                            <p>"No wrestlers assigned to this show"</p>
                            <p class="text-sm">"Add wrestlers from the available pool"</p>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}