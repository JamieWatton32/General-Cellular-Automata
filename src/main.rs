mod api;
mod grid;
mod logic;
mod materials;
use grid::Grid;
use logic::IS_EMPTY;
use std::env;

use materials::Material;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use std::time::Duration;

pub const WINDOW_HEIGHT: i32 = 720;
pub const WINDOW_WIDTH: i32 = 720;
pub const PARTICLE_SIZE:i32 = 5;
fn draw_grid(canvas: &mut Canvas<sdl2::video::Window>, grid: &mut Grid) {
    for i in 0..grid.width {
        for j in 0..grid.height {
            let rect = Rect::new(i as i32, j as i32, PARTICLE_SIZE as u32, PARTICLE_SIZE as u32);
            match grid.cells[(i * grid.width + j) as usize].material {
                Material::Sand => {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(rect).unwrap();
                }
                Material::Empty => {}
            }
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut left_mouse_button_down = false;
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        
        //grid.update_grid();
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
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        left_mouse_button_down = false;
                    }
                }
                _ => {}
            }
        }
        if left_mouse_button_down == true {
            let (x, y) = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
            let radius_squared = PARTICLE_SIZE*5; // Change this to adjust the radius of the circle
            
            for i in (x - radius_squared)..=(x + radius_squared) {
                for j in (y - radius_squared)..=(y + radius_squared) {
                    let dx = i - x;
                    let dy = j - y;
                    let distance_squared = dx * dx + dy * dy;

                    if distance_squared <= radius_squared {
                        // Ensure the cell is within the grid bounds
                        if i > 0 && i < grid.width - PARTICLE_SIZE && j > 0 && j < grid.height - PARTICLE_SIZE {
                            grid.cells[(i * grid.width + j) as usize].material = Material::Sand;
                        }
                    }
                }
            } 
        }


        draw_grid(&mut canvas, &mut grid);
        grid.update_grid();
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 180));
    }
}
