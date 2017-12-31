extern crate bear_lib_terminal;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::{Point, Size};
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use bear_lib_terminal::terminal::config::{Cellsize, font};

const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

fn circles() -> {

}


fn main() {
    terminal::open("Simple example", 41, 41);

	terminal::set(config::Window::empty().resizeable(true).cellsize(Cellsize::Sized(Size::new(8,8))));

	terminal::refresh();
}