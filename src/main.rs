extern crate sdl2;

mod chip8;
use chip8::Chip8;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::keyboard::Keycode;

use std::fs::File;
use std::io::Read;
use std::env;
use std::time::{Duration, Instant};

use sdl2::event::Event;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const WINDOW_SCALE: u32 = 20;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * WINDOW_SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * WINDOW_SCALE;

const INSTRUCTIONS_PER_SECOND: u32 = 500;
const WAIT_TIME: f64 = 1.0 / 60.0;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    // Generates the square wave
    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            }
            else {
                -self.volume
            };

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

enum Keys {
    CTRL,
    R,
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No ROM Specified. Exiting...");
        return;
    }

    // Set up SDL video and audio
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys.window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().opengl().build().expect("Unable to initialise window!");

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();

    // Initialises the CHIP-8 and loads the ROM
    let mut chip8 = Chip8::init();

    let mut rom = File::open(&args[1]).expect("Unable to open file!");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load_rom(&buffer);

    let mut keys_down: [bool; 2] = Default::default();

    'running: loop {
        let time = Instant::now();

        // Runs the desired amount of instructions per second that would happen in a frame
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

                        if key == Keycode::LCtrl {
                            keys_down[Keys::CTRL as usize] = true;
                        }
                        else if key == Keycode::R {
                            keys_down[Keys::R as usize] = true;
                        }
                        
                    },
                    Event::KeyUp {keycode: Some(key), ..} => {
                        if let Some(k) = process_keycode(key) {
                            chip8.key_up(k);
                        }

                        if key == Keycode::LCtrl {
                            keys_down[Keys::CTRL as usize] = false;
                        }
                        else if key == Keycode::R {
                            keys_down[Keys::R as usize] = false;
                        }
                    },
                    _ => {},
                }
            }

            if keys_down[Keys::CTRL as usize] && keys_down[Keys::R as usize] {
                chip8.reset();
                draw(&chip8, &mut canvas);
            }

            if !chip8.has_done_reset()
            {
                chip8.tick();
            }
        }

        chip8.update_timers();

        let should_draw = chip8.get_screen_update_status();

        if should_draw {
            draw(&chip8, &mut canvas);
        }

        if chip8.should_beep() {
            device.resume();
        }
        else {
            device.pause();
        }

        let seconds = time.elapsed().as_secs_f64();

        // If it has taken less time than it should to run the instructions in this frame
        // Then wait for the remaining time so that it is accurate to the number of instructions that should run
        if WAIT_TIME > seconds {
            std::thread::sleep(Duration::from_secs_f64(WAIT_TIME - seconds));
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