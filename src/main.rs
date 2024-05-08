
use sdl2::sys::Window;
use sdl2::{event::Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::Canvas;

mod materials;
mod logic;
use logic::{Grid, Simulator};

// Constants
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;
const CELL_SIZE: i32 = 5;

// Function to render the grid to the SDL2 window
fn render_grid(renderer: &mut Canvas<sdl2::video::Window>, grid: &Grid) {
    renderer.set_draw_color(Color::BLACK);
    renderer.clear();
    renderer.set_draw_color(Color::WHITE);

    for x in 0..grid.width() {
        for y in 0..grid.height() {
            let cell = grid.get_cell(x, y);
            if cell.material == materials::Material::Sand {
                renderer.set_draw_color(Color::RGB(cell.red, cell.green, cell.blue));
                renderer.fill_rect(sdl2::rect::Rect::new(
                    x * CELL_SIZE,
                    y * CELL_SIZE,
                    CELL_SIZE as u32,
                    CELL_SIZE as u32,
                )).expect("Failed to render cell");
            }
        }
    }

    renderer.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut renderer = window.into_canvas().accelerated().build().unwrap();

    let mut grid = Grid::new((WINDOW_WIDTH / CELL_SIZE) as i32, (WINDOW_HEIGHT / CELL_SIZE) as i32);

    // Main loop
    'mainloop: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'mainloop,
                _ => {}
            }
        }

        // Update the simulation
        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let cell = grid.get_cell(x, y);
                cell.update(Simulator {
                    x,
                    y,
                    grid: &mut grid,
                });
            }
        }

        // Render the grid
        render_grid(&mut renderer, &grid);

        // Add a small delay to control the frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}