use leptos::html;
use leptos::{ev::SubmitEvent, prelude::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);
    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <ShowSelector />
      // <button on:click=move |_| {
      //     set_count.update(|n| *n += 1);
      //   }
      // >
      //   "Click me: "
      //   {count}
      // </button>

      // <br/>

      // <input type="text"
      //     bind:value=(name, set_name)
      // />
      // <input type="email"
      //     bind:value=email
      // />
      // <label>
      //     "Please send me lots of spam email."
      //     <input type="checkbox"
      //         bind:checked=spam_me
      //     />
      // </label>
      // <p>"Name is: " {name}</p>
      // <p>"Email is: " {email}</p>
      // <Show when=move || spam_me.get()>
      //     <p>"You’ll receive cool bonus content!"</p>
      // </Show>

      // <br/>

      // <form on:submit=on_submit> // on_submit defined below
      //     <input type="text"
      //         value=name
      //         node_ref=input_element
      //     />
      //     <input type="submit" value="Submit"/>
      // </form>
      // <p>"Name is: " {name}</p>

      // <br/>
      // <SelectInput/>
      // <br/>
      // <NumericInput/>
      // <br/>
      // <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
      //     // these get passed to `children`
      //     "Some text"
      //     <br/>
      //     <span>"A span"</span>
      // </TakesChildren>
      // <br/>
      // <WrapsChildren>
      //     "A"
      //     "B"
      //     "C"
      // </WrapsChildren>
    }
}

#[component]
fn ShowSelector() -> impl IntoView {
    let (show, set_show) = signal("Show One".to_string());

    // List of available shows
    let available_shows = vec![
        "Show One",
        "Show Two",
        "Show Three",
        "Comedy Hour",
        "Drama Special",
    ];

    view! {
        <div class="flex flex-col items-center justify-center">
            <p class="font-bold m-4">"Available Shows"</p>
            <select
              on:change:target=move |ev| {
                  set_show.set(ev.target().value().to_string());
              }
              prop:value=move || show.get()
            >
              {available_shows.into_iter().map(|show_name| {
                  view! { <option value={show_name}>{show_name}</option> }
              }).collect::<Vec<_>>()}
            </select>
            <p class="m-6 text-blue-600">"Selected: " {move || show.get()}</p>
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
        // a button that will cycle through the options
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
                // when input changes, try to parse a number from the input
                set_value.set(ev.target().value().parse::<i32>())
            }/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
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
                    // because `value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `value` changes
                    <strong>{value}</strong>
                </p>
            </ErrorBoundary>
        </label>
    }
}

/// Displays a `render_prop` and some children within markup.
#[component]
pub fn TakesChildren<F, IV>(
    /// Takes a function (type F) that returns anything that can be
    /// converted into a View (type IV)
    render_prop: F,
    /// `children` can take one of several different types, each of which
    /// is a function that returns some view type
    children: Children,
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren/>"</code></h1>
        <h2>"Render Prop"</h2>
        {render_prop()}
        <hr/>
        <h2>"Children"</h2>
        {children()}
    }
}
/// Wraps each child in an `<li>` and embeds them in a `<ul>`.
#[component]
pub fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    // children() returns a `Fragment`, which has a
    // `nodes` field that contains a Vec<View>
    // this means we can iterate over the children
    // to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! {
        <h1><code>"<WrapsChildren/>"</code></h1>
        // wrap our wrapped children in a UL
        <ul>{children}</ul>
    }
}
