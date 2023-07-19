extern crate sdl2;

use sdl2::event::Event;

const WINDOW_SCALE: u32 = 20;
const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();

    let _window = video_subsys.window("CHIP-8 Emulator", SCREEN_WIDTH * WINDOW_SCALE, SCREEN_HEIGHT * WINDOW_SCALE).position_centered().opengl().build().expect("Unable to initialise window!");
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
}