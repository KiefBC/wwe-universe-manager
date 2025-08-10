use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use gloo_timers::future::TimeoutFuture;
use std::sync::atomic::{AtomicBool, Ordering};

// Constants for error recovery and system monitoring
const MAX_RETRY_ATTEMPTS: u32 = 3;
const BASE_RETRY_DELAY_MS: u64 = 1000; // 1 second base delay
const AUTO_REFRESH_INTERVAL_MS: u32 = 30_000; // 30 seconds
const PROGRESS_MULTIPLIER_TEN: i32 = 10; // For progress calculations
const PROGRESS_MULTIPLIER_HUNDRED: i32 = 100; // For percentage calculations

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Executive Command Center Component
/// 
/// Sophisticated control interface for executive management with:
/// - Real-time system status monitoring
/// - Strategic alerts and notifications system
/// - Performance optimization tools
/// - Advanced system administration
#[component]
pub fn ExecutiveCommandCenter(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (system_health, set_system_health) = signal(None::<SystemHealth>);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(None::<String>);
    let (last_updated, set_last_updated) = signal(Utc::now());
    let (retry_count, set_retry_count) = signal(0);
    
    // Load system health data with robust error recovery
    let load_system_data_with_retry = move || {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);
            
            match get_system_health_with_retry().await {
                Ok(health) => {
                    set_system_health.set(Some(health));
                    set_last_updated.set(Utc::now());
                    set_error.set(None);
                    set_retry_count.set(0);
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to load system health after retries: {}", e).into());
                    set_error.set(Some(format!("System monitoring error: {}", e)));
                    set_retry_count.set(retry_count.get() + 1);
                }
            }
            set_loading.set(false);
        });
    };
    
    // Initialize command center data
    Effect::new(move |_| {
        load_system_data_with_retry();
    });
    
    // Auto-refresh with proper cleanup mechanism (fixes memory leak)
    Effect::new(move |_| {
        let should_continue = std::rc::Rc::new(AtomicBool::new(true));
        let should_continue_clone = should_continue.clone();
        
        spawn_local(async move {
            while should_continue_clone.load(Ordering::Relaxed) {
                TimeoutFuture::new(AUTO_REFRESH_INTERVAL_MS).await;
                
                if should_continue_clone.load(Ordering::Relaxed) {
                    load_system_data_with_retry();
                }
            }
        });
        
        // Cleanup function to prevent memory leaks
        move || {
            should_continue.store(false, Ordering::Relaxed);
        }
    });
    
    // System optimization handler
    let optimize_system = move |_| {
        spawn_local(async move {
            web_sys::console::log_1(&"Initiating system optimization...".into());
            // Add optimization logic here
        });
    };
    
    // Data backup handler
    let backup_data = move |_| {
        spawn_local(async move {
            web_sys::console::log_1(&"Initiating data backup...".into());
            // Add backup logic here
        });
    };
    
    // Manual refresh handler with retry capability
    let manual_refresh = move |_| {
        load_system_data_with_retry();
    };
    
    view! {
        <div class="space-y-8">
            // Error display
            {move || {
                if let Some(err) = error.get() {
                    view! {
                        <div class="alert alert-error">
                            <svg class="stroke-current shrink-0 w-6 h-6" fill="none" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                            </svg>
                            <span>{format!("System monitoring error: {}", err)}</span>
                            <button class="btn btn-sm" on:click=manual_refresh>"Retry"</button>
                        </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            // Executive Command Header with Real-time Status
            <div class="hero bg-gradient-to-br from-primary/10 to-secondary/10 rounded-box border border-primary/20">
                <div class="hero-content text-center">
                    <div class="max-w-4xl">
                        <div class="flex items-center justify-center gap-4 mb-6">
                            <div class="indicator">
                                {move || {
                                    if loading.get() {
                                        view! {
                                            <span class="indicator-item badge badge-warning badge-sm animate-pulse">
                                                "LOADING"
                                            </span>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <span class="indicator-item badge badge-success badge-sm animate-pulse">
                                                "LIVE"
                                            </span>
                                        }.into_any()
                                    }
                                }}
                                <div class="w-16 h-16 bg-gradient-to-br from-primary via-accent to-secondary rounded-2xl flex items-center justify-center shadow-2xl">
                                    <svg class="w-10 h-10 text-base-100" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <h1 class="text-5xl font-bold bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent mb-4">
                            "Executive Command Center"
                        </h1>
                        <p class="text-lg text-base-content/80 leading-relaxed mb-6">
                            "Real-time system monitoring and strategic oversight for wrestling empire management. 
                            Advanced administrative controls with executive-level business intelligence."
                        </p>
                        <div class="flex items-center justify-center gap-4">
                            {move || {
                                if let Some(health) = system_health.get() {
                                    let badge_class = match health.status.as_str() {
                                        "Operational" => "badge-success",
                                        "Warning" => "badge-warning", 
                                        "Critical" => "badge-error",
                                        _ => "badge-info",
                                    };
                                    view! {
                                        <div class=format!("badge {} gap-2", badge_class)>
                                            <div class="w-2 h-2 rounded-full bg-current animate-pulse"></div>
                                            {health.status.to_uppercase()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="badge badge-neutral gap-2">
                                            <span class="loading loading-spinner loading-xs"></span>
                                            "LOADING"
                                        </div>
                                    }.into_any()
                                }
                            }}
                            
                            <div class="text-sm text-base-content/70">
                                {move || {
                                    let updated = last_updated.get();
                                    format!("Last updated: {} {}", 
                                        updated.date_naive(),
                                        updated.format("%H:%M:%S")
                                    )
                                }}
                            </div>
                            
                            <button class="btn btn-xs btn-ghost" on:click=manual_refresh>
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                                </svg>
                                "Refresh"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
            
            // Strategic Alerts and Notifications System
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // System Alerts
                <div class="card bg-base-100 shadow-xl border border-base-300/50">
                    <div class="card-body">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 bg-warning/20 rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.99-.833-2.5 0L3.314 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">"Strategic Alerts"</h3>
                            {move || {
                                if let Some(health) = system_health.get() {
                                    view! {
                                        <div class="badge badge-warning badge-sm">
                                            {health.active_alerts.len()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="badge badge-neutral badge-sm">
                                            <span class="loading loading-spinner loading-xs"></span>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                        <div class="space-y-2 max-h-60 overflow-y-auto">
                            {move || {
                                if let Some(health) = system_health.get() {
                                    if health.active_alerts.is_empty() {
                                        view! {
                                            <div class="text-center py-8 text-base-content/60">
                                                <svg class="w-12 h-12 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                </svg>
                                                <p class="text-sm">"All systems operational"</p>
                                                <p class="text-xs">"No active alerts"</p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || health.active_alerts.clone()
                                                key=|alert| format!("{}-{:?}", alert.message, alert.created_at)
                                                children=move |alert| {
                                                    let alert_class = match alert.priority {
                                                        AlertPriority::Critical => "alert-error",
                                                        AlertPriority::High => "alert-warning",
                                                        AlertPriority::Medium => "alert-info",
                                                        AlertPriority::Low => "alert-success",
                                                        AlertPriority::Info => "alert-info",
                                                    };
                                                    
                                                    view! {
                                                        <div class=format!("alert {} alert-sm", alert_class)>
                                                            <svg class="stroke-current shrink-0 w-4 h-4" fill="none" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.99-.833-2.5 0L3.314 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                                            </svg>
                                                            <div>
                                                                <span class="text-sm font-medium">{alert.message}</span>
                                                                <div class="text-xs opacity-70">
                                                                    {format!("{} • {}", alert.category, 
                                                                        alert.created_at.format("%H:%M").to_string())}
                                                                    {if alert.requires_action { " • Action Required" } else { "" }}
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        }.into_any()
                                    }
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            <div class="skeleton h-12 w-full"></div>
                                            <div class="skeleton h-12 w-full"></div>
                                            <div class="skeleton h-12 w-3/4"></div>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
                
                // Performance Metrics
                <div class="card bg-base-100 shadow-xl border border-base-300/50">
                    <div class="card-body">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 bg-success/20 rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">"System Performance"</h3>
                            {move || {
                                if let Some(health) = system_health.get() {
                                    let score = health.performance_metrics.db_health_score;
                                    let color = if score >= 90 { "success" } else if score >= 70 { "warning" } else { "error" };
                                    view! {
                                        <div class=format!("badge badge-{} badge-sm", color)>
                                            {format!("{}%", score)}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="badge badge-neutral badge-sm">
                                            <span class="loading loading-spinner loading-xs"></span>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                        
                        {move || {
                            if let Some(health) = system_health.get() {
                                let metrics = &health.performance_metrics;
                                view! {
                                    <div class="space-y-4">
                                        <div class="stat bg-base-200/50 rounded-lg p-3">
                                            <div class="stat-title text-sm">"Database Response"</div>
                                            <div class="stat-value text-lg text-success">
                                                {format!("{}ms", metrics.db_response_time)}
                                            </div>
                                            <div class="stat-desc">
                                                <div class="radial-progress text-success text-xs" style=format!("--value:{}", metrics.db_health_score)>
                                                    {format!("{}%", metrics.db_health_score)}
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="stat bg-base-200/50 rounded-lg p-3">
                                            <div class="stat-title text-sm">"Memory Usage"</div>
                                            <div class="stat-value text-lg">
                                                {format!("{}MB", metrics.memory_usage)}
                                            </div>
                                            <div class="stat-desc">
                                                <progress class="progress progress-primary w-full" value={metrics.memory_usage as f64} max="1024.0"></progress>
                                            </div>
                                        </div>
                                        
                                        <div class="stat bg-base-200/50 rounded-lg p-3">
                                            <div class="stat-title text-sm">"CPU Usage"</div>
                                            <div class="stat-value text-lg">
                                                {format!("{}%", metrics.cpu_usage)}
                                            </div>
                                            <div class="stat-desc">
                                                <progress 
                                                    class="progress progress-info w-full" 
                                                    value={metrics.cpu_usage as f64} 
                                                    max="100.0">
                                                </progress>
                                            </div>
                                        </div>
                                        
                                        <div class="stats stats-horizontal bg-base-200/50 rounded-lg">
                                            <div class="stat px-3 py-2">
                                                <div class="stat-title text-xs">"Requests/min"</div>
                                                <div class="stat-value text-sm">{metrics.requests_per_minute}</div>
                                            </div>
                                            <div class="stat px-3 py-2">
                                                <div class="stat-title text-xs">"Error Rate"</div>
                                                <div class="stat-value text-sm">{format!("{:.1}%", metrics.error_rate)}</div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-2">
                                        <div class="skeleton h-16 w-full"></div>
                                        <div class="skeleton h-16 w-full"></div>
                                        <div class="skeleton h-16 w-full"></div>
                                        <div class="skeleton h-8 w-3/4"></div>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>
                
                // Pending Decisions Queue
                <div class="card bg-base-100 shadow-xl border border-base-300/50">
                    <div class="card-body">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 bg-info/20 rounded-lg flex items-center justify-center">
                                <svg class="w-5 h-5 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4"/>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold">"Strategic Decisions"</h3>
                            {move || {
                                if let Some(health) = system_health.get() {
                                    view! {
                                        <div class="badge badge-info badge-sm">
                                            {health.pending_decisions.len()}
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <div class="badge badge-neutral badge-sm">
                                            <span class="loading loading-spinner loading-xs"></span>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                        <div class="space-y-2 max-h-60 overflow-y-auto">
                            {move || {
                                if let Some(health) = system_health.get() {
                                    if health.pending_decisions.is_empty() {
                                        view! {
                                            <div class="text-center py-8 text-base-content/60">
                                                <svg class="w-12 h-12 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                </svg>
                                                <p class="text-sm">"No pending decisions"</p>
                                                <p class="text-xs">"All operations current"</p>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || health.pending_decisions.clone()
                                                key=|decision| decision.clone()
                                                children=move |decision| {
                                                    view! {
                                                        <div class="card bg-base-200/50 border border-base-300/30 hover:border-info/50 transition-colors">
                                                            <div class="card-body p-3">
                                                                <div class="flex items-start gap-2">
                                                                    <svg class="w-4 h-4 text-info flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                                    </svg>
                                                                    <div class="flex-1">
                                                                        <p class="text-sm font-medium text-base-content">{decision}</p>
                                                                        <div class="flex gap-2 mt-3">
                                                                            <button class="btn btn-xs btn-primary">"Review"</button>
                                                                            <button class="btn btn-xs btn-ghost">"Later"</button>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }
                                            />
                                        }.into_any()
                                    }
                                } else {
                                    view! {
                                        <div class="space-y-2">
                                            <div class="skeleton h-16 w-full"></div>
                                            <div class="skeleton h-16 w-full"></div>
                                            <div class="skeleton h-16 w-3/4"></div>
                                        </div>
                                    }.into_any()
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>
            
            // Advanced System Administration Tools
            <div class="card bg-gradient-to-br from-base-100 to-base-200/50 shadow-xl border border-base-300/50">
                <div class="card-body">
                    <h2 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <div class="w-8 h-8 bg-primary/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        "System Administration"
                    </h2>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
                        // Data Management
                        <div class="card bg-base-100 border border-base-300/30">
                            <div class="card-body p-4">
                                <h3 class="font-bold mb-3 flex items-center gap-2">
                                    <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7"/>
                                    </svg>
                                    "Data Management"
                                </h3>
                                <div class="space-y-2">
                                    <button 
                                        class="btn btn-sm btn-primary w-full gap-2"
                                        on:click=backup_data
                                    >
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4-4m0 0L8 8m4-4v12"/>
                                        </svg>
                                        "Backup Data"
                                    </button>
                                    <button class="btn btn-sm btn-secondary w-full gap-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4 4m0 0l4 4m-4-4H3"/>
                                        </svg>
                                        "Restore Data"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        // Performance Tools
                        <div class="card bg-base-100 border border-base-300/30">
                            <div class="card-body p-4">
                                <h3 class="font-bold mb-3 flex items-center gap-2">
                                    <svg class="w-5 h-5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                                    </svg>
                                    "Performance"
                                </h3>
                                <div class="space-y-2">
                                    <button 
                                        class="btn btn-sm btn-success w-full gap-2"
                                        on:click=optimize_system
                                    >
                                        <span class="loading loading-spinner loading-xs"></span>
                                        "Optimize System"
                                    </button>
                                    <button class="btn btn-sm btn-info w-full gap-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                        </svg>
                                        "Generate Report"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        // Security & Integrity
                        <div class="card bg-base-100 border border-base-300/30">
                            <div class="card-body p-4">
                                <h3 class="font-bold mb-3 flex items-center gap-2">
                                    <svg class="w-5 h-5 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                                    </svg>
                                    "Security"
                                </h3>
                                <div class="space-y-2">
                                    <button class="btn btn-sm btn-warning w-full gap-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                        </svg>
                                        "Health Check"
                                    </button>
                                    <button class="btn btn-sm btn-error w-full gap-2">
                                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.99-.833-2.5 0L3.314 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                        </svg>
                                        "Audit Log"
                                    </button>
                                </div>
                            </div>
                        </div>
                        
                        // System Info
                        <div class="card bg-base-100 border border-base-300/30">
                            <div class="card-body p-4">
                                <h3 class="font-bold mb-3 flex items-center gap-2">
                                    <svg class="w-5 h-5 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                    </svg>
                                    "System Info"
                                </h3>
                                {move || {
                                    if let Some(health) = system_health.get() {
                                        let uptime_hours = health.uptime_seconds / 3600;
                                        let uptime_minutes = (health.uptime_seconds % 3600) / 60;
                                        let db_size_mb = (health.database_size as f64) / (1024.0 * 1024.0);
                                        
                                        view! {
                                            <div class="space-y-2">
                                                <div class="text-xs text-base-content/70">
                                                    {format!("Version: {}", health.version)}
                                                </div>
                                                <div class="text-xs text-base-content/70">
                                                    {format!("Uptime: {}h {}m", uptime_hours, uptime_minutes)}
                                                </div>
                                                <div class="text-xs text-base-content/70">
                                                    {format!("DB Size: {:.1}MB", db_size_mb)}
                                                </div>
                                                <div class="text-xs text-base-content/70">
                                                    {format!("Memory: {}MB", health.memory_usage)}
                                                </div>
                                                <div class="divider my-2"></div>
                                                <div class="text-xs text-base-content/70">
                                                    {format!("DB Health: {}%", health.database_health.health_score)}
                                                </div>
                                                <div class="text-xs text-base-content/70">
                                                    {format!("Active Connections: {}", health.database_health.active_connections)}
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="space-y-2">
                                                <div class="skeleton h-3 w-24"></div>
                                                <div class="skeleton h-3 w-20"></div>
                                                <div class="skeleton h-3 w-16"></div>
                                                <div class="skeleton h-3 w-18"></div>
                                            </div>
                                        }.into_any()
                                    }
                                }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            
            // Executive Navigation Back
            <div class="text-center">
                <button 
                    class="btn btn-neutral gap-2"
                    on:click=move |_| set_current_page.set("promotion-dashboard".to_string())
                >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/>
                    </svg>
                    "Return to Executive Dashboard"
                </button>
            </div>
        </div>
    }
}

// Real backend data structures matching Rust models
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SystemHealth {
    status: String,
    uptime_seconds: i64,
    database_health: DatabaseHealth,
    performance_metrics: PerformanceMetrics,
    active_alerts: Vec<SystemAlert>,
    recent_operations: Vec<OperationLog>,
    pending_decisions: Vec<String>,
    version: String,
    database_size: i64,
    memory_usage: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DatabaseHealth {
    avg_response_time: i32,
    connection_pool_healthy: bool,
    health_score: i32,
    active_connections: i32,
    queries_last_hour: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PerformanceMetrics {
    db_response_time: i32,
    db_health_score: i32,
    memory_usage: i32,
    cpu_usage: i32,
    requests_per_minute: i32,
    error_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SystemAlert {
    message: String,
    priority: AlertPriority,
    created_at: DateTime<Utc>,
    category: String,
    requires_action: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum AlertPriority {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OperationLog {
    timestamp: DateTime<Utc>,
    operation: String,
    status: String,
    performed_by: String,
    details: Option<String>,
    duration_ms: Option<i32>,
}

// Real Tauri backend functions with robust error recovery
async fn get_system_health() -> Result<SystemHealth, String> {
    let result = invoke("get_system_health", JsValue::NULL).await
        .map_err(|e| format!("Failed to invoke system health: {:?}", e))?;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to parse system health: {}", e))
}

/// Enhanced system health retrieval with exponential backoff retry logic
/// 
/// This function implements robust error recovery for production reliability:
/// - Retries failed requests up to MAX_RETRY_ATTEMPTS times
/// - Uses exponential backoff to avoid overwhelming the backend
/// - Provides graceful degradation for monitoring failures
async fn get_system_health_with_retry() -> Result<SystemHealth, String> {
    let mut attempts = 0;
    
    loop {
        match get_system_health().await {
            Ok(health) => return Ok(health),
            Err(e) if attempts < MAX_RETRY_ATTEMPTS => {
                attempts += 1;
                
                // Exponential backoff: 1s, 2s, 4s delays
                let delay_ms = BASE_RETRY_DELAY_MS * (2_u64.pow(attempts - 1));
                web_sys::console::warn_1(&format!(
                    "System health request failed (attempt {}/{}), retrying in {}ms: {}", 
                    attempts, MAX_RETRY_ATTEMPTS, delay_ms, e
                ).into());
                
                TimeoutFuture::new(delay_ms as u32).await;
            },
            Err(e) => {
                return Err(format!(
                    "System health failed after {} attempts: {}", 
                    MAX_RETRY_ATTEMPTS, e
                ));
            }
        }
    }
}

async fn get_system_alerts() -> Result<Vec<SystemAlert>, String> {
    let result = invoke("get_system_alerts", JsValue::NULL).await
        .map_err(|e| format!("Failed to invoke system alerts: {:?}", e))?;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to parse system alerts: {}", e))
}

async fn get_performance_metrics() -> Result<PerformanceMetrics, String> {
    let result = invoke("get_performance_metrics", JsValue::NULL).await
        .map_err(|e| format!("Failed to invoke performance metrics: {:?}", e))?;
    serde_wasm_bindgen::from_value(result)
        .map_err(|e| format!("Failed to parse performance metrics: {}", e))
}