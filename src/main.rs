mod chip8;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    let rom = "../chip8/ibm_logo.ch8";
    let _ = chip8.load_rom(rom).expect("Failed to read rom file");
}
