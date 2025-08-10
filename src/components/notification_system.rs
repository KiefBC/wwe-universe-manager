use leptos::prelude::*;
use leptos::html;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;

/// Professional Notification System for WWE Universe Manager
/// 
/// Provides executive-quality toast notifications, alerts, and feedback
/// with consistent animations and positioning

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub duration: u32,
    pub actions: Vec<NotificationAction>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
    Loading,
}

#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub label: String,
    pub variant: String,
}

/// Global notification manager
pub struct NotificationManager {
    notifications: RwSignal<HashMap<String, Notification>>,
    max_notifications: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: RwSignal::new(HashMap::new()),
            max_notifications: 5,
        }
    }

    pub fn show_success(&self, title: &str, message: &str) -> String {
        self.add_notification(Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Success,
            duration: 4000,
            actions: vec![],
            timestamp: js_sys::Date::now() as u64,
        })
    }

    pub fn show_error(&self, title: &str, message: &str) -> String {
        self.add_notification(Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Error,
            duration: 6000,
            actions: vec![],
            timestamp: js_sys::Date::now() as u64,
        })
    }

    pub fn show_warning(&self, title: &str, message: &str) -> String {
        self.add_notification(Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Warning,
            duration: 5000,
            actions: vec![],
            timestamp: js_sys::Date::now() as u64,
        })
    }

    pub fn show_info(&self, title: &str, message: &str) -> String {
        self.add_notification(Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Info,
            duration: 4000,
            actions: vec![],
            timestamp: js_sys::Date::now() as u64,
        })
    }

    pub fn show_loading(&self, title: &str, message: &str) -> String {
        self.add_notification(Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: message.to_string(),
            notification_type: NotificationType::Loading,
            duration: 0, // No auto-dismiss for loading
            actions: vec![],
            timestamp: js_sys::Date::now() as u64,
        })
    }

    pub fn dismiss(&self, id: &str) {
        self.notifications.update(|notifications| {
            notifications.remove(id);
        });
    }

    pub fn dismiss_all(&self) {
        self.notifications.update(|notifications| {
            notifications.clear();
        });
    }

    fn add_notification(&self, notification: Notification) -> String {
        let id = notification.id.clone();
        let duration = notification.duration;
        
        self.notifications.update(|notifications| {
            // Remove oldest if at limit
            if notifications.len() >= self.max_notifications {
                if let Some(oldest_key) = notifications.keys()
                    .min_by_key(|k| notifications[*k].timestamp)
                    .cloned()
                {
                    notifications.remove(&oldest_key);
                }
            }
            
            notifications.insert(id.clone(), notification);
        });

        // Auto-dismiss if duration > 0
        if duration > 0 {
            let manager = self.clone();
            let notification_id = id.clone();
            spawn_local(async move {
                gloo_timers::future::TimeoutFuture::new(duration).await;
                manager.dismiss(&notification_id);
            });
        }

        id
    }
}

impl Clone for NotificationManager {
    fn clone(&self) -> Self {
        Self {
            notifications: self.notifications,
            max_notifications: self.max_notifications,
        }
    }
}

/// Professional toast notification component
#[component]
pub fn ExecutiveToast(
    /// Notification data
    notification: Notification,
    /// Dismiss handler
    on_dismiss: Box<dyn Fn() + 'static>,
    /// Position in stack (for staggered animation)
    #[prop(default = 0)]
    position: usize,
) -> impl IntoView {
    let toast_ref = NodeRef::<html::Div>::new();

    let (alert_class, icon_svg, bg_class) = match notification.notification_type {
        NotificationType::Success => (
            "alert-success border-success/20", 
            view! {
                <div class="w-8 h-8 bg-success/20 rounded-lg flex items-center justify-center animate-success-bounce">
                    <svg class="w-5 h-5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                </div>
            },
            "bg-gradient-to-r from-success/5 to-success/10"
        ),
        NotificationType::Error => (
            "alert-error border-error/20",
            view! {
                <div class="w-8 h-8 bg-error/20 rounded-lg flex items-center justify-center animate-error-shake">
                    <svg class="w-5 h-5 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                </div>
            },
            "bg-gradient-to-r from-error/5 to-error/10"
        ),
        NotificationType::Warning => (
            "alert-warning border-warning/20",
            view! {
                <div class="w-8 h-8 bg-warning/20 rounded-lg flex items-center justify-center animate-professional-pulse">
                    <svg class="w-5 h-5 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.732 15c-.77.833.192 2.5 1.732 2.5z"></path>
                    </svg>
                </div>
            },
            "bg-gradient-to-r from-warning/5 to-warning/10"
        ),
        NotificationType::Info => (
            "alert-info border-info/20",
            view! {
                <div class="w-8 h-8 bg-info/20 rounded-lg flex items-center justify-center">
                    <svg class="w-5 h-5 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                </div>
            },
            "bg-gradient-to-r from-info/5 to-info/10"
        ),
        NotificationType::Loading => (
            "alert-info border-info/20",
            view! {
                <div class="w-8 h-8 bg-info/20 rounded-lg flex items-center justify-center">
                    <span class="loading loading-spinner loading-sm text-info"></span>
                </div>
            },
            "bg-gradient-to-r from-info/5 to-info/10"
        ),
    };

    let animation_delay = format!("animation-delay: {}ms", position * 100);

    view! {
        <div 
            node_ref=toast_ref
            class=format!("alert shadow-lg border backdrop-blur-sm {} {} animate-toast-slide-in", alert_class, bg_class)
            style=animation_delay
        >
            {icon_svg}
            
            <div class="flex-1 min-w-0">
                <div class="font-semibold text-base-content">{notification.title}</div>
                {if !notification.message.is_empty() {
                    view! {
                        <div class="text-sm text-base-content/80 mt-1">{notification.message}</div>
                    }
                } else {
                    view! {}
                }}
                
                // Action buttons
                {if !notification.actions.is_empty() {
                    view! {
                        <div class="flex gap-2 mt-3">
                            {notification.actions.into_iter().map(|action| {
                                let button_class = match action.variant.as_str() {
                                    "primary" => "btn-primary",
                                    "secondary" => "btn-secondary", 
                                    "ghost" => "btn-ghost",
                                    _ => "btn-outline",
                                };
                                
                                view! {
                                    <button 
                                        class=format!("btn btn-xs {} transition-all duration-quick hover:animate-button-press", button_class)
                                        // Note: action.action would need to be handled differently in real implementation
                                    >
                                        {action.label}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                } else {
                    view! {}
                }}
            </div>
            
            // Dismiss button
            {if matches!(notification.notification_type, NotificationType::Loading) {
                view! {}.into_any()
            } else {
                view! {
                    <button 
                        class="btn btn-sm btn-circle btn-ghost opacity-70 hover:opacity-100 transition-opacity duration-quick hover:animate-button-press" 
                        on:click=move |_| on_dismiss()
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                }.into_any()
            }}
        </div>
    }
}

/// Executive notification container
#[component]
pub fn ExecutiveNotificationContainer(
    /// Notification manager
    manager: NotificationManager,
    /// Container position
    #[prop(default = "top-right".to_string())]
    position: String,
) -> impl IntoView {
    let position_class = match position.as_str() {
        "top-left" => "toast toast-top toast-start",
        "top-center" => "toast toast-top toast-center",
        "bottom-left" => "toast toast-bottom toast-start", 
        "bottom-center" => "toast toast-bottom toast-center",
        "bottom-right" => "toast toast-bottom toast-end",
        _ => "toast toast-top toast-end",
    };

    let notifications = manager.notifications;

    view! {
        <div class=format!("{} z-50", position_class)>
            {move || {
                let current_notifications: Vec<_> = notifications.get()
                    .values()
                    .cloned()
                    .collect();
                
                // Sort by timestamp (newest first)
                let mut sorted_notifications = current_notifications;
                sorted_notifications.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                
                sorted_notifications.into_iter().enumerate().map(|(index, notification)| {
                    let notification_id = notification.id.clone();
                    let dismiss_handler = {
                        let manager_clone = manager.clone();
                        let id = notification_id.clone();
                        move || manager_clone.dismiss(&id)
                    };
                    
                    view! {
                        <ExecutiveToast 
                            notification=notification
                            on_dismiss=Box::new(dismiss_handler)
                            position=index
                        />
                    }
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}

/// Professional alert banner for page-level notifications
#[component]
pub fn ExecutiveAlertBanner(
    /// Alert title
    title: String,
    /// Alert message
    message: String,
    /// Alert type
    #[prop(default = "info".to_string())]
    alert_type: String,
    /// Show state
    show: ReadSignal<bool>,
    /// Dismiss handler
    #[prop(optional)]
    on_dismiss: Option<Box<dyn Fn() + 'static>>,
    /// Call-to-action button
    #[prop(optional)]
    action_label: Option<String>,
    /// Action handler
    #[prop(optional)]
    on_action: Option<Box<dyn Fn() + 'static>>,
) -> impl IntoView {
    let (alert_class, icon_svg, bg_class) = match alert_type.as_str() {
        "success" => (
            "alert-success border-success/20",
            view! {
                <svg class="w-6 h-6 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                </svg>
            },
            "bg-gradient-to-r from-success/10 to-success/5"
        ),
        "error" => (
            "alert-error border-error/20",
            view! {
                <svg class="w-6 h-6 text-error" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
            },
            "bg-gradient-to-r from-error/10 to-error/5"
        ),
        "warning" => (
            "alert-warning border-warning/20",
            view! {
                <svg class="w-6 h-6 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.732 15c-.77.833.192 2.5 1.732 2.5z"></path>
                </svg>
            },
            "bg-gradient-to-r from-warning/10 to-warning/5"
        ),
        _ => (
            "alert-info border-info/20",
            view! {
                <svg class="w-6 h-6 text-info" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
            },
            "bg-gradient-to-r from-info/10 to-info/5"
        ),
    };

    view! {
        <div class=move || if show.get() { 
            format!("alert shadow-lg border backdrop-blur-sm {} {} animate-card-entrance mb-6", alert_class, bg_class)
        } else { 
            "hidden".to_string() 
        }>
            {icon_svg}
            
            <div class="flex-1 min-w-0">
                <div class="font-bold text-lg">{title}</div>
                <div class="text-sm opacity-80 mt-1">{message}</div>
            </div>
            
            // Action button
            {if let Some(action_text) = action_label {
                let action_handler = on_action.unwrap_or_else(|| Box::new(|| {}));
                view! {
                    <button 
                        class="btn btn-sm btn-outline transition-all duration-quick hover:animate-button-press"
                        on:click=move |_| action_handler()
                    >
                        {action_text}
                    </button>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
            
            // Dismiss button
            {if let Some(dismiss_handler) = on_dismiss {
                view! {
                    <button 
                        class="btn btn-sm btn-circle btn-ghost opacity-70 hover:opacity-100 transition-all duration-quick hover:animate-button-press"
                        on:click=move |_| dismiss_handler()
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                    </button>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}

/// Quick notification helper functions
pub fn show_success(manager: &NotificationManager, message: &str) {
    manager.show_success("Success", message);
}

pub fn show_error(manager: &NotificationManager, message: &str) {
    manager.show_error("Error", message);
}

pub fn show_info(manager: &NotificationManager, message: &str) {
    manager.show_info("Information", message);
}

pub fn show_warning(manager: &NotificationManager, message: &str) {
    manager.show_warning("Warning", message);
}