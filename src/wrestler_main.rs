use leptos::prelude::*;
use wwe_universe_manager_ui::components::WrestlerWindow;

fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| {
        view! {
            <div class="h-full bg-base-100">
                <WrestlerWindow />
            </div>
        }
    });
}