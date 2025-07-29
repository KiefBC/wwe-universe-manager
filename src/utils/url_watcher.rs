use crate::utils::url_parser::extract_wrestler_id_from_url;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget};

/// Custom hook for watching URL changes without memory leaks
/// 
/// This hook replaces the polling mechanism with proper event listeners
/// for hashchange events, preventing memory leaks and improving performance.
pub fn use_url_watcher() -> ReadSignal<Option<i32>> {
    let (current_wrestler_id, set_current_wrestler_id) = signal(extract_wrestler_id_from_url());

    // Set up hashchange event listener instead of polling
    Effect::new(move |_| {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return Box::new(move || {}) as Box<dyn FnOnce()>, // Return empty cleanup if no window
        };

        // Create the event handler closure
        let closure = {
            let set_wrestler_id_clone = set_current_wrestler_id;
            let current_wrestler_id_clone = current_wrestler_id;
            
            Closure::wrap(Box::new(move |_event: Event| {
                let new_id = extract_wrestler_id_from_url();
                if new_id != current_wrestler_id_clone.get_untracked() {
                    set_wrestler_id_clone.set(new_id);
                }
            }) as Box<dyn FnMut(Event)>)
        };

        // Add the event listener
        let event_target: &EventTarget = window.as_ref();
        if let Err(e) = event_target.add_event_listener_with_callback(
            "hashchange",
            closure.as_ref().unchecked_ref(),
        ) {
            web_sys::console::error_1(&format!("Failed to add hashchange listener: {:?}", e).into());
        }

        // Return cleanup function that removes the event listener
        Box::new(move || {
            if let Some(window) = web_sys::window() {
                let event_target: &EventTarget = window.as_ref();
                let _ = event_target.remove_event_listener_with_callback(
                    "hashchange",
                    closure.as_ref().unchecked_ref(),
                );
            }
            // Closure will be properly dropped here, cleaning up memory
            drop(closure);
        }) as Box<dyn FnOnce()>
    });

    current_wrestler_id
}

/// Custom hook for initial URL sync
/// 
/// This hook handles the initial URL parsing when the component mounts
/// without setting up ongoing watching (use use_url_watcher for that).
pub fn use_initial_url_sync() -> Option<i32> {
    extract_wrestler_id_from_url()
}