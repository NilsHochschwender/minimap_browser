use reader::nbt_parser::traits::*;
pub struct ByteTag{
    name: String,
    len: usize,
    val: i8,
}

impl ByteTag{
    pub fn new(name: String, val: i8) -> ByteTag{
        ByteTag{
            name: name,
            len: 1usize,
            val: val,
        }
    }
}

impl TagsMeth<i8> for ByteTag{
    fn get_name(&self) -> String{self.name}
    fn get_len(&self) -> usize{self.len}
    fn get_val(&self) -> i8{self.val}
    fn set_name(&mut self, name: String){self.name = name;}
    fn set_len(&mut self, len: usize){self.len = len;}
    fn set_val(&mut self, val: i8){self.val = val;}
}