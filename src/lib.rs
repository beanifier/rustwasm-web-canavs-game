use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::cell::RefCell;
use std::rc::Rc;

fn swindow() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    swindow()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    swindow()
        .document()
        .expect("should have a document on window")
}

#[allow(deprecated)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = document();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    
    let mut x:f64 = 0.0;
    let mut y:f64 = 0.0;
    let mut vx:f64 = 0.005;
    let mut vy:f64 = 0.0066;
    let mut time:i64 = 0;

    // frame function
    let mut frame = move || -> bool {
        time += 1;
        x = x + vx;
        y = y + vy;
        if (x > 1.0) || (0.0 > x) { vx *= -1.0 }
        if (y > 1.0) || (0.0 > y) { vy *= -1.0 }

        ctx.set_fill_style(&"#000".into());
        ctx.fill_rect(0.0, 0.0, 800.0, 600.0);
        ctx.set_fill_style_str(&(format!("hsl({} 100% 50%)", time)));
        ctx.fill_rect(x*750.0, y*550.0, 50.0, 50.0);
        true 
    };

    // frame callback closure
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        if !(frame()) {return}

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}