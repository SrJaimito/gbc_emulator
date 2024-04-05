use std::env;
use std::process;
use std::fs;

mod gbc;
use gbc::GameBoyColor;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        process::exit(1);
    }

    let rom_path = &args[1];
    println!("ROM Info:\n\t- Name: {}", rom_path);

    let rom = fs::read(rom_path).unwrap();
    println!("\t- Size: {} kB", rom.len() / 1024);

    let mut gbc = GameBoyColor::new();

    gbc.load_rom(rom);
    gbc.run();
}

