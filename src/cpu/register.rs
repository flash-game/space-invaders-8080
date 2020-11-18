/// The register of Intel 8080
pub struct Register {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    // 程序计数器, the address of next opcode
    pub pc: u16,
    // 栈指针
    pub sp: u16,
    // FLAGS
    /// Z (zero) set to 1 when the result is equal to zero
    pub flag_z: bool,
    /// S (sign) set to 1 when bit 7 (the most significant bit or MSB) of the math instruction is set
    pub flag_s: bool,
    /// 答案具有偶数奇偶校验时设置P（奇偶校验），奇数奇偶校验时清除
    pub flag_p: bool,
    /// 当指令导致进位或借位到高位时，CY（进位）设置为1
    pub flag_cy: bool,
    /// AC (auxillary carry) is used mostly for BCD (binary coded decimal) math.
    /// Read the data book for more details, Space Invaders doesn't use it.
    pub flag_ac: bool,
}

impl Register {
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.get_flags() as u16)
    }

    pub fn get_bc(&self) -> u16 { (self.b as u16) << 8 | (self.c as u16) }

    pub fn get_de(&self) -> u16 { (self.d as u16) << 8 | (self.e as u16) }

    pub fn get_hl(&self) -> u16 { (self.h as u16) << 8 | (self.l as u16) }

    // Also known as 'B' in Intel Doc
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    // Also known as 'D' in Intel Doc
    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    // Also known as 'H' in Intel Doc
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn get_flags(&self) -> u8 {
        // S:7  Z:6  A:4  P:2  C:0
        let mut bits = 0b0000_0010;
        if self.flag_z { bits |= 0b0100_0000 }
        if self.flag_s { bits |= 0b1000_0000 }
        if self.flag_p { bits |= 0b0000_0100 }
        if self.flag_cy { bits |= 0b0000_0001 }
        if self.flag_ac { bits |= 0b0001_0000 }
        bits
    }

    pub fn set_flags(&mut self, flags: u8) {
        // S:7  Z:6  A:4  P:2  C:0
        self.flag_z = flags & 0b0100_0000 != 0;
        self.flag_s = flags & 0b1000_0000 != 0;
        self.flag_p = flags & 0b0000_0100 != 0;
        self.flag_cy = flags & 0b0000_0001 != 0;
        self.flag_ac = flags & 0b0001_0000 != 0;
    }
}