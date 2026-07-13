pub mod cartridge;
pub mod cpu;
pub mod gameboy;
pub mod joypad;
pub mod mmu;
pub mod ppu;
pub mod timer;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn harness_smoke() {
        assert_eq!(2 + 2, 4);
    }
}
