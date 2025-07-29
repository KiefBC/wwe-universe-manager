use crate::types::{Title, Wrestler};
use leptos::prelude::*;

/// Form state for champion change operations
#[derive(Clone, Debug, PartialEq)]
pub struct ChampionChangeFormState {
    pub selected_wrestler_id: Option<i32>,
    pub event_name: String,
    pub event_location: String,
    pub change_method: String,
}

impl Default for ChampionChangeFormState {
    fn default() -> Self {
        Self {
            selected_wrestler_id: None,
            event_name: String::new(),
            event_location: String::new(),
            change_method: "won".to_string(),
        }
    }
}

/// Change champion form component
/// 
/// Allows changing the current champion of a title with event details
#[component]
pub fn ChangeChampionForm(
    /// The title being managed
    title: Signal<Title>,
    /// Available wrestlers for assignment (filtered by gender)
    filtered_wrestlers: Signal<Vec<Wrestler>>,
    /// Form state (bidirectional)
    form_state: RwSignal<ChampionChangeFormState>,
    /// Whether update is in progress
    updating: ReadSignal<bool>,
    /// Success message
    update_success: ReadSignal<Option<String>>,
    /// Callback when form is submitted with form data
    on_submit: impl Fn(ChampionChangeFormState) + 'static,
) -> impl IntoView {
    view! {
        <div class="card bg-base-200 border border-base-300">
            <div class="card-body">
                <h3 class="card-title text-xl mb-6">
                    "Change Champion"
                </h3>
            
                <Show when=move || update_success.get().is_some()>
                    <div class="alert alert-success mb-6">
                        <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                        <span>{move || update_success.get().unwrap_or_default()}</span>
                    </div>
                </Show>

            <form on:submit=move |ev| {
                ev.prevent_default();
                on_submit(form_state.get());
            }>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"New Champion" <span class="text-error">"*"</span></span>
                        </label>
                        <select
                            class="select select-bordered w-full"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                form_state.update(|state| {
                                    if value.is_empty() || value == "none" {
                                        state.selected_wrestler_id = None;
                                    } else if let Ok(id) = value.parse::<i32>() {
                                        state.selected_wrestler_id = Some(id);
                                    }
                                });
                            }
                        >
                            <option value="none">"Select a wrestler..."</option>
                            <For
                                each=move || filtered_wrestlers.get()
                                key=|wrestler| wrestler.id
                                children=move |wrestler| {
                                    let wrestler_id_val = wrestler.id;
                                    view! {
                                        <option value={wrestler_id_val.to_string()}>
                                            {wrestler.name.clone()}
                                        </option>
                                    }
                                }
                            />
                        </select>
                        <div class="label">
                            <span class="label-text-alt text-base-content/60">
                                {move || format!("Showing {} division wrestlers", title.get().gender)}
                            </span>
                        </div>
                        <div class="label">
                            <span class="label-text-alt text-warning">"TODO: Filter by selected show in the future"</span>
                        </div>
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Change Method"</span>
                        </label>
                        <select
                            class="select select-bordered w-full"
                            prop:value=move || form_state.get().change_method
                            on:change=move |ev| form_state.update(|state| state.change_method = event_target_value(&ev))
                        >
                            <option value="won">"Won"</option>
                            <option value="awarded">"Awarded"</option>
                            <option value="stripped">"Previous champion stripped"</option>
                            <option value="vacated">"Previous champion vacated"</option>
                        </select>
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Event/Show Name"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered w-full"
                            placeholder="e.g., WrestleMania, Monday Night RAW"
                            prop:value=move || form_state.get().event_name
                            on:input=move |ev| form_state.update(|state| state.event_name = event_target_value(&ev))
                        />
                    </div>

                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Event Location"</span>
                        </label>
                        <input
                            type="text"
                            class="input input-bordered w-full"
                            placeholder="e.g., Madison Square Garden"
                            prop:value=move || form_state.get().event_location
                            on:input=move |ev| form_state.update(|state| state.event_location = event_target_value(&ev))
                        />
                    </div>
                </div>

                <div class="card-actions justify-end mt-6">
                    <button
                        type="submit"
                        class="btn btn-accent gap-2"
                        disabled=move || updating.get() || form_state.get().selected_wrestler_id.is_none()
                    >
                        <Show when=move || updating.get()>
                            <span class="loading loading-spinner loading-sm"></span>
                        </Show>
                        {move || if updating.get() { "Updating..." } else { "Change Champion" }}
                    </button>
                </div>
            </form>
            </div>
        </div>
    }
}