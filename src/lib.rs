extern crate core;

use std::error::Error;
use std::{fmt, fs};
use std::io::Read;
use crate::png::{PngImage, StandardChunk};

pub mod png;

pub struct Config {
	pub file_path: String,
	pub binary_path: String,
}

impl Config {
	pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

		let binary_path = match args.next() {
			Some(arg) => arg,
			None => return Err("Something really went wrong..."),
		};

		let file_path = match args.next() {
			Some(arg) => arg,
			None => return Err("No file provided"),
		};

		Ok(Config {
			binary_path,
			file_path,
		})
	}
}


pub fn vec_to_u32(v: Vec<u8>) -> [u8; 4] {
	let arr = [v[0], v[1], v[2], v[3]];

	return arr
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

	let png_image = PngImage::load(&config.file_path)?;

	let chunks = png_image.chunks;

	for chunk in chunks {
		println!("Chunk Type: {}, Length: {},  CRC: {:?}", chunk.chunk_type, chunk.length, chunk.crc);

		let chunk_struct = chunk.chunk_struct;



		if chunk_struct.is_some() {
			let idhr_struct = chunk_struct.unwrap();
			// if idhr_struct == StandardChunk::IHDR() {
			// 	idhr_struct.debug_chunk();

			// }

			idhr_struct.debug_chunk();

		}


		// } else {
		// 	println!("Found a none for this type.")
		// }
	}

	png_image.getChunksByType(StandardChunk::IHDR(_));

	Ok(())
}
