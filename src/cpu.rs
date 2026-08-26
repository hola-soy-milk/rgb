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
    /// Interrupt master enable flag. Set by EI/RETI, cleared by DI.
    pub ime: bool,
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
            ime: false,
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
            nop, ld_bc_nn, ld_bcm_a, inc_bc, inc_b, dec_b, ld_b_n, rlca,
            // 0x08-0x0F
            ld_nn_sp, add_hl_bc, ld_a_bcm, dec_bc, inc_c, dec_c, ld_c_n, rrca,
            // 0x10-0x17
            stop, ld_de_nn, ld_dem_a, inc_de, inc_d, dec_d, ld_d_n, rla,
            // 0x18-0x1F
            jr_n, add_hl_de, ld_a_dem, dec_de, inc_e, dec_e, ld_e_n, rra,
            // 0x20-0x27
            jr_nz, ld_hl_nn, ldi_hlm_a, inc_hl, inc_h, dec_h, ld_h_n, daa,
            // 0x28-0x2F
            jr_z, add_hl_hl, ldi_a_hlm, dec_hl, inc_l, dec_l, ld_l_n, cpl,
            // 0x30-0x37
            jr_nc, ld_sp_nn, ldd_hlm_a, inc_sp, inc_hlm, dec_hlm, ld_hlm_n, scf,
            // 0x38-0x3F
            jr_c, add_hl_sp, ldd_a_hlm, dec_sp, inc_a, dec_a, ld_a_n, ccf,
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
            ld_hlm_b, ld_hlm_c, ld_hlm_d, ld_hlm_e, ld_hlm_h, ld_hlm_l, halt, ld_hlm_a,
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
            ret_nz, U, jp_nz_nn, jp_nn, call_nz_nn, U, add_a_n, rst_00,
            // 0xC8-0xCF
            ret_z, ret, jp_z_nn, U, call_z_nn, call_nn, adc_a_n, rst_08,
            // 0xD0-0xD7
            ret_nc, U, jp_nc_nn, U, call_nc_nn, U, sub_a_n, rst_10,
            // 0xD8-0xDF
            ret_c, reti, jp_c_nn, U, call_c_nn, U, sbc_a_n, rst_18,
            // 0xE0-0xE7
            ldh_n_a, U, ldh_c_a, U, U, U, and_a_n, rst_20,
            // 0xE8-0xEF
            add_sp_n, jp_hl, ld_nn_a, U, U, U, xor_a_n, rst_28,
            // 0xF0-0xF7
            ldh_a_n, U, ldh_a_c, di, U, U, or_a_n, rst_30,
            // 0xF8-0xFF
            ld_hl_sp_n, ld_sp_hl, ld_a_nn, ei, U, U, cp_a_n, rst_38,
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

    // --- Stack helpers ---

    fn push_u16(gb: &mut GameBoy, v: u16) {
        let (hi, lo) = crate::util::u16_to_u8s(v);
        gb.cpu.sp = gb.cpu.sp.wrapping_sub(1);
        gb.mmu.write(gb.cpu.sp, hi);
        gb.cpu.sp = gb.cpu.sp.wrapping_sub(1);
        gb.mmu.write(gb.cpu.sp, lo);
    }

    fn pop_u16(gb: &mut GameBoy) -> u16 {
        let lo = gb.mmu.read(gb.cpu.sp);
        gb.cpu.sp = gb.cpu.sp.wrapping_add(1);
        let hi = gb.mmu.read(gb.cpu.sp);
        gb.cpu.sp = gb.cpu.sp.wrapping_add(1);
        crate::util::u8s_to_u16(hi, lo)
    }

    // --- Jumps ---

    fn jp_nn(gb: &mut GameBoy) -> u32 {
        gb.cpu.pc = fetch_u16(gb);
        4
    }

    fn jp_hl(gb: &mut GameBoy) -> u32 {
        gb.cpu.pc = gb.cpu.hl();
        1
    }

    macro_rules! jp_cc {
        ($name:ident, $flag:ident, $expect:expr) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = fetch_u16(gb);
                if gb.cpu.$flag() == $expect {
                    gb.cpu.pc = addr;
                    4
                } else {
                    3
                }
            }
        };
    }

    jp_cc!(jp_nz_nn, flag_z, false);
    jp_cc!(jp_z_nn, flag_z, true);
    jp_cc!(jp_nc_nn, flag_c, false);
    jp_cc!(jp_c_nn, flag_c, true);

    // --- Relative jumps (signed 8-bit offset) ---

    fn jr_n(gb: &mut GameBoy) -> u32 {
        let offset = fetch_u8(gb) as i8;
        gb.cpu.pc = gb.cpu.pc.wrapping_add(offset as u16);
        3
    }

    macro_rules! jr_cc {
        ($name:ident, $flag:ident, $expect:expr) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let offset = fetch_u8(gb) as i8;
                if gb.cpu.$flag() == $expect {
                    gb.cpu.pc = gb.cpu.pc.wrapping_add(offset as u16);
                    3
                } else {
                    2
                }
            }
        };
    }

    jr_cc!(jr_nz, flag_z, false);
    jr_cc!(jr_z, flag_z, true);
    jr_cc!(jr_nc, flag_c, false);
    jr_cc!(jr_c, flag_c, true);

    // --- Calls ---

    fn call_nn(gb: &mut GameBoy) -> u32 {
        let addr = fetch_u16(gb);
        push_u16(gb, gb.cpu.pc);
        gb.cpu.pc = addr;
        6
    }

    macro_rules! call_cc {
        ($name:ident, $flag:ident, $expect:expr) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let addr = fetch_u16(gb);
                if gb.cpu.$flag() == $expect {
                    push_u16(gb, gb.cpu.pc);
                    gb.cpu.pc = addr;
                    6
                } else {
                    3
                }
            }
        };
    }

    call_cc!(call_nz_nn, flag_z, false);
    call_cc!(call_z_nn, flag_z, true);
    call_cc!(call_nc_nn, flag_c, false);
    call_cc!(call_c_nn, flag_c, true);

    // --- Returns ---

    fn ret(gb: &mut GameBoy) -> u32 {
        gb.cpu.pc = pop_u16(gb);
        4
    }

    macro_rules! ret_cc {
        ($name:ident, $flag:ident, $expect:expr) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                if gb.cpu.$flag() == $expect {
                    gb.cpu.pc = pop_u16(gb);
                    5
                } else {
                    2
                }
            }
        };
    }

    ret_cc!(ret_nz, flag_z, false);
    ret_cc!(ret_z, flag_z, true);
    ret_cc!(ret_nc, flag_c, false);
    ret_cc!(ret_c, flag_c, true);

    fn reti(gb: &mut GameBoy) -> u32 {
        gb.cpu.pc = pop_u16(gb);
        gb.cpu.ime = true;
        4
    }

    // --- Restarts (call to fixed address in page 0) ---

    macro_rules! rst {
        ($name:ident, $addr:expr) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                push_u16(gb, gb.cpu.pc);
                gb.cpu.pc = $addr;
                4
            }
        };
    }

    rst!(rst_00, 0x00);
    rst!(rst_08, 0x08);
    rst!(rst_10, 0x10);
    rst!(rst_18, 0x18);
    rst!(rst_20, 0x20);
    rst!(rst_28, 0x28);
    rst!(rst_30, 0x30);
    rst!(rst_38, 0x38);

    // --- 16-bit INC/DEC (no flag changes) ---

    macro_rules! inc_rr {
        ($name:ident, $get:ident, $set:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$get().wrapping_add(1);
                gb.cpu.$set(v);
                2
            }
        };
    }

    macro_rules! dec_rr {
        ($name:ident, $get:ident, $set:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let v = gb.cpu.$get().wrapping_sub(1);
                gb.cpu.$set(v);
                2
            }
        };
    }

    inc_rr!(inc_bc, bc, set_bc);
    dec_rr!(dec_bc, bc, set_bc);
    inc_rr!(inc_de, de, set_de);
    dec_rr!(dec_de, de, set_de);
    inc_rr!(inc_hl, hl, set_hl);
    dec_rr!(dec_hl, hl, set_hl);

    fn inc_sp(gb: &mut GameBoy) -> u32 {
        gb.cpu.sp = gb.cpu.sp.wrapping_add(1);
        2
    }

    fn dec_sp(gb: &mut GameBoy) -> u32 {
        gb.cpu.sp = gb.cpu.sp.wrapping_sub(1);
        2
    }

    // --- ADD HL,rr (N cleared, H/C set from 16-bit add, Z preserved) ---

    macro_rules! add_hl {
        ($name:ident, $get:ident) => {
            fn $name(gb: &mut GameBoy) -> u32 {
                let hl = gb.cpu.hl();
                let v = gb.cpu.$get();
                let result = hl as u32 + v as u32;
                gb.cpu.set_flag_n(false);
                gb.cpu
                    .set_flag_h(((hl & 0x0FFF) + (v & 0x0FFF)) > 0x0FFF);
                gb.cpu.set_flag_c(result > 0xFFFF);
                gb.cpu.set_hl(result as u16);
                2
            }
        };
    }

    add_hl!(add_hl_bc, bc);
    add_hl!(add_hl_de, de);
    add_hl!(add_hl_hl, hl);

    fn add_hl_sp(gb: &mut GameBoy) -> u32 {
        let hl = gb.cpu.hl();
        let v = gb.cpu.sp;
        let result = hl as u32 + v as u32;
        gb.cpu.set_flag_n(false);
        gb.cpu
            .set_flag_h(((hl & 0x0FFF) + (v & 0x0FFF)) > 0x0FFF);
        gb.cpu.set_flag_c(result > 0xFFFF);
        gb.cpu.set_hl(result as u16);
        2
    }

    // 0xE8: ADD SP,n — signed offset; Z and N cleared, H/C from low-byte carry.
    fn add_sp_n(gb: &mut GameBoy) -> u32 {
        let raw = fetch_u8(gb);
        let signed = raw as i8 as i32;
        let sp = gb.cpu.sp as i32;
        let result = sp + signed;

        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(((sp ^ signed ^ result) & 0x10) != 0);
        gb.cpu.set_flag_c(((sp ^ signed ^ result) & 0x100) != 0);
        gb.cpu.sp = result as u16;
        4
    }

    // --- Accumulator rotates (Z always cleared; N, H cleared; C gets shifted bit) ---

    fn rlca(gb: &mut GameBoy) -> u32 {
        let a = gb.cpu.a;
        let carry = a & 0x80 != 0;
        gb.cpu.a = a.rotate_left(1);
        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(carry);
        1
    }

    fn rla(gb: &mut GameBoy) -> u32 {
        let a = gb.cpu.a;
        let old_carry = gb.cpu.flag_c() as u8;
        let carry = a & 0x80 != 0;
        gb.cpu.a = (a << 1) | old_carry;
        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(carry);
        1
    }

    fn rrca(gb: &mut GameBoy) -> u32 {
        let a = gb.cpu.a;
        let carry = a & 0x01 != 0;
        gb.cpu.a = a.rotate_right(1);
        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(carry);
        1
    }

    fn rra(gb: &mut GameBoy) -> u32 {
        let a = gb.cpu.a;
        let old_carry = gb.cpu.flag_c() as u8;
        let carry = a & 0x01 != 0;
        gb.cpu.a = (a >> 1) | (old_carry << 7);
        gb.cpu.set_flag_z(false);
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(carry);
        1
    }

    // --- Flag/accumulator adjust ---

    // 0x27: DAA — decimal adjust after BCD add/subtract.
    fn daa(gb: &mut GameBoy) -> u32 {
        let mut a = gb.cpu.a;
        let mut adjust = 0u8;
        let mut carry = gb.cpu.flag_c();

        if gb.cpu.flag_h() || (!gb.cpu.flag_n() && (a & 0x0F) > 9) {
            adjust |= 0x06;
        }
        if gb.cpu.flag_c() || (!gb.cpu.flag_n() && a > 0x99) {
            adjust |= 0x60;
            carry = true;
        }

        a = if gb.cpu.flag_n() {
            a.wrapping_sub(adjust)
        } else {
            a.wrapping_add(adjust)
        };

        gb.cpu.a = a;
        gb.cpu.set_flag_z(a == 0);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(carry);
        1
    }

    // 0x2F: CPL — complement A, set N and H.
    fn cpl(gb: &mut GameBoy) -> u32 {
        gb.cpu.a = !gb.cpu.a;
        gb.cpu.set_flag_n(true);
        gb.cpu.set_flag_h(true);
        1
    }

    // 0x37: SCF — set carry, clear N and H.
    fn scf(gb: &mut GameBoy) -> u32 {
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(true);
        1
    }

    // 0x3F: CCF — complement carry, clear N and H.
    fn ccf(gb: &mut GameBoy) -> u32 {
        let c = gb.cpu.flag_c();
        gb.cpu.set_flag_n(false);
        gb.cpu.set_flag_h(false);
        gb.cpu.set_flag_c(!c);
        1
    }

    // --- CPU control ---

    // 0x76: HALT — no-op until interrupts are implemented.
    fn halt(_: &mut GameBoy) -> u32 {
        1
    }

    // 0x10: STOP — consumes a padding byte; no-op for now.
    fn stop(gb: &mut GameBoy) -> u32 {
        let _ = fetch_u8(gb);
        1
    }

    // 0xF3: DI — disable interrupts.
    fn di(gb: &mut GameBoy) -> u32 {
        gb.cpu.ime = false;
        1
    }

    // 0xFB: EI — enable interrupts (takes effect after the next instruction).
    fn ei(gb: &mut GameBoy) -> u32 {
        gb.cpu.ime = true;
        1
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

        // --- Phase 5: jumps, calls, returns, stack ---

        #[test]
        fn jp_nn_sets_pc() {
            let mut gb = gb_with(&[0xC3, 0x50, 0x01]); // JP 0x0150
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.pc, 0x0150);
        }

        #[test]
        fn jp_hl_sets_pc() {
            let mut gb = gb_with(&[0xE9]); // JP HL
            gb.cpu.set_hl(0xC000);
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.pc, 0xC000);
        }

        #[test]
        fn jp_cc_taken_and_not_taken() {
            // JP NZ,0x0200 with Z set: not taken (3 cycles, PC past operands)
            let mut gb = gb_with(&[0xC2, 0x00, 0x02]);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.pc, 0x0103);

            // JP NZ,0x0200 with Z clear: taken
            let mut gb = gb_with(&[0xC2, 0x00, 0x02]);
            gb.cpu.set_flag_z(false);
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.pc, 0x0200);

            // JP C,0x0200 with C set: taken
            let mut gb = gb_with(&[0xDA, 0x00, 0x02]);
            gb.cpu.set_flag_c(true);
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.pc, 0x0200);
        }

        #[test]
        fn jr_n_relative_forward_and_backward() {
            // JR +2 from 0x0102 -> 0x0104
            let mut gb = gb_with(&[0x18, 0x02]);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.pc, 0x0104);

            // JR -2 from 0x0102 -> 0x0100
            let mut gb = gb_with(&[0x18, 0xFE]);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.pc, 0x0100);
        }

        #[test]
        fn jr_cc_taken_and_not_taken() {
            // JR Z,+2 with Z clear: not taken
            let mut gb = gb_with(&[0x28, 0x02]);
            gb.cpu.set_flag_z(false);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.pc, 0x0102);

            // JR Z,+2 with Z set: taken
            let mut gb = gb_with(&[0x28, 0x02]);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.pc, 0x0104);

            // JR NC,+2 with C set: not taken
            let mut gb = gb_with(&[0x30, 0x02]);
            gb.cpu.set_flag_c(true);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.pc, 0x0102);
        }

        #[test]
        fn call_pushes_return_address_and_jumps() {
            let mut gb = gb_with(&[0xCD, 0x00, 0x02]); // CALL 0x0200
            assert_eq!(gb.step(), 6);
            assert_eq!(gb.cpu.pc, 0x0200);
            assert_eq!(gb.cpu.sp, 0xFFFC);
            // Return address 0x0103 stored little-endian on the stack.
            assert_eq!(gb.mmu.read(0xFFFC), 0x03);
            assert_eq!(gb.mmu.read(0xFFFD), 0x01);
        }

        #[test]
        fn call_cc_respects_condition() {
            // CALL Z,0x0200 with Z clear: not taken
            let mut gb = gb_with(&[0xCC, 0x00, 0x02]);
            gb.cpu.set_flag_z(false);
            assert_eq!(gb.step(), 3);
            assert_eq!(gb.cpu.pc, 0x0103);
            assert_eq!(gb.cpu.sp, 0xFFFE);

            // CALL Z,0x0200 with Z set: taken
            let mut gb = gb_with(&[0xCC, 0x00, 0x02]);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 6);
            assert_eq!(gb.cpu.pc, 0x0200);
            assert_eq!(gb.cpu.sp, 0xFFFC);
        }

        #[test]
        fn call_then_ret_roundtrip() {
            // 0x0100: CALL 0x0150 ... 0x0150: RET
            let mut rom = vec![0u8; 0x8000];
            rom[0x0100] = 0xCD;
            rom[0x0101] = 0x50;
            rom[0x0102] = 0x01;
            rom[0x0150] = 0xC9;
            let mut gb = GameBoy::new(Cartridge::new(rom));
            gb.step(); // CALL
            let cycles = gb.step(); // RET
            assert_eq!(cycles, 4);
            assert_eq!(gb.cpu.pc, 0x0103);
            assert_eq!(gb.cpu.sp, 0xFFFE);
        }

        #[test]
        fn ret_cc_respects_condition() {
            // RET NZ with Z set: not taken
            let mut gb = gb_with(&[0xC0]);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.pc, 0x0101);

            // RET NZ with Z clear: pops
            let mut gb = gb_with(&[0xC0]);
            gb.cpu.set_flag_z(false);
            gb.mmu.write(0xFFFC, 0x34);
            gb.mmu.write(0xFFFD, 0x12);
            gb.cpu.sp = 0xFFFC;
            assert_eq!(gb.step(), 5);
            assert_eq!(gb.cpu.pc, 0x1234);
            assert_eq!(gb.cpu.sp, 0xFFFE);
        }

        #[test]
        fn reti_pops_and_enables_interrupts() {
            let mut gb = gb_with(&[0xD9]); // RETI
            gb.mmu.write(0xFFFC, 0x00);
            gb.mmu.write(0xFFFD, 0x03);
            gb.cpu.sp = 0xFFFC;
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.pc, 0x0300);
            assert!(gb.cpu.ime);
        }

        #[test]
        fn rst_jumps_to_fixed_address() {
            let mut gb = gb_with(&[0xFF]); // RST 0x38
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.pc, 0x0038);
            assert_eq!(gb.cpu.sp, 0xFFFC);
            assert_eq!(gb.mmu.read(0xFFFC), 0x01);
            assert_eq!(gb.mmu.read(0xFFFD), 0x01);
        }

        // --- Phase 5: 16-bit INC/DEC ---

        #[test]
        fn inc_dec_16bit_wrap_and_keep_flags() {
            let mut gb = gb_with(&[0x03, 0x0B]); // INC BC, DEC BC
            gb.cpu.set_bc(0x00FF);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.bc(), 0x0100);
            assert!(gb.cpu.flag_z()); // 16-bit INC does not touch flags
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.bc(), 0x00FF);
        }

        #[test]
        fn inc_dec_sp() {
            let mut gb = gb_with(&[0x33, 0x3B]); // INC SP, DEC SP
            gb.cpu.sp = 0xFFFF;
            gb.step();
            assert_eq!(gb.cpu.sp, 0x0000);
            gb.step();
            assert_eq!(gb.cpu.sp, 0xFFFF);
        }

        // --- Phase 5: ADD HL / ADD SP ---

        #[test]
        fn add_hl_bc_sets_carry_and_halfcarry() {
            let mut gb = gb_with(&[0x09]); // ADD HL,BC
            gb.cpu.set_hl(0x8FFF);
            gb.cpu.set_bc(0x8001);
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 2);
            assert_eq!(gb.cpu.hl(), 0x1000);
            assert!(gb.cpu.flag_z()); // preserved
            assert!(!gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn add_sp_n_signed_offset() {
            let mut gb = gb_with(&[0xE8, 0x02]); // ADD SP,+2
            gb.cpu.sp = 0xFFF8;
            assert_eq!(gb.step(), 4);
            assert_eq!(gb.cpu.sp, 0xFFFA);
            assert!(!gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());

            let mut gb = gb_with(&[0xE8, 0xFE]); // ADD SP,-2
            gb.cpu.sp = 0x0002;
            gb.step();
            assert_eq!(gb.cpu.sp, 0x0000);
            assert!(gb.cpu.flag_h()); // 0x02 + 0xFE carries from bit 3
            assert!(gb.cpu.flag_c());
        }

        // --- Phase 5: rotates ---

        #[test]
        fn rlca_rotates_and_clears_z() {
            let mut gb = gb_with(&[0x07]); // RLCA
            gb.cpu.a = 0x85;
            gb.cpu.set_flag_z(true);
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.a, 0x0B);
            assert!(gb.cpu.flag_c());
            assert!(!gb.cpu.flag_z());
            assert!(!gb.cpu.flag_n());
            assert!(!gb.cpu.flag_h());
        }

        #[test]
        fn rla_shifts_through_carry() {
            let mut gb = gb_with(&[0x17]); // RLA
            gb.cpu.a = 0x80;
            gb.cpu.set_flag_c(true);
            gb.step();
            assert_eq!(gb.cpu.a, 0x01);
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn rrca_rotates_right() {
            let mut gb = gb_with(&[0x0F]); // RRCA
            gb.cpu.a = 0x01;
            gb.step();
            assert_eq!(gb.cpu.a, 0x80);
            assert!(gb.cpu.flag_c());
        }

        #[test]
        fn rra_shifts_right_through_carry() {
            let mut gb = gb_with(&[0x1F]); // RRA
            gb.cpu.a = 0x01;
            gb.cpu.set_flag_c(true);
            gb.step();
            assert_eq!(gb.cpu.a, 0x80);
            assert!(gb.cpu.flag_c());
        }

        // --- Phase 5: DAA, CPL, SCF, CCF, DI, EI ---

        #[test]
        fn daa_after_addition() {
            // 0x15 + 0x27 = 0x3C; DAA -> 0x42 (BCD)
            let mut gb = gb_with(&[0x27]); // DAA
            gb.cpu.a = 0x3C;
            gb.cpu.set_flag_n(false);
            gb.cpu.set_flag_h(true);
            gb.cpu.set_flag_c(false);
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.a, 0x42);
            assert!(!gb.cpu.flag_h());
            assert!(!gb.cpu.flag_c());
        }

        #[test]
        fn daa_after_subtraction() {
            // 0x42 - 0x15 = 0x2D; DAA -> 0x27 (BCD)
            let mut gb = gb_with(&[0x27]);
            gb.cpu.a = 0x2D;
            gb.cpu.set_flag_n(true);
            gb.cpu.set_flag_h(true);
            gb.cpu.set_flag_c(false);
            gb.step();
            assert_eq!(gb.cpu.a, 0x27);
            assert!(!gb.cpu.flag_c());
        }

        #[test]
        fn cpl_complements_a_and_sets_flags() {
            let mut gb = gb_with(&[0x2F]); // CPL
            gb.cpu.a = 0x35;
            assert_eq!(gb.step(), 1);
            assert_eq!(gb.cpu.a, 0xCA);
            assert!(gb.cpu.flag_n());
            assert!(gb.cpu.flag_h());
        }

        #[test]
        fn scf_and_ccf_manage_carry() {
            let mut gb = gb_with(&[0x37, 0x3F]); // SCF, CCF
            gb.cpu.set_flag_c(false);
            gb.step();
            assert!(gb.cpu.flag_c());
            gb.step();
            assert!(!gb.cpu.flag_c());
            assert!(!gb.cpu.flag_n());
            assert!(!gb.cpu.flag_h());
        }

        #[test]
        fn di_ei_toggle_ime() {
            let mut gb = gb_with(&[0xFB, 0xF3]); // EI, DI
            assert!(!gb.cpu.ime);
            gb.step();
            assert!(gb.cpu.ime);
            gb.step();
            assert!(!gb.cpu.ime);
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
