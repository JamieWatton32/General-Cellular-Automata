use materials::Material;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::sys::drand48;
use std::cell;
use std::time::Duration;

mod logic;
mod materials;
use logic::{Cell, Grid, Simulator};

// Constants
const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;
const CELL_SIZE: i32 = 5;

// Function to render the grid to the SDL2 window
fn render_grid(canvas: &mut Canvas<sdl2::video::Window>, grid: &Grid, y:i32) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::WHITE);

    for x in 0..grid.width(){

            let curr = grid.cells[x as usize];
            if curr.material == Material::Sand{
                println!("{:?}",curr);
                canvas.set_draw_color(Color::RGB(curr.red, curr.green, curr.blue));
                canvas.fill_rect(sdl2::rect::Rect::new(
                    x ,
                    300,
                    CELL_SIZE as u32,
                    CELL_SIZE as u32,
                )).expect("Failed to render cell");
            
        }
       
    }
    canvas.present();

}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Falling Sand", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut grid = Grid::new((WINDOW_WIDTH/CELL_SIZE as u32) as i32, (WINDOW_HEIGHT/CELL_SIZE as u32) as i32);

    let mut event_pump = sdl_context.event_pump()?;
    let mut left_mouse_button_down = false;
    // Main loop
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,

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
        
        if left_mouse_button_down == true{
            let (x, y) = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
            grid.cells[x as usize].red = 250;
            grid.cells[x as usize].green = 240;
            grid.cells[x as usize].blue = 230;
            grid.cells[x as usize].material = Material::Sand;
            
        }
        render_grid(&mut canvas, &grid); 

        // Add a small delay to control the frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
    
    Ok(())

}
