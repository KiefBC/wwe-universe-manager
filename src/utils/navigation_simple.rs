// Simplified navigation without For component to avoid parsing errors
use leptos::prelude::*;

pub fn create_step_navigation(
    current_step: ReadSignal<usize>,
    steps: Vec<(String, String)>,
    on_step_click: Option<Callback<usize>>,
) -> impl IntoView {
    view! {
        <div class="w-full">
            <ul class="steps w-full">
                {steps.into_iter().enumerate().map(|(step_index, (step_label, step_description))| {
                    let is_current = move || current_step.get() == step_index;
                    let is_completed = move || current_step.get() > step_index;
                    let is_clickable = on_step_click.is_some();

                    view! {
                        <li class={move || {
                            let mut classes = vec!["step"];
                            if is_current() {
                                classes.push("step-primary");
                            } else if is_completed() {
                                classes.push("step-success");
                            }
                            classes.join(" ")
                        }}>
                            {if is_clickable {
                                view! {
                                    <button 
                                        class="btn btn-ghost btn-sm"
                                        on:click=move |_| {
                                            if let Some(handler) = on_step_click.as_ref() {
                                                handler.run(step_index);
                                            }
                                        }
                                    >
                                        <div class="text-center">
                                            <div class="font-semibold">{step_label.clone()}</div>
                                            <div class="text-xs text-base-content/70">{step_description.clone()}</div>
                                        </div>
                                    </button>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="text-center">
                                        <div class="font-semibold">{step_label.clone()}</div>
                                        <div class="text-xs text-base-content/70">{step_description.clone()}</div>
                                    </div>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}