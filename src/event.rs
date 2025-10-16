use futures::channel::mpsc::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

use crate::vector2::Vector2;
use crate::{alert, shared::*};
use crate::shared::window;

pub enum Event {
    ResizeEvent {
        width: f64,
        height: f64
    },
    PointerDownEvent {
        position: Vector2,
        pointertype: i8,
        button: i8,
        pointerid: i32,
        isprimary: bool
    },
    PointerMoveEvent {
        position: Vector2,
        movement: Vector2,
        pointertype: i8,
        pointerid: i32,
        isprimary: bool    
    },
    PointerUpEvent {
        position: Vector2,
        pointertype: i8,
        button: i8,
        pointerid: i32,
        isprimary: bool
    }
}

pub fn setup_event_listeners(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d, event_listener: UnboundedSender<Event>) {
    // window resize
    {
        let event_listener = event_listener.clone();
        let a = Closure::wrap(Box::new(move || {
            let width= window().inner_width().unwrap().as_f64().unwrap()*window().device_pixel_ratio();
            let height= window().inner_height().unwrap().as_f64().unwrap()*window().device_pixel_ratio();
            event_listener.unbounded_send(Event::ResizeEvent { width: width, height: height }).expect("huh?");
        }) as Box<dyn FnMut()>);
        window().add_event_listener_with_callback("resize", a.as_ref().unchecked_ref()).expect("should add event listener");
        a.forget();
    }
    // canvas pointerdown
    {
        let event_listener = event_listener.clone();
        let a = Closure::wrap(Box::new(move |event: PointerEvent| {            
            event_listener.unbounded_send(
                Event::PointerDownEvent { 
                    position: Vector2 { x: event.x() as f64, y: event.y() as f64 } * window().device_pixel_ratio(), 
                    pointertype: match event.pointer_type().as_str() {
                        "mouse" => 0,
                        "touch" => 1,
                        "pen" => 2,
                        _ => -1
                    }, 
                    button: event.button() as i8, 
                    pointerid: event.pointer_id(),
                    isprimary: event.is_primary()
                }
            ).expect("huh?");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("pointerdown", a.as_ref().unchecked_ref()).expect("should add event listener");
        a.forget();
    }
    // canvas pointermove
    {
        let event_listener = event_listener.clone();
        let a = Closure::wrap(Box::new(move |event: PointerEvent| {            
            event.prevent_default();
            event_listener.unbounded_send(
                Event::PointerMoveEvent { 
                    position: Vector2 { x: event.x() as f64, y: event.y() as f64 } * window().device_pixel_ratio(), 
                    movement: Vector2 { x: event.movement_x() as f64, y: event.movement_y() as f64 } * window().device_pixel_ratio(),
                    pointertype: match event.pointer_type().as_str() {
                        "mouse" => 0,
                        "touch" => 1,
                        "pen" => 2,
                        _ => -1
                    }, 
                    pointerid: event.pointer_id(),
                    isprimary: event.is_primary()
                }
            ).expect("huh?");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("pointermove", a.as_ref().unchecked_ref()).expect("should add event listener");
        a.forget();
    }
    // canvas pointerup
    {
        let event_listener = event_listener.clone();
        let a = Closure::wrap(Box::new(move |event: PointerEvent| {            
            event_listener.unbounded_send(
                Event::PointerUpEvent { 
                    position: Vector2 { x: event.x() as f64, y: event.y() as f64 } * window().device_pixel_ratio(), 
                    pointertype: match event.pointer_type().as_str() {
                        "mouse" => 0,
                        "touch" => 1,
                        "pen" => 2,
                        _ => -1
                    }, 
                    button: event.button() as i8, 
                    pointerid: event.pointer_id(),
                    isprimary: event.is_primary()
                }
            ).expect("huh?");
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("pointerup", a.as_ref().unchecked_ref()).expect("should add event listener");
        canvas.add_event_listener_with_callback("pointercancel", a.as_ref().unchecked_ref()).expect("should add event listener");
        //canvas.add_event_listener_with_callback("pointerout", a.as_ref().unchecked_ref()).expect("should add event listener");
        //canvas.add_event_listener_with_callback("pointerleave", a.as_ref().unchecked_ref()).expect("should add event listener");
        a.forget();
    }
} 