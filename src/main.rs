extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
// "self" imports the "image" module itself as well as everything else we listed
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use std::collections::VecDeque;

const PLAYER_MOVEMENT_SPEED: i32 = 1;

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
    dir_stack: VecDeque<Direction>,
    current_frame: u32,
}


/// Returns the row of the spritesheet corresponding to the given direction
fn direction_spritesheet_row(direction: Direction) -> u32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
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
        let (frame_width, frame_height) = player.sprite.size();
        let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame as i32,
        player.sprite.y() + frame_height as  i32 * direction_spritesheet_row(player.direction) as i32,
        frame_width,
        frame_height,
        );
        let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
        canvas.copy(texture, current_frame, screen_rect)?;
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
        // Only continue to animate if the player is moving
    if player.speed != 0 {
        // Cheat: using the fact that all animations are 3 frames (NOT extensible)
        player.current_frame = (player.current_frame + 1) % 3;
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
    let mut players : [Player; 1] = [
        Player {
            position: Point::new(-10, 55),
            sprite: Rect::new(0*26, 0*36, 26, 36),
            speed: 0,
            direction: Direction::Right,
            dir_stack: VecDeque::with_capacity(4),
            current_frame: 0,
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
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    if let Event::KeyDown{keycode, ..} = event {
                        let new_dir = keycode_to_direction(keycode.unwrap()).unwrap();
                        let old_dir = players[0].direction;
                        if (players[0].speed !=0) && directions_are_opposite(old_dir,new_dir){
                            players[0].speed = 0;
                        }
                        else {
                            players[0].speed = PLAYER_MOVEMENT_SPEED;
                            players[0].direction = new_dir;
                            players[0].dir_stack.push_back(new_dir);
                        }
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    if let Event::KeyUp{keycode, ..} = event {
                        let mut idx = 0;
                        //Find lifted key
                        for pos in players[0].dir_stack.iter() {
                           if pos ==  &keycode_to_direction(keycode.unwrap()).unwrap()   {
                               break;
                           }
                           else {idx+=1;}
                        }
                        players[0].dir_stack.remove(idx);//remove lifted key
                        let x = players[0].dir_stack.back();
                        if x == None {
                            players[0].speed = 0;
                        }
                        else if players[0].speed != 0 {
                            players[0].direction = *x.unwrap();
                        }
                    }
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

fn keycode_to_direction(keycode : Keycode) -> Option<Direction> {
   match keycode    {
       Keycode::Left    => Some(Direction::Left),
       Keycode::Right   => Some(Direction::Right),
       Keycode::Down    => Some(Direction::Down),
       Keycode::Up      => Some(Direction::Up),
       _ => None,
   }
}

fn directions_are_opposite(dir1:Direction, dir2:Direction) -> bool {
    if (dir1 == Direction::Left) && (dir2 == Direction::Right) { return true } 
    if (dir1 == Direction::Right) && (dir2 == Direction::Left) {return true}
    if (dir1 == Direction::Up) && (dir2 == Direction::Down) {return true}
    if (dir1 == Direction::Down) && (dir2 == Direction::Up) {return true}
    false
}
