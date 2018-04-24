
/*##################################
# In-memory XP format is as follows:
# Returned structure is a dictionary with the keys version, layers, width, height, and layer_data
## Version is stored in case it's useful for someone, but as mentioned in the format description it probably won't be unless format changes happen
## Layers is a full 32 bit int, though right now REXPaint only exports or manages up to 4 layers
## Width and height are extracted from the layer with largest width and height - this value will hold true for all layers for now as per the format description
## layer_data is a list of individual layers, which are stored in the following format
### Each layer is a dictionary with keys width, height (see above), and cells. 
### Cells is a row major 2d array of, again, dictionaries with the values 'keycode' (ascii keycode), 'fore_r/g/b', and 'back_r/g/b' (technically ints but in value 0-255)
##################################*/


/*##################################
# Used primarily internally to parse the data, feel free to reference them externally if it's useful. 
# Changing these programattically will, of course, screw up the parsing (unless the format changes and you're using an old copy of this file)
##################################*/
use std::cmp;
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::io;
use std::path::Path;
use std::io::{Read, Cursor, SeekFrom};
use byteorder::{ReadBytesExt, ByteOrder, LittleEndian};



const version_bytes: usize = 4;
const layer_count_bytes: usize = 4;

const layer_width_bytes: usize = 4;
const layer_height_bytes: usize = 4;
const layer_keycode_bytes: usize = 4;
const layer_fore_rgb_bytes: usize = 3;
const layer_back_rgb_bytes: usize = 3;
const layer_cell_bytes: usize = layer_keycode_bytes + layer_fore_rgb_bytes + layer_back_rgb_bytes;



/*##################################
# REXPaint color key for transparent background colors. Not directly used here, but you should reference this when calling libtcod's console_set_key_color on offscreen consoles.
##################################*/

const transparent_cell_back_r: usize = 255;
const transparent_cell_back_g: usize = 0;
const transparent_cell_back_b: usize = 255;

#[derive(Debug)]
pub struct xpString {
	version: i32,
	layer_count: i32,
	width: i32,
	height: i32,
	layer_data: Vec<parseLay>
}

pub fn load_xp_string(file_string: Vec<u8>) -> xpString {
    let ref_file_string = &file_string;
	let read_stream = &file_string[..].to_vec();

	let mut rdr = Cursor::new(read_stream);
	let mut offset: usize = 0;

	rdr.seek(SeekFrom::Start(offset as u64));

	let mut version = rdr.read_i32::<LittleEndian>().unwrap();
	println!("{}", version);
	offset = offset + version_bytes;
	
	rdr.seek(SeekFrom::Start(offset as u64));

	let mut layer_count = rdr.read_i32::<LittleEndian>().unwrap();
	offset = offset + layer_count_bytes;

	rdr.seek(SeekFrom::Start(offset as u64));


	let mut layers = Vec::new();

	let mut current_largest_width = 0;
	let mut current_largest_height = 0;

	for layer in 0..layer_count	{
		//#slight lookahead to figure out how much data to feed load_layer
		let mut this_layer_width = rdr.read_i32::<LittleEndian>().unwrap();
		offset = offset + layer_width_bytes;
		rdr.seek(SeekFrom::Start(offset as u64));
		let mut this_layer_height = rdr.read_i32::<LittleEndian>().unwrap();
		offset += layer_height_bytes;
		rdr.seek(SeekFrom::Start(offset as u64));

		current_largest_width = cmp::max(current_largest_width, this_layer_width);
		current_largest_height = cmp::max(current_largest_height, this_layer_height);

		let layer_data_size = layer_width_bytes + layer_height_bytes + (layer_cell_bytes * (this_layer_width as usize) * (this_layer_height as usize));
		offset -= (layer_height_bytes+layer_width_bytes);
		let layer_data_raw = &ref_file_string[offset..(offset + layer_data_size)];
		
		let layer_data = parse_layer(layer_data_raw.to_vec());
		layers.push(layer_data);

		offset = offset + layer_data_size;
		rdr.seek(SeekFrom::Start(offset as u64));

	}

	xpString {
		version: version as i32,
		layer_count: layer_count as i32,
		width: current_largest_width as i32,
		height: current_largest_height as i32,
		layer_data: layers
	}
}
/*##################################
# Takes a single layer's data and returns the format listed at the top of the file for a single layer.
##################################*/
#[derive(Debug)]
pub struct parseLay {
	width: i32,
	height: i32,
	cells: Vec<Vec<indiCell>>
}


pub fn parse_layer(layer_string: Vec<u8>) -> parseLay {
 	let ref_layer_string = &layer_string;
	let read_stream = &layer_string[..].to_vec();

	let mut offset: usize = 0;
	let mut rdr = Cursor::new(read_stream);

	let mut width = rdr.read_i32::<LittleEndian>().unwrap();
	offset = offset + layer_width_bytes;
	rdr.seek(SeekFrom::Start(offset as u64));

	let mut height = rdr.read_i32::<LittleEndian>().unwrap();
	offset = offset + layer_height_bytes;


	rdr.seek(SeekFrom::Start(offset as u64));



	let mut cells = Vec::new();
	for x in 0..width {
		let mut row = Vec::new();

		for y in 0..height {
			let cell_data_raw = &ref_layer_string[offset..(offset + layer_cell_bytes)];
			let cell_data = parse_individual_cell(cell_data_raw.to_vec());
			row.push(cell_data);
			offset = offset + layer_cell_bytes;
			rdr.seek(SeekFrom::Start(offset as u64));

		}

		cells.push(row);
	}

	parseLay {
		width: width as i32,
		height: height as i32,
		cells: cells
	}

}

/*##################################
# Pulls out the keycode and the foreground/background RGB values from a single cell's data, returning them in the format listed at the top of this file for a single cell.
##################################*/
#[derive(Debug)]
pub struct indiCell {
	keycode: i32,
	fore_r: u8,
	fore_g: u8,
	fore_b: u8,
	back_r: u8,
	back_g: u8,
	back_b: u8
}

pub fn parse_individual_cell(cell_string: Vec<u8>) -> indiCell {
	let mut offset = 0;
	let mut rdr = Cursor::new(cell_string);

	let mut keycode = rdr.read_i32::<LittleEndian>().unwrap(); 

	offset = offset + layer_keycode_bytes;
	rdr.seek(SeekFrom::Start(offset as u64));

	let fore_r = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));

	let fore_g = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));

	let fore_b = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));

	let back_r = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));
	
	let back_g = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));

	let back_b = rdr.read_u8().unwrap();
	offset += 1;
	rdr.seek(SeekFrom::Start(offset as u64));

	indiCell {
		keycode: keycode as i32,
		fore_r: fore_r,
		fore_g: fore_g,
		fore_b: fore_b,
		back_r: back_r,
		back_g: back_g,
		back_b: back_b
	}
}

pub fn file_decompress(filepath: String) -> Vec<u8> {
	let path = Path::new(&filepath);

    let mut file = File::open(&path).expect("Unable to open file");

    let mut bytes = Vec::new(); 
    file.read_to_end(&mut bytes);
    let mut gz = GzDecoder::new(&bytes[..]);

    let mut gz_bytes = Vec::new();
    gz.read_to_end(&mut gz_bytes);
	gz_bytes
}