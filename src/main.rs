extern crate sdl2;
use std::time::Duration;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

/*
pub struct Checkbox {
    state: bool,
    x: usize,
    y: usize,
}

 */

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

/*
pub fn generate() -> (i32, i32) {
    let mut rng = rand::rng();

    let random_x = rng.random_range(0..800);
    let random_y = rng.random_range(0..600);

    (random_x, random_y)
}

 */

// collision check between 2 rects
pub fn has_collided(r1: Rect, r2: Rect) -> bool {
    if r1.intersection(r2).is_some() {
        return true
    }
    false
}

// collision check with the border of the screen
pub fn collided_with_border(rect: Rect) -> bool {
    if rect.y() >= (HEIGHT - rect.size().1) as i32
    || rect.x() >= (WIDTH - rect.size().0) as i32 {
        return true
    }
    false
}

pub fn main() {
    // let mut generated_rectangles: i32 = 0;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("River Demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rectangle = Rect::new(WIDTH as i32, HEIGHT as i32, 32, 32);
    let mut rectangle_position: (i32, i32) = (0, 0);

    let speed = (WIDTH / 15) as i32;

    // let mut cursor_position: (i32, i32) = (0, 0);

    sdl_context.mouse().show_cursor(false);

    'running: loop {
        for event in event_pump.poll_iter() {
                match event {
                Event::Quit {
                    ..
                } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    sdl_context.mouse().show_cursor(true);
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    rectangle_position.0 += speed;
                },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    rectangle_position.0 -= speed;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    rectangle_position.1 += speed;
                }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    rectangle_position.1 -= speed;
                }
                /*
                Event::MouseMotion {x, y, ..} => {
                    cursor_position = (x, y);
                }
                 */
                _ => {}
            }
        }
        // rest of the render loop

        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        canvas.fill_rect(rectangle).unwrap();

        rectangle.set_x(rectangle_position.0);
        rectangle.set_y(rectangle_position.1);

        if collided_with_border(rectangle) == true {
            canvas.clear();
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 240));
    }
}
