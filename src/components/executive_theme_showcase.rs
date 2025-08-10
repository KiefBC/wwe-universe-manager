use leptos::prelude::*;
use crate::components::*;

/// Executive Theme System Showcase
/// 
/// Comprehensive demonstration of Phase 1.2 theme integration:
/// - All 4 theme variants working seamlessly
/// - Standardized gradient system
/// - Professional loading states and animations
/// - Theme-aware components and micro-interactions
/// - Consistent design system across all themes
#[component]
pub fn ExecutiveThemeShowcase(
    /// Navigation callback for routing
    set_current_page: WriteSignal<String>,
) -> impl IntoView {
    
    let (demo_loading, set_demo_loading) = signal(false);
    
    // Trigger demo loading state
    let trigger_loading = move |_| {
        set_demo_loading.set(true);
        set_timeout(move || {
            set_demo_loading.set(false);
        }, std::time::Duration::from_millis(2000));
    };
    
    view! {
        <ThemeTransitionManager transitioning={demo_loading.get()}>
            <ExecutivePageLayout
                title="Executive Theme System".to_string()
                subtitle=Some("Professional theme integration and design system".to_string())
            >
                // Theme Variants Showcase
                <ThemeGradientWrapper variant="executive".to_string() shadow=true>
                    <ExecutiveContentSection
                        title="Theme Variants".to_string()
                        description="All four professional themes with consistent component behavior".to_string()
                    >
                        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-6">
                            <div class="card bg-gradient-wwe border border-primary/20 shadow-professional hover:shadow-executive transition-all duration-200">
                                <div class="card-body p-4">
                                    <h3 class="font-bold text-primary mb-2">"WWE Executive"</h3>
                                    <p class="text-base-content/70 text-sm mb-3">"Premium gold & dark slate"</p>
                                    <ThemeStatusIndicator status="live".to_string() label="Active".to_string() animate=true />
                                </div>
                            </div>
                            
                            <div class="card bg-gradient-aew border border-info/20 shadow-professional hover:shadow-executive transition-all duration-200">
                                <div class="card-body p-4">
                                    <h3 class="font-bold text-info mb-2">"AEW Modern"</h3>
                                    <p class="text-base-content/70 text-sm mb-3">"Tech cyan & modern dark"</p>
                                    <ThemeStatusIndicator status="active".to_string() label="Ready".to_string() />
                                </div>
                            </div>
                            
                            <div class="card bg-gradient-njpw border border-secondary/20 shadow-professional hover:shadow-executive transition-all duration-200">
                                <div class="card-body p-4">
                                    <h3 class="font-bold text-secondary mb-2">"NJPW Premium"</h3>
                                    <p class="text-base-content/70 text-sm mb-3">"Royal purple & luxury dark"</p>
                                    <ThemeStatusIndicator status="pending".to_string() label="Preview".to_string() />
                                </div>
                            </div>
                            
                            <div class="card bg-gradient-corporate border border-accent/20 shadow-professional hover:shadow-executive transition-all duration-200">
                                <div class="card-body p-4">
                                    <h3 class="font-bold text-accent mb-2">"Corporate Dark"</h3>
                                    <p class="text-base-content/70 text-sm mb-3">"Professional blue & business dark"</p>
                                    <ThemeStatusIndicator status="success".to_string() label="Stable".to_string() />
                                </div>
                            </div>
                        </div>
                    </ExecutiveContentSection>
                </ThemeGradientWrapper>

                // Professional Animation System
                <ThemeGradientWrapper variant="premium".to_string()>
                    <ExecutiveContentSection
                        title="Professional Animation System".to_string()
                        description="Subtle, executive-level micro-interactions and loading states".to_string()
                    >
                        <div class="space-y-6">
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                <ProfessionalAnimation animation="fade".to_string() timing="smooth".to_string()>
                                    <div class="card bg-base-100 shadow-professional hover:shadow-executive p-4">
                                        <h4 class="font-semibold text-primary mb-2">"Fade Animation"</h4>
                                        <p class="text-sm text-base-content/70">"Smooth fade-in with executive timing"</p>
                                    </div>
                                </ProfessionalAnimation>
                                
                                <ProfessionalAnimation animation="slide".to_string()>
                                    <div class="card bg-base-100 shadow-professional hover:shadow-executive p-4">
                                        <h4 class="font-semibold text-accent mb-2">"Slide Animation"</h4>
                                        <p class="text-sm text-base-content/70">"Professional slide-in effect"</p>
                                    </div>
                                </ProfessionalAnimation>
                                
                                <ProfessionalAnimation animation="hover".to_string()>
                                    <div class="card bg-base-100 shadow-professional hover:shadow-executive p-4 cursor-pointer">
                                        <h4 class="font-semibold text-secondary mb-2">"Hover Effects"</h4>
                                        <p class="text-sm text-base-content/70">"Sophisticated hover interactions"</p>
                                    </div>
                                </ProfessionalAnimation>
                            </div>
                            
                            <div class="flex justify-center">
                                <button 
                                    class="btn btn-primary shadow-executive hover:shadow-premium"
                                    on:click=trigger_loading
                                >
                                    "Demonstrate Loading States"
                                </button>
                            </div>
                        </div>
                    </ExecutiveContentSection>
                </ThemeGradientWrapper>

                // Gradient System Showcase
                <ThemeGradientWrapper variant="professional".to_string()>
                    <ExecutiveContentSection
                        title="Standardized Gradient System".to_string()
                        description="Consistent gradient patterns that adapt to all theme variants".to_string()
                    >
                        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4">
                            <div class="p-6 rounded-lg bg-gradient-to-br from-primary/15 to-primary/5 border border-primary/20">
                                <div class="text-center">
                                    <div class="font-semibold text-primary mb-1">"Primary"</div>
                                    <div class="text-xs text-primary/70">"Brand gradient"</div>
                                </div>
                            </div>
                            
                            <div class="p-6 rounded-lg bg-gradient-to-br from-accent/15 to-accent/5 border border-accent/20">
                                <div class="text-center">
                                    <div class="font-semibold text-accent mb-1">"Accent"</div>
                                    <div class="text-xs text-accent/70">"Action gradient"</div>
                                </div>
                            </div>
                            
                            <div class="p-6 rounded-lg bg-gradient-to-br from-secondary/15 to-secondary/5 border border-secondary/20">
                                <div class="text-center">
                                    <div class="font-semibold text-secondary mb-1">"Secondary"</div>
                                    <div class="text-xs text-secondary/70">"Support gradient"</div>
                                </div>
                            </div>
                            
                            <div class="p-6 rounded-lg bg-gradient-to-br from-success/15 to-success/5 border border-success/20">
                                <div class="text-center">
                                    <div class="font-semibold text-success mb-1">"Success"</div>
                                    <div class="text-xs text-success/70">"Positive gradient"</div>
                                </div>
                            </div>
                            
                            <div class="p-6 rounded-lg bg-gradient-executive-premium border border-primary/20">
                                <div class="text-center">
                                    <div class="font-semibold text-primary mb-1">"Premium"</div>
                                    <div class="text-xs text-primary/70">"Executive gradient"</div>
                                </div>
                            </div>
                        </div>
                    </ExecutiveContentSection>
                </ThemeGradientWrapper>

                // Loading States Demo
                <ThemeEnhancedContainer 
                    loading={demo_loading.get()}
                    loading_message="Demonstrating professional loading states...".to_string()
                    enhancement_level="professional".to_string()
                >
                    <ThemeGradientWrapper variant="subtle".to_string()>
                        <ExecutiveContentSection
                            title="Professional Loading States".to_string()
                            description="Theme-aware loading indicators with executive-level presentation".to_string()
                        >
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                                <div class="text-center p-6">
                                    <ThemeAwareLoader size="sm".to_string() message="Small Loader".to_string() />
                                </div>
                                <div class="text-center p-6">
                                    <ThemeAwareLoader size="normal".to_string() message="Standard Loader".to_string() />
                                </div>
                                <div class="text-center p-6">
                                    <ThemeAwareLoader size="lg".to_string() message="Large Loader".to_string() />
                                </div>
                            </div>
                        </ExecutiveContentSection>
                    </ThemeGradientWrapper>
                </ThemeEnhancedContainer>

                // Theme Integration Summary
                <ThemeGradientWrapper variant="accent".to_string()>
                    <ExecutiveContentSection
                        title="Phase 1.2 Implementation Complete".to_string()
                        description="Professional theme integration with standardized components".to_string()
                    >
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                            <div class="space-y-4">
                                <h4 class="text-lg font-semibold text-base-content">"✓ Completed Features"</h4>
                                <ul class="space-y-2 text-sm text-base-content/80">
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="success".to_string() size="sm".to_string() />
                                        "All 4 theme variants integrated"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="success".to_string() size="sm".to_string() />
                                        "Standardized gradient system"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="success".to_string() size="sm".to_string() />
                                        "Professional animations & transitions"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="success".to_string() size="sm".to_string() />
                                        "Theme-aware loading states"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="success".to_string() size="sm".to_string() />
                                        "Executive-level micro-interactions"
                                    </li>
                                </ul>
                            </div>
                            
                            <div class="space-y-4">
                                <h4 class="text-lg font-semibold text-base-content">"🎯 Quality Standards Met"</h4>
                                <ul class="space-y-2 text-sm text-base-content/80">
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="live".to_string() size="sm".to_string() />
                                        "CEO Dashboard quality consistency"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="active".to_string() size="sm".to_string() />
                                        "Professional shadow system"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="active".to_string() size="sm".to_string() />
                                        "Smooth theme switching"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="active".to_string() size="sm".to_string() />
                                        "Enhanced Tailwind configuration"
                                    </li>
                                    <li class="flex items-center gap-2">
                                        <ThemeStatusIndicator status="active".to_string() size="sm".to_string() />
                                        "Consistent design patterns"
                                    </li>
                                </ul>
                            </div>
                        </div>
                    </ExecutiveContentSection>
                </ThemeGradientWrapper>

                // Back Button
                <ExecutiveBackButton
                    text="Return to Executive Dashboard".to_string()
                    route="promotion-dashboard".to_string()
                    on_navigate=Callback::new(move |route: String| {
                        set_current_page.set(route);
                    })
                />
            </ExecutivePageLayout>
        </ThemeTransitionManager>
    }
}