#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  console_error_panic_hook::set_once();
  leptos::mount::hydrate_body(site_app::App);
}
