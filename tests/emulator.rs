extern crate yarve;

#[cfg(test)]
mod emulator {
    #[test]
    fn register_index() {
        use yarve::emulator::{Registers, Emulator};
        let emu = Emulator::new();
        assert_eq!(Registers::zero(), 0);
        assert_eq!(emu.register_int[Registers::zero()], 0);
        assert_eq!(Registers::t6(), 31);
        assert_eq!(emu.register_int[Registers::t6()], 0);
    }
}
