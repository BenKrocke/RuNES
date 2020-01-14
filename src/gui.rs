extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use std::string::String;
use sdl2::render::Canvas;
use sdl2::video::Window;

const TITLE: &str = "RuNES Emulator";
const NES_WIDTH: u32 = 256;
const NES_HEIGHT: u32 = 240;
const WIDTH: u32 = NES_WIDTH + 200;
const HEIGHT: u32 = NES_HEIGHT + 100;

pub struct Gui {
    pub renderer: Box<Canvas<Window>>,
}

impl Gui {
    pub fn new() -> (Gui, Sdl)
    {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem.window(&TITLE, WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let renderer = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        (
            Gui {
                renderer: Box::new(renderer),
            },
            sdl,
        )
    }
}
