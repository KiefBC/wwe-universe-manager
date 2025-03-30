use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use leptos::logging;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Show {
    pub id: i32,
    pub name: String,
    pub description: String,
}

async fn fetch_shows() -> Result<Vec<Show>, String> {
    let args = JsValue::NULL;
    let result_js = invoke("get_shows", args).await;

    match serde_wasm_bindgen::from_value(result_js) {
        Ok(shows) => Ok(shows),
        Err(e) => {
            let error_msg = format!("Failed to deserialize shows: {}", e);
            logging::error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center w-full">
            <img loading="lazy" src="public/logo.png" class="object-center aspect-auto w-64 h-auto max-w-full"/>
        </div>
        <ShowSelector />
        <div class="flex justify-center items-center w-full">
            <button class="btn btn-xs sm:btn-sm md:btn-md lg:btn-lg xl:btn-xl mb-4">Responsive</button>
        </div>
    }
}

#[component]
fn ShowSelector() -> impl IntoView {
    let shows_resource = LocalResource::new(|| async move { fetch_shows().await });

    let (selected_show_name, set_selected_show_name) = signal(String::new());

    let options_view = move || {
        shows_resource.get().map(|result| {
            match &*result {
                Ok(shows) => {
                    if shows.is_empty() {
                         vec![view! { <option value=String::from("") disabled=true>"-- No shows found --"</option> }
                            .into_any()
                         ]
                            .into_view()
                    } else {
                        if selected_show_name.get().is_empty() {
                            set_selected_show_name.set(shows[0].name.clone());
                        }
                        shows.into_iter().map(|show| {
                            let current_name = show.name.clone();
                            let is_selected = selected_show_name.get() == current_name;
                             view! { <option value=current_name selected=is_selected>{show.name.clone()}</option> }
                                .into_any()
                        }).collect_view()
                            .into_view()
                    }
                },
                Err(e) => {
                    vec![view! { <option value="" disabled=true>{format!("-- Error: {} --", e)}</option> }
                        .into_any()
                    ]
                        .into_view()
                }
            }
        })
    };

    view! {
        <div class="flex flex-col justify-center items-center">
            <p class="">"Available Shows"</p>
            <Suspense fallback=move || view! { <p>"Loading shows..."</p> }>
                 <select
                    on:change:target=move |ev| {
                        set_selected_show_name.set(ev.target().value());
                    }
                 >
                    {options_view}
                 </select>
            </Suspense>
            <p class="m-6 text-blue-600">"Selected: " {selected_show_name}</p>
        </div>
    }
}

#[component]
fn SelectInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <select
          on:change:target=move |ev| {
              set_value.set(ev.target().value().parse::<i32>());
          }
          prop:value=move || value.get().expect("REASON").to_string()
        >
          <option value="0">"0"</option>
          <option value="1">"1"</option>
          <option value="2">"2"</option>
          <option value="3">"3"</option>
          <option value="4">"4"</option>
          <option value="5">"5"</option>
          <option value="6">"6"</option>
          <option value="7">"7"</option>
          <option value="8">"8"</option>
          <option value="9">"9"</option>
          <option value="10">"10"</option>
        </select>
        <button on:click=move |_| set_value.update(|n| {
            if *n == Ok(10) {
              *n = Ok(0);
            } else if let Ok(val) = *n {
              *n = Ok(val + 1);
            }
        })>
          "Next Option"
        </button>

        <Show
        when=move || { value.get().map_or(false, |val| val > 5) }
          fallback=|| view! { "<Small/>" }
        >
          "<Big/>"
        </Show>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input:target=move |ev| {
                set_value.set(ev.target().value().parse::<i32>())
            }/>
            <ErrorBoundary
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}
