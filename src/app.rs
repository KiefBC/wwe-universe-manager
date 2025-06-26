use leptos::logging;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

/// Fetches shows from the backend via Tauri
async fn fetch_shows() -> Result<Vec<Show>, String> {
    let result_js = invoke("get_shows", JsValue::NULL).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize shows: {}", e);
        logging::error!("{}", error_msg);
        error_msg
    })
}

/// Main application component
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen h-screen flex flex-col bg-gradient-to-br from-red-900 via-black to-yellow-600 overflow-hidden">
            <div class="flex-1 flex flex-col overflow-auto">
                <div class="container mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col">
                    <Header />
                    <div class="flex-1 flex items-center justify-center">
                        <ShowSelector />
                    </div>
                    <Footer />
                </div>
            </div>
        </div>
    }
}

/// Application header with WWE styling
#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="text-center py-4 sm:py-6 lg:py-8 flex-shrink-0">
            <div class="relative">
                <h1 class="text-4xl sm:text-6xl md:text-7xl lg:text-8xl font-black text-transparent bg-clip-text bg-gradient-to-r from-yellow-400 via-red-500 to-yellow-400 drop-shadow-lg">
                    "WWE"
                </h1>
                <div class="absolute -bottom-1 sm:-bottom-2 left-1/2 transform -translate-x-1/2 w-16 sm:w-24 lg:w-32 h-0.5 sm:h-1 bg-gradient-to-r from-yellow-400 to-red-500 rounded-full"></div>
            </div>
            <h2 class="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold text-white mt-2 sm:mt-4 tracking-wider">
                "UNIVERSE MANAGER"
            </h2>
            <p class="text-yellow-300 text-sm sm:text-base lg:text-lg mt-1 sm:mt-2 font-semibold">
                "Take Control of Your Wrestling Universe"
            </p>
        </div>
    }
}

/// WWE-themed show selector component
#[component]
fn ShowSelector() -> impl IntoView {
    let shows_resource = LocalResource::new(fetch_shows);
    let (selected_show_name, set_selected_show_name) = signal(String::new());

    let options_view = move || {
        shows_resource.get().map(|result| {
            let mut options = Vec::new();

            match &*result {
                Ok(shows) => {
                    // Set first show as default if none selected and shows exist
                    if !shows.is_empty() && selected_show_name.get().is_empty() {
                        set_selected_show_name.set(shows[0].name.clone());
                    }

                    if shows.is_empty() {
                        options.push(view! {
                            <option value={String::new()}>
                                {"-- No shows available --".to_string()}
                            </option>
                        });
                    } else {
                        for show in shows {
                            let show_name = show.name.clone();

                            options.push(view! {
                                <option value={show_name.clone()}>
                                    {show_name.clone()}
                                </option>
                            });
                        }
                    }
                }
                Err(e) => {
                    options.push(view! {
                        <option value={String::new()}>
                            {format!("Error: {}", e)}
                        </option>
                    });
                }
            }

            options
        })
    };

    view! {
        <div class="w-full max-w-7xl mx-auto flex-1 flex flex-col">
            // Main content card with WWE styling
            <div class="card bg-gradient-to-b from-gray-900 to-black shadow-2xl border-2 border-yellow-500 flex-1 flex flex-col">
                <div class="card-body text-center flex-1 flex flex-col p-4 sm:p-6 lg:p-8">
                    <div class="flex items-center justify-center mb-4 sm:mb-6">
                        <div class="w-6 h-6 sm:w-8 sm:h-8 bg-yellow-500 rounded-full mr-2 sm:mr-3"></div>
                        <h2 class="card-title text-xl sm:text-2xl lg:text-3xl font-black text-white tracking-wider">
                            "SELECT YOUR SHOW"
                        </h2>
                        <div class="w-6 h-6 sm:w-8 sm:h-8 bg-red-500 rounded-full ml-2 sm:ml-3"></div>
                    </div>

                    <div class="bg-gradient-to-r from-red-600 to-yellow-500 p-1 rounded-lg mb-4 sm:mb-6 flex-1 flex flex-col">
                        <div class="bg-black rounded-md p-4 sm:p-6 flex-1 flex flex-col">
                            <Suspense fallback=move || {
                                view! {
                                    <div class="flex items-center justify-center text-white flex-1">
                                        <div class="loading loading-spinner loading-md sm:loading-lg text-yellow-500 mr-3 sm:mr-4"></div>
                                        <span class="text-base sm:text-lg lg:text-xl font-semibold">"Loading WWE Shows..."</span>
                                    </div>
                                }
                            }>
                                <div class="flex-1 flex flex-col justify-center">
                                    <select
                                        on:change:target=move |ev| {
                                            set_selected_show_name.set(ev.target().value());
                                        }
                                        class="select select-bordered select-md sm:select-lg w-full max-w-sm sm:max-w-md mx-auto bg-gray-800 border-yellow-500 text-white text-base sm:text-lg font-semibold focus:border-red-500"
                                    >
                                        {options_view}
                                    </select>

                                    <div class="mt-3 sm:mt-4 text-center">
                                        <span class="text-yellow-300 text-sm sm:text-base lg:text-lg">"Current Selection: "</span>
                                        <span class="font-black text-base sm:text-lg lg:text-xl text-white bg-gradient-to-r from-red-500 to-yellow-500 bg-clip-text text-transparent">
                                            {move || {
                                                let selection = selected_show_name.get();
                                                if selection.is_empty() {
                                                    "None Selected".to_string()
                                                } else {
                                                    selection
                                                }
                                            }}
                                        </span>
                                    </div>
                                </div>
                            </Suspense>
                        </div>
                    </div>

                    // Action buttons
                    <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 sm:gap-4 mt-auto">
                        <button class="btn btn-primary btn-sm sm:btn-md lg:btn-lg bg-gradient-to-r from-red-600 to-red-700 border-red-800 hover:from-red-700 hover:to-red-800 text-white font-bold">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 sm:h-5 sm:w-5 lg:h-6 lg:w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
                            </svg>
                            <span class="hidden sm:inline">"CREATE SHOW"</span>
                            <span class="sm:hidden">"CREATE"</span>
                        </button>
                        <button class="btn btn-secondary btn-sm sm:btn-md lg:btn-lg bg-gradient-to-r from-yellow-500 to-yellow-600 border-yellow-700 hover:from-yellow-600 hover:to-yellow-700 text-black font-bold">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 sm:h-5 sm:w-5 lg:h-6 lg:w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                            </svg>
                            <span class="hidden sm:inline">"WRESTLERS"</span>
                            <span class="sm:hidden">"ROSTER"</span>
                        </button>
                        <button class="btn btn-accent btn-sm sm:btn-md lg:btn-lg bg-gradient-to-r from-purple-600 to-purple-700 border-purple-800 hover:from-purple-700 hover:to-purple-800 text-white font-bold">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 sm:h-5 sm:w-5 lg:h-6 lg:w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            <span class="hidden sm:inline">"CHAMPIONSHIPS"</span>
                            <span class="sm:hidden">"TITLES"</span>
                        </button>
                    </div>
                </div>
            </div>

            // Stats cards - now more compact and responsive
            <div class="grid grid-cols-3 gap-2 sm:gap-4 lg:gap-6 mt-4 sm:mt-6 lg:mt-8">
                <div class="stat bg-gradient-to-br from-red-800 to-red-900 text-white border border-red-600 rounded-lg p-3 sm:p-4">
                    <div class="stat-figure text-red-300 hidden sm:block">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 sm:w-8 sm:h-8 stroke-current">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                    </div>
                    <div class="stat-title text-red-200 text-xs sm:text-sm">"Shows"</div>
                    <div class="stat-value text-yellow-400 text-lg sm:text-2xl lg:text-3xl">"0"</div>
                    <div class="stat-desc text-red-300 text-xs hidden sm:block">"Create your first show!"</div>
                </div>

                <div class="stat bg-gradient-to-br from-yellow-600 to-yellow-700 text-black border border-yellow-500 rounded-lg p-3 sm:p-4">
                    <div class="stat-figure text-yellow-800 hidden sm:block">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 sm:w-8 sm:h-8 stroke-current">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"></path>
                        </svg>
                    </div>
                    <div class="stat-title text-yellow-900 text-xs sm:text-sm">"Wrestlers"</div>
                    <div class="stat-value text-red-600 text-lg sm:text-2xl lg:text-3xl">"0"</div>
                    <div class="stat-desc text-yellow-800 text-xs hidden sm:block">"Build your roster!"</div>
                </div>

                <div class="stat bg-gradient-to-br from-purple-700 to-purple-800 text-white border border-purple-600 rounded-lg p-3 sm:p-4">
                    <div class="stat-figure text-purple-300 hidden sm:block">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-6 h-6 sm:w-8 sm:h-8 stroke-current">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 8h14M5 8a2 2 0 110-4h14a2 2 0 110 4M5 8v10a2 2 0 002 2h10a2 2 0 002-2V8m-9 4h4"></path>
                        </svg>
                    </div>
                    <div class="stat-title text-purple-200 text-xs sm:text-sm">"Titles"</div>
                    <div class="stat-value text-yellow-400 text-lg sm:text-2xl lg:text-3xl">"0"</div>
                    <div class="stat-desc text-purple-300 text-xs hidden sm:block">"Create championship titles!"</div>
                </div>
            </div>
        </div>
    }
}

/// WWE-themed application footer
#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-4 sm:mt-6 lg:mt-8 text-center flex-shrink-0">
            <div class="bg-gradient-to-r from-black via-red-900 to-black p-3 sm:p-4 lg:p-6 border-t-2 border-yellow-500">
                <div class="flex items-center justify-center mb-2 sm:mb-3 lg:mb-4">
                    <div class="w-8 sm:w-12 lg:w-16 h-0.5 sm:h-1 bg-gradient-to-r from-red-500 to-yellow-500 mr-2 sm:mr-3 lg:mr-4"></div>
                    <p class="text-white font-bold text-sm sm:text-base lg:text-lg tracking-wider">
                        "WWE UNIVERSE MANAGER"
                    </p>
                    <div class="w-8 sm:w-12 lg:w-16 h-0.5 sm:h-1 bg-gradient-to-r from-yellow-500 to-red-500 ml-2 sm:ml-3 lg:ml-4"></div>
                </div>
                <p class="text-yellow-300 text-xs sm:text-sm">
                    "The Ultimate Wrestling Management Experience"
                </p>
                <div class="flex justify-center items-center mt-2 sm:mt-3 lg:mt-4 space-x-2 sm:space-x-4">
                    <span class="text-red-400 text-xs font-semibold">"V1.0"</span>
                    <span class="text-yellow-400 text-xs">"•"</span>
                    <span class="text-yellow-300 text-xs">"Built for Champions"</span>
                </div>
            </div>
        </footer>
    }
}
