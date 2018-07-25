use std::io;
use std::io::prelude::*;
use std::fs::File;

// substruct for the chunklocation in the file
struct Location{
	offset: u32,
	sector_count: u8,
}
impl Location{
	fn new(bytes: [u8;4]) -> Location{
		let loc: [u8;4] = [0x00,bytes[0],bytes[1],bytes[2]];
		Location{
			offset: u32::from_be(u32::from_bytes(loc)),
			sector_count: bytes[3],
		}
	}
}
// timestamp of the chunks
struct Timestamp{
	timestamp: u32,
}

//substruct for chunkdata
struct ChunkData{
	length: u32,
	compression_type: u8,
	compressed_data: Vec<u8>, //this field is the most important one
}

struct Chunk{
	
}

struct Chunks{
	vec
}