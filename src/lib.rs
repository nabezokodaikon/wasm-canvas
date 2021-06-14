use js_sys::Promise;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::console::log_1;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    fn sleep(ms: f64) -> Promise;
}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    #[allow(unused_unsafe)]
    unsafe {
        log_1(&JsValue::from(String::from(s)));
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    #[allow(unused_unsafe)]
    unsafe {
        alert(&format!("Hello, {}!", name));
    }
}

#[wasm_bindgen]
pub async fn timer(count: u32) -> Result<JsValue, JsValue> {
    for i in 1..count {
        unsafe {
            JsFuture::from(sleep(1000f64)).await?;
        }
        console_log(&i.to_string());
    }

    Ok(JsValue::undefined())
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}
