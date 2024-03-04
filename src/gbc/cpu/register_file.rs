pub mod {

    enum SingleRegister {
        A, F,
        B, C,
        D, E,
        H, L
    }

    enum DoubleRegister {
        AF,
        BF,
        DE,
        HL
    }

    struct RegisterFile {
        a: u8, f: u8,
        b: u8, c: u8,
        d: u8, e: u8,
        h: u8, l: u8
    }

    impl RegisterFile {

        fn new() {
            RegisterFile {
                a: 0, f: 0,
                b: 0, c: 0,
                d: 0, e: 0,
                h: 0, l: 0
            }
        }

        fn setSingleRegister(&self, reg: SingleRegister, value: u8) {
            match reg {
                SingleRegister::A => self.a = value,
                SingleRegister::F => self.f = value,
                SingleRegister::B => self.b = value,
                SingleRegister::C => self.c = value,
                SingleRegister::D => self.d = value,
                SingleRegister::E => self.e = value,
                SingleRegister::H => self.h = value,
                SingleRegister::L => self.l = value,
            }
        }

        fn setDoubleRegister(&self, reg: DoubleRegister, value: u16) {
            let lsb: u8 =  value & 0xFF;
            let msb: u8 = (value >> 8) & 0xFF;

            match reg {
                DoubleRegister::AF => {
                    self.a = msb;
                    self.f = lsb;
                },
                DoubleRegister::BC => {
                    self.b = msb;
                    self.c = lsb;
                },
                DoubleRegister::DE => {
                    self.d = msb;
                    self.e = lsb;
                },
                DoubleRegister::HL => {
                    self.h = msb;
                    self.l = lsb;
                },
            }
        }

    }

}

