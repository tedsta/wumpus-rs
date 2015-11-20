extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate glutin_window;

use opengl_graphics::{GlGraphics, OpenGL};
use std::rc::Rc;
use std::cell::RefCell;
use graphics::*;
use piston::window::{AdvancedWindow, WindowSettings};
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: Window =
        WindowSettings::new("piston-example-user_input", [600, 600]).exit_on_esc(true)
                                                                    .opengl(opengl)
                                                                    .build().unwrap();

    let window = Rc::new(RefCell::new(window));
    let ref mut gl = GlGraphics::new(opengl);

    for e in window.clone().events() {
        e.render(|args| {
            gl.draw(args.viewport(), |c, g| {
                    graphics::clear([1.0; 4], g);
                    Rectangle::new([0.0, 0.0, 0.4, 1.0]).draw([0.0, 0.0, 50.0, 50.0],
                                                              &c.draw_state, c.transform, g);
                    //image(&vid_textures[vid_displays[2]],
                    //      c.trans(1280.0 - 350.0 - 5.0, 495.0).scale(350.0/512.0, 200.0/512.0).transform, gl);
                }
            );
        });
        e.update(|_| { });
    }
}

#[derive(Copy, Clone)]
pub enum Object {
    Hero,
    Wumpus,
    Stench,
    Pit,
    Breeze,
    Gold,
    Glimmer,
}

#[derive(Clone)]
struct Tile {
    things: Vec<Object>, // Things inside the tile
}

struct WumpusWorld {
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl WumpusWorld {
    pub fn new(width: usize, height: usize) -> Self {
        WumpusWorld {
            width: width,
            height: height,
            grid: vec![vec![Tile { things: vec![] }; width]; height],
        }
    }
}
