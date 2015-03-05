#![feature(core, std_misc, old_path)]

use std::default::Default;

use sdl2::{event, render, video};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use sdl2_mixer::{AUDIO_S16LSB, DEFAULT_FREQUENCY, Group};

use game_controller::GameController;

extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_mixer;

pub mod game_controller;

const WINDOW_DIMENSIONS: (i32, i32) = (800, 600);

fn main() {
    sdl2::init(sdl2::INIT_AUDIO | sdl2::INIT_VIDEO);
    sdl2_ttf::init();
    sdl2_mixer::init(sdl2_mixer::INIT_MP3); 

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

    // "Capture" mouse so cursor isn't shown and can't leave.
    sdl2::mouse::set_relative_mouse_mode(true);

    // TODO Don't ignore the SdlResult! Demo uses try!... Any unwraps are bad.
    let _ = sdl2_mixer::open_audio(DEFAULT_FREQUENCY, AUDIO_S16LSB, 2, 2048);
    sdl2_mixer::allocate_channels(1);
    let group: Group = Default::default();
    let channel = group.find_available().unwrap();

    let mut gc = GameController::new(w, h, channel);
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
