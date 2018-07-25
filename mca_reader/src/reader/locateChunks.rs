extern crate libflate;
use std::io::{self,Read};
use libflate::zlib::Decoder;
use std::io;
use std::io::prelude::*;
use std::fs::File;

// substruct for the chunklocation in the file
pub struct Location {
	offset: u32,
	sector_count: u8,
}
impl Location {
	pub fn new(bytes: [u8;4]) -> Location {
		Location {
			offset: u32::from_be(u32::from_bytes([0x00,bytes[0],bytes[1],bytes[2]])),
			sector_count: bytes[3],
		}
	}
}
// timestamp of the chunks
pub struct Timestamp {
	timestamp: u32,
}
impl Timestamp {
	pub fn new(bytes: [u8;4]) -> Timestamp {
		Timestamp{
			timestamp: u32::from_be(u32::from_bytes(bytes)),
		}
	}
}

//substruct for chunkdata
pub struct ChunkData {
	length: u32,
	compression_type: u8,
	data: Vec<u8>, //this field is the most important one. length-1 bytes
}
impl ChunkData {
	pub fn new(mut bytes: Vec<u8>) -> ChunkData {
		let len: u32 = u32::from_be(u32::from_bytes(bytes.drain(..4).collect()));
		let compression_type: u8 = bytes.drain(..1).collect();
		let data: Vec<u8> = {
			let mut bytess = &mut bytes;
			let data: Vec<u8> = bytess.drain(..len-1).collect();
			let mut decoder = Decoder::new(&data[..]).unwrap();
			let mut decoded_data: Vec<u8> = Vec::new();
			decoder.read_to_end(&mut decoded_data).unwrap();
			decoded_data
		};
		ChunkData{
			length: len,
			compression_type: compression_type,
			data: data,
		}
	}
}

pub struct Chunk {
	location: Location,
	timestamp: Timestamp,
	data: Option<ChunkData>,
}
r

struct Chunks {
	vec
}