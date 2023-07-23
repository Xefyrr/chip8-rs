use crate::SCREEN_WIDTH;
use crate::SCREEN_HEIGHT;

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
        Chip8 {
            memory: [0; 4096],
            video: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
            registers: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
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
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
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
                self.registers[x] = nn;
            },

            // ADD
            (0x7, _, _, _) => {
                self.registers[x] += nn;
            },

            // LD I, addr
            (0xA, _, _, _) => {
                self.i_reg = nnn;
            },
            
            // DRW Vx, Vy, n
            (0xD, _, _, _) => {
                let mut has_flipped = false;

                for rows in 0..n {
                    let addr = self.i_reg + rows as u16;
                    let pixels = self.memory[addr as usize];

                    for cols in 0..8 {
                        if (pixels & (0b1000_0000 >> cols)) == 1 {
                            let x_coord = (self.registers[x] as u16 + cols) as usize % SCREEN_WIDTH;
                            let y_coord = (self.registers[y] as u16 + rows) as usize % SCREEN_HEIGHT;

                            has_flipped |= self.video[x_coord][y_coord];
                            self.video[x_coord][y_coord] ^= true;
                        }
                    }
                }

                if has_flipped {
                    self.registers[0xF] = 1;
                }
                else {
                    self.registers[0xF] = 0;
                }
            }

            (_, _, _, _) => unimplemented!("Opcode not Implemented! (Opcode: {})", op),
        }
    }
}