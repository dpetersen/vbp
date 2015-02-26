use std::num::Float;
use std::f64::consts::PI;

use sdl2::event::Event;
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, RenderDrawer};
use sdl2_ttf::Font;

const SOULLESS_GREY: Color = Color::RGB(175, 175, 175);

const PADDLE_COLOR: Color = SOULLESS_GREY;
const PADDLE_WIDTH: i32 = 12;
const PADDLE_HEIGHT: i32 = 64;
const PADDLE_WALL_PADDING: i32 = 10;

const BALL_COLOR: Color = SOULLESS_GREY;
const BALL_BREADTH: i32 = 8;
const BALL_VELOCITY: f64 = 10.0;

const SCORE_TOP_PADDING: i32 = 10;
const SCORE_COLOR: Color = SOULLESS_GREY;
const SCORE_FONT_SIZE: isize = 64;

pub struct GameController {
    window_width: i32,
    window_height: i32,

    score_font: Font,

    ball_position: (i32, i32),
    ball_angle: f64,

    player_paddle_y: i32,
    opponent_paddle_y: i32,

    player_score: i32,
    opponent_score: i32
}

impl GameController {
    pub fn new(window_width: i32, window_height: i32) -> GameController {
        let score_font_path = &Path::new("art/alterebro-pixel-font.ttf");
        let score_font = Font::from_file(score_font_path, SCORE_FONT_SIZE).unwrap();

        return GameController{
            window_width: window_width,
            window_height: window_height,
            score_font: score_font,
            ball_position: (0, 0),
            ball_angle: 0.0,
            player_paddle_y: 280,
            opponent_paddle_y: 10,
            player_score: 0,
            opponent_score: 0
        }
    }

    pub fn restart_game(&mut self) {
        self.ball_position = (
            self.window_width / 2 - BALL_BREADTH / 2,
            self.window_height / 2 - BALL_BREADTH / 2
        );
        self.ball_angle = 45.0.to_radians();
    }

    pub fn tick(&mut self, _: &Event, renderer: &Renderer, drawer: &mut RenderDrawer) {
        self.move_player_paddle();
        self.move_opponent_paddle();
        self.move_ball();
        self.draw_scores(&renderer, drawer);
        self.draw_player(drawer);
        self.draw_opponent(drawer);
        self.draw_ball(drawer);
    }

    fn move_player_paddle(&mut self) {
        let (_, _, mut y) = mouse::get_mouse_state();
        if y < 0 { y = 0; }
        else if y + PADDLE_HEIGHT > self.window_height { y = self.window_height - PADDLE_HEIGHT; }

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
        let upper_y_limit = self.window_height - BALL_BREADTH;
        if y > upper_y_limit || y < 0 { self.ball_angle *= -1.0 }
        if y > upper_y_limit { y = upper_y_limit; }
        else if y < 0 { y = 0; }

        // Paddle collision
        // TODO ball dimensions aren't respected. Left side of ball hits on the left, right side of
        // ball hits on the right. Also need to reset ball x on impact.
        let player_paddle_x = PADDLE_WALL_PADDING + PADDLE_WIDTH;
        let player_paddle_impacted = x <= player_paddle_x && y + BALL_BREADTH >= self.player_paddle_y && y <= self.player_paddle_y + PADDLE_HEIGHT;
        let opponent_paddle_x = self.window_width - PADDLE_WALL_PADDING - PADDLE_WIDTH - BALL_BREADTH;
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

            self.restart_game();
        } else {
            self.ball_position = (x, y);
        }
    }

    fn draw_scores(&self, r: &Renderer, d: &mut RenderDrawer) {
        let third = self.window_width / 3;
        self.draw_score(r, d, self.opponent_score, third);
        self.draw_score(r, d, self.player_score, third * 2);
    }

    fn draw_score(&self, r: &Renderer, d: &mut RenderDrawer, score: i32, x: i32) {
        let surface = self.score_font.render_str_blended(&score.to_string(), SCORE_COLOR).unwrap();
        let t = r.create_texture_from_surface(&surface).unwrap();
        let q = t.query();
        let b = Rect::new(x - q.width / 2, SCORE_TOP_PADDING, q.width, q.height);
        d.copy(&t, None, Some(b));
    }

    fn draw_player(&self, d: &mut RenderDrawer) {
        self.draw_box(d, PADDLE_WALL_PADDING, self.player_paddle_y);
    }

    fn draw_opponent(&self, d: &mut RenderDrawer) {
        self.draw_box(d, self.window_width - PADDLE_WIDTH - PADDLE_WALL_PADDING, self.opponent_paddle_y);
    }

    fn draw_box(&self, d: &mut RenderDrawer, x: i32, y: i32) {
        d.set_draw_color(PADDLE_COLOR);
        let r = Rect::new(x, y, PADDLE_WIDTH, PADDLE_HEIGHT);
        d.fill_rect(r);
     }

    fn draw_ball(&self, d: &mut RenderDrawer) {
        d.set_draw_color(BALL_COLOR);
        let (x, y) = self.ball_position;
        let r = Rect::new(x, y, BALL_BREADTH, BALL_BREADTH);
        d.fill_rect(r);
    }
}
