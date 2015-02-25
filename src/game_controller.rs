use std::num::Float;
use std::f64::consts::PI;

use sdl2::event::Event;
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::RenderDrawer;

const PADDLE_COLOR: Color = Color::RGB(255, 255, 255);
const PADDLE_WIDTH: i32 = 12;
const PADDLE_HEIGHT: i32 = 64;
const PADDLE_WALL_PADDING: i32 = 10;

const BALL_COLOR: Color = Color::RGB(175, 175, 175);
const BALL_BREADTH: i32 = 8;
const BALL_VELOCITY: f64 = 10.0;
const BALL_INITIAL_ANGLE_DEGREES: i32 = 45;
const BALL_START_POSITION: (i32, i32) = (800 / 2 - BALL_BREADTH / 2, 600 / 2 - BALL_BREADTH / 2);

pub struct GameController {
    ball_position: (i32, i32),
    ball_angle: f64,

    player_paddle_y: i32,
    opponent_paddle_y: i32,

    player_score: i32,
    opponent_score: i32
}

impl GameController {
    pub fn new() -> GameController {
        return GameController{
            ball_position: BALL_START_POSITION,
            ball_angle: (BALL_INITIAL_ANGLE_DEGREES as f64).to_radians(),
            player_paddle_y: 280,
            opponent_paddle_y: 10,
            player_score: 0,
            opponent_score: 0
        }
    }

    pub fn tick(&mut self, _: &Event, drawer: &mut RenderDrawer) {
        self.move_player_paddle();
        self.move_opponent_paddle();
        self.move_ball();
        self.draw_player(drawer);
        self.draw_opponent(drawer);
        self.draw_ball(drawer);
    }

    fn move_player_paddle(&mut self) {
        let (_, _, y) = mouse::get_mouse_state();
        self.player_paddle_y = y;
    }

    fn move_opponent_paddle(&mut self) {
        let (_, y) = self.ball_position;
        self.opponent_paddle_y = y - PADDLE_HEIGHT / 2;
    }

    // TODO this shouldn't be framerate/vsync dependent.
    // Thanks StackOverflow!
    // http://stackoverflow.com/questions/5830791/find-radians-reflection-angle
    fn move_ball(&mut self) {
        let x_change = (self.ball_angle.cos() * BALL_VELOCITY) as i32;
        let y_change = (self.ball_angle.sin() * BALL_VELOCITY) as i32;

        let (mut x, mut y) = self.ball_position;
        x += x_change;
        y += y_change;

        // Wall collision
        let upper_y_limit = 600 - BALL_BREADTH;
        if y > upper_y_limit || y < 0 { self.ball_angle *= -1.0 }
        if y > upper_y_limit { y = upper_y_limit; }
        else if y < 0 { y = 0; }

        // Paddle collision
        // TODO ball dimensions aren't respected. Left side of ball hits on the left, right side of
        // ball hits on the right. Also need to reset ball x on impact.
        let player_paddle_x = PADDLE_WALL_PADDING + PADDLE_WIDTH;
        let player_paddle_impacted = x <= player_paddle_x && y >= self.player_paddle_y && y <= self.player_paddle_y + PADDLE_HEIGHT;
        let opponent_paddle_x = 800 - PADDLE_WALL_PADDING - PADDLE_WIDTH - BALL_BREADTH;
        let opponent_paddle_impacted = x >= opponent_paddle_x && y >= self.opponent_paddle_y && y <= self.opponent_paddle_y + PADDLE_HEIGHT;

        // Reflect and tweak ball position.
        if player_paddle_impacted || opponent_paddle_impacted {
            self.ball_angle = 0.0 - PI - self.ball_angle; 
        }
        if player_paddle_impacted { x = player_paddle_x; }
        else if opponent_paddle_impacted { x = opponent_paddle_x; }

        if x < player_paddle_x || x > opponent_paddle_x {
            if x < player_paddle_x { self.opponent_score += 1; }
            else if x > opponent_paddle_x { self.player_score += 1; }

            println!("Score - Player: {}, Opponent: {}", self.player_score, self.opponent_score);

            self.ball_position = BALL_START_POSITION;
            self.ball_angle = 45.0.to_radians();
        } else {
            self.ball_position = (x, y);
        }
    }

    fn draw_player(&self, d: &mut RenderDrawer) {
        self.draw_box(d, PADDLE_WALL_PADDING, self.player_paddle_y);
    }

    fn draw_opponent(&self, d: &mut RenderDrawer) {
        self.draw_box(d, 800 - PADDLE_WIDTH - PADDLE_WALL_PADDING, self.opponent_paddle_y);
    }

    fn draw_box(&self, d: &mut RenderDrawer, x: i32, y: i32) {
        d.set_draw_color(PADDLE_COLOR);
        let r = Rect{x: x, y: y, w: PADDLE_WIDTH, h: PADDLE_HEIGHT};
        d.fill_rect(r);
     }

    fn draw_ball(&self, d: &mut RenderDrawer) {
        d.set_draw_color(BALL_COLOR);
        let (x, y) = self.ball_position;
        let r = Rect{x: x, y: y, w: BALL_BREADTH, h: BALL_BREADTH};
        d.fill_rect(r);
    }
}
