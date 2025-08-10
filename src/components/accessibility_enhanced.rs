use leptos::prelude::*;
use leptos::html;
use web_sys::{Element, HtmlElement, KeyboardEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Enhanced Accessibility Components
/// 
/// Advanced accessibility features including keyboard navigation, 
/// motor accessibility, and cognitive accessibility enhancements

/// Executive Accessible Modal with full keyboard trap and focus management
#[component]
pub fn AccessibleExecutiveModal(
    /// Modal title
    title: String,
    /// Whether modal is open
    is_open: ReadWriteSignal<bool>,
    /// Modal content
    children: ChildrenFn,
    /// Modal size
    #[prop(default = "md".to_string())]
    size: String,
    /// Whether modal can be dismissed by clicking outside
    #[prop(default = true)]
    dismissible: bool,
    /// Custom close handler
    #[prop(optional)]
    on_close: Option<Callback<()>>,
    /// Whether modal has destructive actions
    #[prop(default = false)]
    destructive: bool,
) -> impl IntoView {
    let modal_ref = NodeRef::<html::Div>::new();
    let close_button_ref = NodeRef::<html::Button>::new();
    let previous_focus = RwSignal::new(None::<Element>);
    
    // Store focus when opening modal
    Effect::new(move |_| {
        if is_open.get() {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                previous_focus.set(document.active_element());
                
                // Focus close button after modal is rendered
                request_animation_frame(move || {
                    if let Some(button) = close_button_ref.get() {
                        let _ = button.focus();
                    }
                });
            }
        }
    });
    
    // Restore focus when closing modal
    let restore_focus = move || {
        if let Some(element) = previous_focus.get() {
            if let Ok(html_element) = element.dyn_ref::<HtmlElement>() {
                let _ = html_element.focus();
            }
        }
    };
    
    let handle_close = move || {
        restore_focus();
        is_open.set(false);
        if let Some(close_handler) = on_close {
            close_handler.run(());
        }
    };
    
    // Keyboard navigation handler
    let handle_keydown = move |e: KeyboardEvent| {
        match e.key().as_str() {
            "Escape" => {
                if dismissible {
                    e.prevent_default();
                    handle_close();
                }
            },
            "Tab" => {
                // Keep focus within modal
                trap_focus_within_modal(&e, &modal_ref);
            },
            _ => {}
        }
    };
    
    let modal_size_class = match size.as_str() {
        "sm" => "modal-box w-11/12 max-w-md",
        "lg" => "modal-box w-11/12 max-w-4xl",
        "xl" => "modal-box w-11/12 max-w-6xl",
        "full" => "modal-box w-11/12 max-w-none h-5/6",
        _ => "modal-box w-11/12 max-w-2xl", // md
    };
    
    view! {
        <div 
            class={move || if is_open.get() { "modal modal-open" } else { "modal" }}
            role="dialog"
            aria-modal="true"
            aria-labelledby="modal-title"
            aria-describedby="modal-description"
            on:keydown=handle_keydown
        >
            <div 
                node_ref=modal_ref
                class=format!("{} relative", modal_size_class)
                on:click=move |e| e.stop_propagation()
            >
                // Modal header with accessible close button
                <div class="flex items-center justify-between pb-4 mb-4 border-b border-base-300">
                    <h2 
                        id="modal-title"
                        class={format!(
                            "text-xl font-bold {}",
                            if destructive { "text-error" } else { "text-base-content" }
                        )}
                    >
                        {title}
                    </h2>
                    
                    <button 
                        node_ref=close_button_ref
                        type="button"
                        class="btn btn-sm btn-circle btn-ghost hover:bg-base-200
                               focus:ring-4 focus:ring-primary/50 focus:ring-offset-2 
                               focus:ring-offset-base-100 focus:outline-none"
                        aria-label="Close modal"
                        on:click=move |_| handle_close()
                    >
                        <svg 
                            class="w-4 h-4" 
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                        >
                            <path 
                                stroke-linecap="round" 
                                stroke-linejoin="round" 
                                stroke-width="2" 
                                d="M6 18L18 6M6 6l12 12">
                            </path>
                        </svg>
                    </button>
                </div>
                
                // Modal content
                <div id="modal-description">
                    {children()}
                </div>
                
                // Screen reader instructions
                <div class="sr-only">
                    "Press Escape to close this modal, or use the close button."
                    {if destructive {
                        " Warning: This action may be destructive."
                    } else {
                        ""
                    }}
                </div>
            </div>
            
            // Modal backdrop
            {if dismissible {
                view! {
                    <div 
                        class="modal-backdrop bg-black/50"
                        aria-hidden="true"
                        on:click=move |_| handle_close()
                    ></div>
                }.into_any()
            } else {
                view! {
                    <div class="modal-backdrop bg-black/50" aria-hidden="true"></div>
                }.into_any()
            }}
        </div>
    }
}

/// Professional Data Table with full accessibility support
#[component]
pub fn AccessibleExecutiveTable<T: Clone + 'static>(
    /// Table caption for screen readers
    caption: String,
    /// Table data
    data: ReadSignal<Vec<T>>,
    /// Column definitions
    columns: Vec<AccessibleTableColumn<T>>,
    /// Loading state
    #[prop(default = false)]
    loading: bool,
    /// Sort configuration
    #[prop(optional)]
    sort_config: Option<(String, bool)>, // (column_id, ascending)
    /// Sort change handler
    #[prop(optional)]
    on_sort_change: Option<Callback<(String, bool)>>,
    /// Row selection
    #[prop(optional)]
    selected_rows: Option<ReadWriteSignal<Vec<usize>>>,
    /// Select all handler
    #[prop(optional)]
    on_select_all: Option<Callback<bool>>,
    /// Row action handler
    #[prop(optional)]
    on_row_action: Option<Callback<(usize, String)>>,
) -> impl IntoView {
    let table_id = "executive-table".to_string();
    
    view! {
        <div class="overflow-x-auto shadow-lg rounded-lg border border-base-300">
            <table 
                id=table_id
                class="table table-zebra w-full"
                role="table"
                aria-label=caption.clone()
            >
                // Table caption for screen readers
                <caption class="sr-only">{caption}</caption>
                
                <thead>
                    <tr role="row">
                        // Select all column if row selection enabled
                        {if let Some(selected_signal) = selected_rows {
                            let total_rows = data.get().len();
                            let selected_count = selected_signal.get().len();
                            let all_selected = selected_count == total_rows && total_rows > 0;
                            let some_selected = selected_count > 0 && selected_count < total_rows;
                            
                            view! {
                                <th 
                                    scope="col"
                                    class="w-12"
                                    role="columnheader"
                                >
                                    <label class="label cursor-pointer">
                                        <input 
                                            type="checkbox"
                                            class="checkbox checkbox-primary
                                                   focus:ring-4 focus:ring-primary/50 
                                                   focus:ring-offset-2 focus:ring-offset-base-100"
                                            checked=all_selected
                                            indeterminate=some_selected
                                            aria-label=format!(
                                                "Select all rows ({} of {} selected)", 
                                                selected_count, total_rows
                                            )
                                            on:change=move |e| {
                                                let checked = event_target_checked(&e);
                                                if let Some(handler) = on_select_all {
                                                    handler.run(checked);
                                                }
                                            }
                                        />
                                    </label>
                                </th>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }}
                        
                        // Column headers
                        {columns.into_iter().map(|col| {
                            let sortable = col.sortable;
                            let column_id = col.id.clone();
                            let is_sorted = sort_config.as_ref()
                                .map(|(sorted_col, _)| sorted_col == &column_id)
                                .unwrap_or(false);
                            let is_ascending = sort_config.as_ref()
                                .map(|(_, asc)| *asc)
                                .unwrap_or(true);
                            
                            view! {
                                <th 
                                    scope="col"
                                    class={format!(
                                        "text-left font-semibold {}",
                                        if sortable { "cursor-pointer hover:bg-base-200" } else { "" }
                                    )}
                                    role="columnheader"
                                    aria-sort={if is_sorted {
                                        if is_ascending { "ascending" } else { "descending" }
                                    } else if sortable {
                                        "none"
                                    } else {
                                        ""
                                    }}
                                    tabindex={if sortable { "0" } else { "-1" }}
                                    on:click=move |_| {
                                        if sortable {
                                            if let Some(handler) = on_sort_change {
                                                let new_ascending = if is_sorted { !is_ascending } else { true };
                                                handler.run((column_id.clone(), new_ascending));
                                            }
                                        }
                                    }
                                    on:keydown=move |e: KeyboardEvent| {
                                        if sortable && (e.key() == "Enter" || e.key() == " ") {
                                            e.prevent_default();
                                            if let Some(handler) = on_sort_change {
                                                let new_ascending = if is_sorted { !is_ascending } else { true };
                                                handler.run((column_id.clone(), new_ascending));
                                            }
                                        }
                                    }
                                >
                                    <div class="flex items-center gap-2">
                                        {col.header}
                                        {if sortable {
                                            view! {
                                                <svg 
                                                    class={format!(
                                                        "w-4 h-4 transition-transform {}",
                                                        if is_sorted {
                                                            if is_ascending { "rotate-0" } else { "rotate-180" }
                                                        } else {
                                                            "opacity-50"
                                                        }
                                                    )}
                                                    fill="none" 
                                                    stroke="currentColor" 
                                                    viewBox="0 0 24 24"
                                                    aria-hidden="true"
                                                >
                                                    <path 
                                                        stroke-linecap="round" 
                                                        stroke-linejoin="round" 
                                                        stroke-width="2" 
                                                        d="M7 10l5 5 5-5">
                                                    </path>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                    </div>
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                
                <tbody>
                    {move || {
                        let rows = data.get();
                        if loading {
                            // Loading skeleton rows
                            (0..5).map(|i| {
                                view! {
                                    <tr key=i role="row">
                                        {if selected_rows.is_some() {
                                            view! {
                                                <td><div class="skeleton h-4 w-4 rounded"></div></td>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                        <td colspan="100%">
                                            <div class="skeleton h-4 w-full"></div>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        } else if rows.is_empty() {
                            vec![view! {
                                <tr role="row">
                                    <td 
                                        colspan="100%"
                                        class="text-center py-8 text-base-content/70"
                                        role="cell"
                                    >
                                        "No data available"
                                    </td>
                                </tr>
                            }]
                        } else {
                            rows.into_iter().enumerate().map(|(index, item)| {
                                view! {
                                    <tr 
                                        key=index
                                        role="row"
                                        tabindex="0"
                                        class="hover:bg-base-200/50 focus:bg-base-200 focus:outline-none
                                               focus:ring-2 focus:ring-primary/50 focus:ring-inset"
                                        aria-rowindex=(index + 1).to_string()
                                    >
                                        // Row selection checkbox
                                        {if let Some(selected_signal) = selected_rows {
                                            let is_selected = selected_signal.get().contains(&index);
                                            view! {
                                                <td role="cell">
                                                    <label class="label cursor-pointer">
                                                        <input 
                                                            type="checkbox"
                                                            class="checkbox checkbox-primary checkbox-sm
                                                                   focus:ring-4 focus:ring-primary/50 
                                                                   focus:ring-offset-1 focus:ring-offset-base-100"
                                                            checked=is_selected
                                                            aria-label=format!("Select row {}", index + 1)
                                                            on:change=move |e| {
                                                                let checked = event_target_checked(&e);
                                                                let mut selected = selected_signal.get();
                                                                if checked {
                                                                    if !selected.contains(&index) {
                                                                        selected.push(index);
                                                                    }
                                                                } else {
                                                                    selected.retain(|&x| x != index);
                                                                }
                                                                selected_signal.set(selected);
                                                            }
                                                        />
                                                    </label>
                                                </td>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}
                                        
                                        // Data cells would be rendered here
                                        // This is a simplified example - in practice, you'd iterate through columns
                                        <td role="cell">"Row data would go here"</td>
                                    </tr>
                                }
                            }).collect::<Vec<_>>()
                        }
                    }}
                </tbody>
            </table>
        </div>
    }
}

/// Table column configuration
#[derive(Clone)]
pub struct AccessibleTableColumn<T: Clone + 'static> {
    pub id: String,
    pub header: String,
    pub sortable: bool,
    pub render: Box<dyn Fn(&T) -> AnyView + 'static>,
}

// Focus trap utility function
fn trap_focus_within_modal(e: &KeyboardEvent, modal_ref: &NodeRef<html::Div>) {
    if let Some(modal) = modal_ref.get() {
        // Get all focusable elements within modal
        let focusable_selector = "button, [href], input, select, textarea, [tabindex]:not([tabindex='-1'])";
        
        if let Ok(focusable_elements) = modal.query_selector_all(focusable_selector) {
            let length = focusable_elements.length();
            if length == 0 { return; }
            
            let first_focusable = focusable_elements.item(0);
            let last_focusable = focusable_elements.item(length - 1);
            
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(active_element) = document.active_element() {
                    if e.shift_key() {
                        // Shift + Tab
                        if let (Some(last), Some(active)) = (&last_focusable, active_element.as_ref()) {
                            if last.is_same_node(Some(active)) {
                                e.prevent_default();
                                if let Some(first) = &first_focusable {
                                    if let Ok(html_element) = first.dyn_ref::<HtmlElement>() {
                                        let _ = html_element.focus();
                                    }
                                }
                            }
                        }
                    } else {
                        // Tab
                        if let (Some(first), Some(active)) = (&first_focusable, active_element.as_ref()) {
                            if first.is_same_node(Some(active)) {
                                e.prevent_default();
                                if let Some(last) = &last_focusable {
                                    if let Ok(html_element) = last.dyn_ref::<HtmlElement>() {
                                        let _ = html_element.focus();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Executive Breadcrumb Navigation with keyboard support
#[component]
pub fn AccessibleExecutiveBreadcrumb(
    /// Breadcrumb items
    items: Vec<BreadcrumbItem>,
) -> impl IntoView {
    view! {
        <nav 
            class="breadcrumbs text-sm"
            aria-label="Breadcrumb navigation"
        >
            <ul role="list">
                {items.into_iter().enumerate().map(|(index, item)| {
                    let is_current = item.current;
                    let is_last = index == items.len() - 1;
                    
                    view! {
                        <li role="listitem">
                            {if is_current {
                                view! {
                                    <span 
                                        class="text-base-content font-medium"
                                        aria-current="page"
                                    >
                                        {item.label}
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <a 
                                        href=item.href.unwrap_or("#".to_string())
                                        class="link link-primary hover:link-secondary
                                               focus:ring-2 focus:ring-primary/50 focus:ring-offset-1
                                               focus:outline-none rounded px-1"
                                        tabindex="0"
                                    >
                                        {item.label}
                                    </a>
                                }.into_any()
                            }}
                        </li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </nav>
    }
}

#[derive(Clone)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
    pub current: bool,
}