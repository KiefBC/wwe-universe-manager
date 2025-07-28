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
        <div class="max-w-2xl mx-auto">
            <div class="card bg-base-200 border border-base-300">
                <div class="card-body">
                    <div class="text-center mb-8">
                        <h2 class="text-3xl font-bold text-base-content mb-2">
                            "Create New Show"
                        </h2>
                        <p class="text-base-content/70">
                            "Add a new wrestling show to your universe"
                        </p>
                    </div>

                    <form class="space-y-6" on:submit=move |ev| {
                        ev.prevent_default();
                        submit_form();
                    }>
                        <div class="form-control">
                            <label class="label">
                                <span class="label-text">"Show Name"</span>
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
                                <span class="label-text">"Description"</span>
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

                        <Show
                            when=move || !submit_message.get().is_empty()
                            fallback=|| view! {}
                        >
                            <div class={move || {
                                let is_error = submit_message.get().contains("Error");
                                if is_error {
                                    "alert alert-error"
                                } else {
                                    "alert alert-success"
                                }
                            }}>
                                {move || submit_message.get()}
                            </div>
                        </Show>

                        <div class="card-actions justify-end pt-4">
                            <button
                                type="button"
                                class="btn btn-ghost gap-2"
                                on:click=move |_| {
                                    console::log_1(&"Back button clicked!!!".into());
                                    set_current_page.set("home".to_string());
                                }
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                                </svg>
                                "Back"
                        </button>
                            <button
                                type="submit"
                                class="btn btn-primary gap-2"
                                disabled=is_submitting
                            >
                                <Show
                                    when=move || is_submitting.get()
                                    fallback=|| view! {
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                        </svg>
                                        "Create Show"
                                    }
                                >
                                    <span class="loading loading-spinner loading-sm"></span>
                                    "Creating..."
                                </Show>
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
