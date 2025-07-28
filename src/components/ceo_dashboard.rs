use crate::types::{fetch_promotions, fetch_wrestlers, Promotion, Wrestler};
use leptos::prelude::*;

/// CEO Dashboard component with rich promotion cards
/// 
/// Displays large promotion cards showing:
/// - Promotion name and branding
/// - #1 Male wrestler (by win/loss record)
/// - #1 Female wrestler (by win/loss record) 
/// - Current "week" number (placeholder)
#[component]
pub fn CeoDashboard(
    set_current_page: WriteSignal<String>,
    set_selected_promotion: WriteSignal<Option<Promotion>>,
) -> impl IntoView {
    let promotions_resource = LocalResource::new(|| fetch_promotions());
    let wrestlers_resource = LocalResource::new(|| fetch_wrestlers());

    // Helper function to get top wrestler by win rate for a given gender
    let get_top_wrestler = move |wrestlers: &[Wrestler], gender: &str| -> Option<Wrestler> {
        wrestlers
            .iter()
            .filter(|w| w.gender.to_lowercase() == gender.to_lowercase())
            .max_by_key(|w| {
                let total_matches = w.wins + w.losses;
                if total_matches == 0 {
                    0 // Avoid division by zero, treat as 0% win rate
                } else {
                    // Calculate win percentage * 1000 for integer comparison
                    (w.wins * 1000) / total_matches
                }
            })
            .cloned()
    };

    // Navigate to promotion dashboard
    let navigate_to_promotion = move |promotion: Promotion| {
        set_selected_promotion.set(Some(promotion));
        set_current_page.set("promotion-dashboard".to_string());
    };

    view! {
        <div class="space-y-8">
            <div class="text-center mb-8">
                <h2 class="text-4xl font-bold text-base-content mb-4">
                    "WWE Universe Manager"
                </h2>
                
                <div class="mb-4">
                    <button 
                        class="btn btn-primary btn-lg gap-3"
                        on:click=move |_| set_current_page.set("create-promotion".to_string())
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
                        </svg>
                        "Create New Promotion"
                    </button>
                </div>
                
                <p class="text-base-content/70 text-lg">
                    "Select your promotion to manage"
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <Suspense fallback=move || view! { 
                    <div class="flex justify-center items-center h-48">
                        <span class="loading loading-spinner loading-lg"></span>
                    </div>
                }>
                    {move || {
                        let promotions = promotions_resource.get();
                        let wrestlers = wrestlers_resource.get();
                        
                        match (promotions, wrestlers) {
                            (Some(promotions_result), Some(wrestlers_result)) => {
                                match (promotions_result.as_ref(), wrestlers_result.as_ref()) {
                                    (Ok(promos), Ok(wrestlers_list)) => {
                                promos.into_iter().enumerate().map(|(index, promotion)| {
                                    let promo_clone = promotion.clone();
                                    let wrestlers_clone = wrestlers_list.clone();
                                    
                                    // Get top wrestlers for this promotion
                                    let top_male = get_top_wrestler(&wrestlers_clone, "male");
                                    let top_female = get_top_wrestler(&wrestlers_clone, "female");
                                    
                                    // Placeholder week number (will be replaced with real system later)
                                    let week_number = (index + 1) * 12;
                                    
                                    view! {
                                        <div class="card bg-base-200 shadow-xl hover:shadow-2xl transition-all duration-300 cursor-pointer transform hover:-translate-y-1"
                                             on:click=move |_| navigate_to_promotion(promo_clone.clone())>
                                            <div class="card-body">
                                                <h3 class="card-title text-2xl mb-4 text-primary">
                                                    {promotion.name.clone()}
                                                </h3>
                                                
                                                <div class="space-y-4">
                                                    <div class="flex justify-between items-center">
                                                        <span class="text-base-content/70 font-medium">"Current Week:"</span>
                                                        <span class="text-primary font-bold text-lg">{week_number}</span>
                                                    </div>
                                                    
                                                    <div class="divider my-2"></div>
                                                    
                                                    <div class="space-y-3">
                                                        <div>
                                                            <h4 class="text-sm font-semibold text-base-content/80 mb-1">"#1 Male Wrestler"</h4>
                                                            {if let Some(wrestler) = &top_male {
                                                                let win_rate = if wrestler.wins + wrestler.losses == 0 {
                                                                    0.0
                                                                } else {
                                                                    (wrestler.wins as f64 / (wrestler.wins + wrestler.losses) as f64) * 100.0
                                                                };
                                                                view! {
                                                                    <div class="flex justify-between items-center">
                                                                        <span class="font-medium">{wrestler.name.clone()}</span>
                                                                        <span class="text-sm text-success">
                                                                            {format!("{}-{} ({:.1}%)", wrestler.wins, wrestler.losses, win_rate)}
                                                                        </span>
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <div class="flex justify-between items-center">
                                                                        <span class="font-medium text-base-content/50 italic">"No male wrestlers"</span>
                                                                        <span class="text-sm text-base-content/50">""</span>
                                                                    </div>
                                                                }.into_any()
                                                            }}
                                                        </div>
                                                        
                                                        <div>
                                                            <h4 class="text-sm font-semibold text-base-content/80 mb-1">"#1 Female Wrestler"</h4>
                                                            {if let Some(wrestler) = &top_female {
                                                                let win_rate = if wrestler.wins + wrestler.losses == 0 {
                                                                    0.0
                                                                } else {
                                                                    (wrestler.wins as f64 / (wrestler.wins + wrestler.losses) as f64) * 100.0
                                                                };
                                                                view! {
                                                                    <div class="flex justify-between items-center">
                                                                        <span class="font-medium">{wrestler.name.clone()}</span>
                                                                        <span class="text-sm text-success">
                                                                            {format!("{}-{} ({:.1}%)", wrestler.wins, wrestler.losses, win_rate)}
                                                                        </span>
                                                                    </div>
                                                                }.into_any()
                                                            } else {
                                                                view! {
                                                                    <div class="flex justify-between items-center">
                                                                        <span class="font-medium text-base-content/50 italic">"No female wrestlers"</span>
                                                                        <span class="text-sm text-base-content/50">""</span>
                                                                    </div>
                                                                }.into_any()
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                                
                                                <div class="card-actions justify-end mt-6">
                                                    <button class="btn btn-primary">
                                                        "Manage"
                                                        <svg class="w-4 h-4 ml-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                                        </svg>
                                                    </button>
                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                }).collect::<Vec<_>>()
                                    },
                                    (Err(e), _) | (_, Err(e)) => {
                                        vec![view! {
                                            <div class="alert alert-error">
                                                <span>"Error loading data: " {e.clone()}</span>
                                            </div>
                                        }.into_any()]
                                    }
                                }
                            },
                            _ => {
                                vec![view! {
                                    <div class="flex justify-center items-center h-48">
                                        <span class="loading loading-spinner loading-lg"></span>
                                    </div>
                                }.into_any()]
                            }
                        }
                    }}
                </Suspense>
            </div>
        </div>
    }
}