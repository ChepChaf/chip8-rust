use std::{
    thread,
    time::{Duration, Instant},
};

use rand::Rng;

pub struct CPU {
    ram: [u8; 0xFFF],
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    v: [u8; 16],
    i: u16,
    display: [u8; 64 * 32],
    dt: u8,
    last_timer_start: Instant,
    input_reg: usize,
    keyboard: [bool; 16],
    pub waiting_input: bool,
    pub new_draw: bool, // To tell our Renderer that there is a new frame
}

impl CPU {
    pub fn new() -> CPU {
        Self {
            ram: [0; 0xFFF],
            program_counter: 0x200,
            v: [0; 16],
            i: 0,
            display: [0; 64 * 32],
            new_draw: true,
            stack: [0; 16],
            stack_pointer: 0,
            dt: 0,
            last_timer_start: Instant::now(),
            waiting_input: false,
            input_reg: 0,
            keyboard: [false; 16],
        }
    }
    pub fn input(&mut self, input: u8) {
        // TODO: move to keyboard struct
        if input >= 0x10 {
            return;
        }

        if self.waiting_input {
            self.waiting_input = false;
            self.v[self.input_reg] = input;
            self.step_counter();
        }

        self.keyboard[input as usize] = true;
    }
    pub fn clear_input(&mut self, input: u8) {
        if input >= 0x10 {
            return;
        }

        self.keyboard[self.input_reg] = false;
    }

    fn clear_display(&mut self) {
        self.display = [0; 64 * 32];
        self.new_draw = true;
    }
    fn load_fonts(&mut self) {
        // 0
        self.ram[0x000] = 0xF0; // 11110000
        self.ram[0x002] = 0x90; // 10010000
        self.ram[0x004] = 0x90; // 10010000
        self.ram[0x006] = 0x90; // 10010000
        self.ram[0x008] = 0xF0; // 11110000
                                // 1
        self.ram[0x00A] = 0x20; // 00100000
        self.ram[0x00C] = 0x60; // 01100000
        self.ram[0x00E] = 0x20; // 00100000
        self.ram[0x010] = 0x20; // 00100000
        self.ram[0x012] = 0x70; // 01110000
                                // 2
        self.ram[0x014] = 0xF0; // 11110000
        self.ram[0x016] = 0x10; // 00010000
        self.ram[0x018] = 0xF0; // 11110000
        self.ram[0x01A] = 0x80; // 10000000
        self.ram[0x01C] = 0xF0; // 11110000
                                // 3
        self.ram[0x01E] = 0xF0; // 11110000
        self.ram[0x020] = 0x10; // 00010000
        self.ram[0x022] = 0xF0; // 11110000
        self.ram[0x024] = 0x10; // 00010000
        self.ram[0x026] = 0xF0; // 11110000
                                // 4
        self.ram[0x028] = 0x90; // 10010000
        self.ram[0x02A] = 0x90; // 10010000
        self.ram[0x02C] = 0xF0; // 11110000
        self.ram[0x02E] = 0x10; // 00010000
        self.ram[0x030] = 0x10; // 00010000
                                // 5
        self.ram[0x032] = 0xF0; // 11110000
        self.ram[0x034] = 0x80; // 10000000
        self.ram[0x036] = 0xF0; // 11110000
        self.ram[0x038] = 0x10; // 00010000
        self.ram[0x03A] = 0xF0; // 11110000
                                // 6
        self.ram[0x03C] = 0xF0; // 11110000
        self.ram[0x03E] = 0x80; // 10000000
        self.ram[0x040] = 0xF0; // 11110000
        self.ram[0x042] = 0x90; // 10010000
        self.ram[0x044] = 0xF0; // 11110000
                                // 7
        self.ram[0x046] = 0xF0; // 11110000
        self.ram[0x048] = 0x10; // 00010000
        self.ram[0x04A] = 0x20; // 00100000
        self.ram[0x04C] = 0x40; // 01000000
        self.ram[0x04E] = 0x40; // 01000000
                                // 8
        self.ram[0x050] = 0xF0; // 11110000
        self.ram[0x052] = 0x90; // 10010000
        self.ram[0x054] = 0xF0; // 11110000
        self.ram[0x056] = 0x90; // 10010000
        self.ram[0x058] = 0xF0; // 11110000
                                // 9
        self.ram[0x05A] = 0xF0; // 11110000
        self.ram[0x05C] = 0x90; // 10010000
        self.ram[0x05E] = 0xF0; // 11110000
        self.ram[0x060] = 0x10; // 00010000
        self.ram[0x062] = 0xF0; // 11110000
                                // A
        self.ram[0x064] = 0xF0; // 11110000
        self.ram[0x066] = 0x90; // 10010000
        self.ram[0x068] = 0xF0; // 11110000
        self.ram[0x06A] = 0x90; // 10010000
        self.ram[0x06C] = 0x90; // 10010000
                                // B
        self.ram[0x06E] = 0xE0; // 11100000
        self.ram[0x070] = 0x90; // 10010000
        self.ram[0x072] = 0xE0; // 11100000
        self.ram[0x074] = 0x90; // 10010000
        self.ram[0x076] = 0xE0; // 11100000
                                // C
        self.ram[0x078] = 0xF0; // 11110000
        self.ram[0x07A] = 0x80; // 10000000
        self.ram[0x07C] = 0x80; // 10000000
        self.ram[0x07E] = 0x80; // 10000000
        self.ram[0x080] = 0xF0; // 11110000
                                // D
        self.ram[0x082] = 0xE0; // 11100000
        self.ram[0x084] = 0x90; // 10010000
        self.ram[0x086] = 0x90; // 10010000
        self.ram[0x088] = 0x90; // 10010000
        self.ram[0x08A] = 0xE0; // 11100000
                                // E
        self.ram[0x08C] = 0xF0; // 11110000
        self.ram[0x08E] = 0x80; // 10000000
        self.ram[0x090] = 0xF0; // 11110000
        self.ram[0x092] = 0x80; // 10000000
        self.ram[0x094] = 0xF0; // 11110000
                                // F
        self.ram[0x096] = 0xF0; // 11110000
        self.ram[0x098] = 0x80; // 10000000
        self.ram[0x09A] = 0xF0; // 11110000
        self.ram[0x09C] = 0x80; // 10000000
        self.ram[0x09E] = 0x80; // 10000000
    }
    pub fn reset(&mut self) {
        self.ram = [0; 0xFFF];
        self.program_counter = 0x200;
        self.v = [0; 16];
        self.i = 0;
        self.clear_display();

        self.load_fonts();

        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.last_timer_start = Instant::now();
        self.keyboard = [false; 16];
    }
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset();
        for (i, &val) in rom.iter().enumerate() {
            self.ram[0x200 + i] = val
        }
    }
    pub fn ram(&self) -> [u8; 0xFFF] {
        return self.ram;
    }
    pub fn display(&mut self) -> &[u8; 64 * 32] {
        return &mut self.display;
    }
    fn get_operation(&self) -> u16 {
        let index = usize::from(self.program_counter);
        let high_bytes = u16::from(self.ram[index]);
        let low_bytes = u16::from(self.ram[index + 1]);

        (high_bytes << 8) | low_bytes
    }
    fn step_counter(&mut self) {
        self.program_counter = self.program_counter + 2;
    }
    fn update_timer(&mut self) {
        if self.dt > 0 {
            println!("DT: {}", self.dt);
            let duration = self.last_timer_start.elapsed();

            // 60hz
            if duration.as_millis() >= (1000 / 60) {
                self.dt = self.dt - 1;
                self.last_timer_start = Instant::now();
                println!("DT: {}", self.dt);
            }
        }
    }
    fn handle_8_ops(&mut self, operation: u16) {
        let sub_operation = operation & 0x000F;
        let x = ((operation & 0x0F00) >> 8) as usize;
        let y = ((operation & 0x00F0) >> 4) as usize;

        match sub_operation {
            0 => {
                self.v[x] = self.v[y];
                self.step_counter();
            }
            1 => {
                self.v[x] = self.v[x] | self.v[y];
                self.step_counter();
            }
            2 => {
                self.v[x] = self.v[x] & self.v[y];
                self.step_counter();
            }
            3 => {
                self.v[x] = self.v[x] ^ self.v[y];
                self.step_counter();
            }
            4 => {
                let sum: u16 = self.v[x] as u16 + self.v[y] as u16;
                let carry = sum % 0xFF;

                self.v[x] = (sum & 0xFF) as u8;

                if carry > 0 {
                    self.v[0xF] = 1;
                }

                self.step_counter();
            }
            5 => {
                if self.v[x] > self.v[y] {
                    self.v[0xF] = 0;
                } else {
                    self.v[0xF] = 1;
                }
                self.v[x] = (self.v[x] as i16 - self.v[y] as i16) as u8;
                self.step_counter();
            }
            6 => {
                if self.v[x] & 0x01 > 0 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = self.v[x] / 2;
                self.step_counter();
            }
            0xE => {
                if self.v[x] & 0x80 > 0 {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
                self.v[x] = (self.v[x] as u16 * 2) as u8;
                self.step_counter();
            }
            _ => println!("Unknown op code: {:#04x}", operation),
        }
    }
    fn handle_f_ops(&mut self, operation: u16) {
        let sub_operation = operation & 0x00FF;
        let x = (operation & 0x0F00) >> 8;

        match sub_operation {
            0x07 => {
                self.v[x as usize] = self.dt;
                self.step_counter();
            }
            0x15 => {
                self.dt = self.v[x as usize];
                self.last_timer_start = Instant::now();
                self.step_counter();
            }
            0x33 => {
                let address = self.i as usize;
                let n = self.v[x as usize];

                self.ram[address] = n / 100; // Hundreds digits
                self.ram[address + 2] = (n % 100) / 10; // Tens digits
                self.ram[address + 4] = n % 10; // Ones digits

                self.step_counter();
            }
            0x55 => {
                for i in 0..(x + 1) {
                    let index = (self.i + i) as usize;
                    self.ram[index] = self.v[i as usize];
                }
                self.step_counter();
            }
            0x65 => {
                let address = self.i as usize;

                for i in 0..(x + 1) {
                    let index = address + (2 * i) as usize;
                    self.v[i as usize] = self.ram[index];
                }

                self.step_counter();
            }
            0x1E => {
                self.i = self.i + x;
                self.step_counter();
            }
            0x0A => {
                self.input_reg = x as usize;
                self.waiting_input = true;
            }
            _ => println!("Unknown op code: {:#04x}", operation),
        }
    }
    fn handle_d_ops(&mut self, operation: u16) {
        let n = operation & 0x000F;
        let x = self.v[((operation & 0x0F00) >> 8) as usize];
        let y = self.v[((operation & 0x00F0) >> 4) as usize];
        self.v[0xF] = 0x0;

        for i in 0..n {
            let mut sprite = self.ram[(self.i + i) as usize];
            let row = (y as u16 + i) % 32;
            for j in 0..8 {
                let b = ((sprite & 0x80) >> 7) * 0xFF;
                let col = (x as u16 + j) % 64;
                let index = (col + (row * 64)) as usize;

                let collision = b & self.display[index];

                self.display[index] = b ^ self.display[index];
                sprite = sprite << 1;

                if collision > 0 {
                    self.v[0xF] = 1;
                }
            }
        }
        self.new_draw = true;

        self.step_counter();
    }
    fn handle_e_ops(&mut self, operation: u16) {
        let sub_operation = operation & 0x00FF;
        let x = (operation & 0x0F00) >> 8;
        match sub_operation {
            0xA1 => {
                if !self.keyboard[self.v[x as usize] as usize] {
                    self.step_counter();
                    self.step_counter();
                }
            }
            0x9E => {
                println!("x: {:#02x}", x);
                println!("Vx: {:#02x}", self.v[x as usize]);
                println!("Keyboard: {}", self.keyboard[self.v[x as usize] as usize]);
                if self.keyboard[self.v[x as usize] as usize] {
                    self.step_counter();
                    self.step_counter();
                }
            }
            _ => println!("Unknown op code: {:#04x}", operation),
        }
    }
    fn process_operation(&mut self, operation: u16) {
        let high = operation & 0xF000;
        match high {
            0x0000 => {
                let sub_operation = operation & 0x0FFF;

                match sub_operation {
                    0x0E0 => {
                        println!("Clearing display");
                        self.clear_display();
                        self.step_counter();
                    }
                    0x0EE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        self.stack_pointer = self.stack_pointer - 1;

                        self.step_counter();
                    }
                    _ => println!("Unknown op code: {:#04x}", operation),
                }
            }
            0x1000 => {
                let address: u16 = operation & 0x0FFF;
                self.program_counter = address;
            }
            0x2000 => {
                let nnn = operation & 0x0FFF;

                self.stack_pointer = self.stack_pointer + 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;

                self.program_counter = nnn;
            }
            0x3000 => {
                let x = (operation & 0x0F00) >> 8;
                let kk = operation & 0x00FF;

                if self.v[x as usize] == kk as u8 {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x4000 => {
                let x = (operation & 0x0F00) >> 8;
                let kk = operation & 0x00FF;

                if self.v[x as usize] != kk as u8 {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x5000 => {
                let x = (operation & 0x0F00) >> 8;
                let y = (operation & 0x00F0) >> 4;

                if self.v[x as usize] == self.v[y as usize] {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x6000 => {
                let x = usize::from((operation & 0x0F00) >> 8);
                let kk: u8 = (operation & 0x00FF) as u8;

                self.v[x] = kk;
                self.step_counter()
            }
            0x7000 => {
                let x = ((operation & 0x0F00) >> 8) as usize;
                let kk = operation & 0x00FF;

                self.v[x] = ((self.v[x] as u16) + kk) as u8;
                self.step_counter();
            }
            0x8000 => {
                self.handle_8_ops(operation);
            }
            0x9000 => {
                let x = (operation & 0x0F00) >> 8;
                let y = (operation & 0x00F0) >> 4;

                if self.v[x as usize] != self.v[y as usize] {
                    self.step_counter();
                }
                self.step_counter();
            }
            0xA000 => {
                let nnn = operation & 0x0FFF;
                self.i = nnn;
                self.step_counter()
            }
            0xC000 => {
                let x = (operation & 0x0F00) >> 8;
                let kk = (operation & 0x00FF) as u8;

                let mut rng = rand::thread_rng();
                let r: u8 = rng.gen();

                self.v[x as usize] = r & kk;
                self.step_counter();
            }
            0xD000 => {
                self.handle_d_ops(operation);
            }
            0xE000 => {
                self.handle_e_ops(operation);
            }
            0xF000 => {
                self.handle_f_ops(operation);
            }
            _ => println!("Unknown op code: {:#04x}", operation),
        }
    }
    pub fn step(&mut self) {
        if self.waiting_input {
            return;
        }

        self.update_timer();

        let operation: u16 = self.get_operation();
        println!("Current op: {:#04x}", operation);

        self.process_operation(operation);
    }
}
