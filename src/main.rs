mod grid;
mod logic;
mod materials;
mod api;
use grid::Grid;
use logic::IS_EMPTY;
use materials::Material;

use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use std::time::Duration;

pub const WINDOW_HEIGHT: i32 = 400;
pub const WINDOW_WIDTH: i32 = 400;

fn draw_grid(canvas: &mut Canvas<sdl2::video::Window>, grid: &mut Grid) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for i in 0..grid.rows{
        for j in 0..grid.cols{
            if grid.cells[i as usize][j as usize].material == Material::Sand{
                let rect = Rect::new(i as i32, j as i32, 5, 5);
                canvas.draw_rect(rect).expect("SadAD");
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let (mut x , mut y) = (0,0);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut grid = Grid::new(WINDOW_WIDTH,WINDOW_HEIGHT);
    
    let mut left_mouse_button_down = false;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown { mouse_btn,.. } => {
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
            (x , y) = (event_pump.mouse_state().x(), event_pump.mouse_state().y());
            
        }
        grid.cells[x as usize][y as usize].material = Material::Sand; 

        
        draw_grid(&mut canvas, &mut grid);
        grid.update_grid();
        
        
        // The rest of the game loop goes here...
        canvas.present();
       
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}