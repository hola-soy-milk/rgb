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
            nop, ld_bc_nn, ld_bcm_a, U, inc_b, dec_b, ld_b_n, U,
            // 0x08-0x0F
            ld_nn_sp, U, ld_a_bcm, U, inc_c, dec_c, ld_c_n, U,
            // 0x10-0x17
            U, ld_de_nn, ld_dem_a, U, inc_d, dec_d, ld_d_n, U,
            // 0x18-0x1F
            U, U, ld_a_dem, U, inc_e, dec_e, ld_e_n, U,
            // 0x20-0x27
            U, ld_hl_nn, ldi_hlm_a, U, inc_h, dec_h, ld_h_n, U,
            // 0x28-0x2F
            U, U, ldi_a_hlm, U, inc_l, dec_l, ld_l_n, U,
            // 0x30-0x37
            U, ld_sp_nn, ldd_hlm_a, U, inc_hlm, dec_hlm, ld_hlm_n, U,
            // 0x38-0x3F
            U, U, ldd_a_hlm, U, inc_a, dec_a, ld_a_n, U,
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
            // 0x80-0x87 = ADD A,r
            add_a_b, add_a_c, add_a_d, add_a_e, add_a_h, add_a_l, add_a_hlm, add_a_a,
            // 0x88-0x8F = ADC A,r
            adc_a_b, adc_a_c, adc_a_d, adc_a_e, adc_a_h, adc_a_l, adc_a_hlm, adc_a_a,
            // 0x90-0x97 = SUB A,r
            sub_a_b, sub_a_c, sub_a_d, sub_a_e, sub_a_h, sub_a_l, sub_a_hlm, sub_a_a,
            // 0x98-0x9F = SBC A,r
            sbc_a_b, sbc_a_c, sbc_a_d, sbc_a_e, sbc_a_h, sbc_a_l, sbc_a_hlm, sbc_a_a,
            // 0xA0-0xA7 = AND A,r
            and_a_b, and_a_c, and_a_d, and_a_e, and_a_h, and_a_l, and_a_hlm, and_a_a,
            // 0xA8-0xAF = XOR A,r
            xor_a_b, xor_a_c, xor_a_d, xor_a_e, xor_a_h, xor_a_l, xor_a_hlm, xor_a_a,
            // 0xB0-0xB7 = OR A,r
            or_a_b, or_a_c, or_a_d, or_a_e, or_a_h, or_a_l, or_a_hlm, or_a_a,
            // 0xB8-0xBF = CP A,r
            cp_a_b, cp_a_c, cp_a_d, cp_a_e, cp_a_h, cp_a_l, cp_a_hlm, cp_a_a,
            // 0xC0-0xC7
            U, U, U, U, U, U, add_a_n, U,
            // 0xC8-0xCF
            U, U, U, U, U, U, adc_a_n, U,
            // 0xD0-0xD7
            U, U, U, U, U, U, sub_a_n, U,
            // 0xD8-0xDF
            U, U, U, U, U, U, sbc_a_n, U,
            // 0xE0-0xE7
            ldh_n_a, U, ldh_c_a, U, U, U, and_a_n, U,
            // 0xE8-0xEF
            U, U, ld_nn_a, U, U, U, xor_a_n, U,
            // 0xF0-0xF7
            ldh_a_n, U, ldh_a_c, U, U, U, or_a_n, U,
            // 0xF8-0xFF
            ld_hl_sp_n, ld_sp_hl, ld_a_nn, U, U, U, cp_a_n, U,
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

    // --- 8-bit ALU core ---

    fn alu_add(gb: &mut GameBoy, val: u8, carry: bool) {
        let a = gb.cpu.a;
        let c = if carry && gb.cpu.flag_c() { 1 } else { 0 };
        let result = a as u16 + val as u16 + c as u16;
        let r8 = result as u8;
        gb.cpu.set_flag_z(r8 == 0);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h((a & 0x0F) + (val & 0x0F) + c > 0x0F);
        gb.cpu.set_flag_c(result > 0xFF);
        gb.cpu.a = r8;
    }

    fn alu_sub(gb: &mut GameBoy, val: u8, carry: bool) {
        let a = gb.cpu.a;
        let c = if carry && gb.cpu.flag_c() { 1 } else { 0 };
        let result = (a as i16) - (val as i16) - (c as i16);
        let r8 = result as u8;
        gb.cpu.set_flag_z(r8 == 0);
        gb.cpu.set_flag_n(true);
        gb.cpu.set_flag_h((a & 0x0F) < (val & 0x0F) + c);
        gb.cpu.set_flag_c(result < 0);
        gb.cpu.a = r8;
    }

    fn alu_and(gb: &mut GameBoy, val: u8, _carry: bool) {
        gb.cpu.a &= val;
        gb.cpu.set_flag_z(gb.cpu.a == 0);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(true);
        gb.cpu.set_flag_c(false);
    }

    fn alu_or(gb: &mut GameBoy, val: u8, _carry: bool) {
        gb.cpu.a |= val;
        gb.cpu.set_flag_z(gb.cpu.a == 0);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(false);
    }

    fn alu_xor(gb: &mut GameBoy, val: u8, _carry: bool) {
        gb.cpu.a ^= val;
        gb.cpu.set_flag_z(gb.cpu.a == 0);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(false);
    }

    fn alu_cp(gb: &mut GameBoy, val: u8, _carry: bool) {
        let a = gb.cpu.a;
        let result = (a as i16) - (val as i16);
        gb.cpu.set_flag_z(result as u8 == 0);
        gb.cpu.set_flag_n(true);
        gb.cpu.set_flag_h((a & 0x0F) < (val & 0x0F));
        gb.cpu.set_flag_c(result < 0);
    }

    fn alu_inc(gb: &mut GameBoy, val: u8) -> u8 {
        let r = val.wrapping_add(1);
        gb.cpu.set_flag_z(r == 0);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h((val & 0x0F) == 0x0F);
        r
    }

    fn alu_dec(gb: &mut GameBoy, val: u8) -> u8 {
        let r = val.wrapping_sub(1);
        gb.cpu.set_flag_z(r == 0);
        gb.cpu.set_flag_n(true);
        gb.cpu.set_flag_h((val & 0x0F) == 0);
        r
    }

    // --- ALU handler generators ---

    macro_rules! alu_r {
        ($name:ident, $alu:ident, $src:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$src;
                $alu(gb, v, false);
                1
            }
        };
        ($name:ident, $alu:ident, $src:ident, carry) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$src;
                $alu(gb, v, true);
                1
            }
        };
    }

    macro_rules! alu_hlm {
        ($name:ident, $alu:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = hl_addr(gb);
                let v = gb.mmu.read(addr);
                $alu(gb, v, false);
                2
            }
        };
        ($name:ident, $alu:ident, carry) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = hl_addr(gb);
                let v = gb.mmu.read(addr);
                $alu(gb, v, true);
                2
            }
        };
    }

    macro_rules! alu_n {
        ($name:ident, $alu:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = fetch_u8(gb);
                $alu(gb, v, false);
                2
            }
        };
        ($name:ident, $alu:ident, carry) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = fetch_u8(gb);
                $alu(gb, v, true);
                2
            }
        };
    }

    macro_rules! inc_r {
        ($name:ident, $reg:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$reg;
                let r = alu_inc(gb, v);
                gb.cpu.$reg = r;
                1
            }
        };
    }

    macro_rules! dec_r {
        ($name:ident, $reg:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$reg;
                let r = alu_dec(gb, v);
                gb.cpu.$reg = r;
                1
            }
        };
    }

    // ADD
    alu_r!(add_a_b, alu_add, b); alu_r!(add_a_c, alu_add, c); alu_r!(add_a_d, alu_add, d);
    alu_r!(add_a_e, alu_add, e); alu_r!(add_a_h, alu_add, h); alu_r!(add_a_l, alu_add, l);
    alu_r!(add_a_a, alu_add, a);
    alu_hlm!(add_a_hlm, alu_add);
    alu_n!(add_a_n, alu_add);

    // ADC
    alu_r!(adc_a_b, alu_add, b, carry); alu_r!(adc_a_c, alu_add, c, carry);
    alu_r!(adc_a_d, alu_add, d, carry); alu_r!(adc_a_e, alu_add, e, carry);
    alu_r!(adc_a_h, alu_add, h, carry); alu_r!(adc_a_l, alu_add, l, carry);
    alu_r!(adc_a_a, alu_add, a, carry);
    alu_hlm!(adc_a_hlm, alu_add, carry);
    alu_n!(adc_a_n, alu_add, carry);

    // SUB
    alu_r!(sub_a_b, alu_sub, b); alu_r!(sub_a_c, alu_sub, c); alu_r!(sub_a_d, alu_sub, d);
    alu_r!(sub_a_e, alu_sub, e); alu_r!(sub_a_h, alu_sub, h); alu_r!(sub_a_l, alu_sub, l);
    alu_r!(sub_a_a, alu_sub, a);
    alu_hlm!(sub_a_hlm, alu_sub);
    alu_n!(sub_a_n, alu_sub);

    // SBC
    alu_r!(sbc_a_b, alu_sub, b, carry); alu_r!(sbc_a_c, alu_sub, c, carry);
    alu_r!(sbc_a_d, alu_sub, d, carry); alu_r!(sbc_a_e, alu_sub, e, carry);
    alu_r!(sbc_a_h, alu_sub, h, carry); alu_r!(sbc_a_l, alu_sub, l, carry);
    alu_r!(sbc_a_a, alu_sub, a, carry);
    alu_hlm!(sbc_a_hlm, alu_sub, carry);
    alu_n!(sbc_a_n, alu_sub, carry);

    // AND
    alu_r!(and_a_b, alu_and, b); alu_r!(and_a_c, alu_and, c); alu_r!(and_a_d, alu_and, d);
    alu_r!(and_a_e, alu_and, e); alu_r!(and_a_h, alu_and, h); alu_r!(and_a_l, alu_and, l);
    alu_r!(and_a_a, alu_and, a);
    alu_hlm!(and_a_hlm, alu_and);
    alu_n!(and_a_n, alu_and);

    // XOR
    alu_r!(xor_a_b, alu_xor, b); alu_r!(xor_a_c, alu_xor, c); alu_r!(xor_a_d, alu_xor, d);
    alu_r!(xor_a_e, alu_xor, e); alu_r!(xor_a_h, alu_xor, h); alu_r!(xor_a_l, alu_xor, l);
    alu_r!(xor_a_a, alu_xor, a);
    alu_hlm!(xor_a_hlm, alu_xor);
    alu_n!(xor_a_n, alu_xor);

    // OR
    alu_r!(or_a_b, alu_or, b); alu_r!(or_a_c, alu_or, c); alu_r!(or_a_d, alu_or, d);
    alu_r!(or_a_e, alu_or, e); alu_r!(or_a_h, alu_or, h); alu_r!(or_a_l, alu_or, l);
    alu_r!(or_a_a, alu_or, a);
    alu_hlm!(or_a_hlm, alu_or);
    alu_n!(or_a_n, alu_or);

    // CP
    alu_r!(cp_a_b, alu_cp, b); alu_r!(cp_a_c, alu_cp, c); alu_r!(cp_a_d, alu_cp, d);
    alu_r!(cp_a_e, alu_cp, e); alu_r!(cp_a_h, alu_cp, h); alu_r!(cp_a_l, alu_cp, l);
    alu_r!(cp_a_a, alu_cp, a);
    alu_hlm!(cp_a_hlm, alu_cp);
    alu_n!(cp_a_n, alu_cp);

    // INC
    inc_r!(inc_b, b); inc_r!(inc_c, c); inc_r!(inc_d, d); inc_r!(inc_e, e);
    inc_r!(inc_h, h); inc_r!(inc_l, l); inc_r!(inc_a, a);

    fn inc_hlm(gb: &mut GameBoy) -> u32 {
        let addr = hl_addr(gb);
        let v = gb.mmu.read(addr);
        let r = alu_inc(gb, v);
        gb.mmu.write(addr, r);
        3
    }

    // DEC
    dec_r!(dec_b, b); dec_r!(dec_c, c); dec_r!(dec_d, d); dec_r!(dec_e, e);
    dec_r!(dec_h, h); dec_r!(dec_l, l); dec_r!(dec_a, a);

    fn dec_hlm(gb: &mut GameBoy) -> u32 {
        let addr = hl_addr(gb);
        let v = gb.mmu.read(addr);
        let r = alu_dec(gb, v);
        gb.mmu.write(addr, r);
        3
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

        // --- ALU tests ---

        #[test]
        fn add_a_b_sets_carry_and_halfcarry() {
            let mut gb = gb_with(&[0x80]); // ADD A,B
            gb.cpu.a = 0x3A;
            gb.cpu.b = 0xC6;
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.a, 0x00);
            assert!(gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn adc_a_n_uses_carry() {
            let mut gb = gb_with(&[0xCE, 0x3C]); // ADC A,0x3C
            gb.cpu.a = 0xE1;
            gb.cpu.set_flag_c(true);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.a, 0x1E);
            assert!(!gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());
            assert!(!gb.cpu.flag_h());
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn sub_a_b_borrow_flags() {
            let mut gb = gb_with(&[0x90]); // SUB A,B
            gb.cpu.a = 0x3E;
            gb.cpu.b = 0x3E;
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.a, 0x00);
            assert!(gb.cpu.flag_z());
            assert!(gb.cpu.flag_n());
            assert!(!gb.cpu.flag_h());
            assert!(!gb.cpu.flag_c());
        }

        #[test]
        fn sub_a_n_halfcarry_and_carry() {
            let mut gb = gb_with(&[0xD6, 0x40]); // SUB A,0x40
            gb.cpu.a = 0x00;
            gb.step();
            assert_eq!(gb.cpu.a, 0xC0);
            assert!(!gb.cpu.flag_z());
            assert!(gb.cpu.flag_n());
            assert!(!gb.cpu.flag_h());
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn sbc_a_b_includes_carry() {
            let mut gb = gb_with(&[0x98]); // SBC A,B
            gb.cpu.a = 0x10;
            gb.cpu.b = 0x0F;
            gb.cpu.set_flag_c(true);
            gb.step();
            assert_eq!(gb.cpu.a, 0x00);
            assert!(gb.cpu.flag_z());
            assert!(gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
        }

        #[test]
        fn and_sets_half_carry() {
            let mut gb = gb_with(&[0xA7]); // AND A
            gb.cpu.a = 0x5A;
            gb.step();
            assert_eq!(gb.cpu.a, 0x5A);
            assert!(!gb.cpu.flag_z());
            assert!(gb.cpu.flag_h());
            assert!(!gb.cpu.flag_c());
            assert!(!gb.cpu.flag_n());
        }

        #[test]
        fn xor_a_clears_a() {
            let mut gb = gb_with(&[0xAF]); // XOR A
            gb.cpu.a = 0xFF;
            gb.step();
            assert_eq!(gb.cpu.a, 0);
            assert!(gb.cpu.flag_z());
            assert!(!gb.cpu.flag_h());
            assert!(!gb.cpu.flag_c());
        }

        #[test]
        fn or_a_b_result() {
            let mut gb = gb_with(&[0xB0]); // OR A,B
            gb.cpu.a = 0x5A;
            gb.cpu.b = 0x03;
            gb.step();
            assert_eq!(gb.cpu.a, 0x5B);
            assert!(!gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());
        }

        #[test]
        fn cp_leaves_a_and_sets_flags() {
            let mut gb = gb_with(&[0xB8]); // CP A,B
            gb.cpu.a = 0x3C;
            gb.cpu.b = 0x2F;
            gb.step();
            assert_eq!(gb.cpu.a, 0x3C); // unchanged
            assert!(!gb.cpu.flag_z());
            assert!(gb.cpu.flag_n());
            assert!(gb.cpu.flag_h()); // 0xC < 0xF → borrow from bit 4
            assert!(!gb.cpu.flag_c());
        }

        #[test]
        fn inc_b_zero_flag_and_half_carry() {
            let mut gb = gb_with(&[0x04]); // INC B
            gb.cpu.b = 0xFF;
            gb.cpu.set_flag_c(true);
            gb.step();
            assert_eq!(gb.cpu.b, 0x00);
            assert!(gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
            assert!(gb.cpu.flag_c()); // INC preserves C
        }

        #[test]
        fn dec_b_borrow_from_bit4() {
            let mut gb = gb_with(&[0x05]); // DEC B
            gb.cpu.b = 0x10;
            gb.step();
            assert_eq!(gb.cpu.b, 0x0F);
            assert!(!gb.cpu.flag_z());
            assert!(gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
        }

        #[test]
        fn inc_hlm_memory() {
            let mut gb = gb_with(&[0x34]); // INC (HL)
            gb.cpu.set_hl(0xC000);
            gb.mmu.write(0xC000, 0x50);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.mmu.read(0xC000), 0x51);
            assert!(!gb.cpu.flag_z());
            assert!(!gb.cpu.flag_h());
        }
    }
}
