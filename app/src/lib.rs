//! The discrete-event service simulator as an application: a queue and one to five servers, with jobs that arrive, wait,
//! run, time out, retry, fail and are cancelled, drawn on one long-lived `svg-dom-graph` scene.
//!
//! The page is `web/index.html`. It loads this module and calls [`start`], which writes the panel into `#app` and builds it.
//! Everything that the simulator does is in the `service-sim-engine` and `service-sim` crates of this workspace. This crate
//! is the page side of it, and the one place that a browser, a document and a wasm build come in.

mod panel;
mod util;

use wasm_bindgen::prelude::*;

/// The panel's markup, which is written into `#app`, so that the page and the code that finds its elements cannot differ.
const MARKUP: &str = include_str!("panel.html");

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Writes the panel's markup into `#app` and builds the panel.
///
/// # Errors
///
/// Returns the reason as a string if the page has no `#app`, an element of the markup is missing, or the first scenario is
/// refused or cannot be drawn.
#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let document = util::document().map_err(|e| JsValue::from_str(&e))?;
    let app = util::required_element(&document, "app").map_err(|e| JsValue::from_str(&e))?;
    app.set_inner_html(MARKUP);
    panel::build_panel().map_err(|e| JsValue::from_str(&e))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Takes the panel down and leaves the markup as it was written, so that [`start`] can be called again.
///
/// # Errors
///
/// Returns the reason as a string if the page has no document.
#[wasm_bindgen]
pub fn dispose() -> Result<(), JsValue> {
    panel::dispose_panel().map_err(|e| JsValue::from_str(&e))
}
