//pub mod end;
pub mod byte;
pub mod short;
pub mod int;
pub mod long;
pub mod float;
pub mod double;
pub mod byte_array;
pub mod string;
pub mod list;
pub mod compound;
pub mod int_array;
pub mod long_array;

pub enum TagsList{
    ByteTag(byte::ByteTag),
}