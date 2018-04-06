extern crate bear_lib_terminal;
extern crate flate2;
extern crate byteorder;

use bear_lib_terminal::Color;
use bear_lib_terminal::geometry::{Point, Size};
use bear_lib_terminal::terminal::{self, config, Event, KeyCode};
use bear_lib_terminal::terminal::config::{Cellsize, font};
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;
use byteorder::{ReadBytesExt, BigEndian, ByteOrder, LittleEndian};

mod xploader;

// fn main() {
//     let mut file = File::open("xptest.xp").expect("file not found");

//     let mut contents = String::new();
//     file.read_to_string(&mut contents);//read_to_end(&mut contents).expect("Unable to read");
//     //     .expect("something went wrong reading the file");
//     let mut d = GzDecoder::new(contents.as_slice());

//     println!("{:?}", d);

//     let mut s = String::new();
//     d.read_to_string(&mut s);
//     println!("{:?}", s);

//     //let xp_data = xploader::load_xp_string(s, true);

//     // terminal::open("Simple example", 21, 21);
//     // let code = String::from("437");
// 	// terminal::set(config::Window::empty().resizeable(true).cellsize(Cellsize::Sized(Size::new(8,8))));
//     // terminal::set(font::bitmap(font::Origin::Root, "Andux_cp866ish.png").codepage(code).size(Size::new(8, 12)).font_name(String::from("huge")));
// 	// terminal::composition(true);
//     // terminal::refresh();
//     // //framerate

//     // terminal::delay(1000/29);
//     //terminal::print()
// }

fn main() {
    //let mut e = GzEncoder::new(Vec::new(), Compression::default());
    //e.write(b"Hello World").unwrap();
    let path = Path::new("xptest.xp");

    let mut file = File::open(&path).expect("Unable to open file");

    let mut bytes = Vec::new(); 
    file.read_to_end(&mut bytes);
    let mut gz = GzDecoder::new(&bytes[..]);

    let mut gz_bytes =Vec::new();
    gz.read_to_end(&mut gz_bytes);

    let mut rdr = Cursor::new(gz_bytes);

    let mut dst = [0; 6];
    rdr.read_i32_into::<LittleEndian>(&mut dst).unwrap();
    println!("{:?} ", dst);


    //let bytes = e.finish().unwrap();
    //println!("{:?}", decode_reader(bytes));
    //decode_reader(bytes);
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
fn decode_reader(bytes: Vec<u8>) {
    let mut gz = GzDecoder::new(&bytes[..]);
    println!("{:?}", gz);
    let mut s = String::new();
    gz.read_to_string(&mut s);
    println!("{}", s);
    let xpdata = xploader::load_xp_string(s, true);
    //Ok(s)
}