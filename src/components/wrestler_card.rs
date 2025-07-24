use leptos::prelude::*;

#[component]
pub fn WrestlerCardModal(
    #[prop(into)] show_modal: ReadSignal<bool>,
    #[prop(into)] set_show_modal: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <Show when=move || show_modal.get()>
            <div class="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 p-4">
                <div class="relative max-w-4xl w-full max-h-[90vh] overflow-auto">
                    <div class="card-modern rounded-xl relative overflow-hidden">
                        // Close button
                        <button
                            class="absolute top-4 right-4 z-10 bg-slate-700 hover:bg-slate-600 text-slate-300 hover:text-white w-8 h-8 rounded-lg flex items-center justify-center font-bold border border-slate-600"
                            on:click=move |_| set_show_modal.set(false)
                        >
                            "×"
                        </button>
                        
                        // Header with sleek styling
                        <div class="bg-slate-800/80 backdrop-blur-sm border-b border-slate-700 p-6 text-center relative">
                            <div class="absolute top-4 left-6 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                                "WRESTLER"
                            </div>
                            <div class="absolute top-4 right-16 text-xs font-medium text-slate-400 bg-slate-700/50 px-2 py-1 rounded">
                                "#001"
                            </div>
                            <h2 class="text-2xl font-bold text-slate-100">
                                "Wrestler Profile"
                            </h2>
                            <p class="text-slate-400 text-sm mt-1">
                                "Character Details & Statistics"
                            </p>
                        </div>

                        <div class="grid md:grid-cols-2 gap-6 p-6">
                            // Left side - Image and basic info
                            <div class="space-y-4">
                                // Wrestler image placeholder
                                <div class="bg-slate-800/60 border border-slate-700 rounded-lg aspect-[3/4] flex items-center justify-center relative overflow-hidden">
                                    <div class="absolute inset-4 bg-slate-700/50 backdrop-blur-sm rounded border border-slate-600 flex items-center justify-center">
                                        <div class="text-center text-slate-400">
                                            <svg class="w-16 h-16 mx-auto mb-2" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                                            </svg>
                                            <p class="text-sm font-medium">"Photo Coming Soon"</p>
                                        </div>
                                    </div>
                                </div>

                                // Wrestler name banner
                                <div class="bg-slate-800/80 backdrop-blur-sm border border-slate-700 p-4 rounded-lg">
                                    <h3 class="text-3xl font-bold text-slate-100 text-center">
                                        "The Rock"
                                    </h3>
                                    <p class="text-center text-slate-400 text-sm mt-1">"The People's Champion"</p>
                                </div>
                            </div>

                            // Right side - Stats and info
                            <div class="space-y-4">
                                // Real name section
                                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                    <div class="text-indigo-400 text-sm font-medium mb-2">
                                        "Real Name"
                                    </div>
                                    <p class="text-slate-100 font-semibold text-lg">"Dwayne Johnson"</p>
                                </div>

                                // Power ratings
                                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                    <h4 class="text-slate-100 font-bold text-lg mb-4 text-center border-b border-slate-700 pb-2">
                                        "Power Ratings"
                                    </h4>
                                    <div class="space-y-3">
                                        <PowerBar label="STRENGTH" value=7 color="bg-red-500" />
                                        <PowerBar label="SPEED" value=5 color="bg-blue-500" />
                                        <PowerBar label="AGILITY" value=6 color="bg-green-500" />
                                        <PowerBar label="STAMINA" value=8 color="bg-purple-500" />
                                        <PowerBar label="CHARISMA" value=10 color="bg-indigo-500" />
                                        <PowerBar label="TECHNIQUE" value=7 color="bg-cyan-500" />
                                    </div>
                                </div>

                                // Basic stats
                                <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                    <div class="grid grid-cols-2 gap-4 text-sm">
                                        <div>
                                            <span class="text-slate-400 font-medium">"Promotion: "</span>
                                            <span class="text-slate-100">"WWE"</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400 font-medium">"Height: "</span>
                                            <span class="text-slate-100">"6'5\""</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400 font-medium">"Weight: "</span>
                                            <span class="text-slate-100">"260 lbs"</span>
                                        </div>
                                        <div>
                                            <span class="text-slate-400 font-medium">"Debut: "</span>
                                            <span class="text-slate-100">"1996"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Bottom section
                        <div class="px-6 pb-6 space-y-4">
                            // Biography
                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                <h4 class="text-slate-100 font-semibold text-lg mb-3">"Biography"</h4>
                                <p class="text-slate-300 text-sm leading-relaxed">
                                    "The Rock is one of the most electrifying superstars in sports entertainment history. Known for his incredible charisma, devastating finishing moves, and ability to captivate audiences worldwide. From his days as 'Rocky Maivia' to becoming 'The People's Champion,' The Rock has dominated both the wrestling ring and Hollywood."
                                </p>
                            </div>

                            // Signature moves
                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                <h4 class="text-slate-100 font-semibold text-lg mb-3">"Signature Moves"</h4>
                                <div class="flex flex-wrap gap-2">
                                    <span class="bg-indigo-600/80 backdrop-blur-sm border border-indigo-500 text-white px-3 py-1 rounded text-sm font-medium">"Rock Bottom"</span>
                                    <span class="bg-purple-600/80 backdrop-blur-sm border border-purple-500 text-white px-3 py-1 rounded text-sm font-medium">"People's Elbow"</span>
                                    <span class="bg-cyan-600/80 backdrop-blur-sm border border-cyan-500 text-white px-3 py-1 rounded text-sm font-medium">"Samoan Drop"</span>
                                    <span class="bg-blue-600/80 backdrop-blur-sm border border-blue-500 text-white px-3 py-1 rounded text-sm font-medium">"Spinebuster"</span>
                                </div>
                            </div>

                            // Did you know section
                            <div class="bg-slate-800/60 border border-slate-700 rounded-lg p-4">
                                <h4 class="text-slate-100 font-semibold text-lg mb-3">"Did You Know"</h4>
                                <p class="text-slate-300 text-sm leading-relaxed">
                                    "The Rock is not only a wrestling legend but also one of the highest-paid actors in Hollywood! He's starred in major blockbuster films and has become a global icon beyond the wrestling world."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[component]
fn PowerBar(
    #[prop(into)] label: String,
    #[prop(into)] value: u8,
    #[prop(into)] color: String,
) -> impl IntoView {
    let percentage = (value as f32 / 10.0 * 100.0) as u8;
    
    view! {
        <div class="flex items-center space-x-3">
            <span class="text-slate-300 font-medium text-sm w-20 text-right">{label}</span>
            <div class="flex-1 bg-slate-700/50 rounded-full h-3 border border-slate-600">
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