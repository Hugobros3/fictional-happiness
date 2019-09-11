extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod vector;
mod display;
mod sprite;

pub fn main() -> Result<(), String> {

    let mut display = display::Display::new("Salut", 800, 800);
    let texture = display.load_texture("robot.jpg");
    let mut sprite = sprite::Sprite::new(texture);

    // let texture_creator = display.canvas.texture_creator();
    // let texture = texture_creator.load_texture("robot.jpg")?;

    while *display.should_close.borrow_mut() != false {
        display.update();
        // display.canvas.clear();
        // display.canvas.copy(&texture, None, Rect::new(0,0, 128, 128))?;

        display.present();
    }
    println!("Closing, bye");

    Ok(())
}
