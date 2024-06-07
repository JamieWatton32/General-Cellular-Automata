mod api;
mod grid;
mod logic;
mod user_data;


use sdl2::video::Window;


use grid::Grid;
use user_data::UserData;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use std::time::{Duration, Instant};

pub const HEIGHT: i32 = 720;
pub const WIDTH: i32 = 720;
pub const PARTICLE_SIZE: i32 = 2;


fn draw_grid(canvas: &mut Canvas<Window>, grid: &mut Grid) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let rect = Rect::new(
                i as i32,
                j as i32,
                PARTICLE_SIZE as u32,
                PARTICLE_SIZE as u32,
            );
            match grid.cells[i as usize][j as usize].user_data {
                UserData::Sand => {
                    canvas.set_draw_color(Color::RGB(194, 178, 128));
                    canvas.fill_rect(rect).unwrap();
                }
                UserData::Water => {
                    canvas.set_draw_color(Color::RGB(68, 238, 221));
                    canvas.fill_rect(rect).unwrap();
                }
                UserData::Wall => {
                    canvas.set_draw_color(Color::RGB(128, 128, 128));
                    canvas.fill_rect(rect).unwrap();
                }
                UserData::Empty => {}
            }
        }
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::new(WIDTH, HEIGHT);
    let mut left_mouse_button_down = false;
    let mut right_down = false;
    let mut spawn_user_data = UserData::Empty;

    
    let mut last_fps_print_time = Instant::now();
    let mut frame_count = 0;
        'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        left_mouse_button_down = true;
                        spawn_user_data = UserData::Sand;
                    } else if mouse_btn == MouseButton::Right {
                        right_down = true;
                        spawn_user_data = UserData::Water;
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        left_mouse_button_down = false;
                    } else if mouse_btn == MouseButton::Right {
                        right_down = false;
                    }
                }
                _ => {}
            }
        }
        if left_mouse_button_down == true || right_down==true {
            let (x, y) = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
            let radius_squared = PARTICLE_SIZE*10; // Change this to adjust the radius of the circle

            for i in (x - radius_squared)..=(x + radius_squared) {
                for j in (y - radius_squared)..=(y + radius_squared) {

                        // Ensure the cell is within the grid bounds
                        if i > 0 && i < grid.width - PARTICLE_SIZE && j > 0 && j < grid.height - PARTICLE_SIZE {
                            grid.cells[i as usize][j as usize].user_data = spawn_user_data;
                        }
                    
                }
            }

        
        }


        draw_grid(&mut canvas, &mut grid);

        grid.update_grid();
        // Calculate FPS
        frame_count += 1;
        let now = Instant::now();
        let elapsed_time = now.duration_since(last_fps_print_time);

        if elapsed_time.as_secs_f64() >= 1.0 {
            let fps = frame_count as f64 / elapsed_time.as_secs_f64();
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            last_fps_print_time = now;
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 165));
    };
}

