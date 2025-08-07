use crate::components::{BookerDashboard, CreateShow, CreateTitle, CreateWrestler, PromotionDashboard, ShowRosterManagement, TitleDetailsWindow, TitlesList, WrestlerDetailsWindow, WrestlersList};
use leptos::prelude::*;
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal("promotion-dashboard".to_string()); // Start with GM dashboard
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);
    
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
                        <Header />
                        <main class="flex-1 container mx-auto px-6 py-8 overflow-auto">
                            <div class="max-w-6xl mx-auto">
                                {move || {
                                    match current_page.get().as_str() {
                                        "promotion-dashboard" => view! { <PromotionDashboard set_current_page refresh_trigger /> }.into_any(),
                                        "show-roster" => view! { <ShowRosterManagement set_current_page /> }.into_any(),
                                        "create-show" => view! { <CreateShow set_current_page set_refresh_trigger /> }.into_any(),
                                        "create-wrestler" => view! { <CreateWrestler set_current_page /> }.into_any(),
                                        "wrestlers" => view! { <WrestlersList set_current_page /> }.into_any(),
                                        "titles" => view! { <TitlesList set_current_page /> }.into_any(),
                                        "create-title" => view! { <CreateTitle set_current_page /> }.into_any(),
                                        "booker" => view! { <BookerDashboard set_current_page /> }.into_any(),
                                        _ => view! { <PromotionDashboard set_current_page refresh_trigger /> }.into_any(),
                                    }
                                }}
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
fn Header() -> impl IntoView {
    view! {
        <header class="bg-base-200/80 border-b border-base-300 backdrop-blur-sm">
            <div class="container mx-auto px-6 py-3">
                <div class="flex items-center justify-center">
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
