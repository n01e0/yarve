#[derive(Debug)]
pub struct Emulator {
    pub register_int: [u64; 32],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            register_int: [0; 32]
        }
    }
}

#[derive(Debug)]
pub enum Registers {}

#[allow(unused)]
impl Registers {
    /// Hard-wired zero
    pub fn zero() -> usize { 0 }
    /// Return address
    pub fn ra() -> usize { 1 }
    /// Stack pointer
    pub fn sp() -> usize { 2 }
    /// Global pointer
    pub fn gp() -> usize { 3 }
    /// Thread pointer
    pub fn tp() -> usize { 4 }
    /// Temporary/alternate link register
    pub fn t0() -> usize { 5 }
    /// Temporaries
    pub fn t1() -> usize { 6 }
    /// Temporaries
    pub fn t2() -> usize { 7 }
    /// Saved register
    pub fn s0() -> usize { 8 }
    /// Frame pointer
    pub fn fp() -> usize { 8 }
    /// Saved register
    pub fn s1() -> usize { 9 }
    /// Function arguments / return values 
    pub fn a0() -> usize { 10 }
    /// Function arguments / return values
    pub fn a1() -> usize { 11 }
    /// Function arguments
    pub fn a2() -> usize { 12 }
    /// Function arguments
    pub fn a3() -> usize { 13 }
    /// Function arguments
    pub fn a4() -> usize { 14 }
    /// Function arguments
    pub fn a5() -> usize { 15 }
    /// Function arguments
    pub fn a6() -> usize { 16 }
    /// Function arguments
    pub fn a7() -> usize { 17 }
    /// Saved registers
    pub fn s2() -> usize { 18 }
    /// Saved registers
    pub fn s3() -> usize { 19 }
    /// Saved registers
    pub fn s4() -> usize { 20 }
    /// Saved registers
    pub fn s5() -> usize { 21 }
    /// Saved registers
    pub fn s6() -> usize { 22 }
    /// Saved registers
    pub fn s7() -> usize { 23 }
    /// Saved registers
    pub fn s8() -> usize { 24 }
    /// Saved registers
    pub fn s9() -> usize { 25 }
    /// Saved registers
    pub fn s10() -> usize { 26 }
    /// Saved registers
    pub fn s11() -> usize { 27 }
    /// Temporaries
    pub fn t3() -> usize { 28 }
    /// Temporaries
    pub fn t4() -> usize { 29 }
    /// Temporaries
    pub fn t5() -> usize { 30 }
    /// Temporaries
    pub fn t6() -> usize { 31 }
}
