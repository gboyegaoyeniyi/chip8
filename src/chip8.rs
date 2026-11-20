
pub struct Chip8 {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; 64 * 32],
    keypad: [bool; 16],
    opcode: u16
}


const START_ADDR: u16 = 0x200;


impl Chip8 {
    pub fn new() -> Self {
        let chip8 = Chip8 { 
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: START_ADDR,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; 64 * 32],
            keypad: [false; 16],
            opcode: 0,
        };
        
        chip8
    }
}
