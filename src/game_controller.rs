use std::num::Float;
use std::f64::consts::PI;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::RenderDrawer;

static BOX_COLOR: Color = Color::RGB(255, 255, 255);
static BOX_WIDTH: i32 = 12;
static BOX_HEIGHT: i32 = 64;

static BALL_COLOR: Color = Color::RGB(175, 175, 175);
static BALL_BREADTH: i32 = 8;
static BALL_VELOCITY: f64 = 15.0;
static BALL_INITIAL_ANGLE_DEGREES: i32 = 55;

pub struct GameController {
    ball_position: (i32, i32),
    ball_angle: f64
}

impl GameController {
    pub fn new() -> GameController {
        return GameController{
            ball_position: (800 / 2 - BALL_BREADTH / 2, 600 / 2 - BALL_BREADTH / 2),
            ball_angle: (BALL_INITIAL_ANGLE_DEGREES as f64).to_radians() 
        }
    }

    pub fn tick(&mut self, _: &Event, drawer: &mut RenderDrawer) {
        self.move_ball();
        self.draw_player(drawer);
        self.draw_opponent(drawer);
        self.draw_ball(drawer);
    }

    fn move_ball(&mut self) {
        let x_change = (self.ball_angle.cos() * BALL_VELOCITY) as i32;
        let y_change = (self.ball_angle.sin() * BALL_VELOCITY) as i32;

        let (mut x, mut y) = self.ball_position;
        x += x_change;
        y += y_change;

        // Thanks StackOverflow!
        // http://stackoverflow.com/questions/5830791/find-radians-reflection-angle
        if y > 600 || y < 0 { self.ball_angle *= -1.0 }
        if x > 800 || x < 0 { self.ball_angle = 0.0 - PI - self.ball_angle; }

        if y > 600 { y = 600; }
        else if y < 0 { y = 0; }
        if x > 800 { x = 800; }
        else if x < 0 { x = 0; }

        self.ball_position = (x, y);
    }

    fn draw_player(&self, d: &mut RenderDrawer) {
        self.draw_box(d, 10, 333);
    }

    fn draw_opponent(&self, d: &mut RenderDrawer) {
        self.draw_box(d, 800 - BOX_WIDTH - 10, 200);
    }

    fn draw_box(&self, d: &mut RenderDrawer, x: i32, y: i32) {
        d.set_draw_color(BOX_COLOR);
        let r = Rect{x: x, y: y, w: BOX_WIDTH, h: BOX_HEIGHT};
        d.fill_rect(r);
     }

    fn draw_ball(&self, d: &mut RenderDrawer) {
        d.set_draw_color(BALL_COLOR);
        let (x, y) = self.ball_position;
        let r = Rect{x: x, y: y, w: BALL_BREADTH, h: BALL_BREADTH};
        d.fill_rect(r);
    }
}
