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
    opcode: u16,
}

const START_ADDR: usize = 0x200;
const FONT_START_ADDR: usize = 0x50;
const FONT_SIZE: usize = 80;

const FONTS: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
]; // copied fonts at some random site...

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
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

        chip8.memory[FONT_START_ADDR..FONT_START_ADDR+FONT_SIZE].copy_from_slice(&FONTS);
        chip8
    }

    pub fn load_rom(&mut self, rom_path: impl AsRef<Path>) -> io::Result<()> {
        let rom_data = fs::read(rom_path)?;
        let end = START_ADDR + rom_data.len();
        self.memory[START_ADDR..end].copy_from_slice(&rom_data);
        self.pc = START_ADDR as u16;
        Ok(())
    }

    pub fn cycle(&mut self) {
        let pc = self.pc as usize;
        let opcode: u16 = ((self.memory[pc] as u16) << 8) | ((self.memory[pc + 1]) as u16);

        self.pc += 2;

        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F),
        );

        
    }
}
