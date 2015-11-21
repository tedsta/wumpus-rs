extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate glutin_window;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::window::{AdvancedWindow, WindowSettings};
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: Window =
        WindowSettings::new("piston-example-user_input", [640, 740]).exit_on_esc(true)
                                                                    .opengl(opengl)
                                                                    .build().unwrap();

    let window = Rc::new(RefCell::new(window));
    let ref mut gl = GlGraphics::new(opengl);

    let mut wumpus_world = WumpusWorld::new(10, 10);
    wumpus_world.add_thing(5, 5, Object::Wumpus);
    wumpus_world.run(window.clone(), gl);
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Object {
    Hero,
    Wumpus,
    Stench,
    Pit,
    Breeze,
    Gold,
    Glimmer,
}

impl Object {
    pub fn clue(&self) -> Option<Object> {
        use Object::*;
        match *self {
            Hero => None,
            Wumpus => Some(Stench),
            Stench => None,
            Pit => Some(Breeze),
            Breeze => None,
            Gold => Some(Glimmer),
            Glimmer => None,
        }
    }
}

#[derive(Clone)]
struct Tile {
    things: HashSet<Object>, // Things inside the tile
    visible: bool,
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
            grid: vec![vec![Tile { things: HashSet::new(), visible: false }; width]; height],
        }
    }

    pub fn run(&mut self, window: Rc<RefCell<Window>>, gl: &mut GlGraphics) {
        for e in window.clone().events() {
            e.render(|args| {
                gl.draw(args.viewport(), |c, g| {
                    //Clear the screen
                    graphics::clear([1.0; 4], g);
                    
                    // Draw the game board
                    for y in 0..self.height {
                        for x in 0..self.width {
                            let c = c.trans((x as f64)*64.0, (y as f64)*64.0);
                            Rectangle::new([0.0, 0.0, 0.4, 1.0]).draw([0.0, 0.0, 64.0, 64.0],
                                                                      &c.draw_state,
                                                                      c.transform, g);
                        }
                    }

                    //image(&vid_textures[vid_displays[2]],
                    //      c.trans(1280.0 - 350.0 - 5.0, 495.0).scale(350.0/512.0, 200.0/512.0).transform, gl);
                });
            });
            e.update(|_| { });
        }
    }
    
    pub fn add_thing(&mut self, x: usize, y: usize, thing: Object) {
        self.grid[y][x].things.insert(thing);
        if let Some(clue) = thing.clue() {
            // Add clues to 4 adjacent squares
            if x > 0 {
                self.grid[y][x-1].things.insert(clue);
            }
            if x < self.width-1 {
                self.grid[y][x+1].things.insert(clue);
            }
            if y > 0 {
                self.grid[y-1][x].things.insert(clue);
            }
            if y < self.height-1 {
                self.grid[y+1][x].things.insert(clue);
            }
        }
    }
}
