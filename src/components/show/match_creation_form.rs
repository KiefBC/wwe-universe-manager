use leptos::prelude::*;

/// Match creation form component for the booker dashboard
/// 
/// Allows creation of new matches with various types and stipulations
#[component]
pub fn MatchCreationForm(
    /// Whether the form should be shown
    show_form: ReadSignal<bool>,
    /// Callback to hide/show the form
    set_show_form: WriteSignal<bool>,
    /// Match name input
    match_name: ReadSignal<String>,
    /// Match name setter
    set_match_name: WriteSignal<String>,
    /// Match type input
    match_type: ReadSignal<String>,
    /// Match type setter
    set_match_type: WriteSignal<String>,
    /// Match stipulation input
    match_stipulation: ReadSignal<String>,
    /// Match stipulation setter
    set_match_stipulation: WriteSignal<String>,
    /// Loading state
    loading: ReadSignal<bool>,
    /// Callback when form is submitted
    on_create_match: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || show_form.get()>
            <div class="bg-base-100 p-4 rounded-lg mb-6">
                <h4 class="text-lg font-semibold mb-4">"Create New Match"</h4>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Match Name (Optional)"</span>
                        </label>
                        <input
                            type="text"
                            placeholder="e.g., Main Event"
                            class="input input-bordered"
                            prop:value=move || match_name.get()
                            on:input=move |ev| set_match_name.set(event_target_value(&ev))
                        />
                    </div>
                    
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Match Type"</span>
                        </label>
                        <select 
                            class="select select-bordered"
                            prop:value=move || match_type.get()
                            on:change=move |ev| set_match_type.set(event_target_value(&ev))
                        >
                            <option value="Singles">"Singles"</option>
                            <option value="Tag Team">"Tag Team"</option>
                            <option value="Triple Threat">"Triple Threat"</option>
                            <option value="Fatal 4-Way">"Fatal 4-Way"</option>
                            <option value="Battle Royal">"Battle Royal"</option>
                            <option value="Ladder Match">"Ladder Match"</option>
                            <option value="Steel Cage">"Steel Cage"</option>
                        </select>
                    </div>
                    
                    <div class="form-control">
                        <label class="label">
                            <span class="label-text">"Stipulation"</span>
                        </label>
                        <select 
                            class="select select-bordered"
                            prop:value=move || match_stipulation.get()
                            on:change=move |ev| set_match_stipulation.set(event_target_value(&ev))
                        >
                            <option value="Standard">"Standard"</option>
                            <option value="No DQ">"No Disqualification"</option>
                            <option value="No Holds Barred">"No Holds Barred"</option>
                            <option value="Last Man Standing">"Last Man Standing"</option>
                            <option value="Submission Match">"Submission Match"</option>
                            <option value="Hardcore">"Hardcore"</option>
                        </select>
                    </div>
                </div>
                
                <div class="flex justify-end space-x-2 mt-4">
                    <button
                        class="btn btn-ghost"
                        on:click=move |_| set_show_form.set(false)
                    >
                        "Cancel"
                    </button>
                    <button
                        class="btn btn-primary"
                        on:click=move |_| on_create_match.set(true)
                        disabled=move || loading.get()
                    >
                        "Create Match"
                    </button>
                </div>
            </div>
        </Show>
    }
}