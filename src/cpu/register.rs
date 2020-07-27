/// The register of Intel 8080
pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    // 程序计数器
    pub pc: u16,
    pub stack_pointer: u16,
}

impl Register {
    pub fn get_bc(&self) -> u16 { (self.b as u16) << 8 | (self.c as u16) }

    pub fn get_de(&self) -> u16 { (self.d as u16) << 8 | (self.e as u16) }

    pub fn get_hl(&self) -> u16 { (self.h as u16) << 8 | (self.l as u16) }


    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }
}