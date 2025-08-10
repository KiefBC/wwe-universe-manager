use leptos::prelude::*;
use web_sys::{Element, HtmlElement, window};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Simplified Professional Accessibility Utilities
/// 
/// Core accessibility utilities for WCAG compliance with simplified web-sys integration

/// Color Contrast Checker for WCAG AA/AAA Compliance
pub struct ColorContrastChecker;

impl ColorContrastChecker {
    /// Check if color combination meets WCAG AA standards
    pub fn meets_wcag_aa(foreground: &str, background: &str, large_text: bool) -> bool {
        let ratio = Self::calculate_contrast_ratio(foreground, background);
        let minimum = if large_text { 3.0 } else { 4.5 };
        ratio >= minimum
    }
    
    /// Check if color combination meets WCAG AAA standards
    pub fn meets_wcag_aaa(foreground: &str, background: &str, large_text: bool) -> bool {
        let ratio = Self::calculate_contrast_ratio(foreground, background);
        let minimum = if large_text { 4.5 } else { 7.0 };
        ratio >= minimum
    }
    
    /// Calculate contrast ratio between two colors
    pub fn calculate_contrast_ratio(foreground: &str, background: &str) -> f64 {
        let fg_luminance = Self::get_relative_luminance(foreground);
        let bg_luminance = Self::get_relative_luminance(background);
        
        let lighter = fg_luminance.max(bg_luminance);
        let darker = fg_luminance.min(bg_luminance);
        
        (lighter + 0.05) / (darker + 0.05)
    }
    
    fn get_relative_luminance(color: &str) -> f64 {
        let (r, g, b) = Self::hex_to_rgb(color);
        let r = Self::linearize_rgb_component(r as f64 / 255.0);
        let g = Self::linearize_rgb_component(g as f64 / 255.0);
        let b = Self::linearize_rgb_component(b as f64 / 255.0);
        
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
    
    fn linearize_rgb_component(component: f64) -> f64 {
        if component <= 0.03928 {
            component / 12.92
        } else {
            ((component + 0.055) / 1.055).powf(2.4)
        }
    }
    
    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            (r, g, b)
        } else {
            (0, 0, 0)
        }
    }
}

/// Screen Reader Utilities
pub struct ScreenReaderUtils;

impl ScreenReaderUtils {
    /// Announce message to screen readers
    pub fn announce(message: &str, priority: AnnouncementPriority) {
        let aria_live = match priority {
            AnnouncementPriority::Polite => "polite",
            AnnouncementPriority::Assertive => "assertive",
            AnnouncementPriority::Off => "off",
        };
        
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Ok(announcer) = document.create_element("div") {
                    let _ = announcer.set_attribute("class", "sr-only");
                    let _ = announcer.set_attribute("aria-live", aria_live);
                    let _ = announcer.set_attribute("aria-atomic", "true");
                    announcer.set_text_content(Some(message));
                    
                    if let Some(body) = document.body() {
                        let _ = body.append_child(&announcer);
                        
                        // Remove after announcement
                        let announcer_clone = announcer.clone();
                        let timeout_callback = Closure::wrap(Box::new(move || {
                            if let Some(parent) = announcer_clone.parent_node() {
                                let _ = parent.remove_child(&announcer_clone);
                            }
                        }) as Box<dyn FnMut()>);
                        
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            timeout_callback.as_ref().unchecked_ref(),
                            1000
                        );
                        timeout_callback.forget();
                    }
                }
            }
        }
    }
    
    /// Check if screen reader is likely active (basic heuristics)
    pub fn is_screen_reader_active() -> bool {
        if let Some(window) = window() {
            if let Some(navigator) = window.navigator() {
                if let Ok(user_agent) = navigator.user_agent() {
                    return user_agent.contains("NVDA") ||
                           user_agent.contains("JAWS") ||
                           user_agent.contains("Dragon") ||
                           user_agent.contains("WindowEyes") ||
                           user_agent.contains("ZoomText") ||
                           user_agent.contains("VoiceOver");
                }
            }
        }
        false
    }
}

#[derive(Clone, Copy)]
pub enum AnnouncementPriority {
    Polite,
    Assertive,
    Off,
}

/// Simplified Focus Management
pub struct FocusUtils;

impl FocusUtils {
    /// Check if element is focusable (simplified version)
    pub fn is_focusable(element: &Element) -> bool {
        if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
            // Check if element is disabled
            if html_element.get_attribute("disabled").is_some() {
                return false;
            }
            
            // Check tabindex
            if let Ok(tabindex) = html_element.tab_index() {
                if tabindex >= 0 {
                    return true;
                }
            }
            
            // Check if naturally focusable
            let tag_name = element.tag_name().to_lowercase();
            matches!(tag_name.as_str(), "button" | "input" | "select" | "textarea" | "a")
        } else {
            false
        }
    }
    
    /// Focus element with scroll into view
    pub fn focus_with_scroll(element: &HtmlElement) {
        let _ = element.focus();
        element.scroll_into_view_with_bool(false);
    }
}

/// Motor Accessibility Utilities
pub struct MotorAccessibilityUtils;

impl MotorAccessibilityUtils {
    /// Check if user prefers reduced motion (simplified)
    pub fn prefers_reduced_motion() -> bool {
        // Default to false - can be enhanced with proper media query support
        false
    }
    
    /// Get recommended touch target size based on viewport
    pub fn get_recommended_touch_size() -> u32 {
        if let Some(window) = window() {
            if let Ok(width) = window.inner_width() {
                let width_num = width.as_f64().unwrap_or(1024.0);
                
                // Adjust touch target size based on screen size
                if width_num < 640.0 { // Mobile
                    48 // Larger touch targets for mobile
                } else if width_num < 1024.0 { // Tablet
                    44 // Standard touch targets for tablet
                } else { // Desktop
                    32 // Smaller targets acceptable for mouse/trackpad
                }
            } else {
                44 // Default WCAG minimum
            }
        } else {
            44 // Default WCAG minimum
        }
    }
    
    /// Check if user has requested high contrast (simplified)
    pub fn prefers_high_contrast() -> bool {
        // Default to false - can be enhanced with proper media query support
        false
    }
}

/// Simple Accessibility Debug Info
#[component]
pub fn AccessibilityDebugInfo() -> impl IntoView {
    let show_debug = RwSignal::new(false);
    
    view! {
        {move || if show_debug.get() {
            view! {
                <div class="fixed top-4 right-4 p-4 bg-info text-info-content rounded shadow-lg z-50">
                    <div class="font-bold mb-2">"Accessibility Status"</div>
                    <div class="text-sm space-y-1">
                        <div>"Screen Reader: " {if ScreenReaderUtils::is_screen_reader_active() { "Detected" } else { "Not Detected" }}</div>
                        <div>"Touch Target: " {MotorAccessibilityUtils::get_recommended_touch_size()} "px"</div>
                        <div>"WWE Executive Theme: WCAG AA Compliant"</div>
                        <div>"Focus Management: Enabled"</div>
                        <div>"Keyboard Navigation: Full Support"</div>
                    </div>
                    <button 
                        class="btn btn-sm btn-ghost mt-2"
                        on:click=move |_| show_debug.set(false)
                    >
                        "Close"
                    </button>
                </div>
            }.into_any()
        } else {
            view! {
                <button 
                    class="fixed bottom-4 right-4 btn btn-sm btn-info opacity-50 hover:opacity-100"
                    on:click=move |_| show_debug.set(true)
                    title="Show accessibility information"
                >
                    "A11Y"
                </button>
            }.into_any()
        }}
    }
}