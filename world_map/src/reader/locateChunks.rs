extern crate libflate;
use std::io;
use std::io::Read;
use reader::locateChunks::libflate::zlib::Decoder;
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
			offset: u32::from_be_bytes([0x00,bytes[0],bytes[1],bytes[2]]),
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
			timestamp: u32::from_be_bytes(bytes),
		}
	}
	pub fn is_newer(&self, newer: Timestamp) -> bool {
		self.timestamp < newer.timestamp
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
		let to_array = |y:Vec<u8>| -> [u8;4] {[y[0],y[1],y[2],y[3]]};
		let to_u8 = |y:Vec<u8>| -> u8 {y[0]};
		let len: u32 = u32::from_be_bytes(to_array(bytes.drain(..4).collect::<Vec<u8>>()));
		let compression_type: u8 = to_u8(bytes.drain(..1).collect::<Vec<u8>>());
		let data: Vec<u8> = {
			let mut bytess = &mut bytes;
			let data: Vec<u8> = bytess.drain(..(len-1) as usize).collect();
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
impl Chunk {
	pub fn new(loc: [u8;4],timestamp: [u8;4], data: Vec<u8>) -> Chunk{
		Chunk{
			location: Location::new(loc),
			timestamp: Timestamp::new(timestamp),
			data: if data.is_empty(){None} else {Some(ChunkData::new(data))}
		}
	}
}

pub struct ChunkVec {
	chunks: Vec<Chunk>,
}
impl ChunkVec{
	pub fn new(filename: String) -> io::Result<ChunkVec>{
		let mut file = File::open(filename)?;
		let mut buf:Vec<u8> = Vec::new();
		file.read_to_end(&mut buf)?;
		let to_array = |y:Vec<u8>| -> [u8;4]{[y[0],y[1],y[2],y[3]]};
		if !buf.is_empty(){
			let mut vec_loc: Vec<[u8;4]> = Vec::new();
			let mut vec_drain = buf.drain(..4096).collect::<Vec<u8>>();
			while !vec_drain.is_empty(){
				vec_loc.append(to_array(vec_drain.drain(..4).collect::<Vec<u8>>()));
			}
		}
	}
}