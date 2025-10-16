use wasm_bindgen::prelude::*;
use web_sys::{Document, Window};

pub fn window() -> Window {
    web_sys::window().expect("no global window")
}

pub fn document() -> Document {
    window().document().expect("no document in window")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window().request_animation_frame(f.as_ref().unchecked_ref()).expect("should be ok");
}

#[macro_export]
macro_rules! htmlimg {
    ($str:expr) => {
        {
            let img = web_sys::HtmlImageElement::new().expect("didnt create image element");
            img.set_src($str);
            img
        }
    };
}

#[macro_export]
macro_rules! alert {
    ($str:expr) => {
        let _ = window().alert_with_message(format!($str).as_str());
    };
}