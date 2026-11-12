//! The page in a real browser: the panel is built into a page that is made as `web/index.html` makes it, a person's
//! clicks are made on it, and it is taken down and built again. Run with `wasm-pack test --headless --chrome app`.
#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

fn document() -> web_sys::Document {
    web_sys::window().and_then(|w| w.document()).expect("a document")
}

/// A container of the test's own in the test page, which is emptied. The runner writes its report into the body, so the body is left alone.
fn fixture(markup: &str) {
    let document = document();
    if let Some(old) = document.get_element_by_id("fixture") {
        old.remove();
    }
    let holder = document.create_element("div").expect("a div");
    holder.set_id("fixture");
    holder.set_inner_html(markup);
    document.body().expect("a body").append_child(&holder).expect("appended");
}

/// The page's own shell, as `web/index.html` has it.
fn page() {
    fixture(r#"<main id="stage"><p class="demo-error" id="page-error" hidden></p><div id="app"></div></main>"#);
}

fn text(id: &str) -> String {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("no #{id}"))
        .text_content()
        .unwrap_or_default()
        .trim()
        .to_owned()
}

fn click(id: &str) {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("no #{id}"))
        .dyn_into::<web_sys::HtmlElement>()
        .expect("a button")
        .click();
}

/// Waits, a tick at a time, until `done` is true, or says why not after two seconds.
async fn until(what: &str, done: impl Fn() -> bool) -> Result<(), String> {
    for _ in 0..80 {
        if done() {
            return Ok(());
        }
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            let _ = web_sys::window()
                .expect("a window")
                .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 25);
        });
        JsFuture::from(promise).await.map_err(|e| format!("{e:?}"))?;
    }
    Err(format!("it did not happen in two seconds: {what}"))
}

fn scenes() -> u32 {
    document()
        .query_selector_all("#dess-stage svg")
        .map(|list| list.length())
        .unwrap_or(0)
}

#[wasm_bindgen_test]
async fn the_panel_is_built_stepped_switched_taken_down_and_built_again() -> Result<(), String> {
    page();
    dess_app::start().map_err(|e| format!("{e:?}"))?;
    assert_eq!(
        text("dess-i-snapshot"),
        "0 of 12",
        "the first scenario opens at its first snapshot"
    );
    assert_eq!(scenes(), 1, "one scene is drawn");
    assert!(
        document()
            .get_element_by_id("page-error")
            .is_some_and(|e| e.has_attribute("hidden")),
        "no error is shown"
    );

    for _ in 0..3 {
        click("dess-next");
    }
    until("three clicks of Next reach snapshot 3", || text("dess-i-snapshot") == "3 of 12").await?;
    click("dess-prev");
    until("Previous goes back to 2", || text("dess-i-snapshot") == "2 of 12").await?;

    // A scenario with another number of servers is built in a staged host and handed over, and there is still one scene.
    let list = document()
        .get_element_by_id("dess-scenario")
        .and_then(|e| e.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .expect("the scenario list");
    list.set_value("five");
    let change = web_sys::Event::new("change").map_err(|e| format!("{e:?}"))?;
    list.dispatch_event(&change).map_err(|e| format!("{e:?}"))?;
    until("the five-server scenario is shown", || {
        document()
            .query_selector_all("#dess-stage [data-ss-key]")
            .map(|l| l.length())
            .unwrap_or(0)
            > 0
            && text("dess-i-snapshot").starts_with("0 of ")
    })
    .await?;
    assert_eq!(scenes(), 1, "the handover leaves one scene");

    dess_app::dispose().map_err(|e| format!("{e:?}"))?;
    assert_eq!(scenes(), 0, "disposing takes the scene away");
    dess_app::dispose().map_err(|e| format!("{e:?}"))?;

    // The page's shell is written again, as a reload would, and the panel is built once more.
    page();
    dess_app::start().map_err(|e| format!("{e:?}"))?;
    assert_eq!(text("dess-i-snapshot"), "0 of 12", "a second build opens at its first snapshot");
    assert_eq!(scenes(), 1);
    dess_app::dispose().map_err(|e| format!("{e:?}"))
}

#[wasm_bindgen_test]
fn a_page_without_its_app_element_says_so() {
    fixture("");
    let error = dess_app::start().expect_err("there is nowhere to write the panel");
    assert!(format!("{error:?}").contains("#app"), "{error:?}");
}

/// How many elements the scene holds, which is the work of every redraw and every drag.
fn elements() -> u32 {
    document()
        .query_selector_all("#dess-stage svg *")
        .map(|list| list.length())
        .unwrap_or(0)
}

/// The budget on what is drawn: however long the run is, the scene holds a fixed set of elements, and a run that is longer
/// than its declared ceiling is refused before anything is drawn. See `docs/budgets.md`.
#[wasm_bindgen_test]
async fn the_scene_does_not_grow_with_the_length_of_the_run() -> Result<(), String> {
    page();
    dess_app::start().map_err(|e| format!("{e:?}"))?;
    let list = document()
        .get_element_by_id("dess-scenario")
        .and_then(|e| e.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .expect("the scenario list");
    list.set_value("scaled");
    let change = web_sys::Event::new("change").map_err(|e| format!("{e:?}"))?;
    list.dispatch_event(&change).map_err(|e| format!("{e:?}"))?;
    until("the busy scenario is shown", || text("dess-i-snapshot").starts_with("0 of ")).await?;
    let last: u32 = text("dess-i-snapshot")
        .split(" of ")
        .nth(1)
        .and_then(|n| n.parse().ok())
        .ok_or("no length")?;
    assert!(last > 100, "the busy scenario is long enough to show growth: {last} snapshots");
    let at_start = elements();

    let seek = document()
        .get_element_by_id("dess-seek")
        .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
        .expect("the range");
    seek.set_value(&last.to_string());
    let input = web_sys::Event::new("input").map_err(|e| format!("{e:?}"))?;
    seek.dispatch_event(&input).map_err(|e| format!("{e:?}"))?;
    until("the end of the run", || text("dess-i-snapshot") == format!("{last} of {last}")).await?;
    let at_end = elements();
    web_sys::console::log_1(
        &format!("elements: {at_start} at the start, {at_end} at the end of {last} snapshots").into(),
    );
    assert!(
        at_end <= at_start + at_start / 2,
        "the scene grew from {at_start} to {at_end} elements over {last} snapshots"
    );
    dess_app::dispose().map_err(|e| format!("{e:?}"))
}
