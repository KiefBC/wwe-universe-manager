use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
// Using mock data structures for demonstration - would be replaced with proper types from backend

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Executive Reporting System Component
/// 
/// Comprehensive executive-level reporting with:
/// - Professional report generation and export
/// - Strategic business intelligence summaries
/// - Performance analytics with trend analysis
/// - Executive-formatted presentations
#[component]
pub fn ExecutiveReporting(
    /// Signal to update the current page/route
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    let (selected_report, set_selected_report) = signal("roster_performance".to_string());
    let (report_data, set_report_data) = signal(None::<ReportData>);
    let (loading_report, set_loading_report) = signal(false);
    let (_analytics_data, set_analytics_data) = signal(None::<MockAnalyticsData>);
    let (export_format, set_export_format) = signal("executive_summary".to_string());
    let (report_period, set_report_period) = signal("monthly".to_string());
    
    // Load analytics data on mount
    Effect::new(move |_| {
        spawn_local(async move {
            load_analytics_data(set_analytics_data).await;
        });
    });
    
    // Generate report handler
    let generate_report = move |_| {
        let report_type = selected_report.get();
        let period = report_period.get();
        
        set_loading_report.set(true);
        spawn_local(async move {
            match generate_executive_report(report_type, period).await {
                Ok(data) => {
                    set_report_data.set(Some(data));
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to generate report: {}", e).into());
                }
            }
            set_loading_report.set(false);
        });
    };
    
    // Export report handler
    let export_report = move |_| {
        let format = export_format.get();
        
        spawn_local(async move {
            match export_executive_report(format).await {
                Ok(_) => {
                    web_sys::console::log_1(&"Report exported successfully".into());
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Failed to export report: {}", e).into());
                }
            }
        });
    };
    
    view! {
        <div class="space-y-8">
            // Executive Reporting Header
            <div class="text-center">
                <div class="flex items-center justify-center gap-3 mb-4">
                    <div class="w-12 h-12 bg-gradient-to-br from-primary via-accent to-secondary rounded-xl flex items-center justify-center shadow-lg">
                        <svg class="w-7 h-7 text-base-100" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                        </svg>
                    </div>
                </div>
                <h1 class="text-4xl font-bold text-base-content mb-3 bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
                    "Executive Reporting Suite"
                </h1>
                <p class="text-lg text-base-content/80 max-w-3xl mx-auto leading-relaxed">
                    "Comprehensive business intelligence and strategic reporting for wrestling empire management. 
                    Professional analytics with executive-level insights and export capabilities."
                </p>
            </div>
            
            // Report Configuration
            <div class="card bg-base-100 shadow-xl border border-base-300/50">
                <div class="card-body">
                    <h2 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <div class="w-8 h-8 bg-primary/20 rounded-lg flex items-center justify-center">
                            <svg class="w-5 h-5 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
                            </svg>
                        </div>
                        "Report Configuration"
                    </h2>
                    
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        // Report Type Selection
                        <div class="form-control w-full">
                            <label class="label">
                                <span class="label-text font-medium">"Report Type"</span>
                            </label>
                            <select 
                                class="select select-bordered select-primary"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_selected_report.set(value);
                                }
                            >
                                <option value="roster_performance" selected=move || selected_report.get() == "roster_performance">
                                    "🏆 Talent Performance Analysis"
                                </option>
                                <option value="show_analytics" selected=move || selected_report.get() == "show_analytics">
                                    "📺 Show Success Metrics"
                                </option>
                                <option value="championship_impact" selected=move || selected_report.get() == "championship_impact">
                                    "👑 Championship Impact Study"
                                </option>
                                <option value="strategic_planning" selected=move || selected_report.get() == "strategic_planning">
                                    "📊 Strategic Planning Intelligence"
                                </option>
                                <option value="competitive_analysis" selected=move || selected_report.get() == "competitive_analysis">
                                    "🎯 Competitive Analysis"
                                </option>
                            </select>
                        </div>
                        
                        // Report Period
                        <div class="form-control w-full">
                            <label class="label">
                                <span class="label-text font-medium">"Analysis Period"</span>
                            </label>
                            <select 
                                class="select select-bordered select-secondary"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_report_period.set(value);
                                }
                            >
                                <option value="weekly" selected=move || report_period.get() == "weekly">
                                    "📅 Weekly Performance"
                                </option>
                                <option value="monthly" selected=move || report_period.get() == "monthly">
                                    "📈 Monthly Trends"
                                </option>
                                <option value="quarterly" selected=move || report_period.get() == "quarterly">
                                    "📊 Quarterly Review"
                                </option>
                                <option value="annual" selected=move || report_period.get() == "annual">
                                    "🏢 Annual Strategic"
                                </option>
                            </select>
                        </div>
                        
                        // Export Format
                        <div class="form-control w-full">
                            <label class="label">
                                <span class="label-text font-medium">"Export Format"</span>
                            </label>
                            <select 
                                class="select select-bordered select-accent"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_export_format.set(value);
                                }
                            >
                                <option value="executive_summary" selected=move || export_format.get() == "executive_summary">
                                    "📋 Executive Summary"
                                </option>
                                <option value="detailed_report" selected=move || export_format.get() == "detailed_report">
                                    "📖 Detailed Analysis"
                                </option>
                                <option value="presentation" selected=move || export_format.get() == "presentation">
                                    "🖼️ Board Presentation"
                                </option>
                                <option value="data_export" selected=move || export_format.get() == "data_export">
                                    "📊 Raw Data Export"
                                </option>
                            </select>
                        </div>
                    </div>
                    
                    // Action Buttons
                    <div class="flex gap-4 mt-6">
                        <button 
                            class="btn btn-primary gap-2"
                            disabled=loading_report.get()
                            on:click=generate_report
                        >
                            {move || if loading_report.get() { 
                                view! { <span class="loading loading-spinner loading-sm"></span> }.into_any()
                            } else {
                                view! { 
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                    </svg>
                                }.into_any()
                            }}
                            "Generate Report"
                        </button>
                        
                        <button 
                            class="btn btn-secondary gap-2"
                            disabled=report_data.get().is_none()
                            on:click=export_report
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4-4m0 0L8 8m4-4v12"/>
                            </svg>
                            "Export Report"
                        </button>
                    </div>
                </div>
            </div>
            
            // Report Preview and Analytics Dashboard
            {move || {
                if loading_report.get() {
                    view! {
                        <div class="card bg-base-100 shadow-xl border border-base-300/50">
                            <div class="card-body">
                                <div class="flex items-center justify-center py-12">
                                    <div class="text-center">
                                        <span class="loading loading-spinner loading-lg text-primary"></span>
                                        <div class="mt-4 text-lg font-medium">"Generating Executive Report..."</div>
                                        <div class="text-base-content/70">"Analyzing business intelligence data"</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else if let Some(data) = report_data.get() {
                    view! {
                        <div class="space-y-6">
                            // Executive Summary Card
                            <div class="card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-xl">
                                <div class="card-body">
                                    <div class="flex items-center gap-3 mb-4">
                                        <div class="w-10 h-10 bg-primary/20 rounded-xl flex items-center justify-center">
                                            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4M7.835 4.697a3.42 3.42 0 001.946-.806 3.42 3.42 0 014.438 0 3.42 3.42 0 001.946.806 3.42 3.42 0 013.138 3.138 3.42 3.42 0 00.806 1.946 3.42 3.42 0 010 4.438 3.42 3.42 0 00-.806 1.946 3.42 3.42 0 01-3.138 3.138 3.42 3.42 0 00-1.946.806 3.42 3.42 0 01-4.438 0 3.42 3.42 0 00-1.946-.806 3.42 3.42 0 01-3.138-3.138 3.42 3.42 0 00-.806-1.946 3.42 3.42 0 010-4.438 3.42 3.42 0 00.806-1.946 3.42 3.42 0 013.138-3.138z"/>
                                            </svg>
                                        </div>
                                        <h3 class="text-2xl font-bold text-primary">"Executive Summary"</h3>
                                        <div class="badge badge-primary">{data.report_type.clone()}</div>
                                    </div>
                                    
                                    <div class="prose max-w-none">
                                        <p class="text-base-content/80 leading-relaxed mb-4">
                                            {data.executive_summary.clone()}
                                        </p>
                                    </div>
                                    
                                    // Key Metrics
                                    <div class="stats stats-horizontal shadow mt-4">
                                        <div class="stat">
                                            <div class="stat-figure text-primary">
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                                                </svg>
                                            </div>
                                            <div class="stat-title">"Performance Score"</div>
                                            <div class="stat-value text-primary">{data.performance_score}"%"</div>
                                            <div class="stat-desc">"Strategic KPI Achievement"</div>
                                        </div>
                                        
                                        <div class="stat">
                                            <div class="stat-figure text-secondary">
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                                </svg>
                                            </div>
                                            <div class="stat-title">"Growth Metrics"</div>
                                            <div class="stat-value text-secondary">"+"{data.growth_percentage}"%"</div>
                                            <div class="stat-desc">"Period over Period"</div>
                                        </div>
                                        
                                        <div class="stat">
                                            <div class="stat-figure text-accent">
                                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                                                </svg>
                                            </div>
                                            <div class="stat-title">"Strategic Impact"</div>
                                            <div class="stat-value text-accent">{data.strategic_impact}</div>
                                            <div class="stat-desc">"Executive Confidence"</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            // Detailed Analysis
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                                // Key Findings
                                <div class="card bg-base-100 shadow-xl border border-base-300/50">
                                    <div class="card-body">
                                        <h4 class="text-xl font-bold mb-4 flex items-center gap-2">
                                            <svg class="w-6 h-6 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                            </svg>
                                            "Key Findings"
                                        </h4>
                                        <div class="space-y-3">
                                            <For
                                                each=move || data.key_findings.clone()
                                                key=|finding| finding.clone()
                                                children=move |finding| {
                                                    view! {
                                                        <div class="alert alert-success alert-sm">
                                                            <svg class="stroke-current shrink-0 w-4 h-4" fill="none" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                                            </svg>
                                                            <span class="text-sm">{finding}</span>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </div>
                                
                                // Recommendations
                                <div class="card bg-base-100 shadow-xl border border-base-300/50">
                                    <div class="card-body">
                                        <h4 class="text-xl font-bold mb-4 flex items-center gap-2">
                                            <svg class="w-6 h-6 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.99-.833-2.5 0L3.314 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                            </svg>
                                            "Strategic Recommendations"
                                        </h4>
                                        <div class="space-y-3">
                                            <For
                                                each=move || data.recommendations.clone()
                                                key=|rec| rec.clone()
                                                children=move |recommendation| {
                                                    view! {
                                                        <div class="alert alert-warning alert-sm">
                                                            <svg class="stroke-current shrink-0 w-4 h-4" fill="none" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.99-.833-2.5 0L3.314 16.5c-.77.833.192 2.5 1.732 2.5z"/>
                                                            </svg>
                                                            <span class="text-sm">{recommendation}</span>
                                                        </div>
                                                    }
                                                }
                                            />
                                        </div>
                                    </div>
                                </div>
                            </div>
                            
                            // Report Actions
                            <div class="card bg-gradient-to-br from-base-100 to-base-200/50 shadow-xl border border-base-300/50">
                                <div class="card-body">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <h4 class="text-lg font-bold">"Report Actions"</h4>
                                            <p class="text-base-content/70">"Professional export and sharing options"</p>
                                        </div>
                                        <div class="flex gap-2">
                                            <button class="btn btn-sm btn-primary gap-2">
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 17h2a2 2 0 002-2v-4a2 2 0 00-2-2H9.5a2 2 0 01-2-2V5a2 2 0 00-2-2H3"/>
                                                </svg>
                                                "Print Report"
                                            </button>
                                            <button class="btn btn-sm btn-secondary gap-2">
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.367 2.684 3 3 0 00-5.367-2.684z"/>
                                                </svg>
                                                "Share"
                                            </button>
                                            <button class="btn btn-sm btn-accent gap-2">
                                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                </svg>
                                                "Archive"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="card bg-base-100 shadow-xl border border-base-300/50">
                            <div class="card-body">
                                <div class="text-center py-12">
                                    <div class="w-16 h-16 bg-base-200 rounded-xl flex items-center justify-center mx-auto mb-4">
                                        <svg class="w-8 h-8 text-base-content/50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                                        </svg>
                                    </div>
                                    <h3 class="text-xl font-bold mb-2">"Ready to Generate Report"</h3>
                                    <p class="text-base-content/70 mb-4">
                                        "Configure your report settings above and click 'Generate Report' to create comprehensive business intelligence."
                                    </p>
                                    <div class="stats shadow">
                                        <div class="stat">
                                            <div class="stat-title">"Report Types Available"</div>
                                            <div class="stat-value">5</div>
                                            <div class="stat-desc">"Professional Analytics"</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }
            }}
            
            // Navigation
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

// Report data structure
#[derive(Clone, Debug)]
struct ReportData {
    report_type: String,
    executive_summary: String,
    performance_score: u32,
    growth_percentage: i32,
    strategic_impact: String,
    key_findings: Vec<String>,
    recommendations: Vec<String>,
}

#[derive(Clone, Debug)]
struct MockAnalyticsData {
    // Mock structure for demonstration
}

// Helper functions
async fn load_analytics_data(set_analytics_data: WriteSignal<Option<MockAnalyticsData>>) {
    // Mock analytics data for demonstration
    let mock_data = MockAnalyticsData {};
    set_analytics_data.set(Some(mock_data));
}

async fn generate_executive_report(report_type: String, period: String) -> Result<ReportData, String> {
    // Mock report generation - in real implementation, this would call backend
    Ok(ReportData {
        report_type: report_type.clone(),
        executive_summary: format!(
            "Executive analysis of {} performance over {} period shows strong strategic positioning with identified growth opportunities. Key performance indicators demonstrate positive trajectory with specific areas for executive attention and resource allocation.",
            report_type.replace("_", " "), period
        ),
        performance_score: 87,
        growth_percentage: 23,
        strategic_impact: "High".to_string(),
        key_findings: vec![
            "Talent engagement metrics show 15% improvement".to_string(),
            "Championship storylines driving 28% increased audience retention".to_string(),
            "Cross-show talent mobility optimizing roster utilization".to_string(),
            "Performance analytics indicate strategic booking success".to_string(),
        ],
        recommendations: vec![
            "Expand high-performing talent development programs".to_string(),
            "Increase investment in championship prestige initiatives".to_string(),
            "Implement strategic roster rotation for optimal engagement".to_string(),
            "Deploy advanced analytics for real-time strategic decisions".to_string(),
        ],
    })
}

async fn export_executive_report(format: String) -> Result<(), String> {
    // Mock export function - would handle actual file export
    web_sys::console::log_1(&format!("Exporting report in {} format", format).into());
    Ok(())
}