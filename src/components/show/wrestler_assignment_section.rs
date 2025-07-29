use crate::types::Wrestler;
use leptos::prelude::*;

/// Available wrestler assignment component for show roster management
/// 
/// Displays available wrestlers that can be assigned to a show
#[component]
pub fn WrestlerAssignmentSection(
    /// Available wrestlers that can be assigned to the show
    available_wrestlers: ReadSignal<Vec<Wrestler>>,
    /// Loading state
    loading: ReadSignal<bool>,
    /// Callback when a wrestler should be assigned to the roster
    on_assign_wrestler: WriteSignal<Option<i32>>,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200 shadow-xl">
            <div class="card-body">
                <h3 class="card-title text-xl mb-4">
                    "Available Wrestlers"
                    <div class="badge badge-secondary">{move || available_wrestlers.get().len()}</div>
                </h3>
                <div class="space-y-2 max-h-96 overflow-y-auto">
                    <For
                        each=move || available_wrestlers.get()
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
                                        class="btn btn-sm btn-success"
                                        on:click=move |_| on_assign_wrestler.set(Some(wrestler_id))
                                        disabled=move || loading.get()
                                    >
                                        "Assign"
                                    </button>
                                </div>
                            }
                        }
                    />
                    <Show when=move || available_wrestlers.get().is_empty() && !loading.get()>
                        <div class="text-center py-8 text-base-content/50">
                            <p>"All wrestlers are assigned to this show"</p>
                            <p class="text-sm">"Great job building your roster!"</p>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}