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

pub mod ops {
    use crate::gameboy::GameBoy;

    /// One instruction handler. Reads operands from registers/memory,
    /// mutates CPU state, and returns the Game Boy machine cycle count.
    pub type Instr = fn(&mut GameBoy) -> u32;

    pub fn op_table() -> &'static [Instr; 256] {
        static U: Instr = unhandled;
        static TABLE: [Instr; 256] = [
            // 0x00-0x07
            nop, ld_bc_nn, ld_bcm_a, U, U, U, ld_b_n, U,
            // 0x08-0x0F
            ld_nn_sp, U, ld_a_bcm, U, U, U, ld_c_n, U,
            // 0x10-0x17
            U, ld_de_nn, ld_dem_a, U, U, U, ld_d_n, U,
            // 0x18-0x1F
            U, U, ld_a_dem, U, U, U, ld_e_n, U,
            // 0x20-0x27
            U, ld_hl_nn, ldi_hlm_a, U, U, U, ld_h_n, U,
            // 0x28-0x2F
            U, U, ldi_a_hlm, U, U, U, ld_l_n, U,
            // 0x30-0x37
            U, ld_sp_nn, ldd_hlm_a, U, U, U, ld_hlm_n, U,
            // 0x38-0x3F
            U, U, ldd_a_hlm, U, U, U, ld_a_n, U,
            // 0x40-0x47 = LD B,*
            ld_b_b, ld_b_c, ld_b_d, ld_b_e, ld_b_h, ld_b_l, ld_b_hlm, ld_b_a,
            // 0x48-0x4F = LD C,*
            ld_c_b, ld_c_c, ld_c_d, ld_c_e, ld_c_h, ld_c_l, ld_c_hlm, ld_c_a,
            // 0x50-0x57 = LD D,*
            ld_d_b, ld_d_c, ld_d_d, ld_d_e, ld_d_h, ld_d_l, ld_d_hlm, ld_d_a,
            // 0x58-0x5F = LD E,*
            ld_e_b, ld_e_c, ld_e_d, ld_e_e, ld_e_h, ld_e_l, ld_e_hlm, ld_e_a,
            // 0x60-0x67 = LD H,*
            ld_h_b, ld_h_c, ld_h_d, ld_h_e, ld_h_h, ld_h_l, ld_h_hlm, ld_h_a,
            // 0x68-0x6F = LD L,*
            ld_l_b, ld_l_c, ld_l_d, ld_l_e, ld_l_h, ld_l_l, ld_l_hlm, ld_l_a,
            // 0x70-0x77 = LD (HL),*  (0x76 = HALT, unimplemented for now)
            ld_hlm_b, ld_hlm_c, ld_hlm_d, ld_hlm_e, ld_hlm_h, ld_hlm_l, U, ld_hlm_a,
            // 0x78-0x7F = LD A,*
            ld_a_b, ld_a_c, ld_a_d, ld_a_e, ld_a_h, ld_a_l, ld_a_hlm, ld_a_a,
            // 0x80-0xDF: later phases (arithmetic, jumps, CB prefix)
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            U, U, U, U, U, U, U, U,
            // 0xE0-0xE7
            ldh_n_a, U, ldh_c_a, U, U, U, U, U,
            // 0xE8-0xEF
            U, U, ld_nn_a, U, U, U, U, U,
            // 0xF0-0xF7
            ldh_a_n, U, ldh_a_c, U, U, U, U, U,
            // 0xF8-0xFF
            ld_hl_sp_n, ld_sp_hl, ld_a_nn, U, U, U, U, U,
        ];
        &TABLE
    }

    fn nop(_: &mut GameBoy) -> u32 {
        1
    }

    fn unhandled(_: &mut GameBoy) -> u32 {
        1
    }

    // --- helpers ---

    fn fetch_u8(gb: &mut GameBoy) -> u8 {
        let v = gb.mmu.read(gb.cpu.pc);
        gb.cpu.pc = gb.cpu.pc.wrapping_add(1);
        v
    }

    fn fetch_u16(gb: &mut GameBoy) -> u16 {
        let lo = fetch_u8(gb) as u16;
        let hi = fetch_u8(gb) as u16;
        crate::util::u8s_to_u16(hi as u8, lo as u8)
    }

    fn hl_addr(gb: &GameBoy) -> u16 {
        crate::util::u8s_to_u16(gb.cpu.h, gb.cpu.l)
    }

    // --- LD r,n (immediate byte) ---
    fn ld_b_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.b = fetch_u8(gb);
        2
    }
    fn ld_c_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.c = fetch_u8(gb);
        2
    }
    fn ld_d_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.d = fetch_u8(gb);
        2
    }
    fn ld_e_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.e = fetch_u8(gb);
        2
    }
    fn ld_h_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.h = fetch_u8(gb);
        2
    }
    fn ld_l_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.l = fetch_u8(gb);
        2
    }
    fn ld_a_n(gb: &mut GameBoy) -> u32 {
        gb.cpu.a = fetch_u8(gb);
        2
    }

    // --- LD (HL),n --- 0x36
    fn ld_hlm_n(gb: &mut GameBoy) -> u32 {
        let addr = hl_addr(gb);
        let v = fetch_u8(gb);
        gb.mmu.write(addr, v);
        3
    }

    // --- LD r,r' (register to register) ---
    macro_rules! ld_rr {
        ($name:ident, $dst:ident, $src:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                gb.cpu.$dst = gb.cpu.$src;
                1
            }
        };
    }

    macro_rules! ld_r_hlm {
        ($name:ident, $dst:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = hl_addr(gb);
                gb.cpu.$dst = gb.mmu.read(addr);
                2
            }
        };
    }

    macro_rules! ld_hlm_r {
        ($name:ident, $src:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = hl_addr(gb);
                let v = gb.cpu.$src;
                gb.mmu.write(addr, v);
                2
            }
        };
    }

    // B
    ld_rr!(ld_b_b, b, b); ld_rr!(ld_b_c, b, c); ld_rr!(ld_b_d, b, d); ld_rr!(ld_b_e, b, e);
    ld_rr!(ld_b_h, b, h); ld_rr!(ld_b_l, b, l); ld_rr!(ld_b_a, b, a);
    ld_r_hlm!(ld_b_hlm, b);
    // C
    ld_rr!(ld_c_b, c, b); ld_rr!(ld_c_c, c, c); ld_rr!(ld_c_d, c, d); ld_rr!(ld_c_e, c, e);
    ld_rr!(ld_c_h, c, h); ld_rr!(ld_c_l, c, l); ld_rr!(ld_c_a, c, a);
    ld_r_hlm!(ld_c_hlm, c);
    // D
    ld_rr!(ld_d_b, d, b); ld_rr!(ld_d_c, d, c); ld_rr!(ld_d_d, d, d); ld_rr!(ld_d_e, d, e);
    ld_rr!(ld_d_h, d, h); ld_rr!(ld_d_l, d, l); ld_rr!(ld_d_a, d, a);
    ld_r_hlm!(ld_d_hlm, d);
    // E
    ld_rr!(ld_e_b, e, b); ld_rr!(ld_e_c, e, c); ld_rr!(ld_e_d, e, d); ld_rr!(ld_e_e, e, e);
    ld_rr!(ld_e_h, e, h); ld_rr!(ld_e_l, e, l); ld_rr!(ld_e_a, e, a);
    ld_r_hlm!(ld_e_hlm, e);
    // H
    ld_rr!(ld_h_b, h, b); ld_rr!(ld_h_c, h, c); ld_rr!(ld_h_d, h, d); ld_rr!(ld_h_e, h, e);
    ld_rr!(ld_h_h, h, h); ld_rr!(ld_h_l, h, l); ld_rr!(ld_h_a, h, a);
    ld_r_hlm!(ld_h_hlm, h);
    // L
    ld_rr!(ld_l_b, l, b); ld_rr!(ld_l_c, l, c); ld_rr!(ld_l_d, l, d); ld_rr!(ld_l_e, l, e);
    ld_rr!(ld_l_h, l, h); ld_rr!(ld_l_l, l, l); ld_rr!(ld_l_a, l, a);
    ld_r_hlm!(ld_l_hlm, l);
    // A
    ld_rr!(ld_a_b, a, b); ld_rr!(ld_a_c, a, c); ld_rr!(ld_a_d, a, d); ld_rr!(ld_a_e, a, e);
    ld_rr!(ld_a_h, a, h); ld_rr!(ld_a_l, a, l); ld_rr!(ld_a_a, a, a);
    ld_r_hlm!(ld_a_hlm, a);

    // (HL),r
    ld_hlm_r!(ld_hlm_b, b);
    ld_hlm_r!(ld_hlm_c, c);
    ld_hlm_r!(ld_hlm_d, d);
    ld_hlm_r!(ld_hlm_e, e);
    ld_hlm_r!(ld_hlm_h, h);
    ld_hlm_r!(ld_hlm_l, l);
    ld_hlm_r!(ld_hlm_a, a);

    // --- 16-bit immediate loads ---

    fn ld_bc_nn(gb: &mut GameBoy) -> u32 {
        let v = fetch_u16(gb);
        gb.cpu.set_bc(v);
        3
    }

    fn ld_de_nn(gb: &mut GameBoy) -> u32 {
        let v = fetch_u16(gb);
        gb.cpu.set_de(v);
        3
    }

    fn ld_hl_nn(gb: &mut GameBoy) -> u32 {
        let v = fetch_u16(gb);
        gb.cpu.set_hl(v);
        3
    }

    fn ld_sp_nn(gb: &mut GameBoy) -> u32 {
        gb.cpu.sp = fetch_u16(gb);
        3
    }

    // 0x08: LD (nn),SP — store stack pointer little-endian at nn.
    fn ld_nn_sp(gb: &mut GameBoy) -> u32 {
        let addr = fetch_u16(gb);
        let (hi, lo) = crate::util::u16_to_u8s(gb.cpu.sp);
        gb.mmu.write(addr, lo);
        gb.mmu.write(addr.wrapping_add(1), hi);
        5
    }

    // 0xF9: LD SP,HL
    fn ld_sp_hl(gb: &mut GameBoy) -> u32 {
        gb.cpu.sp = gb.cpu.hl();
        2
    }

    // 0xF8: LD HL,SP+n — signed offset, sets H and C on low-byte carry.
    fn ld_hl_sp_n(gb: &mut GameBoy) -> u32 {
        let raw = fetch_u8(gb);
        let signed = raw as i8 as i16;
        let sp = gb.cpu.sp as i32;
        let result = sp + (signed as i32);

        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu
            .set_flag_h(((sp ^ (signed as i32) ^ result) & 0x10) != 0);
        gb.cpu
            .set_flag_c(((sp ^ (signed as i32) ^ result) & 0x100) != 0);

        gb.cpu.set_hl((result as u32 & 0xFFFF) as u16);
        3
    }

    // --- Indirect loads via BC/DE ---

    fn ld_bcm_a(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.bc();
        gb.mmu.write(addr, gb.cpu.a);
        2
    }

    fn ld_dem_a(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.de();
        gb.mmu.write(addr, gb.cpu.a);
        2
    }

    fn ld_a_bcm(gb: &mut GameBoy) -> u32 {
        gb.cpu.a = gb.mmu.read(gb.cpu.bc());
        2
    }

    fn ld_a_dem(gb: &mut GameBoy) -> u32 {
        gb.cpu.a = gb.mmu.read(gb.cpu.de());
        2
    }

    // --- Absolute address loads ---

    fn ld_nn_a(gb: &mut GameBoy) -> u32 {
        let addr = fetch_u16(gb);
        gb.mmu.write(addr, gb.cpu.a);
        4
    }

    fn ld_a_nn(gb: &mut GameBoy) -> u32 {
        let addr = fetch_u16(gb);
        gb.cpu.a = gb.mmu.read(addr);
        4
    }

    // --- High-RAM loads (0xFF00 + n/C) ---

    fn ldh_n_a(gb: &mut GameBoy) -> u32 {
        let n = fetch_u8(gb) as u16;
        gb.mmu.write(0xFF00 + n, gb.cpu.a);
        3
    }

    fn ldh_a_n(gb: &mut GameBoy) -> u32 {
        let n = fetch_u8(gb) as u16;
        gb.cpu.a = gb.mmu.read(0xFF00 + n);
        3
    }

    fn ldh_c_a(gb: &mut GameBoy) -> u32 {
        gb.mmu.write(0xFF00 + gb.cpu.c as u16, gb.cpu.a);
        2
    }

    fn ldh_a_c(gb: &mut GameBoy) -> u32 {
        gb.cpu.a = gb.mmu.read(0xFF00 + gb.cpu.c as u16);
        2
    }

    // --- LDI/LDD (post-increment/decrement via HL) ---

    fn ldi_hlm_a(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.hl();
        gb.mmu.write(addr, gb.cpu.a);
        gb.cpu.set_hl(addr.wrapping_add(1));
        2
    }

    fn ldi_a_hlm(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.hl();
        gb.cpu.a = gb.mmu.read(addr);
        gb.cpu.set_hl(addr.wrapping_add(1));
        2
    }

    fn ldd_hlm_a(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.hl();
        gb.mmu.write(addr, gb.cpu.a);
        gb.cpu.set_hl(addr.wrapping_sub(1));
        2
    }

    fn ldd_a_hlm(gb: &mut GameBoy) -> u32 {
        let addr = gb.cpu.hl();
        gb.cpu.a = gb.mmu.read(addr);
        gb.cpu.set_hl(addr.wrapping_sub(1));
        2
    }

    #[cfg(test)]
    mod tests {
        use crate::cartridge::Cartridge;
        use crate::gameboy::GameBoy;

        fn gb_with(ops: &[u8]) -> GameBoy {
            let mut rom = vec![0u8; 0x8000];
            for (i, b) in ops.iter().enumerate() {
                rom[0x0100 + i] = *b;
            }
            GameBoy::new(Cartridge::new(rom))
        }

        #[test]
        fn nop_advances_pc() {
            let mut gb = gb_with(&[0x00]);
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.pc, 0x0101);
        }

        #[test]
        fn ld_b_n() {
            let mut gb = gb_with(&[0x06, 0x42]);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.b, 0x42);
            assert_eq!(gb.cpu.pc, 0x0102);
        }

        #[test]
        fn ld_a_hl_mem() {
            let mut gb = gb_with(&[0x21, 0x00, 0xC0, 0x7E]);
            gb.mmu.write(0xC000, 0x99);
            gb.step(); // LD HL,0xC000
            let c = gb.step(); // LD A,(HL)
            assert_eq!(c, 2);
            assert_eq!(gb.cpu.a, 0x99);
        }

        #[test]
        fn ld_bc_nn_roundtrip() {
            let mut gb = gb_with(&[0x01, 0x34, 0x12]);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.bc(), 0x1234);
        }

        #[test]
        fn ld_hl_mem_n() {
            let mut gb = gb_with(&[0x36, 0xAB]);
            gb.cpu.set_hl(0xC000);
            let c = gb.step();
            assert_eq!(c, 3);
            assert_eq!(gb.mmu.read(0xC000), 0xAB);
        }

        #[test]
        fn ldh_store_and_load() {
            // LDH (0x80),A then LDH A,(0x80)
            let mut gb = gb_with(&[0xE0, 0x80, 0xF0, 0x80]);
            gb.cpu.a = 0x5A;
            assert_eq!(gb.step(), 3);
            gb.cpu.a = 0;
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.a, 0x5A);
        }

        #[test]
        fn ldi_hlm_a_increments_hl() {
            let mut gb = gb_with(&[0x22]);
            gb.cpu.set_hl(0xC000);
            gb.cpu.a = 0x77;
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.mmu.read(0xC000), 0x77);
            assert_eq!(gb.cpu.hl(), 0xC001);
        }

        #[test]
        fn ldd_hlm_a_decrements_hl() {
            let mut gb = gb_with(&[0x32]);
            gb.cpu.set_hl(0xC000);
            gb.cpu.a = 0x33;
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.mmu.read(0xC000), 0x33);
            assert_eq!(gb.cpu.hl(), 0xBFFF);
        }

        #[test]
        fn ld_nn_sp_little_endian() {
            let mut gb = gb_with(&[0x08, 0x00, 0xD0]);
            gb.cpu.sp = 0xFFFE;
            assert_eq!(gb.step(), 5);
            assert_eq!(gb.mmu.read(0xD000), 0xFE);
            assert_eq!(gb.mmu.read(0xD001), 0xFF);
        }
    }
}
