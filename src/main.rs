use piston_window::graphics::clear;
use piston_window::graphics::types::Color;
use piston_window::*;

mod draw;
mod game;
mod snake;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
  let (width, height) = (30, 30);
  let mut window: PistonWindow =
    WindowSettings::new("Snaku", [to_coord_u32(width), to_coord_u32(height)])
      .exit_on_esc(true)
      .build()
      .unwrap();

  let mut game = Game::new(width, height);
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      game.key_pressed(key);
    }
    window.draw_2d(&event, |c, g, _| {
      clear(BACKGROUND_COLOR, g);
      game.draw(&c, g);
    });

    event.update(|arg| {
      game.update(arg.dt);
    });
  }
}
