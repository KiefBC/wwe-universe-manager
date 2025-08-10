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
        <div class="card bg-gradient-to-br from-primary/5 to-primary/2 border border-primary/20 shadow-lg">
            <div class="card-body p-4 sm:p-6">
                <div class="flex items-center gap-3 mb-6">
                    <div class="w-12 h-12 bg-primary/20 rounded-xl flex items-center justify-center">
                        <svg class="w-7 h-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="text-lg sm:text-xl font-bold text-base-content mb-1">
                            "Current Roster"
                        </h3>
                        <div class="flex items-center gap-2">
                            <div class="badge badge-primary gap-1">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                </svg>
                                {move || format!("{} Active", current_roster.get().len())}
                            </div>
                            <div class="badge badge-ghost badge-sm">"ROSTER"</div>
                        </div>
                    </div>
                </div>
                
                <div class="space-y-3 max-h-[400px] overflow-y-auto pr-2">
                    <For
                        each=move || current_roster.get()
                        key=|wrestler| wrestler.id
                        children=move |wrestler: Wrestler| {
                            let wrestler_id = wrestler.id;
                            let win_rate = if wrestler.wins + wrestler.losses > 0 {
                                (wrestler.wins as f64 / (wrestler.wins + wrestler.losses) as f64) * 100.0
                            } else {
                                0.0
                            };
                            
                            view! {
                                <div class="group flex items-center justify-between p-4 bg-base-100 border border-base-300/50 rounded-lg hover:border-primary/30 hover:shadow-md transition-all duration-200">
                                    <div class="flex items-center space-x-4 flex-1 min-w-0">
                                        <div class="avatar placeholder">
                                            <div class="bg-gradient-to-br from-primary via-accent to-secondary text-base-100 rounded-xl w-12 h-12 flex items-center justify-center">
                                                <span class="text-sm font-bold">{wrestler.name.chars().next().unwrap_or('?').to_string()}</span>
                                            </div>
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <p class="font-semibold text-base-content truncate">{wrestler.name}</p>
                                            <div class="flex items-center gap-2 mt-1">
                                                <div class="badge badge-sm badge-outline">{wrestler.gender}</div>
                                                <div class="text-xs text-base-content/60">
                                                    {format!("{}-{}", wrestler.wins, wrestler.losses)}
                                                </div>
                                                <div class=move || format!("text-xs font-medium {}", 
                                                    if win_rate >= 70.0 { "text-success" } 
                                                    else if win_rate >= 50.0 { "text-primary" } 
                                                    else { "text-base-content/60" }
                                                )>
                                                    {format!("{:.0}%", win_rate)}
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <button
                                        class="btn btn-sm btn-error gap-2 group-hover:btn-outline transition-all duration-200 min-h-[36px]"
                                        on:click=move |_| on_remove_wrestler.set(Some(wrestler_id))
                                        disabled=move || loading.get()
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                        </svg>
                                        <span class="hidden sm:inline">"Remove"</span>
                                    </button>
                                </div>
                            }
                        }
                    />
                    <Show when=move || current_roster.get().is_empty() && !loading.get()>
                        <div class="text-center py-8 sm:py-12">
                            <div class="w-16 h-16 bg-primary/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <svg class="w-8 h-8 text-primary/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                </svg>
                            </div>
                            <h4 class="font-bold text-base-content mb-2">"Empty Roster"</h4>
                            <p class="text-base-content/60 text-sm mb-1">"No wrestlers assigned to this show"</p>
                            <p class="text-base-content/40 text-xs">"Add talent from the available pool to build your roster"</p>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}