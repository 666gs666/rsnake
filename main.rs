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
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("retro-gaming.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    }, TextureSettings::new()).unwrap();

    let mut main: Game = Game::new(WIDTH, HEIGHT);
    main.start();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            main.key_down(key);
        }
        if let Some([x, y]) = event.mouse_cursor_args() {
            if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
                main.mouse_click(x, y);
            }
        }
        window.draw_2d(&event, |ctx, g, _| {
            clear(colors::BACKGROUND, g);          
            main.draw(ctx,g,&mut glyphs);
          
        });
        
        event.update(|arg| {
            main.update(arg.dt);
        });
    }
}
