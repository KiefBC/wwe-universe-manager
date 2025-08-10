use leptos::prelude::*;
use crate::components::*;

/// Simplified Executive Design System Showcase
#[component]
pub fn ExecutiveShowcaseSimple(
    /// Navigation callback for routing
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    
    view! {
        <ExecutivePageLayout
            title="Executive Design System".to_string()
            subtitle=Some("Professional component showcase".to_string())
        >
            // Cards Showcase
            <ExecutiveContentSection
                title="Executive Card System".to_string()
                description="Professional card variants with theme-aware styling".to_string()
                variant="primary".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <div class="card bg-gradient-to-br from-primary/10 to-primary/5 border border-primary/20 shadow-professional hover:shadow-executive transition-all duration-200">
                        <div class="card-body p-4">
                            <h3 class="text-lg font-bold text-primary mb-2">Executive Theme</h3>
                            <p class="text-base-content/70 text-sm">
                                Professional styling with theme-aware gradients and shadows
                            </p>
                            <div class="badge badge-primary badge-sm mt-2">WWE Executive</div>
                        </div>
                    </div>
                    
                    <div class="card bg-gradient-to-br from-accent/10 to-accent/5 border border-accent/20 shadow-professional hover:shadow-executive transition-all duration-200">
                        <div class="card-body p-4">
                            <h3 class="text-lg font-bold text-accent mb-2">Premium Gradient</h3>
                            <p class="text-base-content/70 text-sm">
                                Multi-color gradients with professional presentation
                            </p>
                            <div class="badge badge-accent badge-sm mt-2">Premium</div>
                        </div>
                    </div>
                    
                    <div class="stats shadow bg-gradient-to-br from-success/10 to-success/5 border border-success/20">
                        <div class="stat">
                            <div class="stat-title text-success/80">Active Users</div>
                            <div class="stat-value text-2xl text-success">142</div>
                            <div class="stat-desc text-success/60">"↗ +12%"</div>
                        </div>
                    </div>
                </div>
            </ExecutiveContentSection>

            // Buttons Showcase  
            <ExecutiveContentSection
                title="Executive Button System".to_string()
                description="Professional button variants with consistent styling".to_string()
                variant="secondary".to_string()
            >
                <div class="flex flex-wrap gap-4">
                    <button class="btn btn-primary shadow-executive hover:shadow-premium">
                        "Executive Primary"
                    </button>
                    <button class="btn btn-secondary shadow-professional hover:shadow-executive">
                        "Professional"
                    </button>
                    <button class="btn bg-gradient-to-r from-primary to-accent border-none text-primary-content shadow-executive hover:shadow-premium">
                        "Premium Gradient"
                    </button>
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