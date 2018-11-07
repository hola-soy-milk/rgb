#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref mmu: Mutex<MMU> = {
        let mut m = MMU {
            inbios: 0,
            ie: 0,
            ief: 0,
            romoffset: 0,
            ramoffset: 0,
            bios: Vec::new(),
            eram: Vec::new(),
            wram: Vec::new(),
            zram: Vec::new(),
            rom: "".to_string(),
        };
        Mutex::new(m)
    };

    static ref timer: Mutex<Timer> = {
        let mut m = Timer {
            div: 0,
            sdiv: 0,
            tma: 0,
            tima: 0,
            tac: 0,
        };
        Mutex::new(m)
    };

    static ref timer_clock: Mutex<TimerClock> = {
        let mut m = TimerClock {
            main: 0,
            sub: 0,
            div: 0,
        };
        Mutex::new(m)
    };

    static ref gpu: Mutex<GPU> = {
        let mut m = GPU {
            vram: Vec::new(),
            oam: Vec::new(),
        };
        Mutex::new(m)
    };

    static ref clock: Mutex<Clock> = {
        let mut m = Clock {
            m: 0,
            t: 0,
        };
        Mutex::new(m)
    };

    static ref registers: Mutex<Registers>  = {
        let m = Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
            m: 0,
            t: 0,
        };
        Mutex::new(m)
    };

    static ref cpu: Mutex<CPU>  = {
        let mut m = CPU {
        };
        Mutex::new(m)
    };

}
struct Clock {
    m: i32,
    t: i32,
}

struct TimerClock {
    main: i32,
    sub: i32,
    div: i32
}

struct Timer {
  div: i32,
  sdiv: i32,
  tma: i32,
  tima: i32,
  tac: i32,
}

impl Timer {

    fn reset(&mut self) {
        self.div = 0;
        self.sdiv = 0;
        self.tma = 0;
        self.tima = 0;
        self.tac = 0;
        timer_clock.lock().unwrap().main = 0;
        timer_clock.lock().unwrap().sub = 0;
        timer_clock.lock().unwrap().div = 0;
    }

    fn step(&mut self) {
        self.tima += 1;
        timer_clock.lock().unwrap().main = 0;
        if(self.tima > 255)
        {
            self.tima = self.tma;
            mmu.lock().unwrap().ief |= 4;
        }
    }

  //inc: function() {
    //var oldclk = TIMER._clock.lock().unwrap().main;

    //TIMER._clock.lock().unwrap().sub += Z80._r.m;
    //if(TIMER._clock.lock().unwrap().sub > 3)
    //{
      //TIMER._clock.lock().unwrap().main++;
      //TIMER._clock.lock().unwrap().sub -= 4;

      //TIMER._clock.lock().unwrap().div++;
      //if(TIMER._clock.lock().unwrap().div==16)
      //{
        //TIMER._clock.lock().unwrap().div = 0;
	//TIMER._div++;
	//TIMER._div &= 255;
      //}
    //}

    //if(TIMER._tac & 4)
    //{
      //switch(TIMER._tac & 3)
      //{
        //case 0:
	  //if(TIMER._clock.lock().unwrap().main >= 64) TIMER.step();
	  //break;
	//case 1:
	  //if(TIMER._clock.lock().unwrap().main >=  1) TIMER.step();
	  //break;
	//case 2:
	  //if(TIMER._clock.lock().unwrap().main >=  4) TIMER.step();
	  //break;
	//case 3:
	  //if(TIMER._clock.lock().unwrap().main >= 16) TIMER.step();
	  //break;
      //}
    //}
  //},

    fn rb(&mut self, addr:i32) -> i32 {
        match(addr&0xF000) {
            0xFF04 => {
                self.div
            },
            0xFF05 => {
                self.tima
            },
            0xFF06 => {
                self.tma
            },
            0xFF07 => {
                self.tma
            },
            _ => {
                self.tma
            }
        }
    }

  //wb: function(addr, val) {
    //switch(addr)
    //{
      //case 0xFF04: TIMER._div = 0; break;
      //case 0xFF05: TIMER._tima = val; break;
      //case 0xFF06: TIMER._tma = val; break;
      //case 0xFF07: TIMER._tac = val&7; break;
    //}
  //}
}

struct GPU {
    vram: Vec<i32>,
    oam: Vec<i32>,
}

impl GPU {
    fn rb(&mut self, addr:i32) -> i32 {
        0
    }
}

struct MMU {
    inbios: i32,
    ie: i32,
    ief: i32,
    romoffset: i32,
    ramoffset: i32,
    bios: Vec<i32>,
    eram: Vec<i32>,
    wram: Vec<i32>,
    zram: Vec<i32>,
    rom: String,
}

impl MMU {
    fn rw(&mut self, i:i32) -> i32 {
        1
    }
    fn wb(&mut self, u:i32, d:i32) {
        println!("Running comparison of B and A");
    }
    fn ww(&mut self, u:i32, d:i32) {
        println!("Running comparison of B and A");
    }
    fn rb(&mut self, addr:i32) -> i32 {
        match(addr&0xF000) {
            // ROM bank 0
            0x0000 => {
                if(self.inbios > 0)
                {
                    if(addr<0x0100) { self.bios[addr as usize] }
                    //else if(Z80._r.pc == 0x0100)
                    //{
                        //self.inbios = 0;
                        //0
                    //}
                    else {
                        0
                    }
                }
                else
                {
                    self.rom.as_bytes()[addr as usize] as i32
                }
            },
            0x1000 | 0x2000 | 0x3000 => {
                self.rom.as_bytes()[addr as usize] as i32
            },

            // ROM bank 1
            0x4000 | 0x5000 | 0x6000 | 0x7000 => {
                self.rom.as_bytes()[(self.romoffset+(addr&0x3FFF)) as usize] as i32
            },

            // VRAM
            0x8000 | 0x9000 => {
                gpu.lock().unwrap().vram[( addr&0x1FFF ) as usize]
            },

            // External RAM
            0xA000 | 0xB000 => {
            self.eram[(self.ramoffset+(addr&0x1FFF)) as usize]
            },
            // Work RAM and echo
            0xC000 | 0xD000 | 0xE000 => {
            self.wram[( addr&0x1FFF ) as usize]
            },

            // Everything else
            0xF000 => {
                match(addr&0x0F00) {
                    // Echo RAM
                    0x000 | 0x100 | 0x200 | 0x300 | 0x400 | 0x500 | 0x600 | 0x700 | 0x800 | 0x900 | 0xA00 | 0xB00 | 0xC00 | 0xD00 => {
                        self.wram[( addr&0x1FFF ) as usize]
                    },
                    // OAM
                    0xE00 => {
                        if (addr&0xFF)<0xA0 { gpu.lock().unwrap().oam[( addr&0xFF ) as usize]} else {0}
                    },
                    // Zeropage RAM, I/O, interrupts
                    0xF00 => {
                        if(addr == 0xFFFF) { self.ie }
                        else if(addr > 0xFF7F) { self.zram[( addr&0x7F ) as usize] }
                        else {
                            match(addr&0xF0)
                            {
                                0x00 => {
                                    match(addr&0xF)
                                    {
                                        0 => { 0 },//KEY.rb(); },    // JOYP
                                        4 | 5 | 6 | 7 => { timer.lock().unwrap().rb(addr) },
                                        15 => { self.ief }    // Interrupt flags
                                        _ => { 0 }
                                    }
                                },
                                0x10 | 0x20 | 0x30 => {
                                    0
                                },
                                0x40 | 0x50 | 0x60 | 0x70 => {
                                    gpu.lock().unwrap().rb(addr)
                                },
                                _ => {0}
                            }
                        }
                    },
                    _ => {0}
                }
            },
            _ => {0}
        }
    }
}

struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32,
    h: i32,
    l: i32,
    f: i32,
    pc: i32,
    sp: i32,
    m: i32,
    t: i32,
}

struct CPU {
}

impl CPU {
    fn fzz(&mut self, i: i32) { registers.lock().unwrap().f=0; if(i == 0) { registers.lock().unwrap().f|=128;} registers.lock().unwrap().f|=0; }
    fn fz(&mut self, i: i32, aes: i32) { registers.lock().unwrap().f=0; if(i == 0) { registers.lock().unwrap().f|=128;} registers.lock().unwrap().f|=0x40; }
    fn ld_rr_bb(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_bc(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_bd(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_be(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_bh(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_bl(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ba(&mut self) { registers.lock().unwrap().b=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_cb(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_cc(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_cd(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ce(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ch(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_cl(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ca(&mut self) { registers.lock().unwrap().c=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_db(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_dc(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_dd(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_de(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_dh(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_dl(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_da(&mut self) { registers.lock().unwrap().d=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_eb(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ec(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ed(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ee(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_eh(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_el(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ea(&mut self) { registers.lock().unwrap().e=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_hb(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_hc(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_hd(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_he(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_hh(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_hl(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ha(&mut self) { registers.lock().unwrap().h=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_lb(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_lc(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ld(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_le(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_lh(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ll(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_la(&mut self) { registers.lock().unwrap().l=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ab(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().b; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ac(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().c; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ad(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().d; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ae(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().e; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_ah(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().h; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_al(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().l; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn ld_rr_aa(&mut self) { registers.lock().unwrap().a=registers.lock().unwrap().a; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }

    fn ld_rhlmm_b(&mut self) { registers.lock().unwrap().b=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_c(&mut self) { registers.lock().unwrap().c=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_d(&mut self) { registers.lock().unwrap().d=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_e(&mut self) { registers.lock().unwrap().e=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_h(&mut self) { registers.lock().unwrap().h=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_l(&mut self) { registers.lock().unwrap().l=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rhlmm_a(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_b(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().b); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_c(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().c); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_d(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().d); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_e(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().e); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_h(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().h); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_l(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().l); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlmr_a(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,registers.lock().unwrap().a); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_b(&mut self) { registers.lock().unwrap().b=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_c(&mut self) { registers.lock().unwrap().c=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_d(&mut self) { registers.lock().unwrap().d=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_e(&mut self) { registers.lock().unwrap().e=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_h(&mut self) { registers.lock().unwrap().h=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_l(&mut self) { registers.lock().unwrap().l=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_rn_a(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }

    fn ld_hlmn(&mut self) { let i: i32= mmu.lock().unwrap().rb(registers.lock().unwrap().pc); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l, i); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_bcma(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().b<<8)+registers.lock().unwrap().c, registers.lock().unwrap().a); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_dema(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().d<<8)+registers.lock().unwrap().e, registers.lock().unwrap().a); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_mma(&mut self) { let i: i32 = mmu.lock().unwrap().rw(registers.lock().unwrap().pc); mmu.lock().unwrap().wb(i, registers.lock().unwrap().a); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn ld_abcm(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().b<<8)+registers.lock().unwrap().c); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_adem(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().d<<8)+registers.lock().unwrap().e); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_amm(&mut self) { let i: i32 = mmu.lock().unwrap().rw(registers.lock().unwrap().pc); registers.lock().unwrap().a=mmu.lock().unwrap().rb(i); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn ld_bcnn(&mut self) { registers.lock().unwrap().c=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().b=mmu.lock().unwrap().rb(registers.lock().unwrap().pc+1); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_denn(&mut self) { registers.lock().unwrap().e=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().d=mmu.lock().unwrap().rb(registers.lock().unwrap().pc+1); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_hlnn(&mut self) { registers.lock().unwrap().l=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().h=mmu.lock().unwrap().rb(registers.lock().unwrap().pc+1); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_spnn(&mut self) { registers.lock().unwrap().sp=mmu.lock().unwrap().rw(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=2; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_hlmm(&mut self) { let i: i32=mmu.lock().unwrap().rw(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=2; registers.lock().unwrap().l=mmu.lock().unwrap().rb(i); registers.lock().unwrap().h=mmu.lock().unwrap().rb(i+1); registers.lock().unwrap().m=5; registers.lock().unwrap().t=20; }
    fn ld_mmhl(&mut self) { let i: i32=mmu.lock().unwrap().rw(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=2; mmu.lock().unwrap().ww(i,(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().m=5; registers.lock().unwrap().t=20; }
    fn ld_hlia(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l, registers.lock().unwrap().a); registers.lock().unwrap().l=(registers.lock().unwrap().l+1)&255; if(registers.lock().unwrap().l == 0){ registers.lock().unwrap().h=(registers.lock().unwrap().h+1)&255;} registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_ahli(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().l=(registers.lock().unwrap().l+1)&255; if(registers.lock().unwrap().l == 0){registers.lock().unwrap().h=(registers.lock().unwrap().h+1)&255;} registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlda(&mut self) { mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l, registers.lock().unwrap().a); registers.lock().unwrap().l=(registers.lock().unwrap().l-1)&255; if(registers.lock().unwrap().l==255){registers.lock().unwrap().h=(registers.lock().unwrap().h-1)&255;} registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_ahld(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().l=(registers.lock().unwrap().l-1)&255; if(registers.lock().unwrap().l==255){registers.lock().unwrap().h=(registers.lock().unwrap().h-1)&255;} registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }

    fn ld_aion(&mut self) { let i: i32 = mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().a=mmu.lock().unwrap().rb(0xFF00+i); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_iona(&mut self) { let i: i32 = mmu.lock().unwrap().rb(registers.lock().unwrap().pc); mmu.lock().unwrap().wb(0xFF00+i,registers.lock().unwrap().a); registers.lock().unwrap().pc+=1; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn ld_aioc(&mut self) { registers.lock().unwrap().a=mmu.lock().unwrap().rb(0xFF00+registers.lock().unwrap().c); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_ioca(&mut self) { mmu.lock().unwrap().wb(0xFF00+registers.lock().unwrap().c,registers.lock().unwrap().a); registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn ld_hlspn(&mut self) { let mut i: i32=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); if(i>127){i=-((!i+1)&255);} registers.lock().unwrap().pc+=1; i+=registers.lock().unwrap().sp; registers.lock().unwrap().h=(i>>8)&255; registers.lock().unwrap().l=i&255; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }

    fn swap_r_b(&mut self) { let tr:i32=registers.lock().unwrap().b; registers.lock().unwrap().b=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_c(&mut self) { let tr:i32=registers.lock().unwrap().c; registers.lock().unwrap().c=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_d(&mut self) { let tr:i32=registers.lock().unwrap().d; registers.lock().unwrap().d=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_e(&mut self) { let tr:i32=registers.lock().unwrap().e; registers.lock().unwrap().e=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_h(&mut self) { let tr:i32=registers.lock().unwrap().h; registers.lock().unwrap().h=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_l(&mut self) { let tr:i32=registers.lock().unwrap().l; registers.lock().unwrap().l=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    fn swap_r_a(&mut self) { let tr:i32=registers.lock().unwrap().a; registers.lock().unwrap().a=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); mmu.lock().unwrap().wb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l,tr); registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }
    /*--- Data processing ---*/
    fn addr_b(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().b; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_c(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().c; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_d(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().d; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_e(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().e; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_h(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().h; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_l(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().l; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addr_a(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().a; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    fn addhl(&mut self) { let l:i32 = registers.lock().unwrap().l; registers.lock().unwrap().a+=mmu.lock().unwrap().rb((registers.lock().unwrap().h<<8)+l); let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255){registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn addn(&mut self) { registers.lock().unwrap().a+=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; let i:i32 = registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255){registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    fn addhlbc(&mut self) { let mut hl:i32=(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l; hl+=(registers.lock().unwrap().b<<8)+registers.lock().unwrap().c; if(hl>65535){registers.lock().unwrap().f|=0x10;} else{ registers.lock().unwrap().f&=0xEF;} registers.lock().unwrap().h=(hl>>8)&255; registers.lock().unwrap().l=hl&255; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn addhlde(&mut self) { let mut hl:i32=(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l; hl+=(registers.lock().unwrap().d<<8)+registers.lock().unwrap().e; if(hl>65535) { registers.lock().unwrap().f|=0x10;} else{ registers.lock().unwrap().f&=0xEF;} registers.lock().unwrap().h=(hl>>8)&255; registers.lock().unwrap().l=hl&255; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn addhlhl(&mut self) { let mut hl:i32=(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l; hl+=(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l; if(hl>65535) {registers.lock().unwrap().f|=0x10;} else{ registers.lock().unwrap().f&=0xEF;} registers.lock().unwrap().h=(hl>>8)&255; registers.lock().unwrap().l=hl&255; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn addhlsp(&mut self) { let mut hl:i32=(registers.lock().unwrap().h<<8)+registers.lock().unwrap().l; hl+=registers.lock().unwrap().sp; if(hl>65535){ registers.lock().unwrap().f|=0x10;} else {registers.lock().unwrap().f&=0xEF;} registers.lock().unwrap().h=(hl>>8)&255; registers.lock().unwrap().l=hl&255; registers.lock().unwrap().m=3; registers.lock().unwrap().t=12; }
    fn addspn(&mut self) { let mut i:i32=mmu.lock().unwrap().rb(registers.lock().unwrap().pc); if(i>127){i=-((!i+1)&255);} registers.lock().unwrap().pc+=1; registers.lock().unwrap().sp+=i; registers.lock().unwrap().m=4; registers.lock().unwrap().t=16; }

    //fn adcr_b(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().b; registers.lock().unwrap().a+= if(registers.lock().unwrap().f&0x10 > 0) {1} else{0}; let i:i32= registers.lock().unwrap().a; self.fzz(i); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10;} registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_c(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().c; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_d(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().d; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_e(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().e; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_h(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().h; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_l(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().l; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adcr_a(&mut self) { registers.lock().unwrap().a+=registers.lock().unwrap().a; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0) {1} else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) registers.lock().unwrap().f|=0x10; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=1; registers.lock().unwrap().t=4; }
    //fn adchl(&mut self) { registers.lock().unwrap().a+=MMU.rb((registers.lock().unwrap().h<<8)+registers.lock().unwrap().l); registers.lock().unwrap().a+= if(registers.lock().unwrap().f&0x10 > 0){1}else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10}; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }
    //fn adcn(&mut self) { registers.lock().unwrap().a+=MMU.rb(registers.lock().unwrap().pc); registers.lock().unwrap().pc+=1; registers.lock().unwrap().a+=if(registers.lock().unwrap().f&0x10 > 0){1}else {0}; self.fzz(registers.lock().unwrap().a); if(registers.lock().unwrap().a>255) {registers.lock().unwrap().f|=0x10}; registers.lock().unwrap().a&=255; registers.lock().unwrap().m=2; registers.lock().unwrap().t=8; }

}

fn main() {
    println!("cpu contains clock with {:?} and {:?}", clock.lock().unwrap().m, clock.lock().unwrap().t);
    println!("cpu contains registers.lock().unwrap() with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", registers.lock().unwrap().a, registers.lock().unwrap().b, registers.lock().unwrap().c, registers.lock().unwrap().d, registers.lock().unwrap().e, registers.lock().unwrap().h, registers.lock().unwrap().l, registers.lock().unwrap().f, registers.lock().unwrap().pc, registers.lock().unwrap().sp, registers.lock().unwrap().m, registers.lock().unwrap().t);
    println!("Adding E to A");
    cpu.lock().unwrap().addr_e();
    println!("cpu contains registers.lock().unwrap() with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", registers.lock().unwrap().a, registers.lock().unwrap().b, registers.lock().unwrap().c, registers.lock().unwrap().d, registers.lock().unwrap().e, registers.lock().unwrap().h, registers.lock().unwrap().l, registers.lock().unwrap().f, registers.lock().unwrap().pc, registers.lock().unwrap().sp, registers.lock().unwrap().m, registers.lock().unwrap().t);
    //cpu.cpr_b();
    println!("Running comparison of B and A");
    println!("cpu contains registers.lock().unwrap() with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", registers.lock().unwrap().a, registers.lock().unwrap().b, registers.lock().unwrap().c, registers.lock().unwrap().d, registers.lock().unwrap().e, registers.lock().unwrap().h, registers.lock().unwrap().l, registers.lock().unwrap().f, registers.lock().unwrap().pc, registers.lock().unwrap().sp, registers.lock().unwrap().m, registers.lock().unwrap().t);
}
