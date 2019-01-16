pub trait TagsMeth<T>{
    fn get_name(&self) -> String;
    fn get_len(&self) -> usize;
    fn get_val(&self) -> T;
    fn set_name(&mut self, name: String);
    fn set_len(&mut self, len: usize);
    fn set_val(&mut self, val: T);
    fn search(&self, val_to_search: i8 where T) -> bool{
        let self_val = self.get_val();

    }  
}