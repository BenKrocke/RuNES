mod opcode_tests;

struct Registers {
    a: u8, // Accumulator
    x: u8, // Index Register
    y: u8, // Index Register
    s: u8, // Stack Pointer
    p: u8, // Status Register
    pc: u16, // Program Counter
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            s: 0xfd,
            pc: 0xc000,
            p: 0, //TODO: Needs different value most likely
        }
    }
}
