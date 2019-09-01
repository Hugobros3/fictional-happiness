use crate::display;

pub struct Sprite<'a> {
    width: f32,
    height: f32,
    texture: sdl2::render::Texture<'a>
}

impl<'a> Sprite<'a> {
    pub fn new(texture: sdl2::render::Texture) -> Sprite {
        Sprite{width: 42.0, height: 42.0, texture}
    }
}