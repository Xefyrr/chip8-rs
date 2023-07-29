extern crate sdl2;

mod chip8;
use chip8::Chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

use std::fs::File;
use std::io::Read;
use std::env;

use sdl2::event::Event;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const WINDOW_SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WINDOW_SCALE;

const INSTRUCTIONS_PER_SECOND: u32 = 500;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No ROM Specified. Exiting...");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let _window = video_subsys.window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().opengl().build().expect("Unable to initialise window!");

    let mut canvas = _window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::init();

    let mut rom = File::open(&args[1]).expect("Unable to open file!");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load_rom(&buffer);

    'running: loop {
        for _ in 0..INSTRUCTIONS_PER_SECOND / 60 {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'running;
                    },
                    Event::KeyDown {keycode: Some(key), ..} => {
                        if let Some(k) = process_keycode(key) {
                            chip8.key_down(k);
                        }
                    },
                    Event::KeyUp {keycode: Some(key), ..} => {
                        if let Some(k) = process_keycode(key) {
                            chip8.key_up(k);
                        }
                    },
                    _ => {},
                }
            }

            chip8.tick();
        }

        chip8.update_timers();

        let should_draw = chip8.get_update_status();

        if should_draw {
            draw(&chip8, &mut canvas);
        }
    }
}

fn process_keycode(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

fn draw(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let chip8_screen = chip8.get_video_memory();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    for (y, row) in chip8_screen.iter().enumerate() {
        for (x, &_col) in row.iter().enumerate() {
            if chip8_screen[y][x] {
                let _ = canvas.fill_rect(Rect::new((x as u32 * WINDOW_SCALE) as i32, (y as u32 * WINDOW_SCALE) as i32, WINDOW_SCALE, WINDOW_SCALE));
            }
        }
    }

    canvas.present();
}