extern crate core;

use std::error::Error;
use std::{fmt, fs};
use std::io::Read;
use crate::png::StandardChunk;

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

#[derive(Debug)]
struct PTJError(String);

impl fmt::Display for PTJError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "There is an error: {}", self.0)
	}
}

impl Error for PTJError {}

// TODO: Improve error handling

fn verify_png_signature(signature: Vec<u8>) -> Result<(), Box<dyn Error>> {
	if signature.len() < 8 {
		return Err(Box::new(PTJError("Contents not long enough to be jpeg".into())));
	}

	// TODO: Gotta be a better way to do this
	if  signature[0] != 0x89 ||
		signature[1] != 0x50 ||
		signature[2] != 0x4E ||
		signature[3] != 0x47 ||
		signature[4] != 0xD ||
		signature[5] != 0xA ||
		signature[6] != 0x1A ||
		signature[7] != 0xA {
		return Err(Box::new(PTJError("Invalid Signature".into())));
	}

	Ok(())
}

pub fn vec_to_u32(v: Vec<u8>) -> [u8; 4] {
	let arr = [v[0], v[1], v[2], v[3]];

	return arr
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let mut contents = fs::read(config.file_path)?;

	let signature: Vec<u8> = contents.splice(0..8, []).collect();

	verify_png_signature(signature)?;

	// TODO: Come up with solution that doesn't copy data

	let chunks = png::Chunk::process(contents);

	for chunk in chunks {
		println!("Chunk Type: {}, Length: {},  CRC: {:?}", chunk.chunk_type, chunk.length, chunk.crc);

		let chunk_struct = chunk.chunk_struct;



		if chunk_struct.is_some() {
			if chunk_struct == StandardChunk::IHDR {
				chunk_struct.unwrap().debug_chunk();
			}

		}
		// } else {
		// 	println!("Found a none for this type.")
		// }
	}


	Ok(())
}
