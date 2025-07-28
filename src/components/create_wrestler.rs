use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedWrestlerData {
    pub name: String,
    pub gender: String,
    pub real_name: Option<String>,
    pub nickname: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub debut_year: Option<i32>,
    pub promotion: Option<String>,
    pub strength: Option<i32>,
    pub speed: Option<i32>,
    pub agility: Option<i32>,
    pub stamina: Option<i32>,
    pub charisma: Option<i32>,
    pub technique: Option<i32>,
    pub biography: Option<String>,
    pub trivia: Option<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn create_user_wrestler(wrestler_data: EnhancedWrestlerData) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({
        "wrestlerData": wrestler_data
    }))
    .map_err(|e| e.to_string())?;

    let result = invoke("create_user_wrestler", args).await;
    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

#[component]
pub fn CreateWrestler(
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (gender, set_gender) = signal("Male".to_string());
    let (real_name, set_real_name) = signal(String::new());
    let (nickname, set_nickname) = signal(String::new());
    let (height, set_height) = signal(String::new());
    let (weight, set_weight) = signal(String::new());
    let (debut_year, set_debut_year) = signal(String::new());
    let (promotion, set_promotion) = signal(String::new());
    let (strength, set_strength) = signal(5);
    let (speed, set_speed) = signal(5);
    let (agility, set_agility) = signal(5);
    let (stamina, set_stamina) = signal(5);
    let (charisma, set_charisma) = signal(5);
    let (technique, set_technique) = signal(5);
    let (biography, set_biography) = signal(String::new());
    let (trivia, set_trivia) = signal(String::new());
    
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (success, set_success) = signal(false);

    let submit_wrestler = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        if name.get().trim().is_empty() {
            set_error.set(Some("Wrestler name is required".to_string()));
            return;
        }

        let wrestler_data = EnhancedWrestlerData {
            name: name.get().trim().to_string(),
            gender: gender.get(),
            real_name: if real_name.get().trim().is_empty() { None } else { Some(real_name.get().trim().to_string()) },
            nickname: if nickname.get().trim().is_empty() { None } else { Some(nickname.get().trim().to_string()) },
            height: if height.get().trim().is_empty() { None } else { Some(height.get().trim().to_string()) },
            weight: if weight.get().trim().is_empty() { None } else { Some(weight.get().trim().to_string()) },
            debut_year: if debut_year.get().trim().is_empty() { None } else { debut_year.get().trim().parse().ok() },
            promotion: if promotion.get().trim().is_empty() { None } else { Some(promotion.get().trim().to_string()) },
            strength: Some(strength.get()),
            speed: Some(speed.get()),
            agility: Some(agility.get()),
            stamina: Some(stamina.get()),
            charisma: Some(charisma.get()),
            technique: Some(technique.get()),
            biography: if biography.get().trim().is_empty() { None } else { Some(biography.get().trim().to_string()) },
            trivia: if trivia.get().trim().is_empty() { None } else { Some(trivia.get().trim().to_string()) },
        };

        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match create_user_wrestler(wrestler_data).await {
                Ok(_) => {
                    set_success.set(true);
                    // Navigate back to wrestlers list after a brief delay
                    set_timeout(
                        move || {
                            set_current_page.set("wrestlers".to_string());
                        },
                        std::time::Duration::from_millis(1500),
                    );
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="container mx-auto p-6 bg-base-100 min-h-screen">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <button
                        class="btn btn-ghost gap-2"
                        on:click=move |_| set_current_page.set("wrestlers".to_string())
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                        </svg>
                        "Back to Wrestlers"
                    </button>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-2">
                    "Create New Wrestler"
                </h1>
                <p class="text-base-content/70">
                    "Add a custom wrestler to your universe"
                </p>
            </div>

            <Show when=move || success.get()>
                <div class="alert alert-success mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Wrestler Created Successfully!"</h3>
                        <div class="text-xs">"Redirecting to wrestlers list..."</div>
                    </div>
                </div>
            </Show>

            <Show when=move || error.get().is_some()>
                <div class="alert alert-error mb-6">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <div>
                        <h3 class="font-bold">"Error Creating Wrestler"</h3>
                        <div class="text-xs">{move || error.get().unwrap_or_default()}</div>
                    </div>
                </div>
            </Show>

            <div class="card bg-base-200 border border-base-300">
                <div class="card-body">
                <form on:submit=submit_wrestler>
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                        // Basic Information
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Basic Information"</h3>
                            
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Wrestler Name" <span class="text-error">"*"</span></span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., The Rock"
                                    prop:value=name
                                    on:input=move |ev| set_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Gender" <span class="text-error">"*"</span></span>
                                </label>
                                <select
                                    class="select select-bordered w-full"
                                    prop:value=gender
                                    on:change=move |ev| set_gender.set(event_target_value(&ev))
                                >
                                    <option value="Male">"Male"</option>
                                    <option value="Female">"Female"</option>
                                </select>
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Real Name"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., Dwayne Johnson"
                                    prop:value=real_name
                                    on:input=move |ev| set_real_name.set(event_target_value(&ev))
                                />
                            </div>

                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Nickname"</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered w-full"
                                    placeholder="e.g., The People's Champion"
                                    prop:value=nickname
                                    on:input=move |ev| set_nickname.set(event_target_value(&ev))
                                />
                            </div>
                        </div>

                        // Physical Stats & Career
                        <div class="space-y-6">
                            <h3 class="text-xl font-semibold text-base-content mb-4">"Physical Stats & Career"</h3>
                            
                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Height"</span>
                                    </label>
                                    <input
                                        type="text"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 6'5\""
                                        prop:value=height
                                        on:input=move |ev| set_height.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Weight"</span>
                                    </label>
                                    <input
                                        type="text"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 260 lbs"
                                        prop:value=weight
                                        on:input=move |ev| set_weight.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Debut Year"</span>
                                    </label>
                                    <input
                                        type="number"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., 1996"
                                        prop:value=debut_year
                                        on:input=move |ev| set_debut_year.set(event_target_value(&ev))
                                    />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Promotion"</span>
                                    </label>
                                    <input
                                        type="text"
                                        class="input input-bordered w-full"
                                        placeholder="e.g., WWE"
                                        prop:value=promotion
                                        on:input=move |ev| set_promotion.set(event_target_value(&ev))
                                    />
                                </div>
                            </div>
                        </div>
                    </div>

                    // Power Ratings
                    <div class="mt-8">
                        <h3 class="text-xl font-semibold text-base-content mb-6">"Power Ratings"</h3>
                        <div class="grid grid-cols-2 lg:grid-cols-3 gap-6">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Strength: " <span class="text-secondary">{strength}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=strength
                                    on:input=move |ev| set_strength.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Speed: " <span class="text-secondary">{speed}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=speed
                                    on:input=move |ev| set_speed.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Agility: " <span class="text-secondary">{agility}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=agility
                                    on:input=move |ev| set_agility.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Stamina: " <span class="text-secondary">{stamina}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=stamina
                                    on:input=move |ev| set_stamina.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Charisma: " <span class="text-secondary">{charisma}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=charisma
                                    on:input=move |ev| set_charisma.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Technique: " <span class="text-secondary">{technique}</span></span>
                                </label>
                                <input
                                    type="range"
                                    min="1" max="10"
                                    class="range range-secondary"
                                    prop:value=technique
                                    on:input=move |ev| set_technique.set(event_target_value(&ev).parse().unwrap_or(5))
                                />
                            </div>
                        </div>
                    </div>

                    // Biography & Trivia
                    <div class="mt-8">
                        <h3 class="text-xl font-semibold text-base-content mb-6">"Biography & Trivia"</h3>
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Biography"</span>
                                </label>
                                <textarea
                                    rows="6"
                                    class="textarea textarea-bordered w-full"
                                    placeholder="Tell the wrestler's story..."
                                    prop:value=biography
                                    on:input=move |ev| set_biography.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                            <div class="form-control">
                                <label class="label">
                                    <span class="label-text">"Trivia"</span>
                                </label>
                                <textarea
                                    rows="6"
                                    class="textarea textarea-bordered w-full"
                                    placeholder="Fun facts and trivia..."
                                    prop:value=trivia
                                    on:input=move |ev| set_trivia.set(event_target_value(&ev))
                                ></textarea>
                            </div>
                        </div>
                    </div>

                    // Submit Button
                    <div class="mt-8 flex justify-end space-x-4">
                        <button
                            type="button"
                            class="btn btn-ghost"
                            on:click=move |_| set_current_page.set("wrestlers".to_string())
                        >
                            "Cancel"
                        </button>
                        <button
                            type="submit"
                            class="btn btn-secondary"
                            disabled=move || loading.get()
                        >
                            <Show when=move || loading.get()>
                                <span class="loading loading-spinner loading-sm"></span>
                            </Show>
                            <span>{move || if loading.get() { "Creating..." } else { "Create Wrestler" }}</span>
                        </button>
                    </div>
                </form>
                </div>
            </div>
        </div>
    }
}