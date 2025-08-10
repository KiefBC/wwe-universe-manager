use leptos::prelude::*;
use leptos::html;
use web_sys::{Element, HtmlElement, KeyboardEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

/// Professional Keyboard Navigation System
/// 
/// Comprehensive keyboard navigation support with executive-quality shortcuts,
/// arrow key navigation, and professional focus management

/// Executive Keyboard Shortcut Manager
#[derive(Clone)]
pub struct KeyboardShortcutManager {
    shortcuts: HashMap<String, Callback<()>>,
    scope: Option<String>,
}

impl KeyboardShortcutManager {
    pub fn new() -> Self {
        Self {
            shortcuts: HashMap::new(),
            scope: None,
        }
    }
    
    pub fn with_scope(scope: String) -> Self {
        Self {
            shortcuts: HashMap::new(),
            scope: Some(scope),
        }
    }
    
    pub fn register(&mut self, key_combination: String, callback: Callback<()>) {
        self.shortcuts.insert(key_combination, callback);
    }
    
    pub fn handle_keydown(&self, e: &KeyboardEvent) -> bool {
        let key_combo = format_key_combination(e);
        
        if let Some(callback) = self.shortcuts.get(&key_combo) {
            e.prevent_default();
            callback.run(());
            true
        } else {
            false
        }
    }
}

/// Professional Grid Navigation Component
#[component]
pub fn KeyboardNavigableGrid<T: Clone + 'static>(
    /// Grid items
    items: ReadSignal<Vec<T>>,
    /// Number of columns
    columns: u32,
    /// Currently focused item index
    focused_index: ReadWriteSignal<Option<usize>>,
    /// Item renderer
    render_item: Box<dyn Fn(T, usize, bool) -> AnyView + 'static>,
    /// Selection handler
    on_select: Option<Callback<(usize, T)>>,
    /// Grid container class
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView {
    let grid_ref = NodeRef::<html::Div>::new();
    
    // Handle keyboard navigation
    let handle_keydown = move |e: KeyboardEvent| {
        let items_vec = items.get();
        let total_items = items_vec.len();
        if total_items == 0 { return; }
        
        let current_index = focused_index.get().unwrap_or(0);
        let columns = columns as usize;
        let rows = (total_items + columns - 1) / columns;
        let current_row = current_index / columns;
        let current_col = current_index % columns;
        
        let new_index = match e.key().as_str() {
            "ArrowRight" => {
                e.prevent_default();
                Some(if current_col < columns - 1 && current_index + 1 < total_items {
                    current_index + 1
                } else if current_col == columns - 1 {
                    // Jump to beginning of next row or wrap to start
                    if current_row + 1 < rows {
                        (current_row + 1) * columns
                    } else {
                        0 // Wrap to first item
                    }
                } else {
                    current_index
                })
            },
            "ArrowLeft" => {
                e.prevent_default();
                Some(if current_col > 0 {
                    current_index - 1
                } else if current_row > 0 {
                    // Jump to end of previous row
                    std::cmp::min((current_row * columns) - 1, total_items - 1)
                } else {
                    // Wrap to last item
                    total_items - 1
                })
            },
            "ArrowDown" => {
                e.prevent_default();
                let next_row_index = (current_row + 1) * columns + current_col;
                Some(if next_row_index < total_items {
                    next_row_index
                } else {
                    // Wrap to top of same column
                    current_col
                })
            },
            "ArrowUp" => {
                e.prevent_default();
                Some(if current_row > 0 {
                    (current_row - 1) * columns + current_col
                } else {
                    // Wrap to bottom of same column
                    let last_row = (total_items - 1) / columns;
                    let target_index = last_row * columns + current_col;
                    std::cmp::min(target_index, total_items - 1)
                })
            },
            "Home" => {
                e.prevent_default();
                Some(0)
            },
            "End" => {
                e.prevent_default();
                Some(total_items - 1)
            },
            "Enter" | " " => {
                e.prevent_default();
                if let Some(handler) = &on_select {
                    if let Some(item) = items_vec.get(current_index) {
                        handler.run((current_index, item.clone()));
                    }
                }
                None
            },
            _ => None
        };
        
        if let Some(index) = new_index {
            focused_index.set(Some(index));
            
            // Focus the new item
            focus_grid_item(&grid_ref, index);
        }
    };
    
    // Initialize focus when grid is first rendered
    Effect::new(move |_| {
        let items_len = items.get().len();
        if items_len > 0 && focused_index.get().is_none() {
            focused_index.set(Some(0));
        }
    });
    
    let grid_class = format!(
        "grid gap-4 focus-within:ring-2 focus-within:ring-primary/50 focus-within:ring-offset-2 rounded-lg {}",
        class
    );
    
    view! {
        <div 
            node_ref=grid_ref
            class=grid_class
            style=format!("grid-template-columns: repeat({}, minmax(0, 1fr))", columns)
            role="grid"
            aria-label="Navigable grid - use arrow keys to navigate, Enter to select"
            tabindex="0"
            on:keydown=handle_keydown
            on:focus=move |_| {
                // Set initial focus if none exists
                if focused_index.get().is_none() {
                    focused_index.set(Some(0));
                }
            }
        >
            {move || {
                let items_vec = items.get();
                let focused_idx = focused_index.get();
                
                items_vec.into_iter().enumerate().map(|(index, item)| {
                    let is_focused = focused_idx == Some(index);
                    let item_view = render_item(item, index, is_focused);
                    
                    view! {
                        <div 
                            role="gridcell"
                            tabindex={if is_focused { "0" } else { "-1" }}
                            aria-selected=is_focused.to_string()
                            class={format!(
                                "outline-none {}",
                                if is_focused { 
                                    "ring-2 ring-primary ring-offset-2 ring-offset-base-100 rounded-lg" 
                                } else { 
                                    "" 
                                }
                            )}
                            data-grid-index=index.to_string()
                        >
                            {item_view}
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
        
        // Screen reader instructions
        <div class="sr-only" aria-live="polite">
            {move || {
                if let Some(index) = focused_index.get() {
                    format!("Item {} of {} focused. Use arrow keys to navigate, Enter to select.", 
                           index + 1, items.get().len())
                } else {
                    String::new()
                }
            }}
        </div>
    }
}

/// Executive Keyboard Shortcuts Display
#[component]
pub fn KeyboardShortcutsHelper(
    /// Available shortcuts
    shortcuts: Vec<KeyboardShortcut>,
    /// Whether to show shortcuts
    show: ReadSignal<bool>,
) -> impl IntoView {
    view! {
        <div 
            class={move || if show.get() { 
                "fixed inset-0 z-50 flex items-center justify-center bg-black/50" 
            } else { 
                "hidden" 
            }}
            role="dialog"
            aria-modal="true"
            aria-labelledby="shortcuts-title"
        >
            <div class="card bg-base-100 border border-base-300 shadow-xl max-w-2xl w-full m-4 max-h-96 overflow-y-auto">
                <div class="card-body">
                    <h2 id="shortcuts-title" class="card-title text-xl mb-4">
                        "Keyboard Shortcuts"
                    </h2>
                    
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {shortcuts.into_iter().map(|shortcut| {
                            view! {
                                <div class="flex items-center justify-between p-2 rounded bg-base-200/50">
                                    <span class="text-sm font-medium">{shortcut.description}</span>
                                    <div class="flex gap-1">
                                        {shortcut.keys.into_iter().map(|key| {
                                            view! {
                                                <kbd class="kbd kbd-sm">{key}</kbd>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    
                    <div class="text-sm text-base-content/70 mt-4">
                        "Press ? to toggle this help, Escape to close"
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct KeyboardShortcut {
    pub keys: Vec<String>,
    pub description: String,
}

/// Professional Command Palette
#[component]
pub fn CommandPalette(
    /// Whether command palette is open
    is_open: ReadWriteSignal<bool>,
    /// Available commands
    commands: ReadSignal<Vec<Command>>,
    /// Search query
    search_query: ReadWriteSignal<String>,
    /// Selected command index
    selected_index: ReadWriteSignal<usize>,
    /// Command execution handler
    on_execute: Callback<Command>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    let command_list_ref = NodeRef::<html::Div>::new();
    
    // Filter commands based on search
    let filtered_commands = Signal::derive(move || {
        let query = search_query.get().to_lowercase();
        let all_commands = commands.get();
        
        if query.is_empty() {
            all_commands
        } else {
            all_commands
                .into_iter()
                .filter(|cmd| {
                    cmd.title.to_lowercase().contains(&query) ||
                    cmd.description.as_ref().map_or(false, |desc| desc.to_lowercase().contains(&query))
                })
                .collect()
        }
    });
    
    // Reset selection when commands change
    Effect::new(move |_| {
        let _ = filtered_commands.get();
        selected_index.set(0);
    });
    
    // Focus input when opened
    Effect::new(move |_| {
        if is_open.get() {
            request_animation_frame(move || {
                if let Some(input) = input_ref.get() {
                    let _ = input.focus();
                }
            });
        }
    });
    
    let handle_keydown = move |e: KeyboardEvent| {
        let commands_vec = filtered_commands.get();
        let current_selection = selected_index.get();
        
        match e.key().as_str() {
            "Escape" => {
                e.prevent_default();
                is_open.set(false);
                search_query.set(String::new());
            },
            "ArrowDown" => {
                e.prevent_default();
                if current_selection < commands_vec.len().saturating_sub(1) {
                    selected_index.set(current_selection + 1);
                } else {
                    selected_index.set(0);
                }
            },
            "ArrowUp" => {
                e.prevent_default();
                if current_selection > 0 {
                    selected_index.set(current_selection - 1);
                } else {
                    selected_index.set(commands_vec.len().saturating_sub(1));
                }
            },
            "Enter" => {
                e.prevent_default();
                if let Some(command) = commands_vec.get(current_selection) {
                    on_execute.run(command.clone());
                    is_open.set(false);
                    search_query.set(String::new());
                }
            },
            _ => {}
        }
    };
    
    view! {
        <div 
            class={move || if is_open.get() { 
                "fixed inset-0 z-50 flex items-start justify-center pt-16 bg-black/50" 
            } else { 
                "hidden" 
            }}
            role="dialog"
            aria-modal="true"
            aria-labelledby="command-palette-title"
        >
            <div class="card bg-base-100 border border-base-300 shadow-xl max-w-2xl w-full mx-4">
                <div class="card-body p-0">
                    // Search input
                    <div class="p-4 border-b border-base-300">
                        <label class="sr-only" for="command-search">
                            "Search commands"
                        </label>
                        <input 
                            node_ref=input_ref
                            id="command-search"
                            type="text"
                            class="input input-bordered w-full focus:ring-4 focus:ring-primary/50"
                            placeholder="Search commands..."
                            prop:value=move || search_query.get()
                            on:input=move |e| search_query.set(event_target_value(&e))
                            on:keydown=handle_keydown
                        />
                    </div>
                    
                    // Command list
                    <div 
                        node_ref=command_list_ref
                        class="max-h-96 overflow-y-auto"
                        role="listbox"
                        aria-label="Available commands"
                    >
                        {move || {
                            let commands_vec = filtered_commands.get();
                            let selection = selected_index.get();
                            
                            if commands_vec.is_empty() {
                                vec![view! {
                                    <div class="p-4 text-center text-base-content/70">
                                        "No commands found"
                                    </div>
                                }]
                            } else {
                                commands_vec.into_iter().enumerate().map(|(index, command)| {
                                    let is_selected = index == selection;
                                    
                                    view! {
                                        <div 
                                            role="option"
                                            aria-selected=is_selected.to_string()
                                            class={format!(
                                                "p-4 cursor-pointer border-b border-base-300/50 last:border-b-0 {}",
                                                if is_selected { 
                                                    "bg-primary/10 border-l-4 border-l-primary" 
                                                } else { 
                                                    "hover:bg-base-200/50" 
                                                }
                                            )}
                                            on:click=move |_| {
                                                on_execute.run(command.clone());
                                                is_open.set(false);
                                                search_query.set(String::new());
                                            }
                                        >
                                            <div class="flex items-center justify-between">
                                                <div>
                                                    <div class="font-medium">{command.title.clone()}</div>
                                                    {command.description.as_ref().map(|desc| {
                                                        view! {
                                                            <div class="text-sm text-base-content/70">{desc.clone()}</div>
                                                        }.into_any()
                                                    })}
                                                </div>
                                                {command.shortcut.as_ref().map(|shortcut| {
                                                    view! {
                                                        <kbd class="kbd kbd-sm">{shortcut.clone()}</kbd>
                                                    }.into_any()
                                                })}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub shortcut: Option<String>,
    pub action: String,
}

// Utility functions
fn format_key_combination(e: &KeyboardEvent) -> String {
    let mut parts = Vec::new();
    
    if e.ctrl_key() { parts.push("Ctrl"); }
    if e.alt_key() { parts.push("Alt"); }
    if e.shift_key() { parts.push("Shift"); }
    if e.meta_key() { parts.push("Meta"); }
    
    let key = e.key();
    parts.push(&key);
    
    parts.join("+")
}

fn focus_grid_item(grid_ref: &NodeRef<html::Div>, index: usize) {
    if let Some(grid) = grid_ref.get() {
        let selector = format!("[data-grid-index='{}']", index);
        if let Ok(element) = grid.query_selector(&selector) {
            if let Some(element) = element {
                if let Ok(html_element) = element.dyn_ref::<HtmlElement>() {
                    let _ = html_element.focus();
                }
            }
        }
    }
}