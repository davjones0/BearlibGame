extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::{Point, Size};
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use bear_lib_terminal::terminal::config::{Cellsize, font};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 30;
const ENTITY_COUNT: i32 =  100;

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Copy, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}
impl Tile {
    pub fn empty() -> Self {
        Tile{ blocked: false, block_sight: false}
    }
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color
        }   
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        //if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            self.x += dx;
            self.y += dy;
        //}
    }

    pub fn draw(&self) {
        terminal::with_foreground(self.color, || terminal::put_xy(self.x, self.y, self.char));
    }

    pub fn clear(&self) {
        terminal::put_xy(self.x, self.y, ' ');
    }
}

fn handle_keys(player: &mut Object, map: &Map) -> bool {
    //let key = terminal::wait_event();

    for event in terminal::events() {
        match event {
            Event::KeyPressed{ key: KeyCode::Escape, ctrl: _, shift: _ } => return true, // exit game
            Event::KeyPressed{ key: KeyCode::Up, ctrl: _, shift: _ } => {
                player.move_by(0, -1, map);
                return false
            },
            Event::KeyPressed{ key: KeyCode::Down, ctrl: _, shift: _ } => {
                player.move_by(0, 1, map);
                return false;
            },
            Event::KeyPressed{ key: KeyCode::Left, ctrl: _, shift: _ } => {
                player.move_by(-1, 0, map);
                return false;
            },
            Event::KeyPressed{ key: KeyCode::Right, ctrl: _, shift: _ } =>{ 
                player.move_by(1, 0, map);
                return false;
            },

            _ => (),
        }
    }

    false
}

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); SCREEN_HEIGHT as usize]; SCREEN_WIDTH as usize];
    map
}

fn render_all(objects: &[Object]) {
    for object in objects {
        object.draw();
    }
}

fn main() {
	terminal::open("Simple example", 80, 30);
    
    // cell size for the terminal has been manually set to 8x8, use a 8x12 font to get dat 3d effect
	terminal::set(config::Window::empty().resizeable(true).cellsize(Cellsize::Sized(Size::new(8,8))));
	//terminal::set(font::bitmap(font::Origin::Root, "").size(Size::new(8,12)));
    //terminal::set(font::bitmap(font::Origin::Offset(''), ""));
    //terminal::print_xy(0, 0, "Your mom");
	//terminal::with_colors(Color::from_rgb(0xFA, 0xAF, 0x29), Color::from_rgb(0x05, 0x50, 0xD6), || terminal::print_xy(0, 1, "Colerd"));
	/*for (i, c) in "Coloured letters with pixel-offset!".chars().enumerate() {
		terminal::put_ext(Point::new(i as i32, 2), Point::new(i as i32, i as i32), c, &vec![Color::from_rgb(0xFF, 0x00, 0x00),
		                                                                                    Color::from_rgb(0x00, 0xFF, 0x00),
		                                                                                    Color::from_rgb(0x00, 0x00, 0xFF),
		                                                                                    Color::from_rgb(0xFF, 0xFF, 0xFF)]);
	}*/
	terminal::refresh();
    let player = Object::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, '@', Color::from_rgb(100,100,100));
    let mut objects = [player];

    let mut map = make_map();
    loop {
        render_all(&objects);
        
        terminal::refresh();

        for object in &objects {
            object.clear()
        }
        let player = &mut objects[0];
        let exit = handle_keys(player, &map);
        if exit {
            break;
        }

    }

	/*terminal::set_foreground(Color::from_rgb(0xFF, 0xFF, 0xFF));
	if let Some(string) = terminal::read_str(Point::new(0, 5), 30) {
		terminal::print_xy(0, 5, &*&string);
	}
	terminal::refresh();
	for event in terminal::events() {
		match event {
			Event::Resize{width, height} => {
				terminal::print_xy(0, 0, &*&format!("Width: {}\nHeight: {}", width, height));
				terminal::refresh();
			},
			Event::Close | Event::KeyPressed{key: KeyCode::Escape, ctrl: _, shift: _} => break,
			_                                                                         => (),
		}
	}*/
	terminal::close();
}