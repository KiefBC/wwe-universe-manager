use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use serde_json;
use gloo_timers::future::TimeoutFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Professional Executive Dashboard
/// 
/// Sophisticated homepage for executive-level wrestling management:
/// - Real-time analytics and performance metrics
/// - Professional action center for navigation
/// - System health monitoring and alerts
/// - Responsive design optimized for all devices
/// - Integration with advanced analytics utilities
#[component]
pub fn ProfessionalDashboard(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
    /// Signal that triggers data refresh when incremented  
    refresh_trigger: ReadSignal<u32>,
) -> impl IntoView {
    // Analytics and system data state
    let (analytics_data, set_analytics_data) = signal(None::<DashboardAnalytics>);
    let (system_health, set_system_health) = signal(None::<SystemStatus>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    
    // Load dashboard data
    let load_dashboard_data = move || {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            // Simulate loading analytics data - in real implementation would call analytics endpoints
            match load_dashboard_analytics().await {
                Ok(data) => {
                    set_analytics_data.set(Some(data));
                    set_loading.set(false);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };
    
    // Load data on mount and refresh trigger changes
    Effect::new(move |_| {
        let _trigger = refresh_trigger.get();
        load_dashboard_data();
    });
    
    // Navigation handlers
    let navigate_to_analytics = move |_| set_current_page.set("analytics".to_string());
    let navigate_to_wrestlers = move |_| set_current_page.set("wrestlers".to_string());
    let navigate_to_shows = move |_| set_current_page.set("show-roster".to_string());
    let navigate_to_titles = move |_| set_current_page.set("titles".to_string());
    let navigate_to_booker = move |_| set_current_page.set("booker".to_string());
    let navigate_to_system = move |_| set_current_page.set("system".to_string());
    
    // Test data creation
    let create_test_data = move |_| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap();
            let result = invoke("create_test_data", args).await;
            web_sys::console::log_1(&format!("Test data created: {:?}", result).into());
        });
    };

    view! {
        <div class="min-h-screen bg-base-100">
            // Professional Executive Hero Section
            <div class="hero bg-gradient-to-br from-primary/10 via-accent/10 to-secondary/10 rounded-none border-b border-primary/20 mb-8">
                <div class="hero-content text-center py-6">
                    <div class="max-w-6xl">
                        <h1 class="text-3xl lg:text-4xl font-bold bg-gradient-to-r from-primary via-accent to-secondary bg-clip-text text-transparent mb-6">
                            "Wrestling Management System (WMS)"
                        </h1>
                    </div>
                </div>
            </div>
            
            <div class="px-4 lg:px-6 pb-12">
                <div class="space-y-8">
                    
                    // Executive Analytics Overview
                    <section>
                        <div class="flex items-center justify-between mb-6">
                            <div>
                                <h2 class="text-3xl font-bold text-base-content mb-2">"Performance Analytics"</h2>
                                <p class="text-base-content/70">"Real-time business intelligence and key performance indicators"</p>
                            </div>
                            <button 
                                class="btn btn-primary gap-2"
                                on:click=navigate_to_analytics
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                </svg>
                                "Full Analytics"
                            </button>
                        </div>
                        
                        {move || {
                            if loading.get() {
                                view! {
                                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
                                        {(0..4).map(|_| {
                                            view! {
                                                <div class="card bg-base-100 shadow-lg border border-base-300/50">
                                                    <div class="card-body">
                                                        <div class="skeleton h-4 w-20 mb-2"></div>
                                                        <div class="skeleton h-8 w-16 mb-2"></div>
                                                        <div class="skeleton h-3 w-24"></div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            } else if let Some(data) = analytics_data.get() {
                                view! {
                                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 w-full">
                                        // Active Roster
                                        <div class="bg-base-100 border border-base-300/50 shadow-lg rounded-lg p-4 min-w-0">
                                            <div class="flex items-center gap-3 mb-2">
                                                <div class="w-10 h-10 bg-primary/20 rounded-xl flex items-center justify-center">
                                                    <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                                    </svg>
                                                </div>
                                            </div>
                                            <div class="text-sm text-base-content/70 font-medium mb-1">"Active Roster"</div>
                                            <div class="text-3xl font-bold text-primary mb-1">{data.total_wrestlers}</div>
                                            <div class="text-xs text-primary/70">"Global talent pool"</div>
                                        </div>
                                        
                                        // Live Shows
                                        <div class="bg-base-100 border border-base-300/50 shadow-lg rounded-lg p-4 min-w-0">
                                            <div class="flex items-center gap-3 mb-2">
                                                <div class="w-10 h-10 bg-secondary/20 rounded-xl flex items-center justify-center">
                                                    <svg class="w-6 h-6 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                                    </svg>
                                                </div>
                                            </div>
                                            <div class="text-sm text-base-content/70 font-medium mb-1">"Live Shows"</div>
                                            <div class="text-3xl font-bold text-secondary mb-1">{data.total_shows}</div>
                                            <div class="text-xs text-secondary/70">"Active programming"</div>
                                        </div>
                                        
                                        // Championships
                                        <div class="bg-base-100 border border-base-300/50 shadow-lg rounded-lg p-4 min-w-0">
                                            <div class="flex items-center gap-3 mb-2">
                                                <div class="w-10 h-10 bg-accent/20 rounded-xl flex items-center justify-center">
                                                    <svg class="w-6 h-6 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                                    </svg>
                                                </div>
                                            </div>
                                            <div class="text-sm text-base-content/70 font-medium mb-1">"Championships"</div>
                                            <div class="text-3xl font-bold text-accent mb-1">{data.total_titles}</div>
                                            <div class="text-xs text-accent/70">"Title portfolio"</div>
                                        </div>
                                        
                                        // Match Success
                                        <div class="bg-base-100 border border-base-300/50 shadow-lg rounded-lg p-4 min-w-0">
                                            <div class="flex items-center gap-3 mb-2">
                                                <div class="w-10 h-10 bg-success/20 rounded-xl flex items-center justify-center">
                                                    <svg class="w-6 h-6 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                                                    </svg>
                                                </div>
                                            </div>
                                            <div class="text-sm text-base-content/70 font-medium mb-1">"Match Success"</div>
                                            <div class="text-3xl font-bold text-success mb-1">{format!("{:.0}%", data.match_success_rate)}</div>
                                            <div class="text-xs text-success/70">"Booking efficiency"</div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="alert alert-info">
                                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                        </svg>
                                        <div>
                                            <h3 class="font-bold">"Analytics Loading"</h3>
                                            <div class="text-xs opacity-80">"Gathering performance data..."</div>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </section>
                    
                    // Professional Action Center
                    <section>
                        <div class="mb-6">
                            <h2 class="text-3xl font-bold text-base-content mb-2">"Command Center"</h2>
                            <p class="text-base-content/70">"Executive management tools and system administration"</p>
                        </div>
                        
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                            // Wrestling Management
                            <div class="card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_wrestlers>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-primary/20 rounded-xl flex items-center justify-center group-hover:bg-primary/30 transition-colors">
                                            <svg class="w-7 h-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-primary badge-sm">"ROSTER"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-primary transition-colors">
                                        "Talent Management"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">
                                        "Manage wrestler profiles, contracts, and performance analytics"
                                    </p>
                                    <div class="flex items-center justify-between">
                                        <span class="text-xs text-base-content/50">"Active roster"</span>
                                        <svg class="w-4 h-4 text-base-content/40 group-hover:text-primary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                        </svg>
                                    </div>
                                </div>
                            </div>
                            
                            // Show Management
                            <div class="card bg-gradient-to-br from-secondary/10 to-secondary/5 border border-secondary/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_shows>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-secondary/20 rounded-xl flex items-center justify-center group-hover:bg-secondary/30 transition-colors">
                                            <svg class="w-7 h-7 text-secondary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-secondary badge-sm">"PROGRAMMING"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-secondary transition-colors">
                                        "Show Management"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">
                                        "Manage show rosters, scheduling, and content programming"
                                    </p>
                                    <div class="flex items-center justify-between">
                                        <span class="text-xs text-base-content/50">"Live programming"</span>
                                        <svg class="w-4 h-4 text-base-content/40 group-hover:text-secondary transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                        </svg>
                                    </div>
                                </div>
                            </div>
                            
                            // Championship Management
                            <div class="card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_titles>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-accent/20 rounded-xl flex items-center justify-center group-hover:bg-accent/30 transition-colors">
                                            <svg class="w-7 h-7 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-accent badge-sm">"PRESTIGE"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-accent transition-colors">
                                        "Championship Titles"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">
                                        "Manage championships, title holders, and prestige rankings"
                                    </p>
                                    <div class="flex items-center justify-between">
                                        <span class="text-xs text-base-content/50">"Title portfolio"</span>
                                        <svg class="w-4 h-4 text-base-content/40 group-hover:text-accent transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                        </svg>
                                    </div>
                                </div>
                            </div>
                            
                            // Match Booking
                            <div class="card bg-gradient-to-br from-info/10 to-info/5 border border-info/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_booker>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-info/20 rounded-xl flex items-center justify-center group-hover:bg-info/30 transition-colors">
                                            <svg class="w-7 h-7 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-info badge-sm">"BOOKING"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-info transition-colors">
                                        "Match Booker"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed mb-4">
                                        "Book matches, create storylines, and manage card layouts"
                                    </p>
                                    <div class="flex items-center justify-between">
                                        <span class="text-xs text-base-content/50">"Creative control"</span>
                                        <svg class="w-4 h-4 text-base-content/40 group-hover:text-info transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                        </svg>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </section>
                    
                    // System Administration
                    <section>
                        <div class="mb-6">
                            <h2 class="text-3xl font-bold text-base-content mb-2">"System Administration"</h2>
                            <p class="text-base-content/70">"Advanced tools and utilities for system management"</p>
                        </div>
                        
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                            // Analytics Dashboard
                            <div class="card bg-gradient-to-br from-success/10 to-success/5 border border-success/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_analytics>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-success/20 rounded-xl flex items-center justify-center group-hover:bg-success/30 transition-colors">
                                            <svg class="w-7 h-7 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-success badge-sm">"ANALYTICS"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-success transition-colors">
                                        "Business Intelligence"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed">
                                        "Advanced analytics, performance metrics, and strategic planning tools"
                                    </p>
                                </div>
                            </div>
                            
                            // System Command Center  
                            <div class="card bg-gradient-to-br from-warning/10 to-warning/5 border border-warning/20 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=navigate_to_system>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-warning/20 rounded-xl flex items-center justify-center group-hover:bg-warning/30 transition-colors">
                                            <svg class="w-7 h-7 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-warning badge-sm">"SYSTEM"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-warning transition-colors">
                                        "System Monitor"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed">
                                        "Real-time system health, performance metrics, and administrative tools"
                                    </p>
                                </div>
                            </div>
                            
                            // Test Data Utilities
                            <div class="card bg-gradient-to-br from-base-300/20 to-base-300/10 border border-base-300/40 shadow-lg hover:shadow-xl cursor-pointer transition-all duration-200 group"
                                 on:click=create_test_data>
                                <div class="card-body p-6">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-12 h-12 bg-base-300/30 rounded-xl flex items-center justify-center group-hover:bg-base-300/40 transition-colors">
                                            <svg class="w-7 h-7 text-base-content/70" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"/>
                                            </svg>
                                        </div>
                                        <div class="badge badge-neutral badge-sm">"UTILITIES"</div>
                                    </div>
                                    <h3 class="text-xl font-bold text-base-content mb-2 group-hover:text-primary transition-colors">
                                        "Test Data Generator"
                                    </h3>
                                    <p class="text-base-content/70 text-sm leading-relaxed">
                                        "Generate sample wrestlers, shows, and championships for testing"
                                    </p>
                                </div>
                            </div>
                        </div>
                    </section>
                    
                </div>
            </div>
        </div>
    }
}

// Data models for dashboard analytics
#[derive(Clone, Debug, Serialize, Deserialize)]
struct DashboardAnalytics {
    total_wrestlers: i32,
    total_shows: i32,
    total_titles: i32,
    match_success_rate: f64,
    active_matches: i32,
    system_health: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SystemStatus {
    status: String,
    uptime: i64,
    performance_score: f64,
}

// Mock data loading function - in real implementation would call actual analytics endpoints
async fn load_dashboard_analytics() -> Result<DashboardAnalytics, String> {
    // Simulate API call delay
    TimeoutFuture::new(500).await;
    
    Ok(DashboardAnalytics {
        total_wrestlers: 47,
        total_shows: 3,
        total_titles: 15,
        match_success_rate: 87.5,
        active_matches: 12,
        system_health: 95.2,
    })
}
