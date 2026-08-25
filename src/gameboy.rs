use crate::cartridge::Cartridge;
use crate::cpu::ops::op_table;
use crate::cpu::Cpu;
use crate::mmu::Mmu;

/// Root emulator state. All emulation advances through this struct.
pub struct GameBoy {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl GameBoy {
    /// Build a fresh emulator with DMG power-on register values.
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(cartridge),
        }
    }

    /// Fetch the next opcode, execute its handler, and return machine cycles.
    ///
    /// Handlers advance `pc` for any extra operand bytes they need.
    pub fn step(&mut self) -> u32 {
        let opcode = self.mmu.read(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);
        let table = op_table();
        let handler = table[opcode as usize];
        handler(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::Cartridge;

    fn gb_with_rom(ops: &[u8]) -> GameBoy {
        let mut rom = vec![0u8; 0x8000];
        let start = 0x0100;
        for (i, b) in ops.iter().enumerate() {
            rom[start + i] = *b;
        }
        GameBoy::new(Cartridge::new(rom))
    }

    #[test]
    fn step_fetches_and_advances_pc() {
        // 0x00 is NOP; handler returns 1 m-cycle.
        let mut gb = gb_with_rom(&[0x00]);
        let cycles = gb.step();
        assert_eq!(cycles, 1);
        assert_eq!(gb.cpu.pc, 0x101);
    }
}
