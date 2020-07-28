pub trait Memory {
    fn get(&self, addr: u16) -> u8;

    /// Change the value of the address
    fn set(&mut self, addr: u16, val: u8);
}

