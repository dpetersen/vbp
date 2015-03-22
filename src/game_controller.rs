use std::num::Float;
use std::f64::{INFINITY, NEG_INFINITY};
use std::f64::consts::PI;

use sdl2::event::Event;
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Renderer, RenderDrawer};
use sdl2_mixer::{Channel, Chunk};
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
    channel: Channel,
    boop: Chunk,
    lose_point: Chunk,

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
    pub fn new(window_width: i32, window_height: i32, channel: Channel) -> GameController {
        let score_font_path = &Path::new("art/alterebro-pixel-font.ttf");
        let score_font = Font::from_file(score_font_path, SCORE_FONT_SIZE).unwrap();
        let boop = Chunk::from_file(&Path::new("art/boop.wav")).unwrap();
        let lose_point = Chunk::from_file(&Path::new("art/lose_point.wav")).unwrap();

        return GameController{
            channel: channel,
            boop: boop,
            lose_point: lose_point,
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
        let player_paddle_x = PADDLE_WALL_PADDING;
        let opponent_paddle_x = self.window_width - PADDLE_WIDTH - PADDLE_WALL_PADDING;
        let vx = (self.ball_angle.cos() * BALL_VELOCITY) as i32;
        let vy = (self.ball_angle.sin() * BALL_VELOCITY) as i32;

        let (mut x, mut y) = self.ball_position;

        // Wall collision
        let upper_y_limit = self.window_height - BALL_BREADTH;
        if y + vy > upper_y_limit || y + vy < 0 {
            x += vx;
            y += vy;

            if y > upper_y_limit { y = upper_y_limit; }
            else if y < 0 { y = 0; }

            self.play_sound(&self.boop);
            self.ball_angle *= -1.0;

            self.ball_position = (x, y);
            return;
        }

        let (p_normal_x, p_normal_y, p_collision_time) = self.swept_aabb(vx, vy, player_paddle_x, self.player_paddle_y);
        let (o_normal_x, o_normal_y, o_collision_time) = self.swept_aabb(vx, vy, opponent_paddle_x, self.opponent_paddle_y);

        // TODO You haven't accounted for remainingtime, like they do in the example. That would be
        // more accurate since there wouldn't be unaccounted-for velocity loss on an impact frame.
        if p_collision_time < 1.0 || o_collision_time < 1.0 {
            if p_normal_x != 0.0 || o_normal_x != 0.0 {
                self.play_sound(&self.boop);
                self.ball_angle = 0.0 - PI - self.ball_angle; 

                let mut time: f64;
                if p_collision_time < 1.0  { time = p_collision_time; }
                else { time = o_collision_time; }

                x += (vx as f64 * time) as i32;
                y += (vy as f64 * time) as i32;
            } else {
                // TODO there's a bug here where your y collision is firing when the ball is far
                // far away from any paddles. Seemingly always the opponent paddle, but the x
                // position can be on any part of the screen when it bounces back.
                x += (vx as f64 * 1.0) as i32;
                y += (vy as f64 * 1.0) as i32;
            }
        } else {
            x += vx;
            y += vy;
        }

        if x < 0 || x > self.window_width {
            if x < player_paddle_x { self.opponent_score += 1; }
            else if x > opponent_paddle_x { self.player_score += 1; }

            self.play_sound(&self.lose_point);
            self.restart_game();
        } else {
            self.ball_position = (x, y);
        }
    }

    // Stolen and partially understood from:
    // http://www.gamedev.net/page/resources/_/technical/game-programming/swept-aabb-collision-detection-and-response-r3084
    fn swept_aabb(&self, vx: i32, vy: i32, paddle_x: i32, paddle_y: i32) -> (f64, f64, f64) {
        let (ball_x, ball_y) = self.ball_position;

        let mut x_inv_entry: i32;
        let mut y_inv_entry: i32;
        let mut x_inv_exit: i32;
        let mut y_inv_exit: i32;

        // find the distance between the objects on the near and far sides for both x and y
        if vx > 0 {
            x_inv_entry = paddle_x - (ball_x + BALL_BREADTH);
            x_inv_exit = (paddle_x + PADDLE_WIDTH) - ball_x;
        } else {
            x_inv_entry = (paddle_x + PADDLE_WIDTH) - ball_x;
            x_inv_exit = paddle_x - (ball_x + BALL_BREADTH);
        }

        if vy > 0 {
            y_inv_entry = paddle_y - (ball_y + BALL_BREADTH);
            y_inv_exit = (paddle_y + PADDLE_HEIGHT) - ball_y;
        } else {
            y_inv_entry = (paddle_y + PADDLE_HEIGHT) - ball_y;
            y_inv_exit = paddle_y - (ball_y + BALL_BREADTH);
        }


        let mut x_entry: f64;
        let mut y_entry: f64;
        let mut x_exit: f64;
        let mut y_exit: f64;

        // find time of collision and time of leaving for each axis
        if vx == 0 {
            x_entry = NEG_INFINITY;
            x_exit = INFINITY;
        } else {
            x_entry = x_inv_entry as f64 / vx as f64;
            x_exit = x_inv_exit as f64 / vx as f64;
        }

        if vy == 0 {
            y_entry = NEG_INFINITY;
            y_exit = INFINITY;
        } else {
            y_entry = y_inv_entry as f64 / vy as f64;
            y_exit = y_inv_exit as f64 / vy as f64;
        }

        // find the earliest/latest times of collision
        let entry_time = x_entry.max(y_entry);
        let exit_time = x_exit.max(y_exit);

        if entry_time > exit_time || (x_entry < 0.0 && y_entry < 0.0) || x_entry > 1.0 || y_entry > 1.0 {
            // there was no collision
            return (0.0, 0.0, 1.0);
        } else {
            if x_entry > y_entry {
                if x_inv_entry < 0 {
                    return (1.0, 0.0, entry_time);
                } else {
                    return (-1.0, 0.0, entry_time);
                }
            } else {
                if y_inv_entry < 0 {
                    return (0.0, 1.0, entry_time);
                } else {
                    return (0.0, -1.0, entry_time);
                }
            }
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

    fn play_sound(&self, s: &Chunk) {
        let _ = self.channel.play(s, 0);
    }
}
