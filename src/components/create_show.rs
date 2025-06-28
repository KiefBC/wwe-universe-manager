use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::console;
use crate::types::{ShowData, create_show};

/// WWE-themed create show form component
#[component]
pub fn CreateShow(set_current_page: WriteSignal<String>, set_refresh_trigger: WriteSignal<u32>) -> impl IntoView {
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
        
        console::log_1(&format!("Show data: name={}, description={}", show_data.name, show_data.description).into());

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
        <div class="w-full max-w-2xl mx-auto flex-1 flex flex-col">
            <div class="card bg-gradient-to-b from-gray-900 to-black shadow-2xl border-2 border-yellow-500 flex-1 flex flex-col">
                <div class="card-body text-center flex-1 flex flex-col p-4 sm:p-6 lg:p-8">
                    <div class="flex items-center justify-center mb-4 sm:mb-6">
                        <div class="w-6 h-6 sm:w-8 sm:h-8 bg-yellow-500 rounded-full mr-2 sm:mr-3"></div>
                        <h2 class="card-title text-xl sm:text-2xl lg:text-3xl font-black text-white tracking-wider">
                            "CREATE NEW SHOW"
                        </h2>
                        <div class="w-6 h-6 sm:w-8 sm:h-8 bg-red-500 rounded-full ml-2 sm:ml-3"></div>
                    </div>

                    <div class="bg-gradient-to-r from-red-600 to-yellow-500 p-1 rounded-lg mb-4 sm:mb-6 flex-1 flex flex-col">
                        <div class="bg-black rounded-md p-4 sm:p-6 flex-1 flex flex-col">
                            <form class="space-y-4 flex-1 flex flex-col">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text text-yellow-300 font-bold">"Show Name"</span>
                                    </label>
                                    <input
                                        type="text"
                                        placeholder="Enter show name (e.g., Monday Night Raw)"
                                        class="input input-bordered w-full bg-gray-800 border-yellow-500 text-white placeholder-gray-400 focus:border-red-500"
                                        on:input:target=move |ev| {
                                            set_name.set(ev.target().value());
                                        }
                                        prop:value=name
                                    />
                                </div>

                                <div class="form-control flex-1">
                                    <label class="label">
                                        <span class="label-text text-yellow-300 font-bold">"Description"</span>
                                    </label>
                                    <textarea
                                        placeholder="Enter show description..."
                                        class="textarea textarea-bordered h-32 w-full bg-gray-800 border-yellow-500 text-white placeholder-gray-400 focus:border-red-500 resize-none"
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
                                            "text-center font-semibold text-red-400"
                                        } else {
                                            "text-center font-semibold text-green-400"
                                        }
                                    }}>
                                        {move || submit_message.get()}
                                    </div>
                                </Show>
                            </form>
                        </div>
                    </div>

                    <div class="flex gap-4 mt-auto">
                        <button 
                            type="button"
                            class="btn btn-secondary btn-sm sm:btn-md lg:btn-lg bg-gradient-to-r from-gray-600 to-gray-700 border-gray-800 hover:from-gray-700 hover:to-gray-800 text-white font-bold flex-1"
                            on:click=move |_| {
                                console::log_1(&"Back button clicked!!!".into());
                                set_current_page.set("home".to_string());
                            }
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 sm:h-5 sm:w-5 lg:h-6 lg:w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                            </svg>
                            "BACK"
                        </button>
                        <button
                            type="button"
                            class="btn btn-primary btn-sm sm:btn-md lg:btn-lg bg-gradient-to-r from-red-600 to-red-700 border-red-800 hover:from-red-700 hover:to-red-800 text-white font-bold flex-1"
                            class:loading=is_submitting
                            disabled=is_submitting
                            on:click=move |_| {
                                console::log_1(&"Create Show button clicked".into());
                                submit_form();
                            }
                        >
                            <Show
                                when=move || is_submitting.get()
                                fallback=|| view! {
                                    <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 sm:h-5 sm:w-5 lg:h-6 lg:w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                                    </svg>
                                    "CREATE SHOW"
                                }
                            >
                                <span class="loading loading-spinner loading-sm"></span>
                                "CREATING..."
                            </Show>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}