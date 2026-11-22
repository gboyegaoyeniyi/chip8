use std::io;

mod chip8;

fn main() -> io::Result<()> {
    let mut chip8 = chip8::Chip8::new();
    let rom = "../chip8/ibm_logo.ch8";
    chip8.load_rom(rom).expect("Failed to read rom file");


    for _ in 0..25 {
        chip8.cycle();
    }

    for y in 0..32 {
        for x in 0..64 {
            let pixel = chip8.display[y * 64 + x];
            print!("{}", if pixel {"##"} else {"  "});
        }
        println!();
    }

    Ok(())
}
