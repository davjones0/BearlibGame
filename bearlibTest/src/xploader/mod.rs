
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
extern crate hex;
use std::cmp;


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


pub struct xpString {
	version: i16,
	layer_count: i16,
	width: i16,
	height: i16,
	layer_data: Vec<parseLay>
}

pub fn load_xp_string(file_string: String, reverse_endian: bool) -> xpString {

	let mut offset: usize = 0;

	let mut version = String::from(&file_string[offset..(offset + version_bytes)]);
	println!("{}", version);
	offset = offset + version_bytes;
	let mut layer_count = String::from(&file_string[offset..(offset + layer_count_bytes)]);
	offset = offset + layer_count_bytes;

	if reverse_endian {
		version = version.chars().rev().collect::<String>(); //this reverses version
		layer_count = layer_count.chars().rev().collect::<String>();
    }

//	# hex-encodes the numbers then converts them to an int
	let vers = hex::encode(version).parse::<i16>().unwrap();
	let layercount = hex::encode(layer_count).parse::<i16>().unwrap();

	let mut layers = Vec::new();

	let mut current_largest_width = 0;
	let mut current_largest_height = 0;

	for layer in 0..layercount	{
		//#slight lookahead to figure out how much data to feed load_layer

		let mut this_layer_width = String::from(&file_string[offset..(offset + layer_width_bytes)]); //
		let mut this_layer_height = String::from(&file_string[(offset + layer_width_bytes)..(offset + layer_width_bytes + layer_height_bytes)]);

		if reverse_endian {
			this_layer_width = this_layer_width.chars().rev().collect::<String>();
			this_layer_height = this_layer_height.chars().rev().collect::<String>();
		}

		let this_layerwidth = hex::encode(this_layer_width).parse::<i16>().unwrap();
		let this_layerheight = hex::encode(this_layer_height).parse::<i16>().unwrap();

		current_largest_width = cmp::max(current_largest_width, this_layerwidth);
		current_largest_height = cmp::max(current_largest_height, this_layerheight);

		let layer_data_size = layer_width_bytes + layer_height_bytes + (layer_cell_bytes * this_layerwidth as usize * this_layerheight as usize);

		let layer_data_raw = String::from(&file_string[offset..(offset + layer_data_size)]);
		let layer_data = parse_layer(String::from(&file_string[offset..(offset + layer_data_size)]), reverse_endian);
		layers.push(layer_data);

		offset = offset + layer_data_size;
	}

	xpString {
		version: vers,
		layer_count: layercount,
		width: current_largest_width,
		height: current_largest_height,
		layer_data: layers
	}
// 	return {
// 		'version':version,
// 		'layer_count':layer_count,
// 		'width':current_largest_width,
// 		'height':current_largest_height,
// 		'layer_data':layers
// 	}
}
/*##################################
# Takes a single layer's data and returns the format listed at the top of the file for a single layer.
##################################*/
pub struct parseLay {
	width: i16,
	height: i16,
	cells: Vec<Vec<indiCell>>
}


pub fn parse_layer(layer_string: String, reverse_endian: bool) -> parseLay {
	let mut offset: usize = 0;

	let mut width = String::from(&layer_string[offset..(offset + layer_width_bytes)]);
	offset = offset + layer_width_bytes;
	let mut height = String::from(&layer_string[offset..(offset + layer_height_bytes)]);
	offset = offset + layer_height_bytes;

	if reverse_endian {
		width = width.chars().rev().collect::<String>();
		height = height.chars().rev().collect::<String>();
	}

	let widthI = hex::encode(width).parse::<i16>().unwrap();
	let heightI = hex::encode(height).parse::<i16>().unwrap();

	let mut cells = Vec::new();
	for x in 0..widthI {
		let mut row = Vec::new();

		for y in 0..heightI {
			let cell_data_raw = String::from(&layer_string[offset..(offset + layer_cell_bytes)]);
			let cell_data = parse_individual_cell(cell_data_raw, reverse_endian);
			row.push(cell_data);
			offset = offset + layer_cell_bytes;
		}

		cells.push(row);
	}

	parseLay {
		width: widthI,
		height: heightI,
		cells: cells
	}

// 	return {
// 		'width':width,
// 		'height':height,
// 		'cells':cells
// 	}
}

/*##################################
# Pulls out the keycode and the foreground/background RGB values from a single cell's data, returning them in the format listed at the top of this file for a single cell.
##################################*/

pub struct indiCell {
	keycode: i16,
	fore_r: i16,
	fore_g: i16,
	fore_b: i16,
	back_r: i16,
	back_g: i16,
	back_b: i16
}

pub fn parse_individual_cell(cell_string: String, reverse_endian: bool) -> indiCell {
	let mut offset = 0;

	let mut keycode = String::from(&cell_string[offset..(offset + layer_keycode_bytes)]);
	if reverse_endian {
		keycode = keycode.chars().rev().collect::<String>();
	}

	let _keycode = hex::encode(keycode).parse::<i16>().unwrap();
	offset = offset + layer_keycode_bytes;

	let fore_r = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;
	let fore_g = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;
	let fore_b = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;

	let back_r = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;
	let back_g = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;
	let back_b = hex::encode(&cell_string[offset..(offset+1)]).parse::<i16>().unwrap();
	offset += 1;

	indiCell {
		keycode: _keycode,
		fore_r: fore_r,
		fore_g: fore_g,
		fore_b: fore_b,
		back_r: back_r,
		back_g: back_g,
		back_b: back_b
	}
// 	return {
// 		'keycode':keycode,
// 		'fore_r':fore_r,
// 		'fore_g':fore_g,
// 		'fore_b':fore_b,
// 		'back_r':back_r,
// 		'back_g':back_g,
// 		'back_b':back_b,
// 	}
}