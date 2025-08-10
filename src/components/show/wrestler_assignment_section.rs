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
        <div class="card bg-gradient-to-br from-secondary/5 to-secondary/2 border border-secondary/20 shadow-lg">
            <div class="card-body p-4 sm:p-6">
                <div class="flex items-center gap-3 mb-6">
                    <div class="w-12 h-12 bg-secondary/20 rounded-xl flex items-center justify-center">
                        <svg class="w-7 h-7 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="text-lg sm:text-xl font-bold text-base-content mb-1">
                            "Available Talent"
                        </h3>
                        <div class="flex items-center gap-2">
                            <div class="badge badge-secondary gap-1">
                                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                </svg>
                                {move || format!("{} Available", available_wrestlers.get().len())}
                            </div>
                            <div class="badge badge-ghost badge-sm">"TALENT POOL"</div>
                        </div>
                    </div>
                </div>
                
                <div class="space-y-3 max-h-[400px] overflow-y-auto pr-2">
                    <For
                        each=move || available_wrestlers.get()
                        key=|wrestler| wrestler.id
                        children=move |wrestler: Wrestler| {
                            let wrestler_id = wrestler.id;
                            let win_rate = if wrestler.wins + wrestler.losses > 0 {
                                (wrestler.wins as f64 / (wrestler.wins + wrestler.losses) as f64) * 100.0
                            } else {
                                0.0
                            };
                            
                            view! {
                                <div class="group flex items-center justify-between p-4 bg-base-100 border border-base-300/50 rounded-lg hover:border-secondary/30 hover:shadow-md transition-all duration-200">
                                    <div class="flex items-center space-x-4 flex-1 min-w-0">
                                        <div class="avatar placeholder">
                                            <div class="bg-gradient-to-br from-secondary via-accent to-primary text-base-100 rounded-xl w-12 h-12 flex items-center justify-center">
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
                                        class="btn btn-sm btn-secondary gap-2 group-hover:btn-outline transition-all duration-200 min-h-[36px]"
                                        on:click=move |_| on_assign_wrestler.set(Some(wrestler_id))
                                        disabled=move || loading.get()
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                        </svg>
                                        <span class="hidden sm:inline">"Assign"</span>
                                    </button>
                                </div>
                            }
                        }
                    />
                    <Show when=move || available_wrestlers.get().is_empty() && !loading.get()>
                        <div class="text-center py-8 sm:py-12">
                            <div class="w-16 h-16 bg-secondary/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <svg class="w-8 h-8 text-secondary/40" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                            <h4 class="font-bold text-base-content mb-2">"Roster Complete"</h4>
                            <p class="text-base-content/60 text-sm mb-1">"All available wrestlers are assigned"</p>
                            <p class="text-base-content/40 text-xs">"Outstanding roster management! Your show is fully staffed."</p>
                        </div>
                    </Show>
                </div>
            </div>
        </div>
    }
}