extern crate sdl2;

use sdl2::event::Event;

const WINDOW_SCALE: u32 = 20;
const SIZE_WIDTH: u32 = 64;
const SIZE_HEIGHT: u32 = 32;

fn main() -> Result<(), String> {
    let sdl_ctx = sdl2::init()?;
    let sdl_video = sdl_ctx.video()?;

    let _window = sdl_video.window("CHIP-8 Emulator", SIZE_WIDTH * WINDOW_SCALE, SIZE_HEIGHT * WINDOW_SCALE).position_centered().build().expect("Unable to initialise window!");
    let mut event_pump = sdl_ctx.event_pump()?;

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

    Ok(())
}