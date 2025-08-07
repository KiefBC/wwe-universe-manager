use leptos::prelude::*;
use serde::{Deserialize, Serialize};

/// Standardized error state for consistent error handling across components
///
/// **Note**: Many components are future infrastructure for comprehensive error handling.

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppError {
    /// Network or API related errors
    Network(String),
    /// Validation errors with field-specific information
    Validation { field: String, message: String },
    /// General application errors
    General(String),
    /// Authentication/authorization errors
    Auth(String),
}

impl AppError {
    #[allow(dead_code)]
    pub fn network(message: impl Into<String>) -> Self {
        Self::Network(message.into())
    }
    
    #[allow(dead_code)]
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }
    
    #[allow(dead_code)]
    pub fn general(message: impl Into<String>) -> Self {
        Self::General(message.into())
    }
    
    #[allow(dead_code)]
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth(message.into())
    }
    
    /// Get the error message for display
    pub fn message(&self) -> &str {
        match self {
            AppError::Network(msg) => msg,
            AppError::Validation { message, .. } => message,
            AppError::General(msg) => msg,
            AppError::Auth(msg) => msg,
        }
    }
    
    /// Get the error type for styling purposes
    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::Network(_) => "network",
            AppError::Validation { .. } => "validation",
            AppError::General(_) => "general",
            AppError::Auth(_) => "auth",
        }
    }
}

/// Standardized loading state for consistent loading patterns
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum LoadingState {
    /// Not loading
    Idle,
    /// Loading with optional message
    Loading(Option<String>),
    /// Operation completed successfully
    Success(Option<String>),
    /// Operation failed with error
    Error(AppError),
}

impl LoadingState {
    pub fn is_loading(&self) -> bool {
        matches!(self, LoadingState::Loading(_))
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self, LoadingState::Success(_))
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, LoadingState::Error(_))
    }
    
    #[allow(dead_code)]
    pub fn is_idle(&self) -> bool {
        matches!(self, LoadingState::Idle)
    }
    
    pub fn success_message(&self) -> Option<&str> {
        match self {
            LoadingState::Success(Some(msg)) => Some(msg),
            _ => None,
        }
    }
    
    pub fn error(&self) -> Option<&AppError> {
        match self {
            LoadingState::Error(err) => Some(err),
            _ => None,
        }
    }
    
    pub fn loading_message(&self) -> Option<&str> {
        match self {
            LoadingState::Loading(Some(msg)) => Some(msg),
            _ => None,
        }
    }
}

impl Default for LoadingState {
    fn default() -> Self {
        Self::Idle
    }
}

/// Component for displaying standardized error messages
#[component]
pub fn ErrorDisplay(
    /// The error to display
    error: Signal<Option<AppError>>,
    /// Optional custom class for styling
    #[prop(optional)]
    class: Option<String>,
) -> impl IntoView {
    let class_name = class.unwrap_or_default();
    view! {
        <Show when=move || error.get().is_some()>
            <div class=format!("alert alert-error {}", class_name)>
                <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <div>
                    <h3 class="font-bold">
                        {move || {
                            match error.get().as_ref().map(|e| e.error_type()) {
                                Some("network") => "Network Error",
                                Some("validation") => "Validation Error", 
                                Some("auth") => "Authentication Error",
                                _ => "Error"
                            }
                        }}
                    </h3>
                    <div class="text-xs">
                        {move || error.get().map(|e| e.message().to_string()).unwrap_or_default()}
                    </div>
                </div>
            </div>
        </Show>
    }
}

/// Component for displaying standardized loading states
#[component]
#[allow(dead_code)]
pub fn LoadingDisplay(
    /// The loading state to display
    #[allow(unused_variables)]
    loading_state: ReadSignal<LoadingState>,
    /// Optional custom class for styling
    #[prop(optional)]
    #[allow(unused_variables)]
    class: Option<String>,
) -> impl IntoView {
    let class_name = class.unwrap_or_default();
    view! {
        <div class=format!("loading-display {}", class_name)>
            <Show when=move || loading_state.get().is_loading()>
                <div class="flex justify-center items-center py-4">
                    <span class="loading loading-spinner loading-md text-accent"></span>
                    <Show when=move || loading_state.get().loading_message().is_some()>
                        <span class="ml-3 text-base-content/70">
                            {move || loading_state.get().loading_message().unwrap_or("").to_string()}
                        </span>
                    </Show>
                </div>
            </Show>
            
            <Show when=move || loading_state.get().is_success()>
                <div class="alert alert-success">
                    <svg xmlns="http://www.w3.org/2000/svg" class="stroke-current shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                    <span>
                        {move || loading_state.get().success_message().unwrap_or("Operation completed successfully").to_string()}
                    </span>
                </div>
            </Show>
            
            <Show when=move || loading_state.get().is_error()>
                <ErrorDisplay error=Signal::derive(move || loading_state.get().error().cloned()) />
            </Show>
        </div>
    }
}