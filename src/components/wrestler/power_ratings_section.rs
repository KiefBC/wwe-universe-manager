use crate::constants::{MIN_POWER_RATING, MAX_POWER_RATING};
use crate::services::wrestler_api::*;
use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

/// Power ratings section component for wrestler details
/// 
/// Handles display and editing of wrestler power ratings (strength, speed, agility, etc.)
/// with proper state management and error handling.
#[component]
pub fn PowerRatingsSection(
    /// The wrestler whose power ratings to display/edit
    wrestler: ReadSignal<Option<WrestlerDetails>>,
    /// Callback when wrestler is updated after power ratings change
    on_wrestler_updated: WriteSignal<Option<WrestlerDetails>>,
    /// Callback for error reporting
    on_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (editing_power_ratings, set_editing_power_ratings) = signal(false);
    
    // Temporary state for editing power ratings
    let (temp_strength, set_temp_strength) = signal(0i32);
    let (temp_speed, set_temp_speed) = signal(0i32);
    let (temp_agility, set_temp_agility) = signal(0i32);
    let (temp_stamina, set_temp_stamina) = signal(0i32);
    let (temp_charisma, set_temp_charisma) = signal(0i32);
    let (temp_technique, set_temp_technique) = signal(0i32);

    // Handler to start editing mode
    let start_editing = move |_| {
        if let Some(w) = wrestler.get() {
            // Initialize temp values with current wrestler stats
            set_temp_strength.set(w.strength.unwrap_or(0));
            set_temp_speed.set(w.speed.unwrap_or(0));
            set_temp_agility.set(w.agility.unwrap_or(0));
            set_temp_stamina.set(w.stamina.unwrap_or(0));
            set_temp_charisma.set(w.charisma.unwrap_or(0));
            set_temp_technique.set(w.technique.unwrap_or(0));
            set_editing_power_ratings.set(true);
        }
    };

    // Handler to save power ratings
    let save_power_ratings = move |_| {
        if let Some(w) = wrestler.get() {
            spawn_local(async move {
                let strength = if temp_strength.get() > 0 { Some(temp_strength.get()) } else { None };
                let speed = if temp_speed.get() > 0 { Some(temp_speed.get()) } else { None };
                let agility = if temp_agility.get() > 0 { Some(temp_agility.get()) } else { None };
                let stamina = if temp_stamina.get() > 0 { Some(temp_stamina.get()) } else { None };
                let charisma = if temp_charisma.get() > 0 { Some(temp_charisma.get()) } else { None };
                let technique = if temp_technique.get() > 0 { Some(temp_technique.get()) } else { None };
                
                match update_wrestler_power_ratings(
                    w.id,
                    strength,
                    speed,
                    agility,
                    stamina,
                    charisma,
                    technique
                ).await {
                    Ok(updated_wrestler) => {
                        on_wrestler_updated.set(Some(updated_wrestler));
                        set_editing_power_ratings.set(false);
                    }
                    Err(e) => {
                        on_error.set(Some(format!("Failed to update power ratings: {}", e)));
                    }
                }
            });
        }
    };

    // Handler to cancel editing
    let cancel_editing = move |_| {
        set_editing_power_ratings.set(false);
    };

    view! {
        <div class="card bg-base-200 border border-base-100">
            <div class="card-body">
            <Show
                when=move || {
                    if let Some(w) = wrestler.get() {
                        w.strength.is_some() || w.speed.is_some() || w.agility.is_some() || 
                        w.stamina.is_some() || w.charisma.is_some() || w.technique.is_some()
                    } else {
                        false
                    }
                }
                fallback=move || view! {
                    <div class="text-center text-base-content/60 text-sm">
                        "No power ratings available"
                    </div>
                }
            >
                <div class="flex items-center justify-between mb-4 border-b border-base-content/20 pb-2">
                    <h4 class="text-base-content font-bold text-lg">
                        "Power Ratings"
                    </h4>
                    <button
                        class="btn btn-ghost btn-sm gap-1"
                        on:click=start_editing
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        <span>"Edit"</span>
                    </button>
                </div>
                <Show 
                    when=move || !editing_power_ratings.get()
                    fallback=move || view! {
                        <div class="space-y-3">
                            <PowerBarEdit label="STRENGTH" value=temp_strength set_value=set_temp_strength />
                            <PowerBarEdit label="SPEED" value=temp_speed set_value=set_temp_speed />
                            <PowerBarEdit label="AGILITY" value=temp_agility set_value=set_temp_agility />
                            <PowerBarEdit label="STAMINA" value=temp_stamina set_value=set_temp_stamina />
                            <PowerBarEdit label="CHARISMA" value=temp_charisma set_value=set_temp_charisma />
                            <PowerBarEdit label="TECHNIQUE" value=temp_technique set_value=set_temp_technique />
                        </div>
                        <div class="flex space-x-2 mt-4">
                            <button
                                class="btn btn-success btn-sm flex-1"
                                on:click=save_power_ratings
                            >
                                "Save"
                            </button>
                            <button
                                class="btn btn-ghost btn-sm flex-1"
                                on:click=cancel_editing
                            >
                                "Cancel"
                            </button>
                        </div>
                    }
                >
                    <div class="space-y-3">
                        {move || wrestler.get().and_then(|w| w.strength).map(|val| view! { <PowerBar label="STRENGTH" value=val color="bg-error" /> })}
                        {move || wrestler.get().and_then(|w| w.speed).map(|val| view! { <PowerBar label="SPEED" value=val color="bg-info" /> })}
                        {move || wrestler.get().and_then(|w| w.agility).map(|val| view! { <PowerBar label="AGILITY" value=val color="bg-success" /> })}
                        {move || wrestler.get().and_then(|w| w.stamina).map(|val| view! { <PowerBar label="STAMINA" value=val color="bg-secondary" /> })}
                        {move || wrestler.get().and_then(|w| w.charisma).map(|val| view! { <PowerBar label="CHARISMA" value=val color="bg-primary" /> })}
                        {move || wrestler.get().and_then(|w| w.technique).map(|val| view! { <PowerBar label="TECHNIQUE" value=val color="bg-accent" /> })}
                    </div>
                </Show>
            </Show>
            </div>
        </div>
    }
}

/// Display component for individual power rating bars
#[component]
fn PowerBar(
    #[prop(into)] label: String,
    #[prop(into)] value: i32,
    #[prop(into)] color: String,
) -> impl IntoView {
    let percentage = (value as f32 / MAX_POWER_RATING as f32 * 100.0) as i32;
    
    view! {
        <div class="flex items-center space-x-3">
            <span class="text-base-content/80 font-medium text-sm w-20 text-right">{label}</span>
            <div class="flex-1 bg-base-300 rounded-full h-3 border border-base-content/20">
                <div 
                    class=format!("h-full rounded-full {} flex items-center justify-end pr-1", color)
                    style=format!("width: {}%", percentage)
                >
                    <span class="text-xs font-medium text-white">{value}</span>
                </div>
            </div>
        </div>
    }
}

/// Edit component for individual power rating bars
#[component]
fn PowerBarEdit(
    #[prop(into)] label: String,
    value: ReadSignal<i32>,
    set_value: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <div class="flex items-center space-x-3">
            <span class="text-base-content/80 font-medium text-sm w-20 text-right">{label}</span>
            <div class="flex-1 flex items-center space-x-2">
                <input
                    type="range"
                    min={MIN_POWER_RATING.to_string()}
                    max={MAX_POWER_RATING.to_string()}
                    class="range range-secondary flex-1"
                    prop:value=move || value.get().to_string()
                    on:input:target=move |ev| {
                        if let Ok(val) = ev.target().value().parse::<i32>() {
                            set_value.set(val.clamp(MIN_POWER_RATING, MAX_POWER_RATING));
                        }
                    }
                />
                <div class="w-12 text-center">
                    <input
                        type="number"
                        min={MIN_POWER_RATING.to_string()}
                        max={MAX_POWER_RATING.to_string()}
                        class="input input-bordered input-sm w-full text-center"
                        prop:value=move || value.get().to_string()
                        on:input:target=move |ev| {
                            if let Ok(val) = ev.target().value().parse::<i32>() {
                                set_value.set(val.clamp(MIN_POWER_RATING, MAX_POWER_RATING));
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}