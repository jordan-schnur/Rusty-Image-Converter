use std::{fmt, fs};
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Formatter;

#[allow(non_camel_case_types)]
pub enum StandardChunk {
	IHDR(IDHR),
	PLTE,
	IDAT(IDAT),
	IEND(IEND),
	acTL,
	cHRM(cHRM),
	cICP,
	gAMA,
	iCCP(iCCP),
	sBIT,
	sRGB,
	bkGD,
	hIST,
	tRNS,
	eXIf,
	fcTL,
	pHYs(pHYs),
	sPLT,
	fdAT,
	tIME,
	iTXt,
	tEXt,
	zTXt,
}

impl StandardChunk {
	pub fn process_chunk(data: Vec<u8>) {
		println!("Processing Chunks")
	}

	pub fn str_to_val(str: &str, mut data: Vec<u8>, data_length: usize) -> Option<StandardChunk> {
		match str {
			"IHDR" => Some(StandardChunk::IHDR(IDHR::init_chunk(data, data_length))),
			"PLTE" => Some(StandardChunk::PLTE),
			"IDAT" => Some(StandardChunk::IDAT(IDAT::init_chunk(data, data_length))),
			"IEND" => Some(StandardChunk::IEND(IEND::init_chunk(data, data_length))),
			"pHYs" => Some(StandardChunk::pHYs(pHYs::init_chunk(data, data_length))),
			"iCCP" => Some(StandardChunk::iCCP(iCCP::init_chunk(data, data_length))),
			"cHRM" => Some(StandardChunk::cHRM(cHRM::init_chunk(data, data_length))),
			_ => None
		}
	}

	pub fn debug_chunk(&self) {
		match self {
			Self::IHDR(IDHR) => IDHR.debug_chunk(),
			Self::pHYs(pHYs) => pHYs.debug_chunk(),
			Self::iCCP(iCCP) => iCCP.debug_chunk(),
			Self::cHRM(cHRM) => cHRM.debug_chunk(),
			Self::IDAT(IDAT) => IDAT.debug_chunk(),
			Self::IEND(IEND) => IEND.debug_chunk(),
			_ => println!("Unknown"),
		}
	}
}

pub trait Chunks {
	fn init_chunk(data: Vec<u8>, data_length: usize) -> Self;
	fn debug_chunk(&self);
}

enum ColourType {
	// Bit Depth allowed:
	Greyscale = 0, // : 1, 2, 4, 8, 16
	Truecolour = 2, // : 8, 16
	IndexedColour = 3, // : 1, 2, 3, 8
	GreyscaleWithAlpha = 4, // : 8, 16
	TruecolourWithAlpha = 6, // : 8, 16
}

enum InterlaceMethod {
	None = 0,
	Adam7 = 1,
}

// IHDR
// Width	4 bytes
// Height	4 bytes
// Bit depth	1 byte
// Colour type	1 byte
// Compression method	1 byte
// Filter method	1 byte
// Interlace method	1 byte
pub struct IDHR {
	width: u32,
	height: u32,
	bit_depth: u8,
	// colour_type: ColourType,
	colour_type: u8,
	compression_method: u8, // Only valid is 0
	filter_method: u8,
	interlace_method: u8,
	// interlace_method: InterlaceMethod, // 0, 1 Adam7 interlace
}

impl IDHR {
}

impl Chunks for IDHR {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> IDHR {
		let width: Vec<u8> = data.splice(0..4, []).collect();
		let width = vec_to_u32(width);

		let height: Vec<u8> = data.splice(0..4, []).collect();
		let height = vec_to_u32(height);

		let bit_depth: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		let colour_type: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		let compression_method: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		let filter_method: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		let interlace_method: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		IDHR {
			width,
			height,
			bit_depth,
			colour_type,
			compression_method,
			filter_method,
			interlace_method,
		}
	}

	fn debug_chunk(&self) {
		println!("Width: {}; Height:{}; Bit Depth: {}; Colour type: {}; Compression Method: {}; Filter method: {}; Interlace method: {}; ", self.width, self.height, self.bit_depth, self.colour_type, self.compression_method, self.filter_method, self.interlace_method);
	}
}

pub struct pHYs {
	ppu_x: u32,
	ppu_y: u32,
	unit_specifier: u8,
}

impl Chunks for pHYs {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> pHYs {
		let ppu_x: Vec<u8> = data.splice(0..4, []).collect();
		let ppu_x = vec_to_u32(ppu_x);

		let ppu_y: Vec<u8> = data.splice(0..4, []).collect();
		let ppu_y = vec_to_u32(ppu_y);

		let unit_specifier: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

		pHYs {
			ppu_x,
			ppu_y,
			unit_specifier
		}
	}

	fn debug_chunk(&self) {
		println!("Pixels per unit, X axis: {}; Pixels per unit, Y axis:{}; Unit specifier: {};", self.ppu_x, self.ppu_y, self.unit_specifier);
	}
}

pub struct iCCP {
	profile_name: String,
	compression_method: u8,
	compressed_profile: Vec<u8>,
}

impl Chunks for iCCP {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> iCCP {
		let mut nullFound = false;
		let mut profile_name: Vec<u8> = Vec::new();

		let mut length = data_length.clone();

		while !nullFound {
			let currentData: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];

			length = length - 1;
			match currentData {
				0x0 => nullFound = true,
				_ => {
					profile_name.push(currentData);
					continue;
				},
			}
		}


		let profile_name = String::from_utf8(profile_name).unwrap();

		println!("Found profile name: {profile_name}");

		let compression_method: u8 = data.splice(0..1, []).collect::<Vec<u8>>()[0];
		length = length - 1;

		let compressed_profile: Vec<u8> = data.splice(0..length, []).collect();


		iCCP {
			profile_name,
			compression_method,
			compressed_profile
		}
	}

	fn debug_chunk(&self) {
		println!("Profile name: {}; Compression method:{}; Compressed profile size: {}", self.profile_name, self.compression_method, self.compressed_profile.len());
	}
}

pub struct Vec2 {
	x: u32,
	y: u32,
}

impl Vec2 {
	pub fn from(x: u32, y: u32) -> Vec2 {
		Vec2 {
			x, y
		}
	}

	pub fn new() -> Vec2 {
		Vec2 {
			x: 0,
			y: 0,
		}
	}
}

impl fmt::Display for Vec2 {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "(")?;
		write!(f, "{}", self.x)?;
		write!(f, ", ")?;
		write!(f, "{}", self.y)?;
		write!(f, ")")?;

		Ok(())
	}
}

pub struct cHRM {
	white_point: Vec2,
	red: Vec2,
	green: Vec2,
	blue: Vec2,
}


impl Chunks for cHRM {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> cHRM {
		let white_point_x: Vec<u8> = data.splice(0..4, []).collect();
		let white_point_x = vec_to_u32(white_point_x);

		let white_point_y: Vec<u8> = data.splice(0..4, []).collect();
		let white_point_y = vec_to_u32(white_point_y);

		let white_point = Vec2::from(white_point_x, white_point_y);

		let red_x: Vec<u8> = data.splice(0..4, []).collect();
		let red_x = vec_to_u32(red_x);

		let red_y: Vec<u8> = data.splice(0..4, []).collect();
		let red_y = vec_to_u32(red_y);

		let red = Vec2::from(red_x, red_x);

		let green_x: Vec<u8> = data.splice(0..4, []).collect();
		let green_x = vec_to_u32(green_x);

		let green_y: Vec<u8> = data.splice(0..4, []).collect();
		let green_y = vec_to_u32(green_y);

		let green = Vec2::from(green_x, green_y);

		let blue_x: Vec<u8> = data.splice(0..4, []).collect();
		let blue_x = vec_to_u32(blue_x);

		let blue_y: Vec<u8> = data.splice(0..4, []).collect();
		let blue_y = vec_to_u32(blue_y);

		let blue = Vec2::from(blue_x, blue_y);


		cHRM {
			white_point,
			red,
			green,
			blue,
		}
	}

	fn debug_chunk(&self) {
		println!("White Point: {}; Red:{}; Green: {}; Blue: {}", self.white_point, self.red, self.green, self.blue);
	}
}

pub struct IDAT {
	image_data: Vec<u8>,
}


impl Chunks for IDAT {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> IDAT {
		let mut length = data_length.clone();

		let image_data: Vec<u8> = data.splice(0..length, []).collect();

		IDAT {
			image_data,
		}
	}

	fn debug_chunk(&self) {
		println!("Nothing to display for IDAT");
	}
}

pub struct IEND {}


impl Chunks for IEND {
	fn init_chunk(mut data: Vec<u8>, data_length: usize) -> IEND {
		IEND {}
	}

	fn debug_chunk(&self) {
		println!("Nothing to display for IEND");
	}
}

// pub struct Chunk<T> where T: ReadChunk {
pub struct Chunk {
	pub length: u32,
	pub chunk_type: String,
	pub crc: [u8; 4],
	pub chunk_struct: Option<StandardChunk>,
}

fn vec_to_u32(v: Vec<u8>) -> u32 {
	let arr = [v[0], v[1], v[2], v[3]];

	u32::from_be_bytes(arr)
}

impl Chunk {
	// pub fn chunk_type_to_type(chunk_type: &str) -> Box<dyn ReadChunk> {
	// 	match chunk_type {
	// 		"IHDR" => Box::new(IDHR),
	// 		_ => {}
	// 	}
	// }

	pub fn process(mut image_data: Vec<u8>) -> Vec<Chunk> {
		let mut chunks: Vec<Chunk> = Vec::new();

		while !image_data.is_empty() {
			let length: Vec<u8> = image_data.splice(0..4, []).collect();
			let length = vec_to_u32(length);

			let chunk_type: Vec<u8> = image_data.splice(0..4, []).collect();
			let chunk_type = String::from_utf8(chunk_type).unwrap();

			let dataLength: usize = length.clone().try_into().unwrap();

			let mut data: Vec<u8> = image_data.splice(0..dataLength, []).collect();

			let crc: Vec<u8> = image_data.splice(0..4, []).collect();
			let crc: [u8; 4] = [crc[0], crc[1], crc[2], crc[3]];

			let chunk_struct = StandardChunk::str_to_val(chunk_type.as_str(), data, dataLength);


			chunks.push(Chunk {
				length,
				chunk_type,
				crc,
				chunk_struct
			});
		}

		chunks
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

pub struct PngImage {
	pub name: String,
	pub chunks: Vec<Chunk>
}

impl PngImage {
	pub fn load(path: &String) -> Result<PngImage, Box<dyn Error>> {
		let mut contents = fs::read(path)?;

		let signature: Vec<u8> = contents.splice(0..8, []).collect();

		verify_png_signature(signature)?;

		// TODO: Come up with solution that doesn't copy data

		let chunks = Chunk::process(contents);

		Ok(PngImage {
			chunks,
			name: path.to_string(),
		})
	}

	pub fn getChunksByType(&self, chunk_type: StandardChunk) -> Option<Vec<&Chunk>> {
		let mut foundChunks = Vec::new();

		for chunk in &self.chunks {
			if !chunk.chunk_struct.is_some() {
				continue;
			}

			let chunk_struct: &StandardChunk = chunk.chunk_struct.as_ref().unwrap();

			if matches!(chunk_struct, chunk_type) {
				foundChunks.push(chunk);
			}
		}

		if foundChunks.len() == 0 {
			return None
		}

		return Some(foundChunks)
	}
}
