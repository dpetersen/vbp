#![feature(core, std_misc)]
// For to_radians

use sdl2::{event, render, video};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use game_controller::GameController;

extern crate sdl2;

pub mod game_controller;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let window = video::Window::new(
        "VBP",
        video::WindowPos::PosCentered, video::WindowPos::PosCentered,
        800, 600,
        video::SHOWN
    ).unwrap();

    let renderer = render::Renderer::from_window(
        window, render::RenderDriverIndex::Auto,
        render::ACCELERATED | render::PRESENTVSYNC
    ).unwrap();

    let mut gc = GameController::new();
    let mut d = renderer.drawer();
    let background_color = Color::RGB(0, 0, 0);

    loop {
        let e = event::poll_event();
        d.set_draw_color(background_color);
        d.clear();
        match e {
            Event::Quit{..} | Event::KeyDown{keycode: KeyCode::Escape, ..} => {
                break;
            }
            _ => { gc.tick(&e, &mut d); }
        }
        d.present();
    }
}
