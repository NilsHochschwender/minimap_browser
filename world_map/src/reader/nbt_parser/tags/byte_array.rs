pub struct ByteArrayTag{
    name: String,
    len: usize,
    val: Vec<i8>,
}

impl ByteArrayTag{
    pub fn new(name: String, val: Vec<i8>) -> ByteArrayTag{
        ByteArrayTag{
            name: name,
            len: val.len(),
            val: val
        }
    }
}