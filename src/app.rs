use crate::components::{AnalyticsDashboard, BookerDashboard, CreateShow, CreateTitle, CreateWrestler, ProfessionalDashboard, ShowRosterManagement, SystemMonitor, TitleDetailsWindow, TitlesList, WrestlerDetailsWindow, WrestlersList};
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
                        {move || {
                            let page = current_page.get();
                            // Full-width layout for professional pages
                            if page == "promotion-dashboard" || page == "wrestlers" || page == "analytics" || page == "system" {
                                view! {
                                    <main class="flex-1 overflow-auto">
                                        {move || {
                                            match current_page.get().as_str() {
                                                "promotion-dashboard" => view! { <ProfessionalDashboard set_current_page refresh_trigger /> }.into_any(),
                                                "wrestlers" => view! { <WrestlersList set_current_page /> }.into_any(),
                                                "analytics" => view! { <AnalyticsDashboard set_current_page /> }.into_any(),
                                                "system" => view! { <SystemMonitor set_current_page /> }.into_any(),
                                                _ => view! { <div>"Page not found"</div> }.into_any(),
                                            }
                                        }}
                                    </main>
                                }.into_any()
                            } else {
                                // Constrained layout for form pages
                                view! {
                                    <main class="flex-1 container mx-auto px-6 py-8 overflow-auto">
                                        <div class="max-w-6xl mx-auto">
                                            {move || {
                                                match current_page.get().as_str() {
                                                    "show-roster" => view! { <ShowRosterManagement set_current_page /> }.into_any(),
                                                    "create-show" => view! { <CreateShow set_current_page set_refresh_trigger /> }.into_any(),
                                                    "create-wrestler" => view! { <CreateWrestler set_current_page /> }.into_any(),
                                                    "titles" => view! { <TitlesList set_current_page /> }.into_any(),
                                                    "create-title" => view! { <CreateTitle set_current_page /> }.into_any(),
                                                    "booker" => view! { <BookerDashboard set_current_page /> }.into_any(),
                                                    _ => view! { <div>"Page not found"</div> }.into_any(),
                                                }
                                            }}
                                        </div>
                                    </main>
                                }.into_any()
                            }
                        }}
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
            <div class="container mx-auto px-6 py-1">
                <div class="h-2"></div>
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
                        <span>"© 2025 Wrestling Management System (WMS)"</span>
                        <div class="w-1 h-1 bg-base-content/40 rounded-full"></div>
                        <span>"v0.3"</span>
                    </div>
                    <div class="hidden sm:flex items-center space-x-4">
                        <span>"Built with Rust & Tauri"</span>
                    </div>
                </div>
            </div>
        </footer>
    }
}
