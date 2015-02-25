#![feature(core, std_misc, old_path)]

use sdl2::{event, render, video};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use game_controller::GameController;

extern crate sdl2;
extern crate sdl2_ttf;

pub mod game_controller;

const WINDOW_DIMENSIONS: (i32, i32) = (800, 600);

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    sdl2_ttf::init();

    let (w, h) = WINDOW_DIMENSIONS;
    let window = video::Window::new(
        "VBP",
        video::WindowPos::PosCentered, video::WindowPos::PosCentered,
        w, h,
        video::SHOWN
    ).unwrap();

    let renderer = render::Renderer::from_window(
        window, render::RenderDriverIndex::Auto,
        render::ACCELERATED | render::PRESENTVSYNC
    ).unwrap();

    let mut gc = GameController::new(w, h);
    gc.restart_game();
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
            _ => { gc.tick(&e, &renderer, &mut d); }
        }
        d.present();
    }
}
