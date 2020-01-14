use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod gui;

pub fn main()
{
    let (mut gui, mut sdl) = gui::Gui::new();


    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        gui.renderer.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        gui.renderer.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
