use SIZE_WIDTH;
use SIZE_HEIGHT;

struct Chip8 {
    memory: [u8; 4096],
    video: [[u8; SIZE_WIDTH]; SIZE_HEIGHT],
    registers: [u8; 16],
    i_reg: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

impl Chip8 {
    fn init() -> self {
        Chip8 {
            memory: [0; 4096],
            video: [[0; SIZE_WIDTH]; SIZE_HEIGHT],
            registers: [0; 16],
            i_reg: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
    }
}