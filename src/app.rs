use crate::components::{BookerDashboard, CeoDashboard, CreatePromotion, CreateShow, CreateTitle, CreateWrestler, PromotionDashboard, ShowRosterManagement, TitleDetailsWindow, TitlesList, WrestlerDetailsWindow, WrestlersList};
use crate::types::Promotion;
use leptos::prelude::*;
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal("ceo".to_string()); // Start with CEO dashboard
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);
    let (selected_promotion, set_selected_promotion) = signal(None::<Promotion>);
    
    // Check if this is a wrestler window based on URL hash
    let is_wrestler_window = move || {
        window()
            .and_then(|w| w.location().hash().ok())
            .map(|hash| hash.starts_with("#wrestler"))
            .unwrap_or(false)
    };

    // Check if this is a title window based on URL hash
    let is_title_window = move || {
        window()
            .and_then(|w| w.location().hash().ok())
            .map(|hash| hash.starts_with("#title"))
            .unwrap_or(false)
    };

    view! {
        <div data-theme="wwe-gm-dark" class="min-h-screen bg-base-100 text-base-content">
            <Show
                when=is_wrestler_window
                fallback=move || {
                    view! {
                        <Show
                            when=is_title_window
                            fallback=move || view! {
                    <div class="flex flex-col h-screen">
                        <Header selected_promotion set_selected_promotion set_current_page />
                        <main class="flex-1 container mx-auto px-6 py-8 overflow-auto">
                            <div class="max-w-6xl mx-auto">
                                <Show
                                    when=move || current_page.get() == "create-show"
                                    fallback=move || {
                                        view! {
                                            <Show
                                                when=move || current_page.get() == "create-wrestler"
                                                fallback=move || {
                                                    view! {
                                                        <Show
                                                            when=move || current_page.get() == "wrestlers"
                                                            fallback=move || {
                                                                view! {
                                                                    <Show
                                                                        when=move || current_page.get() == "titles"
                                                                        fallback=move || {
                                                                            view! {
                                                                                <Show
                                                                                    when=move || current_page.get() == "create-promotion"
                                                                                    fallback=move || {
                                                                                        view! {
                                                                                            <Show
                                                                                                when=move || current_page.get() == "create-title"
                                                                                                fallback=move || {
                                                                                                    view! {
                                                                                                        <Show
                                                                                                            when=move || current_page.get() == "booker"
                                                                                                            fallback=move || {
                                                                                                                view! {
                                                                                                                    <Show
                                                                                                                        when=move || current_page.get() == "show-roster"
                                                                                                                        fallback=move || {
                                                                                                                            view! {
                                                                                                                                <Show
                                                                                                                                    when=move || current_page.get() == "promotion-dashboard"
                                                                                                                                    fallback=move || view! { <CeoDashboard set_current_page set_selected_promotion /> }
                                                                                                                                >
                                                                                                                                    <PromotionDashboard set_current_page refresh_trigger selected_promotion />
                                                                                                                                </Show>
                                                                                                                            }
                                                                                                                        }
                                                                                                                    >
                                                                                                                        <ShowRosterManagement set_current_page selected_promotion />
                                                                                                                    </Show>
                                                                                                                }
                                                                                                            }
                                                                                                        >
                                                                                                            <BookerDashboard set_current_page selected_promotion />
                                                                                                        </Show>
                                                                                                    }
                                                                                                }
                                                                                            >
                                                                                                <CreateTitle set_current_page />
                                                                                            </Show>
                                                                                        }
                                                                                    }
                                                                                >
                                                                                    <CreatePromotion set_current_page />
                                                                                </Show>
                                                                            }
                                                                        }
                                                                    >
                                                                        <TitlesList set_current_page />
                                                                    </Show>
                                                                }
                                                            }
                                                        >
                                                            <WrestlersList set_current_page />
                                                        </Show>
                                                    }
                                                }
                                            >
                                                <CreateWrestler set_current_page />
                                            </Show>
                                        }
                                    }
                                >
                                    <CreateShow set_current_page set_refresh_trigger />
                                </Show>
                            </div>
                        </main>
                        <Footer />
                    </div>
                            }
                        >
                            <TitleDetailsWindow />
                        </Show>
                    }
                }
            >
                <WrestlerDetailsWindow />
            </Show>
        </div>
    }
}

#[component]
fn Header(
    selected_promotion: ReadSignal<Option<Promotion>>,
    set_selected_promotion: WriteSignal<Option<Promotion>>,
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    use crate::types::fetch_promotions;
    
    let promotions_resource = LocalResource::new(|| fetch_promotions());
    
    // Handle promotion change
    let on_promotion_change = move |event| {
        let value = event_target_value(&event);
        if let Ok(promotion_id) = value.parse::<i32>() {
            // Find the promotion by ID and set it
            if let Some(promotions_result) = promotions_resource.get() {
                if let Ok(promotions) = promotions_result.as_ref() {
                if let Some(promotion) = promotions.iter().find(|p| p.id == promotion_id) {
                    set_selected_promotion.set(Some(promotion.clone()));
                    set_current_page.set("promotion-dashboard".to_string());
                }
            }
            }
        } else if value == "ceo" {
            set_selected_promotion.set(None);
            set_current_page.set("ceo".to_string());
        }
    };
    
    view! {
        <header class="bg-base-200/80 border-b border-base-300 backdrop-blur-sm">
            <div class="container mx-auto px-6 py-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <div class="w-10 h-10 bg-gradient-to-r from-primary to-secondary rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-primary-content" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path>
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-lg font-bold text-base-content">
                                "WWE Universe Manager"
                            </h1>
                            <p class="text-base-content/70 text-xs">
                                "Wrestling Management System (WMS)"
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="flex items-center space-x-2">
                            <label class="text-sm font-medium text-base-content">"Promotion:"</label>
                            <select 
                                class="select select-bordered select-sm w-48 max-w-xs"
                                on:change=on_promotion_change
                            >
                                <option value="ceo" selected=move || selected_promotion.get().is_none()>
                                    "CEO Dashboard"
                                </option>
                                <Suspense fallback=move || view! { <option>"Loading promotions..."</option> }>
                                    {move || {
                                        if let Some(promotions_result) = promotions_resource.get() {
                                            if let Ok(promotions) = promotions_result.as_ref() {
                                                promotions.iter().map(|promotion| {
                                                    let is_selected = selected_promotion.get()
                                                        .map(|p| p.id == promotion.id)
                                                        .unwrap_or(false);
                                                    let id_str = promotion.id.to_string();
                                                    let name_str = promotion.name.clone();
                                                    
                                                    view! {
                                                        <option value=id_str selected=is_selected>
                                                            {name_str}
                                                        </option>
                                                    }.into_any()
                                                }).collect::<Vec<_>>()
                                            } else {
                                                vec![view! { <option>"Error loading promotions"</option> }.into_any()]
                                            }
                                        } else {
                                            vec![view! { <option>"Loading..."</option> }.into_any()]
                                        }
                                    }}
                                </Suspense>
                            </select>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-base-200/30 border-t border-base-300 mt-auto">
            <div class="container mx-auto px-6 py-4">
                <div class="flex items-center justify-between text-sm text-base-content/60">
                    <div class="flex items-center space-x-4">
                        <span>"© 2024 WWE Universe Manager"</span>
                        <div class="w-1 h-1 bg-base-content/40 rounded-full"></div>
                        <span>"v1.0.0"</span>
                    </div>
                    <div class="hidden sm:flex items-center space-x-4">
                        <span>"Built with Rust & Tauri"</span>
                    </div>
                </div>
            </div>
        </footer>
    }
}
