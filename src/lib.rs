use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;
use futures::channel::mpsc::*;

use crate::game::GameSystem;
use crate::shared::*;
use crate::shared::window;
use crate::event::*;
use crate::event::Event;
use crate::vector2::Vector2;

mod render;
mod shared;
mod event;
mod vector2;
mod game;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    alert!("hi");
    let document = document();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let mut game = GameSystem::start();
    
    let performance = window().performance().unwrap();

    let (eventtx, mut eventreceive) = unbounded();
    setup_event_listeners(&canvas, &ctx, eventtx.clone());

    let mut width = window().inner_width().unwrap().as_f64().unwrap()*window().device_pixel_ratio();
    let mut height = window().inner_height().unwrap().as_f64().unwrap()*window().device_pixel_ratio();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut prevtime = performance.now();
    let mut totaltime: f64 = 0.0;

    *g.borrow_mut() = Some(Closure::new(move || {
        let curtime = performance.now();
        let deltatime = curtime - prevtime;
        prevtime = curtime;
        totaltime += deltatime;
        game.draw(&ctx, width, height, deltatime, totaltime);


        while let Ok(Some(x)) = eventreceive.try_next() {
            match x {
                Event::ResizeEvent { width: w, height: h } => {
                    width = w; height = h;
                    window().scroll_to_with_x_and_y(0.0, 0.0);
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);
                    game.on_resize(Vector2 { x: width, y: height });
                },
                Event::PointerDownEvent { position, pointertype, button, pointerid, isprimary } => {
                    game.pointer_down(position, pointertype, button, pointerid, isprimary);
                },
                Event::PointerMoveEvent { position, movement, pointertype, pointerid, isprimary } => {
                    game.pointer_move(position, movement, pointertype, pointerid, isprimary);
                },
                Event::PointerUpEvent { position, pointertype, button, pointerid, isprimary } => {
                    game.pointer_up(position, pointertype, button, pointerid, isprimary);
                },
                _ => {
                    alert!("todo");
                }
            }
        }

        
        
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

#[allow(unused_variables)]
pub trait Game {
    fn start() -> Self;

    fn draw(self: &mut Self, ctx: &CanvasRenderingContext2d, width: f64, height: f64, dt: f64, tt: f64);

    fn on_resize(self: &mut Self, size: Vector2) {}

    fn pointer_down(self: &mut Self, position: Vector2, pointertype: i8, button: i8, pointerid: i32, isprimary: bool) {}

    fn pointer_move(self: &mut Self, position: Vector2, movement: Vector2, pointertype: i8, pointerid: i32, isprimary: bool) {}

    fn pointer_up(self: &mut Self, position: Vector2, pointertype: i8, button: i8, pointerid: i32, isprimary: bool) {}
}