extern crate rand;

use rand::Rng;

use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

const FONT_SIZE: usize = 80;

const FONT_SET: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0x10, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    memory: [u8; 4096],
    video: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
    registers: [u8; 16],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    keyboard: [bool; 16],
}

impl Chip8 {
    pub fn init() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            video: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            registers: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            keyboard: [false; 16],
        };

        for i in 0..FONT_SIZE {
            chip8.memory[i] = FONT_SET[i];
        }

        chip8
    }

    pub fn key_down(&mut self, key: usize) {
        if key < 16 {
            self.keyboard[key] = true;
        }
    }

    pub fn key_up(&mut self, key: usize) {
        if key < 16 {
            self.keyboard[key] = false;
        }
    }

    pub fn get_video_memory(&self) -> &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] {
        &self.video
    }

    pub fn load_rom(&mut self, buf: &Vec<u8>) {
        let start: usize = 0x200; // May be better as a const value for starting address.
        let end: usize = start + buf.len();
        self.memory[start..end].copy_from_slice(buf);
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        self.execute(opcode);
    }

    fn fetch(&mut self) -> u16 {
        let high = self.memory[self.pc as usize] as u16;
        let low = self.memory[(self.pc + 1) as usize] as u16;
        let op = (high << 8) | low;

        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let nibbles = (
            (op & 0xF000) >> 12,
            (op & 0x0F00) >> 8,
            (op & 0x00F0) >> 4,
            (op & 0x000F),
        );

        let nnn = op & 0x0FFF;
        let nn = (op & 0x00FF) as u8;
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3;

        match nibbles {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                self.video = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
            },

            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },

            // JP nnn
            (0x1, _, _, _) => {
                self.pc = nnn;
            },

            // CALL nnn
            (0x2, _, _, _) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;

                self.pc = nnn;
            },

            // SE Vx, nn
            (0x3, _, _, _) => {
                if self.registers[x] == nn {
                    self.pc += 2;
                }
            },

            // SNE Vx, nn
            (0x4, _, _, _) => {
                if self.registers[x] != nn {
                    self.pc += 2;
                }
            },

            // SE Vx, Vy
            (0x5, _, _, 0x0) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            },

            // LD Vx, nn
            (0x6, _, _, _) => {
                self.registers[x] = nn;
            },

            // ADD Vx, nn
            (0x7, _, _, _) => {
                self.registers[x] += nn;
            },

            // LD Vx, Vy
            (0x8, _, _, 0x0) => {
                self.registers[x] = self.registers[y];
            },

            // OR Vx, Vy
            (0x8, _, _, 0x1) => {
                self.registers[x] |= self.registers[y];
            },

            // AND Vx, Vy
            (0x8, _, _, 0x2) => {
                self.registers[x] &= self.registers[y];
            },

            // XOR Vx, Vy
            (0x8, _, _, 0x3) => {
                self.registers[x] ^= self.registers[y];
            },

            // ADD Vx, Vy
            (0x8, _, _, 0x4) => {
                let num = (self.registers[x] + self.registers[y]) as u16;

                self.registers[x] = (num & 0x00FF) as u8;

                if num > 255 {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }
            },

            // SUB Vx, Vy
            (0x8, _, _, 0x5) => {
                if self.registers[x] > self.registers[y] {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }

                self.registers[x] -= self.registers[y];
            },

            // SHR Vx {, Vy}
            (0x8, _, _, 0x6) => {
                let lsb = self.registers[x] & 1;

                self.registers[0xF] = lsb;
                self.registers[x] >>= 1;
            },

            // SUBN Vx, Vy
            (0x8, _, _, 0x7) => {
                if self.registers[y] > self.registers[x] {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }

                self.registers[x] = self.registers[y] - self.registers[x];
            },

            // SHL Vx {, Vy}
            (0x8, _, _, 0xE) => {
                let msb = (self.registers[x] >> 7) & 1;

                self.registers[0xF] = msb;
                self.registers[x] <<= 1;
            },

            // SNE Vx, Vy
            (0x9, _, _, 0x0) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            },

            // LD I, nnn
            (0xA, _, _, _) => {
                self.i_reg = nnn;
            },

            // JP V0, nnn
            (0xB, _, _, _) => {
                self.pc = nnn + (self.registers[0x0] as u16);
            },

            // RND Vx, nn
            (0xC, _, _, _) => {
                let num = rand::thread_rng().gen_range(0..=255);

                self.registers[x] = num & nn;
            },
            
            // DRW Vx, Vy, n
            (0xD, _, _, _) => {
                self.registers[0xF] = 0;
                let mut collision = false;

                for col in 0..n {
                    let y_coord = ((self.registers[y] as u16 + col) as usize) % SCREEN_HEIGHT;
                    let sprite = self.memory[(self.i_reg + col) as usize];

                    for row in 0..8 {
                        let x_coord = ((self.registers[x] as u16 + row) as usize) % SCREEN_WIDTH;
                        let pixel = ((sprite >> 7 - row) & 1) == 1;

                        collision = self.video[y_coord][x_coord] && pixel;
                        self.video[y_coord][x_coord] ^= pixel;
                    }
                }

                if collision {
                    self.registers[0xF] = 1;
                }
            },

            // SKP Vx
            (0xE, _, 0x9, 0xE) => {
                if self.keyboard[self.registers[x] as usize] {
                    self.pc += 2;
                }
            },

            // SKNP Vx
            (0xE, _, 0xA, 0x1) => {
                if !self.keyboard[self.registers[x] as usize] {
                    self.pc += 2;
                }
            },

            // LD Vx, DT
            (0xF, _, 0x0, 0x7) => {
                self.registers[x] = self.delay_timer;
            },

            // LD Vx, K
            (0xF, _, 0x0, 0xA) => {
                let mut key_pressed = false;

                for i in 0..self.keyboard.len() {
                    if self.keyboard[i] {
                        self.registers[x] = i as u8;
                        key_pressed = true;

                        break;
                    }
                }

                if !key_pressed {
                    self.pc -= 2;
                }
            },

            // LD DT, Vx
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer = self.registers[x];
            },

            // LD ST, Vx
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer = self.registers[x];
            },

            // ADD I, Vx
            (0xF, _, 0x1, 0xE) => {
                self.i_reg += self.registers[x] as u16;
            },

            // LD F, Vx
            (0xF, _, 0x2, 0x9) => {
                self.i_reg = (self.registers[x] * 5) as u16;
            },

            // LD B, Vx
            (0xF, _, 0x3, 0x3) => {
                let num = self.registers[x] as f32;

                self.memory[self.i_reg as usize] = (num / 100.0).floor() as u8; // Hundreds
                self.memory[(self.i_reg + 1) as usize] = ((num / 10.0) % 10.0).floor() as u8; // Tens
                self.memory[(self.i_reg + 2) as usize] = (num % 10.0) as u8; // Ones
            },

            // LD [I], Vx
            (0xF, _, 0x5, 0x5) => {
                let mem_location = self.i_reg as usize;

                for i in 0..=x {
                    self.memory[mem_location + i]  = self.registers[i];
                }
            },

            // LD Vx, [I]
            (0xF, _, 0x6, 0x5) => {
                let mem_location = self.i_reg as usize;

                for i in 0..=x {
                    self.registers[i] = self.memory[mem_location + i];
                }
            },

            (_, _, _, _) => unimplemented!("Opcode not Implemented! (Opcode: {})", op),
        }
    }
}