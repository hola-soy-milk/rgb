#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CartridgeType {
    RomOnly,
}

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    #[allow(dead_code)]
    kind: CartridgeType,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        // For RomOnly cartridges the RAM size is typically 0.
        Self {
            ram: vec![0; 0],
            rom,
            kind: CartridgeType::RomOnly,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let idx = addr as usize;
                if idx < self.rom.len() {
                    self.rom[idx]
                } else {
                    0
                }
            }
            0x4000..=0x7FFF => {
                // RomOnly mirrors higher banks poorly; treat as flat address.
                let idx = addr as usize;
                if idx < self.rom.len() {
                    self.rom[idx]
                } else {
                    0
                }
            }
            0xA000..=0xBFFF => {
                let idx = (addr - 0xA000) as usize;
                if idx < self.ram.len() {
                    self.ram[idx]
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7FFF => {
                // RomOnly ignores ROM writes.
            }
            0xA000..=0xBFFF => {
                let idx = (addr - 0xA000) as usize;
                if idx < self.ram.len() {
                    self.ram[idx] = value;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rom() -> Vec<u8> {
        let mut rom = vec![0u8; 0x8000];
        rom[0x0000] = 0x11;
        rom[0x3FFF] = 0x22;
        rom[0x4000] = 0x33;
        rom[0x7FFF] = 0x44;
        rom
    }

    #[test]
    fn reads_bank0() {
        let cart = Cartridge::new(sample_rom());
        assert_eq!(cart.read(0x0000), 0x11);
        assert_eq!(cart.read(0x3FFF), 0x22);
    }

    #[test]
    fn reads_bank1() {
        let cart = Cartridge::new(sample_rom());
        assert_eq!(cart.read(0x4000), 0x33);
        assert_eq!(cart.read(0x7FFF), 0x44);
    }

    #[test]
    fn rom_writes_ignored() {
        let mut cart = Cartridge::new(sample_rom());
        cart.write(0x0000, 0xFF);
        assert_eq!(cart.read(0x0000), 0x11);
    }

    #[test]
    fn unmapped_ranges_return_zero() {
        let cart = Cartridge::new(sample_rom());
        assert_eq!(cart.read(0x8000), 0);
        assert_eq!(cart.read(0xC000), 0);
    }
}
