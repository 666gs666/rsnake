use piston_window::*;
use rand::Rng;

use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::Snake;

const FPS: f64 = 10.0;
<<<<<<< HEAD
// const RESTART_TIME: f64 = 1.0;
=======
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();
<<<<<<< HEAD

=======
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
    Position {
        x: rng.gen_range(0..width as i32),
        y: rng.gen_range(0..height as i32),
    }
}

<<<<<<< HEAD
=======
#[derive(PartialEq)]
pub enum GameState {
    StartScreen,
    Playing,
    GameOver,
}

>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
<<<<<<< HEAD
    over: bool,
    paused: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        // use fn defined at eof to calc random fruit / snake pos here
=======
    paused: bool,
    state: GameState,
    background: G2dTexture,
    start_button: G2dTexture,
    exit_button: G2dTexture,
}

impl Game {
    pub fn new(width: u32, height: u32, background: G2dTexture, start_button: G2dTexture, exit_button: G2dTexture) -> Self {
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
<<<<<<< HEAD
            over: false,
            paused: true,
=======
            paused: true,
            state: GameState::StartScreen,
            background,
            start_button,
            exit_button,
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
        }
    }

    pub fn start(&mut self) {
        self.paused = false;
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

<<<<<<< HEAD
    // pub fn toggle_game_state(&mut self) {
    //     if self.paused {
    //         self.start();
    //     } else {
    //         self.pause();
    //     }
    // }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        draw_block(&ctx, g, colors::FRUIT, &self.fruit);
        self.snake.draw(&ctx, g);
        // draw_text(&ctx, g, colors::SCORE, self.score.to_string());

        if self.over {
            draw_overlay(&ctx, g, colors::OVERLAY, self.size)
=======
    pub fn draw(&self, ctx: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        match self.state {
            GameState::StartScreen => {
                image(&self.background, ctx.transform.scale(0.4, 0.5), g);

                image(&self.start_button,
                    ctx.transform.trans(200.0, 180.0).scale(
                        200.0 / self.start_button.get_width() as f64,
                        80.0 / self.start_button.get_height() as f64),
                    g);

                image(&self.exit_button,
                    ctx.transform.trans(200.0, 300.0).scale(
                        200.0 / self.exit_button.get_width() as f64,
                        80.0 / self.exit_button.get_height() as f64),
                    g);
            }

            GameState::Playing => {
                draw_block(&ctx, g, colors::FRUIT, &self.fruit);
                self.snake.draw(&ctx, g);

                text::Text::new_color(colors::SCORE, 24)
                    .draw(
                        &format!("Score: {}", self.score),
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(10.0, 30.0),
                        g,
                    )
                    .unwrap();

                text::Text::new_color(colors::BUTTON, 16)
                    .draw(
                        "Press P: Pause | Press ESC: Exit",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(10.0, 60.0),
                        g,
                    )
                    .unwrap();
            }

            GameState::GameOver => {
                text::Text::new_color(colors::BUTTON, 32)
                    .draw(
                        "Game Over",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(100.0, 100.0),
                        g,
                    )
                    .unwrap();

                text::Text::new_color(colors::SCORE, 24)
                    .draw(
                        &format!("Score: {}", self.score),
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(100.0, 150.0),
                        g,
                    )
                    .unwrap();

                text::Text::new_color(colors::BUTTON, 20)
                    .draw(
                        "Press R to Restart, ESC to Exit",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(100.0, 200.0),
                        g,
                    )
                    .unwrap();
            }
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

<<<<<<< HEAD
        // if self.over {
        // if self.waiting_time > RESTART_TIME {
        //     self.restart();
        // }
        // return;
        // }

        if self.waiting_time > fps_in_ms(FPS) && !self.over && !self.paused {
            // self.check_colision() use snake.get_head_pos;
=======
        if self.waiting_time > fps_in_ms(FPS) && self.state == GameState::Playing && !self.paused {
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
            self.waiting_time = 0.0;

            if !self.snake.is_tail_overlapping() && !self.snake.will_tail_overlapp() {
                self.snake.update(self.size.0, self.size.1);

                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.snake.update(self.size.0, self.size.1);
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                    self.calc_score();
                }
            } else {
<<<<<<< HEAD
                self.over = true;
=======
                self.state = GameState::GameOver;
            }
        }
    }

    pub fn mouse_click(&mut self, x: f64, y: f64) {
        if self.state == GameState::StartScreen {
            if x >= 200.0 && x <= 400.0 && y >= 180.0 && y <= 260.0 {
                self.state = GameState::Playing;
                self.start();
            }

            if x >= 200.0 && x <= 400.0 && y >= 300.0 && y <= 380.0 {
                std::process::exit(0);
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
            }
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        use keyboard::Key;

<<<<<<< HEAD
        // match key {
        //     Key::R => self.over = false, // temp solution -> replace current game state trough new one
        //     Key::Space => self.toggle_game_state(),
        //     _ => self.start(),
        // }

=======
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
        match key {
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
<<<<<<< HEAD
=======
            Key::P => {
                if self.state == GameState::Playing {
                    self.paused = !self.paused;
                }
            }
            Key::R => {
                if self.state == GameState::GameOver {
                    *self = Game::new(self.size.0, self.size.1, self.background.clone(), self.start_button.clone(), self.exit_button.clone());
                    self.state = GameState::Playing;
                    self.start();
                }
            }
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
            _ => {}
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calc_score(&mut self) {
<<<<<<< HEAD
        self.score = (self.snake.get_len() * 10) as u32
    }

    // IMPORTANT!! -

    // fn update_snake(&mut self, dir: Option<Direction>) {
    //     if self.check_if_snake_alive(dir) {
    //         self.snake.move_forward(dir);
    //         self.check_eating();
    //     } else {
    //         self.game_over = true;
    //     }
    //     self.waiting_time = 0.0;
    // }
}

// fn calc_not_overlapping_pos(pos_vec: Vec<Position>, width: u32, height: u32) {
//     let mut fruit_pos: Position = calc_random_pos(width, height);

//     loop {
//         // if snake_pos.y != fruit_pos.y {
//         //     break;
//         // }

//         for pos in pos_vec {
//             if
//         }

//         snake_pos = calc_random_pos(width, height);
//         fruit_pos = calc_random_pos(width, height);
//     }
// }
=======
        self.score = self.snake.get_len() as u32;
    }
}
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
