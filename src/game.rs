use std::ops::IndexMut;
use rand::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[allow(unused)]
use crate::{alert};
use crate::{vector2::Vector2};


pub struct GameSystem {
    bird: Bird,
    pipes: Vec<Pipe>,
    tim: f64,
    random: ThreadRng,
    started: bool,
    score: i32,
    width: f64,
}

fn worldtoscreen(world: Vector2, width: f64, height: f64, scale: f64) -> Vector2 {
    let minsize = f64::min(width, height)/scale;
    (world*minsize)+(Vector2{x:width,y:height}/2.0)
}

fn pointinbox(boxcenter: Vector2, boxsize: Vector2, point: Vector2) -> bool {
    let min = boxcenter - (boxsize/2.0);
    let max = boxcenter + (boxsize/2.0);
    (point.x >= min.x)&&(point.y >= min.y)&&(point.x <= max.x)&&(point.y <= max.y)
}

impl crate::Game for GameSystem {
    fn start() -> Self {
        Self {
            bird: Bird { 
                position: Vector2::zero(), 
                velocity: Vector2::zero(),
                direction: false,
            },
            pipes: Vec::new(),
            tim: 0.0,
            random: rand::rng(),
            started: false,
            score: 0,
            width: 0.0
        }
    }
    fn draw(self: &mut Self, ctx: &CanvasRenderingContext2d, width: f64, height: f64, dt: f64, tt: f64) {
        if !self.started {return}
        let minsize = f64::min(width, height);
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, width, height);
        ctx.set_fill_style_str("#202");
        let mcdonaldscheeseburger = worldtoscreen(Vector2 { x: -5.0, y: -5.0 }, width, height, 10.0);
        ctx.fill_rect(mcdonaldscheeseburger.x, mcdonaldscheeseburger.y, minsize, minsize);
        self.width = width;
        self.bird.update(dt);
        let birdscreenpos = worldtoscreen(self.bird.position, width, height, 10.0);
        ctx.set_fill_style_str("#fff");
        ctx.begin_path();
        ctx.ellipse(birdscreenpos.x , birdscreenpos.y, minsize*0.05, minsize*0.04, self.bird.velocity.y * (if self.bird.direction {-0.05} else {0.05}), 0.0, 6.283).unwrap();
        ctx.fill();
        ctx.set_fill_style_str("#090");
        for i in self.pipes.iter_mut() {
            i.update(dt);
            let screenpos = worldtoscreen(i.position, width, height, 10.0);
            ctx.fill_rect(screenpos.x-(minsize*0.05), screenpos.y-(minsize*0.4), minsize*0.1, minsize*0.8);
            if pointinbox(i.position, Vector2 { x: 1.0, y: 8.0 }, self.bird.position) {
                self.started = false;
            }
        }
        if !pointinbox(Vector2 { x: 0.0, y: 0.0 }, Vector2 { x: 10.0, y: 10.0 }, self.bird.position) {
            self.started = false;
        }
        self.pipes.retain(|value| {
            value.position.x > -15.0
        });
        self.tim += dt;
        if self.tim > 1700.0 {
            self.score += 1;
            let dir = self.random.random_bool(0.5);
            self.tim = 0.0;
            let a = self.random.random_range(-3.8..3.8);
            let spread = self.random.random_range(5.0..7.0);
            let speed = self.random.random_range(1.8..5.5);
            let g = self.random.random_range(-0.1..0.1);
            if dir {
                self.pipes.push(Pipe { position: Vector2 { x: 10.0+g, y: a+spread }, direction: false, speed: speed });
                self.pipes.push(Pipe { position: Vector2 { x: 10.0-g, y: a-spread }, direction: false, speed: speed });
            } else {
                self.pipes.push(Pipe { position: Vector2 { x: -10.0+g, y: a+spread }, direction: true, speed: speed });
                self.pipes.push(Pipe { position: Vector2 { x: -10.0-g, y: a-spread }, direction: true, speed: speed });
            }
        }
        ctx.set_fill_style_str("#fff");
        ctx.set_text_baseline("middle");
        ctx.set_text_align("center");
        ctx.set_font("5em 'Papyrus'");
        let score = self.score;
        ctx.fill_text(score.to_string().as_str(), width/2.0, height*0.2);
    }
    fn pointer_down(self: &mut Self, position: Vector2, pointertype: i8, button: i8, pointerid: i32, isprimary: bool) {
        if !self.started {
            self.bird = Bird { 
                position: Vector2::zero(), 
                velocity: Vector2::zero(),
                direction: false,
            };
            self.pipes = Vec::new();
            self.tim = 0.0;
            self.score = 0;
            self.started = true;
            return
        }
        if position.x > (self.width*0.5) {
            self.bird.velocity = Vector2 { x: 5.5, y: -10.5 };
            self.bird.direction = false;
        } else {
            self.bird.velocity = Vector2 { x: -5.5, y: -10.5 };
            self.bird.direction = true;
        }
        
    }
    /*fn pointer_move(self: &mut Self, position: Vector2, movement: Vector2, pointertype: i8, pointerid: i32, isprimary: bool) {
        if let Some(index) = self.pointers.iter().position(|value| value.0 == pointerid) {
            self.pointers.index_mut(index).1 = position;
            self.pointers.index_mut(index).3 = isprimary;
        }
    }
    fn pointer_up(self: &mut Self, position: Vector2, pointertype: i8, button: i8, pointerid: i32, isprimary: bool) {
        while let Some(index) = self.pointers.iter().position(|value| value.0 == pointerid) {
            self.pointers.swap_remove(index);
        }
    }*/
}

pub struct Bird {
    position: Vector2,
    velocity: Vector2,
    direction: bool,
}

impl Bird {
    pub fn update(self: &mut Self, dt: f64) {
        self.velocity += Vector2 { x: 0.0, y: 30.0 } * (dt/1000.0);
        self.velocity *= 0.985;
        self.position += self.velocity * (dt/1000.0);
    }
}

pub struct Pipe {
    position: Vector2,
    direction: bool,
    speed: f64,
}

impl Pipe {
    pub fn update(self: &mut Self, dt: f64) {
        match self.direction {
            false => {self.position += Vector2 { x: self.speed*-1.0, y: 0.0 } * (dt/1000.0)},
            true => {self.position += Vector2 { x: self.speed, y: 0.0 } * (dt/1000.0)}
        }
    }
}
