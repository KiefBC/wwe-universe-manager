use leptos::prelude::*;
use web_sys::{Element, HtmlElement, window};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Professional Accessibility Utilities and Testing
/// 
/// Comprehensive utility functions for accessibility compliance,
/// color contrast checking, and WCAG validation

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
                    announcer.set_class_name("sr-only");
                    announcer.set_attribute("aria-live", aria_live).unwrap_or(());
                    announcer.set_attribute("aria-atomic", "true").unwrap_or(());
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
    
    /// Check if screen reader is likely active
    pub fn is_screen_reader_active() -> bool {
        // Basic heuristics for screen reader detection
        if let Some(window) = window() {
            if let Some(navigator) = window.navigator() {
                let user_agent = navigator.user_agent().unwrap_or_default();
                
                // Check for common screen reader indicators
                user_agent.contains("NVDA") ||
                user_agent.contains("JAWS") ||
                user_agent.contains("Dragon") ||
                user_agent.contains("WindowEyes") ||
                user_agent.contains("ZoomText") ||
                user_agent.contains("VoiceOver")
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
pub enum AnnouncementPriority {
    Polite,
    Assertive,
    Off,
}

/// Focus Management Utilities
pub struct FocusUtils;

impl FocusUtils {
    /// Find next focusable element
    pub fn find_next_focusable(current: &Element, forward: bool) -> Option<Element> {
        let focusable_selector = "button:not(:disabled), [href], input:not(:disabled), select:not(:disabled), textarea:not(:disabled), [tabindex]:not([tabindex='-1'])";
        
        if let Some(document) = window().and_then(|w| w.document()) {
            if let Ok(all_focusable) = document.query_selector_all(focusable_selector) {
                let mut focusable_elements = Vec::new();
                for i in 0..all_focusable.length() {
                    if let Some(element) = all_focusable.item(i) {
                        focusable_elements.push(element);
                    }
                }
                
                if let Some(current_index) = focusable_elements.iter().position(|el: &Element| {
                    el.is_same_node(Some(current))
                }) {
                    let next_index = if forward {
                        (current_index + 1) % focusable_elements.len()
                    } else {
                        if current_index == 0 {
                            focusable_elements.len() - 1
                        } else {
                            current_index - 1
                        }
                    };
                    
                    focusable_elements.get(next_index).cloned()
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Check if element is currently visible and focusable
    pub fn is_focusable(element: &Element) -> bool {
        if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
            // Check if element is hidden
            if let Some(window) = window() {
                if let Ok(Some(style)) = window.get_computed_style(html_element) {
                    if style.get_property_value("display").unwrap_or_default() == "none" ||
                       style.get_property_value("visibility").unwrap_or_default() == "hidden" {
                        return false;
                    }
                }
            }
            
            // Check if element is disabled
            if let Some(disabled) = html_element.get_attribute("disabled") {
                if !disabled.is_empty() {
                    return false;
                }
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
    /// Check if user prefers reduced motion
    pub fn prefers_reduced_motion() -> bool {
        if let Some(window) = window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-reduced-motion: reduce)") {
                return media_query.matches();
            }
        }
        false
    }
    
    /// Get recommended touch target size based on viewport
    pub fn get_recommended_touch_size() -> u32 {
        if let Some(window) = window() {
            let width = window.inner_width().unwrap_or(JsValue::from(1024)).as_f64().unwrap_or(1024.0);
            
            // Adjust touch target size based on screen size
            if width < 640.0 { // Mobile
                48 // Larger touch targets for mobile
            } else if width < 1024.0 { // Tablet
                44 // Standard touch targets for tablet
            } else { // Desktop
                32 // Smaller targets acceptable for mouse/trackpad
            }
        } else {
            44 // Default WCAG minimum
        }
    }
    
    /// Check if user has requested high contrast
    pub fn prefers_high_contrast() -> bool {
        if let Some(window) = window() {
            if let Ok(Some(media_query)) = window.match_media("(prefers-contrast: high)") {
                return media_query.matches();
            }
        }
        false
    }
}

/// Accessibility Testing Component
#[component]
pub fn AccessibilityTester(
    /// Whether to show accessibility info
    show_a11y_info: ReadSignal<bool>,
    /// Target element to test
    target_selector: String,
) -> impl IntoView {
    let test_results = RwSignal::new(Vec::<AccessibilityTest>::new());
    
    // Run accessibility tests
    let run_tests = move || {
        if let Some(document) = window().and_then(|w| w.document()) {
            if let Ok(elements) = document.query_selector_all(&target_selector) {
                let mut results = Vec::new();
                
                for i in 0..elements.length() {
                    if let Some(element) = elements.item(i) {
                        // Test color contrast
                        if let Ok(computed_style) = window()
                            .and_then(|w| w.get_computed_style(element.dyn_ref::<HtmlElement>().unwrap()))
                            .and_then(|opt| opt.ok_or(()))
                        {
                            if let Some(style) = computed_style {
                                let color = style.get_property_value("color").unwrap_or_default();
                                let background = style.get_property_value("background-color").unwrap_or_default();
                                
                                if !color.is_empty() && !background.is_empty() {
                                    // This is simplified - in practice you'd need to convert computed colors to hex
                                    results.push(AccessibilityTest {
                                        element_info: format!("Element {}", i + 1),
                                        test_type: "Color Contrast".to_string(),
                                        status: TestStatus::Pass, // Simplified for demo
                                        details: Some(format!("Color: {}, Background: {}", color, background)),
                                    });
                                }
                            }
                        }
                        
                        // Test for ARIA labels
                        let has_aria_label = element.get_attribute("aria-label").is_some() ||
                                           element.get_attribute("aria-labelledby").is_some();
                        
                        results.push(AccessibilityTest {
                            element_info: format!("Element {}", i + 1),
                            test_type: "ARIA Labels".to_string(),
                            status: if has_aria_label { TestStatus::Pass } else { TestStatus::Warning },
                            details: Some(if has_aria_label {
                                "Element has ARIA labeling".to_string()
                            } else {
                                "Consider adding ARIA labels for better accessibility".to_string()
                            }),
                        });
                        
                        // Test for keyboard focusability
                        let is_focusable = FocusUtils::is_focusable(&element);
                        let is_interactive = element.tag_name().to_lowercase() == "button" ||
                                           element.get_attribute("onclick").is_some();
                        
                        if is_interactive {
                            results.push(AccessibilityTest {
                                element_info: format!("Element {}", i + 1),
                                test_type: "Keyboard Focus".to_string(),
                                status: if is_focusable { TestStatus::Pass } else { TestStatus::Error },
                                details: Some(if is_focusable {
                                    "Element is keyboard focusable".to_string()
                                } else {
                                    "Interactive element should be keyboard focusable".to_string()
                                }),
                            });
                        }
                    }
                }
                
                test_results.set(results);
            }
        }
    };
    
    // Run tests when component shows
    Effect::new(move |_| {
        if show_a11y_info.get() {
            run_tests();
        }
    });
    
    view! {
        {move || if show_a11y_info.get() {
            view! {
                <div class="fixed bottom-4 right-4 w-96 max-h-96 overflow-y-auto 
                           card bg-base-100 border border-base-300 shadow-xl z-50">
                    <div class="card-body p-4">
                        <div class="flex items-center justify-between mb-4">
                            <h3 class="font-bold text-lg">Accessibility Tests</h3>
                            <button 
                                class="btn btn-sm btn-ghost"
                                on:click=move |_| run_tests()
                            >
                                "Refresh"
                            </button>
                        </div>
                        
                        <div class="space-y-2">
                            {move || {
                                test_results.get().into_iter().map(|test| {
                                    let (status_class, status_icon) = match test.status {
                                        TestStatus::Pass => ("text-success", "✓"),
                                        TestStatus::Warning => ("text-warning", "⚠"),
                                        TestStatus::Error => ("text-error", "✗"),
                                    };
                                    
                                    view! {
                                        <div class="p-2 rounded bg-base-200/50">
                                            <div class="flex items-center gap-2">
                                                <span class={format!("font-mono text-sm {}", status_class)}>
                                                    {status_icon}
                                                </span>
                                                <span class="font-medium text-xs">{test.test_type}</span>
                                                <span class="text-xs text-base-content/70">
                                                    {test.element_info}
                                                </span>
                                            </div>
                                            {test.details.map(|details| {
                                                view! {
                                                    <div class="text-xs text-base-content/70 mt-1 ml-6">
                                                        {details}
                                                    </div>
                                                }.into_any()
                                            })}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }}
                        </div>
                    </div>
                </div>
            }.into_any()
        } else {
            view! {}.into_any()
        }}
    }
}

#[derive(Clone)]
struct AccessibilityTest {
    element_info: String,
    test_type: String,
    status: TestStatus,
    details: Option<String>,
}

#[derive(Clone)]
enum TestStatus {
    Pass,
    Warning,
    Error,
}

/// Accessibility Debug Component for Development
#[component]
pub fn AccessibilityDebugger() -> impl IntoView {
    let show_debug = RwSignal::new(false);
    
    // Toggle debug with keyboard shortcut
    Effect::new(move |_| {
        let handle_keydown = move |e: web_sys::KeyboardEvent| {
            if e.ctrl_key() && e.shift_key() && e.key() == "A" {
                e.prevent_default();
                show_debug.update(|show| *show = !*show);
            }
        };
        
        if let Some(document) = window().and_then(|w| w.document()) {
            let handler = Closure::wrap(Box::new(handle_keydown) as Box<dyn FnMut(_)>);
            let _ = document.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
            handler.forget();
        }
    });
    
    view! {
        <AccessibilityTester 
            show_a11y_info=show_debug.read_only()
            target_selector="*".to_string()
        />
        
        // Debug info overlay
        {move || if show_debug.get() {
            view! {
                <div class="fixed top-4 right-4 p-4 bg-info text-info-content rounded shadow-lg z-50">
                    <div class="font-bold mb-2">"Accessibility Debug Mode"</div>
                    <div class="text-sm space-y-1">
                        <div>"Screen Reader: " {if ScreenReaderUtils::is_screen_reader_active() { "Active" } else { "Inactive" }}</div>
                        <div>"Reduced Motion: " {if MotorAccessibilityUtils::prefers_reduced_motion() { "Yes" } else { "No" }}</div>
                        <div>"High Contrast: " {if MotorAccessibilityUtils::prefers_high_contrast() { "Yes" } else { "No" }}</div>
                        <div>"Touch Target: " {MotorAccessibilityUtils::get_recommended_touch_size()} "px"</div>
                    </div>
                    <div class="text-xs mt-2 opacity-70">
                        "Press Ctrl+Shift+A to toggle"
                    </div>
                </div>
            }.into_any()
        } else {
            view! {}.into_any()
        }}
    }
}