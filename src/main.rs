extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
	texture: &Texture,
    position: Point,
    sprite: Rect,
    ) -> Result<(),String>{
    canvas.set_draw_color(color);
    canvas.clear();
    
    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;
    
    canvas.present();
    
    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;
    
    let position = Point::new(0, 0);
    // src position in the spritesheet
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut column = 0; let mut row = 0;
    'running: loop {
        //process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        //Update
        i = (i + 1) % 255;
        column = (column + 1) % 12;
        if column == 0 {row = {row + 1} % 8}
        
        let sprite = Rect::new(column*26, row*36, 26, 36);
    
        
        //render
        //
        render(&mut canvas, Color::RGB(i, 64, 255 - i),&texture,position,sprite)?;
        
        //time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
