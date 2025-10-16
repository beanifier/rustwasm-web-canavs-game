#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;
use futures::channel::mpsc::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlImageElement;

use crate::shared::*;
use crate::event::*;
use crate::vector2::*;

pub trait Drawable {
    fn draw(self: &Self, ctx: &CanvasRenderingContext2d, width: f64, height: f64);
}

#[derive(Clone)]
pub struct BasicDrawableImage {
    pub image: HtmlImageElement,
    pub position: Vector2,
    pub size: Vector2
}

impl Drawable for BasicDrawableImage {
    fn draw(self: &Self, ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
        ctx.draw_image_with_html_image_element_and_dw_and_dh(&self.image, self.position.x, self.position.y, self.size.x, self.size.y);
    }
}