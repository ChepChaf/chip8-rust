use std::{env, fs};

pub mod chip8;
pub mod display;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let test_rom = fs::read(path).expect("Failed to read test rom.");

    let mut chip8_cpu = chip8::cpu::CPU::new();
    chip8_cpu.load_rom(test_rom);

    let mut display = display::sdl::Display::init(640, 320);

    'gameloop: loop {
        chip8_cpu.step();
        if chip8_cpu.new_draw {
            display.set_draw(chip8_cpu.display());
            chip8_cpu.new_draw = false;
        }

        display.update();

        if display.should_close() {
            break 'gameloop;
        }
    }
}
