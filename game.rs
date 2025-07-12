use piston_window::*;
use rand::Rng;
use crate::colors;
use crate::draw::*;
use crate::physics::{Direction, Position};
use crate::snake::Snake;

const FPS: f64 = 10.0;

fn fps_in_ms(fps: f64) -> f64 {
    1.0 / fps
}

fn calc_random_pos(width: u32, height: u32) -> Position {
    let mut rng = rand::thread_rng();
    Position {
        x: rng.gen_range(0..width as i32),
        y: rng.gen_range(0..height as i32),
    }
}

pub enum GameState {
    StartScreen,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    snake: Snake,
    fruit: Position,
    size: (u32, u32),
    waiting_time: f64,
    score: u32,
    state: GameState,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new(calc_random_pos(width, height)),
            fruit: calc_random_pos(width, height),
            size: (width, height),
            waiting_time: 0.0,
            score: 0,
            state: GameState::StartScreen,
        }
    }

    pub fn start(&mut self) {
        self.state = GameState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = GameState::Paused;
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            GameState::Playing => self.state = GameState::Paused,
            GameState::Paused => self.state = GameState::Playing,
            _ => {}
        }
    }

    pub fn game_over(&mut self) {
        self.state = GameState::GameOver;
    }

    pub fn restart(&mut self) {
        *self = Game::new(self.size.0, self.size.1);
        self.state = GameState::Playing;
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        match self.state {
            GameState::StartScreen => {
                // 绘制"开始游戏"按钮
                rectangle(
                    colors::SCORE,
                    [150.0, 150.0, 200.0, 50.0], // x, y, width, height
                    ctx.transform,
                    g,
                );
                text::Text::new_color(colors::BUTTON, 24)
                    .draw(
                        "Start Game",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(170.0, 180.0),
                        g,
                    )
                    .unwrap();
       
                // 绘制"退出游戏"按钮
                rectangle(
                    colors::SCORE,
                    [150.0, 250.0, 200.0, 50.0], // x, y, width, height
                    ctx.transform,
                    g,
                );
                text::Text::new_color(colors::BUTTON, 24)
                    .draw(
                        "Quit Game",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(170.0, 280.0),
                        g,
                    )
                    .unwrap();
            }
            
            GameState::Playing | GameState::Paused => {
                // 左上角显示分数
                text::Text::new_color(colors::SCORE, 20)
                    .draw(
                        format!("Score: {}", self.score).as_ref(),
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(10.0, 30.0),
                        g,
                    )
                    .unwrap();
                
                // 绘制水果
                draw_fruit(&ctx, g, colors::FRUIT, &self.fruit);
                
                // 绘制蛇
                self.snake.draw(&ctx, g);
                
                // 如果是暂停状态，绘制覆盖层和文本
                if let GameState::Paused = self.state {
                    draw_overlay(&ctx, g, colors::OVERLAY, self.size);
                    
                    text::Text::new_color(colors::BUTTON, 32)
                        .draw(
                            "PAUSED",
                            glyphs,
                            &ctx.draw_state,
                            ctx.transform.trans(100.0, 150.0),
                            g,
                        )
                        .unwrap();
                    
                    text::Text::new_color(colors::BUTTON, 24)
                        .draw(
                            "Press P to continue",
                            glyphs,
                            &ctx.draw_state,
                            ctx.transform.trans(70.0, 200.0),
                            g,
                        )
                        .unwrap();
                }
            }
            
            GameState::GameOver => {
                // 绘制游戏内容
                draw_fruit(&ctx, g, colors::FRUIT, &self.fruit);
                self.snake.draw(&ctx, g);
                
                // 绘制覆盖层
                draw_overlay(&ctx, g, colors::OVERLAY, self.size);
                
                // 游戏结束文本
                text::Text::new_color(colors::BUTTON, 32)
                    .draw(
                        "GAME OVER",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(50.0, 100.0),
                        g,
                    )
                    .unwrap();
                
                // 显示得分
                text::Text::new_color(colors::BUTTON, 24)
                    .draw(
                        format!("Score: {}", self.score).as_ref(),
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(50.0, 150.0),
                        g,
                    )
                    .unwrap();
                
                // 操作提示
                text::Text::new_color(colors::BUTTON, 24)
                    .draw(
                        "Press R to restart",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(50.0, 200.0),
                        g,
                    )
                    .unwrap();
                
                text::Text::new_color(colors::BUTTON, 24)
                    .draw(
                        "Press ESC to quit",
                        glyphs,
                        &ctx.draw_state,
                        ctx.transform.trans(50.0, 240.0),
                        g,
                    )
                    .unwrap();
            }
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // 只在游戏进行中更新逻辑
        if let GameState::Playing = self.state {
            self.waiting_time += delta_time;
            
            if self.waiting_time > fps_in_ms(FPS) {
                self.waiting_time = 0.0;
                
                // 检查碰撞
                if self.snake.is_tail_overlapping() || self.snake.will_tail_overlapp() {
                    self.game_over();
                    return;
                }
                
                // 更新蛇的位置
                self.snake.update(self.size.0, self.size.1);
                
                // 检查是否吃到水果
                if *self.snake.get_head_pos() == self.fruit {
                    self.snake.grow();
                    self.score = self.snake.get_len() as u32;
                    self.fruit = calc_random_pos(self.size.0, self.size.1);
                }
            }
        }
    }

    pub fn mouse_click(&mut self, x: f64, y: f64) {
        match self.state {
            GameState::StartScreen => {
                // 检查鼠标点击是否在"开始游戏"按钮区域
                if x >= 150.0 && x <= 350.0 && y >= 150.0 && y <= 200.0 {
                    self.state = GameState::Playing;
                }
                // 检查鼠标点击是否在"退出游戏"按钮区域
                if x >= 150.0 && x <= 350.0 && y >= 250.0 && y <= 300.0 {
                    std::process::exit(0);
                }
            }
            _ => {}
        }
    }

    pub fn key_down(&mut self, key: keyboard::Key) {
        use keyboard::Key;
        
        match key {
            // 方向控制
            Key::A | Key::Left => self.snake.set_dir(Direction::Left),
            Key::W | Key::Up => self.snake.set_dir(Direction::Up),
            Key::D | Key::Right => self.snake.set_dir(Direction::Right),
            Key::S | Key::Down => self.snake.set_dir(Direction::Down),
            
            // 暂停/继续
            Key::P => self.toggle_pause(),
            
            // 游戏结束后的操作
            Key::R if matches!(self.state, GameState::GameOver) => self.restart(),
            Key::Escape if matches!(self.state, GameState::GameOver) => std::process::exit(0),
            
            _ => {}
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}