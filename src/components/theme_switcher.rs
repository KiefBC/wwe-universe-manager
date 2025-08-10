use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = localStorage)]
    fn setItem(key: &str, value: &str);
    
    #[wasm_bindgen(js_namespace = localStorage)]
    fn getItem(key: &str) -> Option<String>;
}

/// Professional theme switcher component with localStorage persistence
/// 
/// Provides seamless switching between enterprise-grade themes:
/// - WWE Executive (gold/dark slate)
/// - AEW Modern (cyan/tech dark) 
/// - NJPW Premium (purple/luxury dark)
/// - Corporate Dark (blue/business dark)
/// 
/// Features:
/// - Theme persistence across sessions
/// - Smooth transitions between themes
/// - Executive-level branding consistency
#[component]
pub fn ThemeSwitcher() -> impl IntoView {
    // Current theme state with localStorage persistence
    let (current_theme, set_current_theme) = signal("wwe-executive".to_string());
    
    // Load saved theme from localStorage on component mount
    Effect::new(move |_| {
        if let Some(saved_theme) = getItem("app-theme") {
            set_current_theme.set(saved_theme);
        }
    });
    
    // Update data-theme attribute when theme changes
    Effect::new(move |_| {
        let theme = current_theme.get();
        
        // Update the document root data-theme attribute
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(html_element) = document.document_element() {
                let _ = html_element.set_attribute("data-theme", &theme);
            }
        }
        
        // Also update any existing theme containers
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            if let Some(theme_container) = document.query_selector("[data-theme]").ok().flatten() {
                let _ = theme_container.set_attribute("data-theme", &theme);
            }
        }
        
        // Save to localStorage for persistence
        setItem("app-theme", &theme);
    });
    
    let theme_options = vec![
        ("wwe-executive", "WWE Executive", "Premium gold & dark slate"),
        ("aew-modern", "AEW Modern", "Tech cyan & modern dark"),
        ("njpw-premium", "NJPW Premium", "Royal purple & luxury dark"),
        ("corporate-dark", "Corporate", "Professional blue & business dark"),
    ];
    
    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost gap-2">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zM21 5a2 2 0 00-2-2h-4a2 2 0 00-2 2v12a4 4 0 004 4h4a2 2 0 002-2V5z"></path>
                </svg>
                <span class="hidden sm:inline">"Theme"</span>
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
            </div>
            
            <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-80 border border-base-300">
                <li class="menu-title">
                    <span class="text-base-content/70 text-xs font-semibold uppercase tracking-wider">
                        "Executive Themes"
                    </span>
                </li>
                {theme_options.into_iter().map(|(theme_id, theme_name, description)| {
                    let is_current = current_theme.get() == theme_id;
                    let theme_id = theme_id.to_string();
                    let theme_name = theme_name.to_string(); 
                    let description = description.to_string();
                    
                    view! {
                        <li>
                            <button 
                                class={format!("flex flex-col items-start gap-1 p-3 {}", 
                                    if is_current { "active" } else { "" })}
                                on:click={
                                    let theme_id = theme_id.clone();
                                    let set_current_theme = set_current_theme.clone();
                                    move |_| {
                                        set_current_theme.set(theme_id.clone());
                                    }
                                }
                            >
                                <div class="flex items-center justify-between w-full">
                                    <span class="font-medium text-sm">{theme_name}</span>
                                    {if is_current {
                                        view! {
                                            <svg class="w-4 h-4 text-primary" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }}
                                </div>
                                <span class="text-xs text-base-content/60 text-left">{description}</span>
                            </button>
                        </li>
                    }
                }).collect_view()}
                
                <div class="divider my-1"></div>
                
                <li class="menu-title">
                    <span class="text-base-content/50 text-xs">
                        "Theme automatically saves and persists across sessions"
                    </span>
                </li>
            </ul>
        </div>
    }
}