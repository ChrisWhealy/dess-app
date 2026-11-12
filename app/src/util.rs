//! The few helpers that the panel needs from the page.

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// The page's document, or why there is none.
pub fn document() -> Result<web_sys::Document, String> {
    web_sys::window()
        .ok_or_else(|| "no global window".to_owned())?
        .document()
        .ok_or_else(|| "no document on window".to_owned())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Looks up `id` in `document`, or `Err` if the markup does not define it.
pub fn required_element(document: &web_sys::Document, id: &str) -> Result<web_sys::Element, String> {
    document
        .get_element_by_id(id)
        .ok_or_else(|| format!("the page must define #{id}"))
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Says `message` in the console and in the banner `banner_id`, which is shown.
pub fn report_error(banner_id: &str, message: &str) {
    web_sys::console::error_1(&message.into());
    let Ok(document) = document() else { return };
    if let Some(banner) = document.get_element_by_id(banner_id) {
        banner.set_text_content(Some(message));
        let _ = banner.remove_attribute("hidden");
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
/// Hides the banner `banner_id` and empties it, once what failed has worked again.
pub fn clear_error(banner_id: &str) {
    let Ok(document) = document() else { return };
    if let Some(banner) = document.get_element_by_id(banner_id) {
        banner.set_text_content(None);
        let _ = banner.set_attribute("hidden", "");
    }
}
