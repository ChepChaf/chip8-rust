extern crate libc;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;

pub struct Display {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    width: u32,
    heigh: u32,
    exit: bool,
}

impl Display {
    pub fn init(width: u32, heigh: u32) -> Display {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("chip8-ru", width, heigh)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = context.event_pump().unwrap();

        Self {
            event_pump,
            canvas,
            width,
            heigh,
            exit: false,
        }
    }
    pub fn should_close(&self) -> bool {
        return self.exit;
    }
    fn bw_to_rgb(&self, frame: &[u8; 64 * 32]) -> [u8; 64 * 32 * 3] {
        let mut new: [u8; 64 * 32 * 3] = [0; 64 * 32 * 3];

        for y in 0..32 {
            for x in 0..64 {
                let frame_index = x + (y * 64);
                let new_index = (x + (y * 64)) * 3;
                new[new_index] = frame[frame_index];
                new[new_index + 1] = frame[frame_index];
                new[new_index + 2] = frame[frame_index];
            }
        }

        new
    }
    // TODO: Allow other dimensions?
    pub fn set_draw(&mut self, frame: &[u8; 64 * 32]) {
        self.canvas.clear();

        let mut rgb_data = self.bw_to_rgb(frame);

        let surface = sdl2::surface::Surface::from_data(
            &mut rgb_data,
            64,
            32,
            64 * 3,
            sdl2::pixels::PixelFormatEnum::RGB24,
        )
        .unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = surface.as_texture(&texture_creator).unwrap();

        match self.canvas.copy(
            &texture,
            None,
            sdl2::rect::Rect::new(0, 0, self.width, self.heigh),
        ) {
            Ok(_) => (),
            Err(_) => println!("Failed to copy texture into display."),
        }
    }
    pub fn update(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.exit = true;
                }
                _ => {}
            }
        }

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
