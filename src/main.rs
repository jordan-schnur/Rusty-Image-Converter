use std::{env, process};
use pngtojpeg::{Config};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

// TODO: Should we use this? https://docs.rs/bytes/latest/bytes/
// https://en.wikipedia.org/wiki/PNG#PNG_Working_Group
// https://www.w3.org/TR/png/#5PNG-file-signature

fn main() -> Result<(), String> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = pngtojpeg::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
                                .position_centered()
                                .build()
                                .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
                           .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump
        = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGBA(i, 64, 255, 255 - i));
        canvas.fill_rect(Rect::new(0, 0, 100, 100)).expect("TODO: panic message");

        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
