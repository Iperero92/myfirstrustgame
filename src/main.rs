extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    direction: Direction,
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
	texture: &Texture,
    players: &[Player],
    ) -> Result<(),String>{
    canvas.set_draw_color(color);
    canvas.clear();
    
    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    for player in players
    { 
        let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
        canvas.copy(texture, player.sprite, screen_rect)?;
    }
    
    canvas.present();
    
    Ok(())
}

// Update player a fixed amount based on their speed.
// WARNING: Calling this function too often or at a variable speed will cause the player's speed
// to be unpredictable!
fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
            player.position = player.position.offset(-player.speed, 0);
        },
        Right => {
            player.position = player.position.offset(player.speed, 0);
        },
        Up => {
            player.position = player.position.offset(0, -player.speed);
        },
        Down => {
            player.position = player.position.offset(0, player.speed);
        },
    }
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
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    let mut column = 0; let mut row = 0;
    let mut players : [Player; 1] = [
        Player {
            position: Point::new(-10, 55),
            sprite: Rect::new(0*26, 0*36, 26, 36),
            speed: 0,
            direction: Direction::Right,
        },
    ];
    
    'running: loop {
        //process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    players[0].speed = PLAYER_MOVEMENT_SPEED;
                    players[0].direction = Direction::Left;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    players[0].speed = PLAYER_MOVEMENT_SPEED;
                    players[0].direction = Direction::Right;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    players[0].speed = PLAYER_MOVEMENT_SPEED;
                    players[0].direction = Direction::Up;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    players[0].speed = PLAYER_MOVEMENT_SPEED;
                    players[0].direction = Direction::Down;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    players[0].speed = 0;
                },
                _ => {}
            }
        }

        //Update
        i = (i + 1) % 255;
        update_player(&mut players[0]);
        
        //render
        //
        render(&mut canvas, Color::RGB(i, 64, 255 - i),&texture,&players)?;
        
        //time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    Ok(())
}
