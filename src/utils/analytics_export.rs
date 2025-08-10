use crate::types::AnalyticsData;
use wasm_bindgen::JsCast;

/// Export analytics report in HTML format
pub fn export_report(report_type: &str, data: &Option<AnalyticsData>) {
    let Some(analytics) = data else {
        web_sys::console::warn_1(&"No analytics data available for export".into());
        return;
    };

    let report_content = match report_type {
        "talent" => generate_talent_report(analytics),
        "championships" => generate_championships_report(analytics),
        "shows" => generate_shows_report(analytics),
        "strategic" => generate_strategic_report(analytics),
        _ => "Unknown report type".to_string(),
    };

    download_html_report(&format!("{}_analytics_report.html", report_type), &report_content);
}

/// Export analytics data as CSV
pub fn export_csv(data: &Option<AnalyticsData>) {
    let Some(analytics) = data else {
        web_sys::console::warn_1(&"No analytics data available for CSV export".into());
        return;
    };

    let mut csv_content = String::new();
    
    // Wrestler performance CSV
    csv_content.push_str("Wrestler Performance Data\n");
    csv_content.push_str("Name,Wins,Losses,Total Matches,Win Rate,Championship Count,Shows Assigned\n");
    
    for wrestler in &analytics.top_wrestlers {
        csv_content.push_str(&format!(
            "{},{},{},{},{:.1}%,{},{}\n",
            wrestler.name,
            wrestler.wins,
            wrestler.losses,
            wrestler.total_matches,
            wrestler.win_rate,
            wrestler.championship_count,
            wrestler.shows_assigned
        ));
    }
    
    csv_content.push_str("\nShow Performance Data\n");
    csv_content.push_str("Show Name,Roster Size,Total Matches,Title Matches,Avg Match Quality\n");
    
    for show in &analytics.show_analytics {
        csv_content.push_str(&format!(
            "{},{},{},{},{:.1}\n",
            show.show_name,
            show.roster_size,
            show.total_matches,
            show.title_matches,
            show.avg_match_quality
        ));
    }

    download_file("analytics_data.csv", &csv_content, "text/csv");
}

/// Export analytics data as JSON
pub fn export_json(data: &Option<AnalyticsData>) {
    let Some(analytics) = data else {
        web_sys::console::warn_1(&"No analytics data available for JSON export".into());
        return;
    };

    match serde_json::to_string_pretty(analytics) {
        Ok(json_content) => {
            download_file("analytics_data.json", &json_content, "application/json");
        },
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to serialize analytics data: {}", e).into());
        }
    }
}

/// Generate talent performance report HTML
fn generate_talent_report(analytics: &AnalyticsData) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Talent Performance Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #d4af37, #ffd700); padding: 30px; border-radius: 10px; color: white; }}
        .metric-card {{ background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .wrestler-table {{ width: 100%; border-collapse: collapse; }}
        .wrestler-table th, .wrestler-table td {{ padding: 12px; border: 1px solid #ddd; text-align: left; }}
        .wrestler-table th {{ background: #f8f9fa; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>WWE Universe Manager - Talent Performance Report</h1>
        <p>Executive Business Intelligence Report</p>
        <small>Generated on {}</small>
    </div>
    
    <div class="metric-card">
        <h2>Key Performance Metrics</h2>
        <p><strong>Total Active Wrestlers:</strong> {}</p>
        <p><strong>Top Performer Win Rate:</strong> {:.1}%</p>
        <p><strong>Championship Contenders:</strong> {}</p>
        <p><strong>Roster Utilization Rate:</strong> {:.1}%</p>
    </div>
    
    <div class="metric-card">
        <h2>Top Performing Wrestlers</h2>
        <table class="wrestler-table">
            <tr>
                <th>Rank</th>
                <th>Name</th>
                <th>Wins</th>
                <th>Losses</th>
                <th>Win Rate</th>
                <th>Championships</th>
            </tr>
            {}
        </table>
    </div>
</body>
</html>"#,
        js_sys::Date::new_0().to_string().as_string().unwrap_or_default(),
        analytics.total_active_wrestlers,
        analytics.top_wrestler_win_rate,
        analytics.championship_contenders,
        analytics.roster_utilization_rate,
        analytics.top_wrestlers.iter().take(10).enumerate().map(|(i, w)| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td><td>{}</td></tr>",
                i + 1, w.name, w.wins, w.losses, w.win_rate, w.championship_count
            )
        }).collect::<Vec<_>>().join("")
    )
}

/// Generate championships report HTML  
fn generate_championships_report(analytics: &AnalyticsData) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Championship Analytics Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #d4af37, #ffd700); padding: 30px; border-radius: 10px; color: white; }}
        .metric-card {{ background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .title-table {{ width: 100%; border-collapse: collapse; }}
        .title-table th, .title-table td {{ padding: 12px; border: 1px solid #ddd; text-align: left; }}
        .title-table th {{ background: #f8f9fa; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Championship Analytics Report</h1>
        <p>Title Portfolio and Prestige Analysis</p>
        <small>Generated on {}</small>
    </div>
    
    <div class="metric-card">
        <h2>Championship Overview</h2>
        <p><strong>Total Championships:</strong> {}</p>
        <p><strong>Active Champions:</strong> {}</p>
        <p><strong>Vacant Titles:</strong> {}</p>
        <p><strong>Average Prestige Tier:</strong> {:.1}</p>
    </div>
    
    <div class="metric-card">
        <h2>Title Analysis</h2>
        <table class="title-table">
            <tr>
                <th>Title</th>
                <th>Prestige Tier</th>
                <th>Status</th>
                <th>Show Assignment</th>
            </tr>
            {}
        </table>
    </div>
</body>
</html>"#,
        js_sys::Date::new_0().to_string().as_string().unwrap_or_default(),
        analytics.total_championships,
        analytics.active_title_holders,
        analytics.vacant_titles,
        analytics.average_prestige_tier,
        analytics.title_analytics.iter().map(|t| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                t.title_name,
                t.prestige_tier,
                if t.has_holder { "HELD" } else { "VACANT" },
                t.show_assignment.as_deref().unwrap_or("Not Assigned")
            )
        }).collect::<Vec<_>>().join("")
    )
}

/// Generate shows report HTML
fn generate_shows_report(analytics: &AnalyticsData) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Show Performance Analysis</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #d4af37, #ffd700); padding: 30px; border-radius: 10px; color: white; }}
        .metric-card {{ background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .show-table {{ width: 100%; border-collapse: collapse; }}
        .show-table th, .show-table td {{ padding: 12px; border: 1px solid #ddd; text-align: left; }}
        .show-table th {{ background: #f8f9fa; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Show Performance Analysis</h1>
        <p>Content Production and Booking Effectiveness</p>
        <small>Generated on {}</small>
    </div>
    
    <div class="metric-card">
        <h2>Show Portfolio Overview</h2>
        <p><strong>Total Shows:</strong> {}</p>
        <p><strong>Total Matches Booked:</strong> {}</p>
        <p><strong>Average Matches per Show:</strong> {:.1}</p>
        <p><strong>Title Match Percentage:</strong> {}%</p>
    </div>
    
    <div class="metric-card">
        <h2>Show Performance Analysis</h2>
        <table class="show-table">
            <tr>
                <th>Show</th>
                <th>Roster Size</th>
                <th>Total Matches</th>
                <th>Title Matches</th>
                <th>Content Quality</th>
            </tr>
            {}
        </table>
    </div>
</body>
</html>"#,
        js_sys::Date::new_0().to_string().as_string().unwrap_or_default(),
        analytics.total_shows,
        analytics.total_matches,
        analytics.avg_matches_per_show,
        analytics.title_matches_percentage,
        analytics.show_analytics.iter().map(|s| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.1}</td></tr>",
                s.show_name,
                s.roster_size,
                s.total_matches,
                s.title_matches,
                s.avg_match_quality
            )
        }).collect::<Vec<_>>().join("")
    )
}

/// Generate strategic planning report HTML
fn generate_strategic_report(analytics: &AnalyticsData) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Strategic Planning Dashboard</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }}
        .header {{ background: linear-gradient(135deg, #d4af37, #ffd700); padding: 30px; border-radius: 10px; color: white; }}
        .metric-card {{ background: white; padding: 20px; margin: 20px 0; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .insight {{ background: #e7f3ff; padding: 15px; margin: 10px 0; border-left: 4px solid #2196F3; border-radius: 4px; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Strategic Planning Dashboard</h1>
        <p>Business Intelligence and Growth Opportunities</p>
        <small>Generated on {}</small>
    </div>
    
    <div class="metric-card">
        <h2>Strategic Overview</h2>
        <p><strong>Business Health Score:</strong> {:.0}/100</p>
        <p><strong>Growth Opportunities:</strong> {}</p>
        <p><strong>Strategic Priorities:</strong> {}</p>
    </div>
    
    <div class="metric-card">
        <h2>Performance Tier Distribution</h2>
        <p><strong>Elite Performers (80%+ Win Rate):</strong> {} wrestlers ({:.1}%)</p>
        <p><strong>Strong Performers (60-79% Win Rate):</strong> {} wrestlers ({:.1}%)</p>
        <p><strong>Average Performers (40-59% Win Rate):</strong> {} wrestlers ({:.1}%)</p>
        <p><strong>Developing Talent (<40% Win Rate):</strong> {} wrestlers ({:.1}%)</p>
    </div>
    
    <div class="metric-card">
        <h2>Strategic Insights</h2>
        <div class="insight">
            <strong>Talent Development:</strong> Focus on developing {} wrestlers currently in the developing tier to improve overall roster strength.
        </div>
        <div class="insight">
            <strong>Content Strategy:</strong> With {} total matches across {} shows, consider expanding match booking to maximize talent exposure.
        </div>
        <div class="insight">
            <strong>Championship Strategy:</strong> {} vacant titles present opportunities for compelling storylines and title tournaments.
        </div>
    </div>
</body>
</html>"#,
        js_sys::Date::new_0().to_string().as_string().unwrap_or_default(),
        analytics.business_health_score,
        analytics.growth_opportunities,
        analytics.strategic_priorities,
        analytics.performance_tiers.elite_count,
        analytics.performance_tiers.elite_percentage,
        analytics.performance_tiers.strong_count,
        analytics.performance_tiers.strong_percentage,
        analytics.performance_tiers.average_count,
        analytics.performance_tiers.average_percentage,
        analytics.performance_tiers.developing_count,
        analytics.performance_tiers.developing_percentage,
        analytics.performance_tiers.developing_count,
        analytics.total_matches,
        analytics.total_shows,
        analytics.vacant_titles
    )
}

/// Download HTML report file
fn download_html_report(filename: &str, content: &str) {
    download_file(filename, content, "text/html");
}

/// Generic file download function  
fn download_file(filename: &str, content: &str, _mime_type: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    
    // Create blob
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(content));
    
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&array);
    
    let blob = web_sys::Blob::new_with_str_sequence(&blob_parts)
        .expect("Failed to create blob");
    
    // Create download link
    let url = web_sys::Url::create_object_url_with_blob(&blob)
        .expect("Failed to create object URL");
    
    let link = document.create_element("a").expect("Failed to create link element");
    let html_link: web_sys::HtmlAnchorElement = link.dyn_into().expect("Failed to cast to HtmlAnchorElement");
    
    html_link.set_href(&url);
    html_link.set_download(filename);
    html_link.set_attribute("style", "display: none;").expect("Failed to set style");
    
    document.body().expect("should have body").append_child(&html_link).expect("Failed to append link");
    
    // Trigger download
    html_link.click();
    
    // Cleanup
    document.body().expect("should have body").remove_child(&html_link).expect("Failed to remove link");
    web_sys::Url::revoke_object_url(&url).expect("Failed to revoke object URL");
}