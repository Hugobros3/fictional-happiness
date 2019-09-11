use sdl2::image::InitFlag;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use std::cell::RefCell;

pub struct Display {
    // Private field
    event_pump: RefCell<sdl2::EventPump>,
    // Accessible field
    pub texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub canvas: RefCell<sdl2::render::Canvas<sdl2::video::Window>>,
    pub should_close: RefCell<bool>
}

impl Display {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);

        let _win = video_subsystem
            .window(title, width, height)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let mut canvas = _win
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        
        canvas.set_draw_color(Color::RGBA(51, 51, 51, 255));
        
        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_creator = canvas.texture_creator();   

        return Display{event_pump: RefCell::new(event_pump), texture_creator, canvas: RefCell::new(canvas), should_close: RefCell::new(false)};
    }

    // Poll events and clear screen
    pub fn update(&self) {
        for event in self.event_pump.borrow_mut().poll_iter() {
            match event {
                Event::Quit { .. } => *self.should_close.borrow_mut() = true,
                _ => {}
            }
        }
        self.canvas.borrow_mut().clear();
    }

    // Show current frame to screen
    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }

    // Load a texture
    pub fn load_texture(&self, src: &str) -> sdl2::render::Texture {
        self.texture_creator.load_texture(src).unwrap()
    }
}