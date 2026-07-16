use crate::cartridge::Cartridge;

pub struct Mmu {
    cartridge: Cartridge,
    vram: [u8; 0x2000],
    eram: [u8; 0x2000],
    wram: [u8; 0x2000],
    zram: [u8; 0x80],
    ie: u8,
    if_: u8,
}

impl Mmu {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            zram: [0; 0x80],
            ie: 0,
            if_: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF | 0x4000..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize],
            0xC000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize],
            0xFE00..=0xFE9F => 0, // OAM owned by PPU in a later phase.
            0xFEA0..=0xFEFF => 0, // Reserved/unusable.
            0xFF00..=0xFF7F => 0, // I/O registers stubbed; PPU/timer/joypad will own these later.
            0xFF80..=0xFFFE => self.zram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => self.cartridge.write(addr, value),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.eram[(addr - 0xA000) as usize] = value,
            0xC000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize] = value,
            0xFE00..=0xFE9F => { /* OAM writes stubbed. */ }
            0xFEA0..=0xFEFF => { /* Reserved; ignore. */ }
            0xFF00..=0xFF7F => { /* I/O registers stubbed. */ }
            0xFF80..=0xFFFE => self.zram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
        }
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.if_ |= flag;
    }

    pub fn clear_interrupt_flag(&mut self, flag: u8) {
        self.if_ &= !flag;
    }

    pub fn interrupt_enable(&self) -> u8 {
        self.ie
    }

    pub fn interrupt_flags(&self) -> u8 {
        self.if_
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::Cartridge;

    fn empty_mmu() -> Mmu {
        Mmu::new(Cartridge::new(vec![0; 0x8000]))
    }

    #[test]
    fn wram_write_read() {
        let mut mmu = empty_mmu();
        mmu.write(0xC000, 0xAB);
        assert_eq!(mmu.read(0xC000), 0xAB);
    }

    #[test]
    fn echo_ram() {
        let mut mmu = empty_mmu();
        mmu.write(0xE000, 0xCD);
        assert_eq!(mmu.read(0xC000), 0xCD);
        assert_eq!(mmu.read(0xE000), 0xCD);

        mmu.write(0xD000, 0xEF);
        assert_eq!(mmu.read(0xF000), 0xEF);
        assert_eq!(mmu.read(0xD000), 0xEF);
    }

    #[test]
    fn zram_write_read() {
        let mut mmu = empty_mmu();
        mmu.write(0xFF80, 0xEF);
        assert_eq!(mmu.read(0xFF80), 0xEF);
    }

    #[test]
    fn ie_register() {
        let mut mmu = empty_mmu();
        mmu.write(0xFFFF, 0b0001_0101);
        assert_eq!(mmu.read(0xFFFF), 0b0001_0101);
    }

    #[test]
    fn interrupt_flags_helpers() {
        let mut mmu = empty_mmu();
        mmu.set_interrupt_flag(0b0000_0100);
        assert_eq!(mmu.interrupt_flags(), 0b0000_0100);
        mmu.clear_interrupt_flag(0b0000_0100);
        assert_eq!(mmu.interrupt_flags(), 0);
    }

    #[test]
    fn cartridge_read_through_mmu() {
        let mut rom = vec![0u8; 0x8000];
        rom[0x0100] = 0x11;
        rom[0x4100] = 0x22;
        let mmu = Mmu::new(Cartridge::new(rom));
        assert_eq!(mmu.read(0x0100), 0x11);
        assert_eq!(mmu.read(0x4100), 0x22);
    }
}
