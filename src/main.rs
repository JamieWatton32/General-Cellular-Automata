mod api;
mod grid;
mod logic;
mod materials;
use grid::Grid;

use materials::Material;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use std::time::Duration;

pub const WINDOW_HEIGHT: i32 = 720;
pub const WINDOW_WIDTH: i32 = 1280;

fn draw_grid(canvas: &mut Canvas<sdl2::video::Window>, grid: &mut Grid) {
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            match grid.cells[i as usize][j as usize].material {
                Material::Sand => {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    let rect = Rect::new(i as i32, j as i32, 1, 1);
                    canvas.draw_rect(rect).expect("SadAD");
                    canvas.fill_rect(rect).unwrap();
                }
                Material::Empty => {}
                //canvas.present();
            }
        }
    }
}

fn main() {
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
    //let (mut x , mut y) = (0,0);
    //
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    //grid.cells[x as usize][y as usize].material = Material::Empty;
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

            // Iterate over a square area with side length 11 around (x, y)
            for i in (x )..=(x + 1) {
                for j in (y )..=(y + 1) {
                    // Check if the current coordinates are within the bounds of the grid
                    if i >= 0
                        && i < grid.cells.len() as i32
                        && j >= 0
                        && j < grid.cells[0].len() as i32
                    {
                        // Set the material to sand for the cells within the radius
                        grid.cells[i as usize][j as usize].material = Material::Sand;
                    }
                }
            }
        }
        draw_grid(&mut canvas, &mut grid);
        grid.update_grid();

        // The rest of the game loop goes here...
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
