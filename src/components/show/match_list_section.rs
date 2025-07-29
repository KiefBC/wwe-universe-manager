use crate::types::{Match, Wrestler};
use leptos::prelude::*;

/// Match list display component for the booker dashboard
/// 
/// Displays existing matches for a show with wrestler assignment capabilities
#[component]
pub fn MatchListSection(
    /// List of matches for the selected show
    matches: ReadSignal<Vec<Match>>,
    /// List of wrestlers available for the show
    show_wrestlers: ReadSignal<Vec<Wrestler>>,
    /// Loading state
    loading: ReadSignal<bool>,
    /// Callback when a wrestler should be added to a match (match_id, wrestler_id)
    on_add_wrestler_to_match: WriteSignal<Option<(i32, i32)>>,
) -> impl IntoView {
    view! {
        <div class="space-y-4">
            <For
                each=move || matches.get()
                key=|match_item| match_item.id
                children=move |match_item: Match| {
                    let match_id = match_item.id;
                    let match_name_display = match_item.match_name.clone()
                        .unwrap_or_else(|| format!("{} Match", match_item.match_type));
                    
                    view! {
                        <div class="card bg-base-100 border border-base-300">
                            <div class="card-body">
                                <div class="flex items-center justify-between">
                                    <div>
                                        <h4 class="text-lg font-semibold">{match_name_display}</h4>
                                        <p class="text-base-content/70">
                                            {match_item.match_type.clone()}
                                            {match_item.match_stipulation.as_ref().map(|s| format!(" - {}", s)).unwrap_or_default()}
                                        </p>
                                        <Show when=move || match_item.winner_id.is_some()>
                                            <div class="badge badge-success gap-2 mt-2">
                                                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                                                </svg>
                                                "Match Complete"
                                            </div>
                                        </Show>
                                    </div>
                                    <div class="flex space-x-2">
                                        <div class="dropdown dropdown-end">
                                            <div tabindex="0" role="button" class="btn btn-sm btn-outline">
                                                "Add Wrestler"
                                            </div>
                                            <ul tabindex="0" class="dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow max-h-48 overflow-y-auto">
                                                <For
                                                    each=move || show_wrestlers.get()
                                                    key=|wrestler| wrestler.id
                                                    children=move |wrestler: Wrestler| {
                                                        let wrestler_id = wrestler.id;
                                                        view! {
                                                            <li>
                                                                <a on:click=move |_| on_add_wrestler_to_match.set(Some((match_id, wrestler_id)))>
                                                                    {wrestler.name}
                                                                </a>
                                                            </li>
                                                        }
                                                    }
                                                />
                                            </ul>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }
            />
            
            <Show when=move || matches.get().is_empty() && !loading.get()>
                <div class="text-center py-8 text-base-content/50">
                    <svg class="w-16 h-16 mx-auto mb-4 text-base-content/30" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 0V3a2 2 0 00-2-2V1a2 2 0 00-2 2v2H9z" />
                    </svg>
                    <p class="text-lg mb-2">"No matches scheduled"</p>
                    <p class="text-sm">"Create your first match to start building the card"</p>
                </div>
            </Show>
        </div>
    }
}