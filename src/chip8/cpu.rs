pub struct CPU {
    ram: [u8; 0xFFF],
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    v: [u8; 16],
    i: u16,
    display: [u8; 64 * 32],
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
        }
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
    fn handle8000(&mut self, operation: u16) {
        let sub_operation = operation & 0x000F;
        let x = (operation & 0x0F00 >> 8) as usize;
        let y = (operation & 0x00F0 >> 8) as usize;

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
                    self.v[0xF] = 1;
                    self.v[x] = self.v[x] - self.v[y];
                } else {
                    self.v[0xF] = 0;
                    self.v[x] = self.v[y] - self.v[x];
                }
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
                self.v[x] = self.v[x] * 2;
                self.step_counter();
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
                let x = operation & 0x0F00 >> 8;
                let kk = operation & 0x00FF;

                if self.v[x as usize] == kk as u8 {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x4000 => {
                let x = operation & 0x0F00 >> 8;
                let kk = operation & 0x00FF;

                if self.v[x as usize] != kk as u8 {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x5000 => {
                let x = operation & 0x0F00 >> 8;
                let y = operation & 0x00F0 >> 4;

                if self.v[x as usize] == self.v[y as usize] {
                    self.step_counter()
                }
                self.step_counter()
            }
            0x6000 => {
                let x = usize::from((operation & 0x0F00) >> 8);
                println!("x: {}", x);
                let kk: u8 = (operation & 0x00FF) as u8;

                self.v[x] = kk;
                self.step_counter()
            }
            0x7000 => {
                let x = operation & 0x0F00 >> 8;
                let kk = operation & 0x00FF;

                self.v[x as usize] = self.v[x as usize] + kk as u8;
                self.step_counter();
            }
            0x8000 => {
                self.handle8000(operation);
            }
            0x9000 => {
                let x = operation & 0x0F00 >> 8;
                let y = operation & 0x00F0 >> 4;

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
            0xD000 => {
                let n = operation & 0x000F;
                let x = self.v[((operation & 0x0F00) >> 8) as usize];
                let y = self.v[((operation & 0x00F0) >> 4) as usize];

                for i in 0..n {
                    let mut sprite = self.ram[(self.i + i) as usize];
                    let row = (y as u16 + i) % 32;
                    for j in 0..8 {
                        let b = (sprite & 0x80 >> 7) * 0xFF;
                        let col = (x as u16 + j) % 64;
                        let collision = b & self.display[col as usize];

                        let current_val = self.display[(col + (row * 64)) as usize];
                        self.display[(col + (row * 64)) as usize] = b ^ current_val;
                        sprite = sprite >> 1;

                        if collision > 0 {
                            self.v[0xF] = 1;
                        }
                    }
                }
                self.new_draw = true;

                self.step_counter()
            }
            _ => println!("Unknown op code: {:#04x}", operation),
        }
    }
    pub fn step(&mut self) {
        let operation: u16 = self.get_operation();
        println!("Current op: {:#04x}", operation);

        self.process_operation(operation);
    }
}
