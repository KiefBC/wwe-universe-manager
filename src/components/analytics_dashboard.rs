use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Performance Analytics Dashboard Component
/// 
/// Executive-level business intelligence with sophisticated analytics:
/// - Talent performance metrics and progression tracking
/// - Show success analytics and audience engagement
/// - Championship impact analysis and prestige optimization
/// - Strategic planning tools with competitive intelligence
#[component]
pub fn AnalyticsDashboard(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    // Navigation handlers only - analytics are static for now

    // Navigation handlers
    let back_to_dashboard = move |_| {
        set_current_page.set("promotion-dashboard".to_string());
    };


    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-primary/10 via-accent/10 to-secondary/10 rounded-none border-b border-primary/20 mb-6 sm:mb-8">
                <div class="hero-content text-center py-4 sm:py-6">
                    <div class="max-w-6xl w-full">
                        <h1 class="text-2xl sm:text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent mb-6">
                            "Wrestling Management System (WMS)"
                        </h1>
                        
                        // Action buttons row - mobile responsive
                        <div class="flex flex-col sm:flex-row items-center justify-center gap-3 sm:gap-4 px-4 mt-2">
                            <button
                                class="btn btn-primary gap-2 w-full sm:w-auto min-h-[44px]"
                                on:click=back_to_dashboard
                            >
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                </svg>
                                "Back to Command Hub"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="px-4 sm:px-6 lg:px-8 pb-8 sm:pb-12">
                <div class="max-w-6xl mx-auto space-y-6">

                    // Analytics Overview Section
                    <section>
                        <div class="mb-6">
                            <h2 class="text-2xl sm:text-3xl font-bold text-base-content mb-2">"Business Intelligence Overview"</h2>
                            <p class="text-base-content/70 text-sm sm:text-base">"Comprehensive performance analytics and strategic insights for your wrestling empire"</p>
                        </div>

                        // Executive Analytics Dashboard
                        <div class="grid grid-cols-1 lg:grid-cols-4 gap-6 mb-8">
                            // Talent Metrics
                            <div class="stats stats-vertical shadow-lg bg-base-100 border border-base-300/50">
                                <div class="stat px-3 sm:px-6 py-4">
                                    <div class="stat-figure text-primary">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                        </svg>
                                    </div>
                                    <div class="stat-title text-primary/80 font-medium">"Active Roster"</div>
                                    <div class="stat-value text-2xl font-bold text-primary">"47"</div>
                                    <div class="stat-desc text-primary/60">"Global talent pool"</div>
                                </div>
                            </div>

                            // Show Metrics
                            <div class="stats stats-vertical shadow-lg bg-base-100 border border-base-300/50">
                                <div class="stat px-3 sm:px-6 py-4">
                                    <div class="stat-figure text-secondary">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                        </svg>
                                    </div>
                                    <div class="stat-title text-secondary/80 font-medium">"Active Shows"</div>
                                    <div class="stat-value text-2xl font-bold text-secondary">"3"</div>
                                    <div class="stat-desc text-secondary/60">"Programming portfolio"</div>
                                </div>
                            </div>

                            // Championship Metrics  
                            <div class="stats stats-vertical shadow-lg bg-base-100 border border-base-300/50">
                                <div class="stat px-3 sm:px-6 py-4">
                                    <div class="stat-figure text-accent">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                        </svg>
                                    </div>
                                    <div class="stat-title text-accent/80 font-medium">"Championships"</div>
                                    <div class="stat-value text-2xl font-bold text-accent">"15"</div>
                                    <div class="stat-desc text-accent/60">"Title portfolio"</div>
                                </div>
                            </div>

                            // Performance Metrics
                            <div class="stats stats-vertical shadow-lg bg-base-100 border border-base-300/50">
                                <div class="stat px-3 sm:px-6 py-4">
                                    <div class="stat-figure text-success">
                                        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                                        </svg>
                                    </div>
                                    <div class="stat-title text-success/80 font-medium">"Win Rate"</div>
                                    <div class="stat-value text-2xl font-bold text-success">"87.5%"</div>
                                    <div class="stat-desc text-success/60">"Match success rate"</div>
                                </div>
                            </div>
                        </div>

                        // Coming Soon Features
                        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                            <div class="card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-lg">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-10 h-10 bg-primary/20 rounded-xl flex items-center justify-center">
                                            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-primary badge-sm">"TALENT"</div>
                                    </div>
                                    <h3 class="text-lg font-bold text-base-content mb-2">"Performance Tracking"</h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">"Detailed wrestler analytics with win/loss ratios, performance trends, and career progression tracking."</p>
                                    <div class="badge badge-outline">"Phase 5 Feature"</div>
                                </div>
                            </div>

                            <div class="card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20 shadow-lg">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-10 h-10 bg-secondary/20 rounded-xl flex items-center justify-center">
                                            <svg class="w-6 h-6 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-secondary badge-sm">"SHOWS"</div>
                                    </div>
                                    <h3 class="text-lg font-bold text-base-content mb-2">"Audience Analytics"</h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">"Show performance metrics including viewer engagement, match ratings, and content optimization insights."</p>
                                    <div class="badge badge-outline">"Coming Soon"</div>
                                </div>
                            </div>

                            <div class="card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20 shadow-lg">
                                <div class="card-body p-4 sm:p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-10 h-10 bg-accent/20 rounded-xl flex items-center justify-center">
                                            <svg class="w-6 h-6 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-accent badge-sm">"STRATEGY"</div>
                                    </div>
                                    <h3 class="text-lg font-bold text-base-content mb-2">"Strategic Planning"</h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">"AI-powered business intelligence with competitive analysis and strategic recommendations."</p>
                                    <div class="badge badge-outline">"Future Release"</div>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
