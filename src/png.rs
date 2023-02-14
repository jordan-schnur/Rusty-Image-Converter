pub enum StandardChunk {
	IHDR(IDHR),
	PLTE,
	IDAT,
	IEND,
	acTL,
	cHRM,
	cICP,
	gAMA,
	iCCP,
	sBIT,
	sRGB,
	bkGD,
	hIST,
	tRNS,
	eXIf,
	fcTL,
	pHYs,
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

	pub fn str_to_val<T>(str: &str, mut data: Vec<u8>) -> Option<StandardChunk> {
		match str {
			"IHDR" => Some(StandardChunk::IHDR(IDHR::init_chunk(data))),
			"PLTE" => Some(StandardChunk::PLTE),
			"IDAT" => Some(StandardChunk::IDAT),
			"IEND" => Some(StandardChunk::IEND),
			_ => None
		}
	}
}

pub trait Chunks {
	fn init_chunk(data: Vec<u8>) -> Self;
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
	fn init_chunk(mut data: Vec<u8>) -> IDHR {
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

			let chunk_struct = StandardChunk::str_to_val(chunk_type.as_str(), data);


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
