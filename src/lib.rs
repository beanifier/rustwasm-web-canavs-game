use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
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

fn create_img_element_from_url(s: &str) -> HtmlImageElement {
    let elem = HtmlImageElement::new().expect("Error making img element");
    elem.set_src(s);
    elem
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
    let animtging = document.timeline();
    let catimg = create_img_element_from_url("./cat.png");
    
    let mut x:f64 = 0.0;
    let mut y:f64 = 0.0;
    let mut vx:f64 = 0.0005;
    let mut vy:f64 = 0.0006;
    let mut prevtime:f64 = animtging.current_time().expect("time??");
    let mut totaltime:f64 = 0.0;
    // frame function
    let mut frame = move |curtime: f64| -> bool {
        let deltatime = curtime - prevtime;
        prevtime = curtime;
        totaltime += deltatime;
        x = x + (vx*deltatime);
        y = y + (vy*deltatime);
        if (x > 1.0) || (0.0 > x) { vx *= -1.0 }
        if (y > 1.0) || (0.0 > y) { vy *= -1.0 }

        ctx.set_fill_style_str(&"#000");
        ctx.fill_rect(0.0, 0.0, 800.0, 600.0);

        ctx.set_fill_style_str(&(format!("hsl({} 100% 50%)", totaltime * 0.5)));
        ctx.fill_rect(x*750.0, y*550.0, 50.0, 50.0);
        ctx.draw_image_with_html_image_element_and_dw_and_dh(&catimg, x*750.0, y*550.0, 50.0, 50.0).unwrap();

        ctx.set_fill_style_str(&"#FFF");
        ctx.set_font(&"bold 1em monospace");
        ctx.fill_text(&(format!("dt: {} tt: {} fps: {}", deltatime, totaltime, (1000.0/deltatime).round())), 0.0, 10.0).unwrap();
        true 
    };

    // frame callback closure
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        if !(frame(animtging.current_time().expect("time??"))) {return}

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}