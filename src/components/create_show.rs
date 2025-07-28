use crate::types::{create_show, ShowData};
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::console;

/// WWE-themed create show form component
#[component]
pub fn CreateShow(
    set_current_page: WriteSignal<String>,
    set_refresh_trigger: WriteSignal<u32>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_message, set_submit_message) = signal(String::new());

    let submit_form = move || {
        console::log_1(&"Form submitted".into());

        if name.get().trim().is_empty() || description.get().trim().is_empty() {
            console::log_1(&"Validation failed - empty fields".into());
            set_submit_message.set("Please fill in all fields.".to_string());
            return;
        }

        console::log_1(&"Form validation passed, submitting...".into());
        set_is_submitting.set(true);
        set_submit_message.set(String::new());

        let show_data = ShowData {
            name: name.get().trim().to_string(),
            description: description.get().trim().to_string(),
        };

        console::log_1(
            &format!(
                "Show data: name={}, description={}",
                show_data.name, show_data.description
            )
            .into(),
        );

        spawn_local(async move {
            console::log_1(&"Calling create_show...".into());
            match create_show(show_data).await {
                Ok(show) => {
                    console::log_1(&format!("Show created successfully: {:?}", show).into());
                    set_submit_message.set("Show created successfully!".to_string());
                    // Trigger refresh of shows
                    set_refresh_trigger.update(|n| *n += 1);
                    // Navigate back to home after a short delay
                    gloo_timers::future::TimeoutFuture::new(1500).await;
                    set_current_page.set("home".to_string());
                }
                Err(e) => {
                    console::log_1(&format!("Error creating show: {}", e).into());
                    set_submit_message.set(format!("Error creating show: {}", e));
                    set_is_submitting.set(false);
                }
            }
        });
    };

    view! {
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn btn-ghost gap-2"
                        on:click=move |_| set_current_page.set("home".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Dashboard"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "Create New Show"
                </h1>
                <p class="text-base-content/70">
                    "Add a new wrestling show to your universe"
                </p>
            </div>

            <div class="card bg-base-200 border border-base-300">
                <div class="card-body">

                <form on:submit=move |ev| {
                    ev.prevent_default();
                    submit_form();
                }>
                    <div class="space-y-6">
                        <h3 class="text-xl font-semibold text-base-content mb-4">"Show Information"</h3>
                        
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Show Name" <span class="text-error">"*"</span></span>
                            </label>
                            <input
                                type="text"
                                placeholder="Enter show name (e.g., Monday Night Raw)"
                                class="input input-bordered w-full"
                                on:input:target=move |ev| {
                                    set_name.set(ev.target().value());
                                }
                                prop:value=name
                            />
                        </div>

                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Description" <span class="text-error">"*"</span></span>
                            </label>
                            <textarea
                                placeholder="Enter show description..."
                                rows="4"
                                class="textarea textarea-bordered w-full resize-none"
                                on:input:target=move |ev| {
                                    set_description.set(ev.target().value());
                                }
                                prop:value=description
                            ></textarea>
                        </div>
                    </div>

                    // Submit Message
                    <Show when=move || !submit_message.get().is_empty()>
                        <div class={move || {
                            let is_error = submit_message.get().contains("Error");
                            if is_error {
                                "alert alert-error mt-6"
                            } else {
                                "alert alert-success mt-6"
                            }
                        }}>
                            <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={move || {
                                    let is_error = submit_message.get().contains("Error");
                                    if is_error {
                                        "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    } else {
                                        "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    }
                                }} />
                            </svg>
                            <div>
                                <h3 class="font-bold">{move || {
                                    let is_error = submit_message.get().contains("Error");
                                    if is_error { "Error Creating Show" } else { "Show Created Successfully!" }
                                }}</h3>
                                <div class="text-xs">{move || submit_message.get()}</div>
                            </div>
                        </div>
                    </Show>

                    // Submit Button
                    <div class="mt-8 flex justify-end space-x-4">
                        <button
                            type="button"
                            class="btn btn-ghost"
                            on:click=move |_| set_current_page.set("home".to_string())
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="btn btn-primary"
                            disabled=move || is_submitting.get()
                        >
                            <Show when=move || is_submitting.get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            <span>{move || if is_submitting.get() { "Creating..." } else { "Create Show" }}</span>
                        </button>
                    </div>
                </form>
                </div>
            </div>
        </div>
    }
}
