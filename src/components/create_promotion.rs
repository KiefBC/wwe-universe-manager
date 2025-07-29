use crate::types::{create_promotion, PromotionData};
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::console;

/// Create promotion form component for CEO Dashboard
#[component]
pub fn CreatePromotion(
    set_current_page: WriteSignal<String>,
    set_refresh_trigger: WriteSignal<u32>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (is_submitting, set_is_submitting) = signal(false);
    let (submit_message, set_submit_message) = signal(String::new());

    let submit_form = move || {
        console::log_1(&"Promotion form submitted".into());

        if name.get().trim().is_empty() || description.get().trim().is_empty() {
            console::log_1(&"Validation failed - empty fields".into());
            set_submit_message.set("Please fill in all fields.".to_string());
            return;
        }

        console::log_1(&"Form validation passed, submitting...".into());
        set_is_submitting.set(true);
        set_submit_message.set(String::new());

        let promotion_data = PromotionData {
            name: name.get().trim().to_string(),
            description: description.get().trim().to_string(),
        };

        spawn_local(async move {
            console::log_1(&"Calling create_promotion async function...".into());
            
            match create_promotion(promotion_data).await {
                Ok(promotion) => {
                    console::log_1(&format!("✅ Promotion created successfully: {:?}", promotion).into());
                    set_submit_message.set(format!("Promotion '{}' created successfully!", promotion.name));
                    set_is_submitting.set(false);
                    
                    // Trigger refresh of CEO dashboard resources
                    set_refresh_trigger.update(|x| *x += 1);
                    
                    // Navigate back to CEO dashboard after a short delay
                    set_timeout(
                        move || {
                            set_current_page.set("ceo".to_string());
                        },
                        std::time::Duration::from_secs(2),
                    );
                },
                Err(e) => {
                    console::log_1(&format!("❌ Failed to create promotion: {}", e).into());
                    set_submit_message.set(format!("Failed to create promotion: {}", e));
                    set_is_submitting.set(false);
                }
            }
        });
    };

    let back_to_ceo = move |_| {
        set_current_page.set("ceo".to_string());
    };

    view! {
        <div class="max-w-2xl mx-auto">
            <div class="card bg-base-200 shadow-xl">
                <div class="card-body">
                    <div class="flex items-center justify-between mb-6">
                        <h2 class="text-2xl font-bold text-base-content">
                            "Create New Promotion"
                        </h2>
                        <button
                            class="btn btn-ghost btn-sm"
                            on:click=back_to_ceo
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>

                    <form on:submit=move |e| {
                        e.prevent_default();
                        submit_form();
                    }>
                        <div class="space-y-6">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text font-semibold">"Promotion Name"</span>
                                </label>
                                <input
                                    type="text"
                                    placeholder="e.g., WWE, AEW, NJPW, Impact Wrestling"
                                    class="input input-bordered w-full"
                                    prop:value=move || name.get()
                                    on:input=move |e| set_name.set(event_target_value(&e))
                                    prop:disabled=move || is_submitting.get()
                                />
                                <label class="label">
                                    <span class="label-text-alt text-base-content/60">
                                        "The official name of the wrestling promotion"
                                    </span>
                                </label>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text font-semibold">"Description"</span>
                                </label>
                                <textarea
                                    placeholder="Describe the promotion's style, history, or unique characteristics..."
                                    class="textarea textarea-bordered h-24 w-full"
                                    prop:value=move || description.get()
                                    on:input=move |e| set_description.set(event_target_value(&e))
                                    prop:disabled=move || is_submitting.get()
                                ></textarea>
                                <label class="label">
                                    <span class="label-text-alt text-base-content/60">
                                        "Brief description of the promotion"
                                    </span>
                                </label>
                            </div>

                            <div class="flex justify-between items-center pt-4">
                                <button
                                    type="button"
                                    class="btn btn-ghost"
                                    on:click=back_to_ceo
                                    prop:disabled=move || is_submitting.get()
                                >
                                    "← Back to CEO Dashboard"
                                </button>
                                
                                <button
                                    type="submit"
                                    class="btn btn-primary"
                                    prop:disabled=move || is_submitting.get()
                                >
                                    <Show when=move || is_submitting.get()>
                                        <span class="loading loading-spinner loading-sm"></span>
                                    </Show>
                                    {move || if is_submitting.get() { "Creating..." } else { "Create Promotion" }}
                                </button>
                            </div>

                            <Show when=move || !submit_message.get().is_empty()>
                                <div class={move || {
                                    if submit_message.get().contains("successfully") {
                                        "alert alert-success"
                                    } else {
                                        "alert alert-error"
                                    }
                                }}>
                                    <span>{move || submit_message.get()}</span>
                                </div>
                            </Show>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}