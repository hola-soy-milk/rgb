struct Clock {
    m: i32,
    t: i32,
}

struct MMU {
}

impl MMU {
    fn rb(&mut self, i:i32) -> i32{
        1
    }
    fn rw(&mut self, i:i32) -> i32{
        1
    }
    fn wb(&mut self, u:i32, d:i32){
        println!("Running comparison of B and A");
    }
    fn ww(&mut self, u:i32, d:i32){
        println!("Running comparison of B and A");
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
    clock: Clock,
    registers: Registers,
}

impl CPU {
    fn fzz(&mut self, i: i32) { self.registers.f=0; if(i == 0) { self.registers.f|=128;} self.registers.f|=0; }
    fn fz(&mut self, i: i32, aes: i32) { self.registers.f=0; if(i == 0) { self.registers.f|=128;} self.registers.f|=0x40; }
    fn ld_rr_bb(&mut self) { self.registers.b=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_bc(&mut self) { self.registers.b=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_bd(&mut self) { self.registers.b=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_be(&mut self) { self.registers.b=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_bh(&mut self) { self.registers.b=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_bl(&mut self) { self.registers.b=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ba(&mut self) { self.registers.b=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_cb(&mut self) { self.registers.c=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_cc(&mut self) { self.registers.c=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_cd(&mut self) { self.registers.c=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ce(&mut self) { self.registers.c=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ch(&mut self) { self.registers.c=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_cl(&mut self) { self.registers.c=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ca(&mut self) { self.registers.c=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_db(&mut self) { self.registers.d=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_dc(&mut self) { self.registers.d=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_dd(&mut self) { self.registers.d=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_de(&mut self) { self.registers.d=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_dh(&mut self) { self.registers.d=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_dl(&mut self) { self.registers.d=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_da(&mut self) { self.registers.d=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_eb(&mut self) { self.registers.e=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ec(&mut self) { self.registers.e=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ed(&mut self) { self.registers.e=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ee(&mut self) { self.registers.e=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_eh(&mut self) { self.registers.e=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_el(&mut self) { self.registers.e=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ea(&mut self) { self.registers.e=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_hb(&mut self) { self.registers.h=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_hc(&mut self) { self.registers.h=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_hd(&mut self) { self.registers.h=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_he(&mut self) { self.registers.h=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_hh(&mut self) { self.registers.h=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_hl(&mut self) { self.registers.h=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ha(&mut self) { self.registers.h=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_lb(&mut self) { self.registers.l=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_lc(&mut self) { self.registers.l=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ld(&mut self) { self.registers.l=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_le(&mut self) { self.registers.l=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_lh(&mut self) { self.registers.l=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ll(&mut self) { self.registers.l=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_la(&mut self) { self.registers.l=self.registers.a; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ab(&mut self) { self.registers.a=self.registers.b; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ac(&mut self) { self.registers.a=self.registers.c; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ad(&mut self) { self.registers.a=self.registers.d; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ae(&mut self) { self.registers.a=self.registers.e; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_ah(&mut self) { self.registers.a=self.registers.h; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_al(&mut self) { self.registers.a=self.registers.l; self.registers.m=1; self.registers.t=4; }
    fn ld_rr_aa(&mut self) { self.registers.a=self.registers.a; self.registers.m=1; self.registers.t=4; }

    fn ld_rhlmm_b(&mut self, mmu: &mut MMU) { self.registers.b=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_c(&mut self, mmu: &mut MMU) { self.registers.c=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_d(&mut self, mmu: &mut MMU) { self.registers.d=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_e(&mut self, mmu: &mut MMU) { self.registers.e=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_h(&mut self, mmu: &mut MMU) { self.registers.h=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_l(&mut self, mmu: &mut MMU) { self.registers.l=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_rhlmm_a(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_b(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.b); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_c(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.c); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_d(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.d); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_e(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.e); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_h(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.h); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_l(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.l); self.registers.m=2; self.registers.t=8; }
    fn ld_hlmr_a(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l,self.registers.a); self.registers.m=2; self.registers.t=8; }
    fn ld_rn_b(&mut self, mmu: &mut MMU) { self.registers.b=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_c(&mut self, mmu: &mut MMU) { self.registers.c=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_d(&mut self, mmu: &mut MMU) { self.registers.d=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_e(&mut self, mmu: &mut MMU) { self.registers.e=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_h(&mut self, mmu: &mut MMU) { self.registers.h=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_l(&mut self, mmu: &mut MMU) { self.registers.l=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }
    fn ld_rn_a(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb(self.registers.pc); self.registers.pc+=1; self.registers.m=2; self.registers.t=8; }

    fn ld_hlmn(&mut self, mmu: &mut MMU) { let i: i32= mmu.rb(self.registers.pc); mmu.wb((self.registers.h<<8)+self.registers.l, i); self.registers.pc+=1; self.registers.m=3; self.registers.t=12; }
    fn ld_bcma(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.b<<8)+self.registers.c, self.registers.a); self.registers.m=2; self.registers.t=8; }
    fn ld_dema(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.d<<8)+self.registers.e, self.registers.a); self.registers.m=2; self.registers.t=8; }
    fn ld_mma(&mut self, mmu: &mut MMU) { let i: i32 = mmu.rw(self.registers.pc); mmu.wb(i, self.registers.a); self.registers.pc+=2; self.registers.m=4; self.registers.t=16; }
    fn ld_abcm(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb((self.registers.b<<8)+self.registers.c); self.registers.m=2; self.registers.t=8; }
    fn ld_adem(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb((self.registers.d<<8)+self.registers.e); self.registers.m=2; self.registers.t=8; }
    fn ld_amm(&mut self, mmu: &mut MMU) { let i: i32 = mmu.rw(self.registers.pc); self.registers.a=mmu.rb(i); self.registers.pc+=2; self.registers.m=4; self.registers.t=16; }
    fn ld_bcnn(&mut self, mmu: &mut MMU) { self.registers.c=mmu.rb(self.registers.pc); self.registers.b=mmu.rb(self.registers.pc+1); self.registers.pc+=2; self.registers.m=3; self.registers.t=12; }
    fn ld_denn(&mut self, mmu: &mut MMU) { self.registers.e=mmu.rb(self.registers.pc); self.registers.d=mmu.rb(self.registers.pc+1); self.registers.pc+=2; self.registers.m=3; self.registers.t=12; }
    fn ld_hlnn(&mut self, mmu: &mut MMU) { self.registers.l=mmu.rb(self.registers.pc); self.registers.h=mmu.rb(self.registers.pc+1); self.registers.pc+=2; self.registers.m=3; self.registers.t=12; }
    fn ld_spnn(&mut self, mmu: &mut MMU) { self.registers.sp=mmu.rw(self.registers.pc); self.registers.pc+=2; self.registers.m=3; self.registers.t=12; }
    fn ld_hlmm(&mut self, mmu: &mut MMU) { let i: i32=mmu.rw(self.registers.pc); self.registers.pc+=2; self.registers.l=mmu.rb(i); self.registers.h=mmu.rb(i+1); self.registers.m=5; self.registers.t=20; }
    fn ld_mmhl(&mut self, mmu: &mut MMU) { let i: i32=mmu.rw(self.registers.pc); self.registers.pc+=2; mmu.ww(i,(self.registers.h<<8)+self.registers.l); self.registers.m=5; self.registers.t=20; }
    fn ld_hlia(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l, self.registers.a); self.registers.l=(self.registers.l+1)&255; if(self.registers.l == 0){ self.registers.h=(self.registers.h+1)&255;} self.registers.m=2; self.registers.t=8; }
    fn ld_ahli(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.l=(self.registers.l+1)&255; if(self.registers.l == 0){self.registers.h=(self.registers.h+1)&255;} self.registers.m=2; self.registers.t=8; }
    fn ld_hlda(&mut self, mmu: &mut MMU) { mmu.wb((self.registers.h<<8)+self.registers.l, self.registers.a); self.registers.l=(self.registers.l-1)&255; if(self.registers.l==255){self.registers.h=(self.registers.h-1)&255;} self.registers.m=2; self.registers.t=8; }
    fn ld_ahld(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb((self.registers.h<<8)+self.registers.l); self.registers.l=(self.registers.l-1)&255; if(self.registers.l==255){self.registers.h=(self.registers.h-1)&255;} self.registers.m=2; self.registers.t=8; }

    fn ld_aion(&mut self, mmu: &mut MMU) { let i: i32 = mmu.rb(self.registers.pc); self.registers.a=mmu.rb(0xFF00+i); self.registers.pc+=1; self.registers.m=3; self.registers.t=12; }
    fn ld_iona(&mut self, mmu: &mut MMU) { let i: i32 = mmu.rb(self.registers.pc); mmu.wb(0xFF00+i,self.registers.a); self.registers.pc+=1; self.registers.m=3; self.registers.t=12; }
    fn ld_aioc(&mut self, mmu: &mut MMU) { self.registers.a=mmu.rb(0xFF00+self.registers.c); self.registers.m=2; self.registers.t=8; }
    fn ld_ioca(&mut self, mmu: &mut MMU) { mmu.wb(0xFF00+self.registers.c,self.registers.a); self.registers.m=2; self.registers.t=8; }
    fn ld_hlspn(&mut self, mmu: &mut MMU) { let mut i: i32=mmu.rb(self.registers.pc); if(i>127){i=-((!i+1)&255);} self.registers.pc+=1; i+=self.registers.sp; self.registers.h=(i>>8)&255; self.registers.l=i&255; self.registers.m=3; self.registers.t=12; }

    fn swap_r_b(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.b; self.registers.b=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_c(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.c; self.registers.c=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_d(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.d; self.registers.d=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_e(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.e; self.registers.e=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_h(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.h; self.registers.h=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_l(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.l; self.registers.l=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    fn swap_r_a(&mut self, mmu: &mut MMU) { let tr:i32=self.registers.a; self.registers.a=mmu.rb((self.registers.h<<8)+self.registers.l); mmu.wb((self.registers.h<<8)+self.registers.l,tr); self.registers.m=4; self.registers.t=16; }
    /*--- Data processing ---*/
    fn addr_b(&mut self) { self.registers.a+=self.registers.b; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_c(&mut self) { self.registers.a+=self.registers.c; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_d(&mut self) { self.registers.a+=self.registers.d; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_e(&mut self) { self.registers.a+=self.registers.e; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_h(&mut self) { self.registers.a+=self.registers.h; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_l(&mut self) { self.registers.a+=self.registers.l; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }
    fn addr_a(&mut self) { self.registers.a+=self.registers.a; let i:i32 = self.registers.a; self.fzz(i); if(self.registers.a>255) {self.registers.f|=0x10;} self.registers.a&=255; self.registers.m=1; self.registers.t=4; }

    // Compare B to A, setting flags (CP A, B)
    fn cpr_b(&mut self) {
        // Temporary copy of A
        let mut i : i32;
        i = self.registers.a;
        // Subtract B
        i -= self.registers.b;
        // Set subtraction flag
        self.registers.f |= 0x40;
        // Check for 0
        if (i & 255) == 0 {
            self.registers.f |= 0x80;
        }
        // Check for underflow
        if i < 0 {
            self.registers.f |= 0x10;
        }
        // 1 M-time taken
        self.registers.m = 1;
        self.registers.t = 4;
    }

    fn nop(&mut self) {
        // 1 M-time taken
        self.registers.m = 1;
        self.registers.t = 4;
    }

}

fn main() {
    let clock = Clock {
        m: 0,
        t: 0,
    };

    let registers = Registers {
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

    let mut cpu = CPU {
        clock: clock,
        registers: registers,
    };



    println!("cpu contains clock with {:?} and {:?}", cpu.clock.m, cpu.clock.t);
    println!("cpu contains registers with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.f, cpu.registers.pc, cpu.registers.sp, cpu.registers.m, cpu.registers.t);
    println!("Adding E to A");
    cpu.addr_e();
    println!("cpu contains registers with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.f, cpu.registers.pc, cpu.registers.sp, cpu.registers.m, cpu.registers.t);
    cpu.cpr_b();
    println!("Running comparison of B and A");
    println!("cpu contains registers with a:{:?}, b:{:?}, c:{:?}, d:{:?}, e:{:?}, h:{:?}, l:{:?}, f:{:?}, pc:{:?}, sp:{:?}, m:{:?} and t:{:?}", cpu.registers.a, cpu.registers.b, cpu.registers.c, cpu.registers.d, cpu.registers.e, cpu.registers.h, cpu.registers.l, cpu.registers.f, cpu.registers.pc, cpu.registers.sp, cpu.registers.m, cpu.registers.t);
}
