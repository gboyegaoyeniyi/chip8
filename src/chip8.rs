use std::{fs, io, path::Path};

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


const START_ADDR: usize = 0x200;


impl Chip8 {
    pub fn new() -> Self {
        let chip8 = Chip8 { 
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: START_ADDR as u16,
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


    pub fn load_rom(&mut self, rom_path: impl AsRef<Path>) -> io::Result<()> {
        let rom_data = fs::read(rom_path)?;
        let end = START_ADDR + rom_data.len();
        self.memory[START_ADDR..end].copy_from_slice(&rom_data);
        Ok(())
    }
}
