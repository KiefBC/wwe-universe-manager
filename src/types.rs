use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ShowData {
    pub name: String,
    pub description: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Fetches shows from the backend via Tauri
pub async fn fetch_shows() -> Result<Vec<Show>, String> {
    let result_js = invoke("get_shows", JsValue::NULL).await;

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize shows: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}

/// Creates a new show via Tauri
pub async fn create_show(show_data: ShowData) -> Result<Show, String> {
    console::log_1(&format!("create_show called with: {:?}", show_data).into());

    // Tauri expects arguments to be wrapped in an object with parameter names as keys
    let args = serde_json::json!({
        "showData": show_data
    });

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(|e| {
        let error_msg = format!("Failed to serialize show data: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })?;

    console::log_1(&"Invoking Tauri command 'create_show'...".into());
    let result_js = invoke("create_show", args_value).await;
    console::log_1(&format!("Tauri command returned: {:?}", result_js).into());

    serde_wasm_bindgen::from_value(result_js).map_err(|e| {
        let error_msg = format!("Failed to deserialize show result: {}", e);
        console::log_1(&error_msg.clone().into());
        error_msg
    })
}
