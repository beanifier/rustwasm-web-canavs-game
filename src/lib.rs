use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};

#[allow(deprecated)]

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    ctx.set_fill_style(&"#222".into());

    ctx.fill_rect(0.0, 0.0, 800.0, 600.0);

    ctx.set_fill_style(&"#0f0".into());
    ctx.fill_rect(100.0, 100.0, 50.0, 50.0);

    Ok(())
}

pub fn game_loop() -> Result<(), JsValue> {
    Ok(())
}

