use leptos::prelude::*;
use crate::components::*;

/// Executive Design System Showcase - Simplified Version
/// 
/// Demonstration component showcasing Phase 1 executive design system components
#[component]
pub fn ExecutiveShowcase(
    /// Navigation callback for routing
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    
    let (demo_toggle, set_demo_toggle) = signal(false);
    
    view! {
        <ExecutivePageLayout
            title="Executive Design System Showcase".to_string()
            subtitle=Some("Professional component library demonstration".to_string())
        >
            // Typography Showcase
            <ExecutiveContentSection
                title="Executive Typography System".to_string()
                description="Professional heading hierarchy and text components".to_string()
                variant="primary".to_string()
            >
                <div class="space-y-6">
                    <ExecutiveHeading
                        level=1
                        text="Executive Level 1 Heading".to_string()
                        gradient=true
                        align="center".to_string()
                    />
                    
                    <ExecutiveHeading
                        level=2
                        text="Professional Level 2 Heading".to_string()
                        variant="primary".to_string()
                    />
                    
                    <ExecutiveText 
                        text="Professional body text with proper spacing and readability optimized for executive interfaces".to_string()
                        variant="body".to_string()
                        leading="relaxed".to_string()
                    />
                </div>
            </ExecutiveContentSection>

            // Cards Showcase
            <ExecutiveContentSection
                title="Executive Card System".to_string()
                description="Professional card variants with theme-aware styling".to_string()
                variant="secondary".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <ExecutiveCard
                        variant="primary".to_string()
                        title="Primary Card".to_string()
                        subtitle="Executive styling with professional presentation".to_string()
                        badge="EXECUTIVE".to_string()
                        badge_variant="primary".to_string()
                    >
                        <ExecutiveText 
                            text="Professional content with executive-level presentation".to_string()
                            color="muted".to_string()
                        />
                    </ExecutiveCard>
                    
                    <ExecutiveCard
                        variant="gradient".to_string()
                        title="Premium Gradient".to_string()
                        subtitle="Multi-color gradient with professional shadows".to_string()
                        badge="PREMIUM".to_string()
                        badge_variant="accent".to_string()
                        clickable=true
                    >
                        <ExecutiveText 
                            text="Interactive premium card with gradient background".to_string()
                            color="muted".to_string()
                        />
                    </ExecutiveCard>
                    
                    <div class="stats shadow">
                        <ExecutiveMetricCard
                            title="Active Users".to_string()
                            value="142".to_string()
                            description="Professional metrics display".to_string()
                            color="primary".to_string()
                            show_trend=true
                            trend="up".to_string()
                        />
                    </div>
                </div>
            </ExecutiveContentSection>

            // Button Showcase
            <ExecutiveContentSection
                title="Executive Button System".to_string()
                description="Professional button variants with consistent styling".to_string()
                variant="info".to_string()
            >
                <div class="space-y-6">
                    <ExecutiveButtonGroup alignment="start".to_string() spacing="executive".to_string()>
                        <ExecutiveButton
                            text="Executive Primary".to_string()
                            variant="executive".to_string()
                            size="executive".to_string()
                        />
                        
                        <ExecutiveButton
                            text="Professional".to_string()
                            variant="professional".to_string()
                            size="lg".to_string()
                        />
                        
                        <ExecutiveButton
                            text="Premium".to_string()
                            variant="premium".to_string()
                            size="lg".to_string()
                        />
                    </ExecutiveButtonGroup>
                    
                    <ExecutiveActionButton
                        title="Strategic Planning Center".to_string()
                        description="Access comprehensive strategic planning tools".to_string()
                        icon=view! {
                            <svg class="w-8 h-8 text-current" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                            </svg>
                        }.into_any()
                        on_click=Callback::new(move |_| {
                            web_sys::console::log_1(&"Strategic planning accessed!".into());
                        })
                        variant="executive".to_string()
                        badge="EXECUTIVE".to_string()
                    />
                    
                    <ExecutiveToggleButton
                        label="Advanced Analytics".to_string()
                        description="Enable executive-level business intelligence".to_string()
                        checked=demo_toggle
                        set_checked=set_demo_toggle
                        size="lg".to_string()
                    />
                </div>
            </ExecutiveContentSection>

            // Interactive Gallery
            <ExecutiveContentSection
                title="Interactive Component Gallery".to_string()
                description="Live demonstration of professional interactions".to_string()
                variant="success".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <ExecutiveActionCard
                        title="Talent Analytics".to_string()
                        description="Advanced wrestler performance metrics".to_string()
                        category="ANALYTICS".to_string()
                        category_color="info".to_string()
                        icon=view! {
                            <svg class="w-6 h-6 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                            </svg>
                        }.into_any()
                        on_click=Callback::new(move |_| {
                            set_current_page.set("wrestlers".to_string());
                        })
                        status="LIVE".to_string()
                    />
                    
                    <ExecutiveActionCard
                        title="Championships".to_string()
                        description="Title lineage and prestige analytics".to_string()
                        category="PRESTIGE".to_string()
                        category_color="accent".to_string()
                        icon=view! {
                            <svg class="w-6 h-6 text-accent" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"/>
                            </svg>
                        }.into_any()
                        on_click=Callback::new(move |_| {
                            set_current_page.set("titles".to_string());
                        })
                    />
                    
                    <ExecutiveActionCard
                        title="Creative Direction".to_string()
                        description="Strategic match booking and storylines".to_string()
                        category="CREATIVE".to_string()
                        category_color="warning".to_string()
                        icon=view! {
                            <svg class="w-6 h-6 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                            </svg>
                        }.into_any()
                        on_click=Callback::new(move |_| {
                            set_current_page.set("booker".to_string());
                        })
                    />
                    
                    <ExecutiveActionCard
                        title="Business Intelligence".to_string()
                        description="Executive reporting and strategic planning".to_string()
                        category="EXECUTIVE".to_string()
                        category_color="primary".to_string()
                        icon=view! {
                            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                        }.into_any()
                        on_click=Callback::new(move |_| {
                            set_current_page.set("analytics".to_string());
                        })
                    />
                </div>
            </ExecutiveContentSection>

            // Back Button
            <ExecutiveBackButton
                text="Return to Executive Dashboard".to_string()
                route="promotion-dashboard".to_string()
                on_navigate=Callback::new(move |route: String| {
                    set_current_page.set(route);
                })
            />
        </ExecutivePageLayout>
    }
}