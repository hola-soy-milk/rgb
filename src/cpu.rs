/// Zero flag: set when the result of an operation is 0.
pub const FLAG_Z: u8 = 0x80;
/// Subtract flag: set when the last operation was a subtraction.
pub const FLAG_N: u8 = 0x40;
/// Half-carry flag: set on carry/borrow from bit 3.
pub const FLAG_H: u8 = 0x20;
/// Carry flag: set on carry/borrow from bit 7.
pub const FLAG_C: u8 = 0x10;

/// Game Boy CPU registers and program state.
///
/// The LR35902 is an 8-bit CPU. 16-bit values are formed by pairing
/// registers: AF, BC, DE, HL. PC is the program counter, SP is the stack pointer.
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    /// Flags register. Only the upper 4 bits are used (Z N H C).
    pub f: u8,
    /// Program counter — address of the next instruction to fetch.
    pub pc: u16,
    /// Stack pointer — grows downward on the Game Boy.
    pub sp: u16,
}

impl Cpu {
    /// Create a CPU with DMG post-BIOS register values.
    ///
    /// After the boot ROM finishes, the Game Boy jumps to 0x0100 with these
    /// exact values. Using them lets us skip emulating the boot ROM for now.
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0, // Z=1 N=0 H=1 C=1
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    // --- 16-bit register pairs ---

    pub fn af(&self) -> u16 {
        crate::util::u8s_to_u16(self.a, self.f)
    }

    pub fn set_af(&mut self, value: u16) {
        let (hi, lo) = crate::util::u16_to_u8s(value);
        self.a = hi;
        // Hardware always keeps the lower 4 bits of F as 0.
        self.f = lo & 0xF0;
    }

    pub fn bc(&self) -> u16 {
        crate::util::u8s_to_u16(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        let (hi, lo) = crate::util::u16_to_u8s(value);
        self.b = hi;
        self.c = lo;
    }

    pub fn de(&self) -> u16 {
        crate::util::u8s_to_u16(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        let (hi, lo) = crate::util::u16_to_u8s(value);
        self.d = hi;
        self.e = lo;
    }

    pub fn hl(&self) -> u16 {
        crate::util::u8s_to_u16(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        let (hi, lo) = crate::util::u16_to_u8s(value);
        self.h = hi;
        self.l = lo;
    }

    // --- Flag accessors ---

    pub fn flag_z(&self) -> bool {
        crate::util::get_bit(self.f, 7)
    }

    pub fn flag_n(&self) -> bool {
        crate::util::get_bit(self.f, 6)
    }

    pub fn flag_h(&self) -> bool {
        crate::util::get_bit(self.f, 5)
    }

    pub fn flag_c(&self) -> bool {
        crate::util::get_bit(self.f, 4)
    }

    pub fn set_flag_z(&mut self, high: bool) {
        crate::util::set_bit(&mut self.f, 7, high);
    }

    pub fn set_flag_n(&mut self, high: bool) {
        crate::util::set_bit(&mut self.f, 6, high);
    }

    pub fn set_flag_h(&mut self, high: bool) {
        crate::util::set_bit(&mut self.f, 5, high);
    }

    pub fn set_flag_c(&mut self, high: bool) {
        crate::util::set_bit(&mut self.f, 4, high);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmg_power_on_values() {
        let cpu = Cpu::new();
        assert_eq!(cpu.a, 0x01);
        assert_eq!(cpu.f, 0xB0);
        assert_eq!(cpu.b, 0x00);
        assert_eq!(cpu.c, 0x13);
        assert_eq!(cpu.d, 0x00);
        assert_eq!(cpu.e, 0xD8);
        assert_eq!(cpu.h, 0x01);
        assert_eq!(cpu.l, 0x4D);
        assert_eq!(cpu.pc, 0x0100);
        assert_eq!(cpu.sp, 0xFFFE);
    }

    #[test]
    fn register_pairs_get() {
        let cpu = Cpu::new();
        assert_eq!(cpu.af(), 0x01B0);
        assert_eq!(cpu.bc(), 0x0013);
        assert_eq!(cpu.de(), 0x00D8);
        assert_eq!(cpu.hl(), 0x014D);
    }

    #[test]
    fn register_pairs_set() {
        let mut cpu = Cpu::new();
        cpu.set_bc(0xABCD);
        assert_eq!(cpu.b, 0xAB);
        assert_eq!(cpu.c, 0xCD);
        assert_eq!(cpu.bc(), 0xABCD);

        cpu.set_de(0x1234);
        assert_eq!(cpu.d, 0x12);
        assert_eq!(cpu.e, 0x34);

        cpu.set_hl(0x5678);
        assert_eq!(cpu.h, 0x56);
        assert_eq!(cpu.l, 0x78);
    }

    #[test]
    fn set_af_masks_low_nibble() {
        let mut cpu = Cpu::new();
        // Attempt to set low nibble of F; hardware forces it to 0.
        cpu.set_af(0x12FF);
        assert_eq!(cpu.a, 0x12);
        assert_eq!(cpu.f, 0xF0);
        assert_eq!(cpu.af(), 0x12F0);
    }

    #[test]
    fn flag_masks() {
        let mut cpu = Cpu::new();
        // Power-on F = 0xB0 = Z|H|C
        assert!(cpu.flag_z());
        assert!(!cpu.flag_n());
        assert!(cpu.flag_h());
        assert!(cpu.flag_c());

        cpu.set_flag_z(false);
        cpu.set_flag_n(true);
        cpu.set_flag_h(false);
        cpu.set_flag_c(false);

        assert!(!cpu.flag_z());
        assert!(cpu.flag_n());
        assert!(!cpu.flag_h());
        assert!(!cpu.flag_c());
        assert_eq!(cpu.f, FLAG_N);
    }

    #[test]
    fn flag_constants() {
        assert_eq!(FLAG_Z, 0x80);
        assert_eq!(FLAG_N, 0x40);
        assert_eq!(FLAG_H, 0x20);
        assert_eq!(FLAG_C, 0x10);
    }
}
