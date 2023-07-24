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
        };

        for i in 0..FONT_SIZE {
            chip8.memory[i] = FONT_SET[i];
        }

        chip8
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

        let nnn = (op & 0x0FFF);
        let nn = (op & 0x00FF) as u8;
        let x = nibbles.1;
        let y = nibbles.2;
        let n = nibbles.3;

        match nibbles {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                self.video = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];
            },

            // JP nnn
            (0x1, _, _, _) => {
                self.pc = nnn;
            },

            // LD Vx, nn
            (0x6, _, _, _) => {
                self.registers[x as usize] = nn;
            },

            // ADD
            (0x7, _, _, _) => {
                self.registers[x as usize] += nn;
            },

            // LD I, addr
            (0xA, _, _, _) => {
                self.i_reg = nnn;
            },
            
            // DRW Vx, Vy, n
            (0xD, _, _, _) => {
                self.registers[0xF] = 0;
                let mut collision = false;

                for col in 0..n {
                    let y_coord = ((self.registers[y as usize] as u16 + col) as usize) % SCREEN_HEIGHT;
                    let sprite = self.memory[(self.i_reg + col) as usize];

                    for row in 0..8 {
                        let x_coord = ((self.registers[x as usize] as u16 + row) as usize) % SCREEN_WIDTH;
                        let pixel = ((sprite >> 7 - row) & 1) == 1;

                        collision = self.video[y_coord][x_coord] && pixel;
                        self.video[y_coord][x_coord] ^= pixel;
                    }
                }

                if collision {
                    self.registers[0xF] = 1;
                }
            }

            (_, _, _, _) => unimplemented!("Opcode not Implemented! (Opcode: {})", op),
        }
    }
}