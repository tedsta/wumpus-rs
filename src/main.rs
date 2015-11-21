extern crate piston;
extern crate opengl_graphics;
extern crate graphics;
extern crate glutin_window;

use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

use graphics::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::window::{AdvancedWindow, WindowSettings};
use piston::input::*;
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;

fn main() {
    let opengl = OpenGL::V3_2;
    let window: Window =
        WindowSettings::new("piston-example-user_input", [500, 600]).exit_on_esc(true)
                                                                    .opengl(opengl)
                                                                    .build().unwrap();

    let window = Rc::new(RefCell::new(window));
    let ref mut gl = GlGraphics::new(opengl);

    let mut wumpus_world = WumpusWorld::new(10, 10);
    wumpus_world.reset_board();
    wumpus_world.run(window.clone(), gl);
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum Object {
    Wumpus,
    DeadWumpus,
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
            Wumpus => Some(Stench),
            DeadWumpus => Some(Stench),
            Stench => None,
            Pit => Some(Breeze),
            Breeze => None,
            Gold => Some(Glimmer),
            Glimmer => None,
        }
    }
    
    pub fn texture_id(&self) -> usize {
        use Object::*;
        match *self {
            Wumpus => 0,
            DeadWumpus => 1,
            Stench => 2,
            Pit => 3,
            Breeze => 4,
            Gold => 5,
            Glimmer => 6,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
enum GameState {
    Playing,
    Win,
    Lose,
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
    textures: Vec<Texture>,
    hero_texture: Texture,
    bow_texture: Texture,
    play_texture: Texture,
    win_texture: Texture,
    lose_texture: Texture,
    hero_x: usize,
    hero_y: usize,
    hero_dir: Direction,
    arrows: u8,
    state: GameState,
}

impl WumpusWorld {
    pub fn new(width: usize, height: usize) -> Self {
        WumpusWorld {
            width: width,
            height: height,
            grid: vec![vec![Tile { things: HashSet::new(), visible: false }; width]; height],
            textures: vec![Texture::from_path(&Path::new("content/wumpus.png")).unwrap(),
                           Texture::from_path(&Path::new("content/dead_wumpus.png")).unwrap(),
                           Texture::from_path(&Path::new("content/stench.png")).unwrap(),
                           Texture::from_path(&Path::new("content/pit.png")).unwrap(),
                           Texture::from_path(&Path::new("content/breeze.png")).unwrap(),
                           Texture::from_path(&Path::new("content/gold.png")).unwrap(),
                           Texture::from_path(&Path::new("content/glimmer.png")).unwrap()],
            hero_texture: Texture::from_path(&Path::new("content/hero.png")).unwrap(),
            bow_texture: Texture::from_path(&Path::new("content/bow.png")).unwrap(),
            play_texture: Texture::from_path(&Path::new("content/playing.png")).unwrap(),
            win_texture: Texture::from_path(&Path::new("content/win.png")).unwrap(),
            lose_texture: Texture::from_path(&Path::new("content/lose.png")).unwrap(),
            hero_x: 0,
            hero_y: 0,
            hero_dir: Direction::Right,
            arrows: 1, // Only get 1 arrow
            state: GameState::Playing,
        }
    }

    pub fn run(&mut self, window: Rc<RefCell<Window>>, gl: &mut GlGraphics) {
        self.hero_visit(); // Visit the first cell
        for e in window.clone().events() {
            e.render(|args| {
                gl.draw(args.viewport(), |c, g| {
                    //Clear the screen
                    graphics::clear([1.0; 4], g);
                    
                    // Draw the game board
                    for y in 0..self.height {
                        for x in 0..self.width {
                            let c = c.trans((x as f64)*50.0, (y as f64)*50.0);
                            Rectangle::new([0.5, 0.5, 0.5, 1.0]).draw([0.0, 0.0, 50.0, 50.0],
                                                                      &c.draw_state,
                                                                      c.transform, g);

                            if self.grid[y][x].visible {
                                for thing in self.grid[y][x].things.iter() {
                                    image(&self.textures[thing.texture_id()], c.transform, g);
                                }
                            }
                        }
                    }

                    // Draw depending on state
                    match self.state {
                        GameState::Playing => {
                            image(&self.play_texture, c.trans(0.0, 500.0).transform, g);

                            let c = c.trans((self.hero_x as f64)*50.0, (self.hero_y as f64)*50.0);
                            image(&self.hero_texture, c.transform, g);

                            if self.arrows > 0 {
                                let c =
                                    match self.hero_dir {
                                        Direction::Up => {
                                            c.trans(0.0, -50.0)
                                        },
                                        Direction::Down => {
                                            c.trans(0.0, 50.0)
                                        },
                                        Direction::Left => {
                                            c.trans(-50.0, 0.0)
                                        },
                                        Direction::Right => {
                                            c.trans(50.0, 0.0)
                                        },
                                    };
                                image(&self.bow_texture, c.transform, g);
                            }
                        },
                        GameState::Win => {
                            image(&self.win_texture, c.trans(0.0, 500.0).transform, g);

                            let c = c.trans((self.hero_x as f64)*50.0, (self.hero_y as f64)*50.0);
                            image(&self.hero_texture, c.transform, g);
                        },
                        GameState::Lose => {
                            image(&self.lose_texture, c.trans(0.0, 500.0).transform, g);
                        },
                    }
                });
            });
            e.update(|_| { });
            e.press(|b| {
                match b {
                    Button::Keyboard(key) => {
                        match key {
                            Key::Up => { self.move_up(); },
                            Key::Down => { self.move_down(); },
                            Key::Left => { self.move_left(); },
                            Key::Right => { self.move_right(); },
                            Key::W => { self.hero_dir = Direction::Up; },
                            Key::S => { self.hero_dir = Direction::Down; },
                            Key::A => { self.hero_dir = Direction::Left; },
                            Key::D => { self.hero_dir = Direction::Right; },
                            Key::Space => {
                                self.fire_arrow();
                            },
                            Key::RShift | Key::LShift => {
                                self.reset_board();
                            },
                            Key::Return => {
                                self.reset_board();
                            },
                            _ => { },
                        }
                    },
                    _ => { },
                }
            });
        }
    }

    pub fn move_up(&mut self) {
        if self.hero_y == 0 { return; }
        self.hero_y -= 1;
        self.hero_visit();
    }

    pub fn move_down(&mut self) {
        if self.hero_y == self.height-1 { return; }
        self.hero_y += 1;
        self.hero_visit();
    }

    pub fn move_left(&mut self) {
        if self.hero_x == 0 { return; }
        self.hero_x -= 1;
        self.hero_visit();
    }

    pub fn move_right(&mut self) {
        if self.hero_x == self.width-1 { return; }
        self.hero_x += 1;
        self.hero_visit();
    }
    
    pub fn fire_arrow(&mut self) {
        if self.arrows == 0 { return; } // No more arrows
        self.arrows -= 1;

        let x_step: i32 =
            match self.hero_dir {
                Direction::Left => -1,
                Direction::Right => 1,
                _ => { 0 },
            };
        let y_step: i32 =
            match self.hero_dir {
                Direction::Up => -1,
                Direction::Down => 1,
                _ => { 0 },
            };

        let mut arrow_x = self.hero_x as i32;
        let mut arrow_y = self.hero_y as i32;

        while arrow_x > 0 && (arrow_x as usize) < self.width && arrow_y > 0 && (arrow_y as usize) < self.height {
            if self.grid[arrow_y as usize][arrow_x as usize].things.contains(&Object::Wumpus) {
                self.grid[arrow_y as usize][arrow_x as usize].things.remove(&Object::Wumpus);
                self.grid[arrow_y as usize][arrow_x as usize].things.insert(Object::DeadWumpus);
            }
            
            arrow_x += x_step;
            arrow_y += y_step;
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

    pub fn remove_thing(&mut self, x: usize, y: usize, thing: Object) {
        self.grid[y][x].things.remove(&thing);
    }

    fn hero_visit(&mut self) {
        self.grid[self.hero_y][self.hero_x].visible = true;

        if self.state == GameState::Playing {
            if (self.grid[self.hero_y][self.hero_x].things.contains(&Object::Wumpus) ||
               self.grid[self.hero_y][self.hero_x].things.contains(&Object::Pit)) {
                self.state = GameState::Lose; // Game over!
            } else if self.grid[self.hero_y][self.hero_x].things.contains(&Object::Gold) {
                self.state = GameState::Win;
            }
        }
    }

    fn reset_board(&mut self) {
        // Clear the game board
        for y in 0..self.height {
            for x in 0..self.width {
                self.grid[y][x].things.clear();
                self.grid[y][x].visible = false;
            }
        }

        self.hero_x = 0;
        self.hero_y = 0;
        self.arrows = 1;
        self.hero_visit();

        self.state = GameState::Playing;

        // Randomize board
        self.add_thing(5, 5, Object::Wumpus);
        self.add_thing(3, 5, Object::Pit);
        self.add_thing(9, 9, Object::Gold);
    }
}
