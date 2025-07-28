use crate::components::{CreateShow, CreateTitle, CreateWrestler, Dashboard, TitleDetailsWindow, TitlesList, WrestlerDetailsWindow, WrestlersList};
use leptos::prelude::*;
use web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal("home".to_string());
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
        <div data-theme="modern-dark" class="min-h-screen bg-slate-950 text-slate-100">
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
                                                                                    when=move || current_page.get() == "create-title"
                                                                                    fallback=move || view! { <Dashboard set_current_page refresh_trigger /> }
                                                                                >
                                                                                    <CreateTitle set_current_page />
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
fn Header() -> impl IntoView {
    view! {
        <header class="bg-slate-900/50 border-b border-slate-800 backdrop-blur-sm">
            <div class="container mx-auto px-6 py-6">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <div class="w-10 h-10 bg-gradient-to-r from-indigo-500 to-purple-600 rounded-lg flex items-center justify-center">
                            <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-2xl font-bold text-slate-100">
                                "Universe Manager"
                            </h1>
                            <p class="text-slate-400 text-sm">
                                "Wrestling management system"
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="hidden sm:flex items-center space-x-2 text-sm text-slate-400">
                            <div class="w-2 h-2 bg-green-500 rounded-full"></div>
                            <span>"Online"</span>
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
        <footer class="bg-slate-900/30 border-t border-slate-800 mt-auto">
            <div class="container mx-auto px-6 py-4">
                <div class="flex items-center justify-between text-sm text-slate-400">
                    <div class="flex items-center space-x-4">
                        <span>"© 2024 Universe Manager"</span>
                        <div class="w-1 h-1 bg-slate-600 rounded-full"></div>
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
