/* Copyright (C) 2019 by Mara Schulke */

/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
<<<<<<< HEAD

=======
use piston_window::{Event, PressEvent, MouseButton};
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
mod colors;
mod draw;
mod game;
mod physics;
mod snake;

use draw::blocks_in_pixels;
use game::Game;
use piston_window::*;

const WINDOW_TITLE: &'static str = "rsnake";
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;

fn main() {
    let size = [blocks_in_pixels(WIDTH), blocks_in_pixels(HEIGHT)];

    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, size)
        .resizable(false)
<<<<<<< HEAD
        .build()
        .unwrap();

=======
        .exit_on_esc(true)
        .build()
        .unwrap();
    
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("retro-gaming.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    }, TextureSettings::new()).unwrap();
<<<<<<< HEAD

    let mut main: Game = Game::new(WIDTH, HEIGHT);
    main.start();

=======
    let background = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let path = assets.join("bk1.png");
        
        Texture::from_path(
            &mut TextureContext { // 使用 TextureContext 而不是 Factory
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into(),
            },
            &path,
            Flip::None,
            &TextureSettings::new().filter(Filter::Nearest),
        ).expect("Failed to load background image")
    };
    let startbutton = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let path = assets.join("start.png");
        
        Texture::from_path(
            &mut TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into(),
            },
            &path,
            Flip::None,
            &TextureSettings::new().filter(Filter::Nearest),
        ).expect("Failed to load start button image")
    };
    let exitbutton = {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let path = assets.join("exit.png");
        
        Texture::from_path(
            &mut TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into(),
            },
            &path,
            Flip::None,
            &TextureSettings::new().filter(Filter::Nearest),
        ).expect("Failed to load exit button image")
    };
    let mut main: Game = Game::new(WIDTH, HEIGHT,background, startbutton, exitbutton);
    main.start();
    let mut last_mouse_pos = [0.0, 0.0];
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }
<<<<<<< HEAD

        window.draw_2d(&event, |ctx, g, _| {
            clear(colors::BACKGROUND, g);
            text::Text::new_color(colors::SCORE, 20)
                .draw(
                    main.get_score().to_string().as_ref(),
                    &mut glyphs,
                    &ctx.draw_state,
                    ctx.transform.trans(0.0, 20.0),
                    g,
                )
                .unwrap();
            main.draw(ctx, g);
        });
=======
      // 单独处理鼠标点击
      if let Some(pos) = event.mouse_cursor_args() {
        last_mouse_pos = pos;
      }

    // 处理点击事件
       if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
          println!("Clicked at: {:?}", last_mouse_pos);
          main.mouse_click(last_mouse_pos[0], last_mouse_pos[1]);
       }
    
       window.draw_2d(&event, |ctx, g, device| {
    clear(colors::BACKGROUND, g);
    main.draw(ctx, g, &mut glyphs);

    // 清除缓存（避免字体显示异常）
    glyphs.factory.encoder.flush(device);
});
>>>>>>> 4eb88db (增加结束界面，修复计分板功能，实现R键ESC键P键功能)

        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
