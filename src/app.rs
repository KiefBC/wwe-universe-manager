use leptos::prelude::*;
use crate::components::{CreateShow, ShowSelector};

#[component]
pub fn App() -> impl IntoView {
    let (current_page, set_current_page) = signal("home".to_string());
    let (refresh_trigger, set_refresh_trigger) = signal(0u32);

    view! {
        <div class="min-h-screen h-screen flex flex-col bg-gradient-to-br from-red-900 via-black to-yellow-600 overflow-hidden">
            <div class="flex-1 flex flex-col overflow-auto">
                <div class="container mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col">
                    <Header />
                    <div class="flex-1 flex items-center justify-center">
                        <Show
                            when=move || current_page.get() == "create-show"
                            fallback=move || view! { <ShowSelector set_current_page refresh_trigger /> }
                        >
                            <CreateShow set_current_page set_refresh_trigger />
                        </Show>
                    </div>
                    <Footer />
                </div>
            </div>
        </div>
    }
}

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
