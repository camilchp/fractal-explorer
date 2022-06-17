use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::WindowCanvas;
use sdl2::rect::{Point};
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
//use std::{thread, time};
use num_complex;

const WIN_X : u32 = 500;
const WIN_Y : u32 = WIN_X;

fn main() {
    let (mut canvas, mut event_pump) = init();

    let mut size = 4.0;
    let mut nb_iterations = 90;
    let mut x_offset = 0.;
    let mut y_offset = 0.;
    let mut dx = 0.7;
    let mut dy = dx;

    let mut modif_flag = false;

    draw(&mut canvas, size, x_offset, y_offset, nb_iterations);

    'running:loop{
        let event = event_pump.poll_event();
            match event {
                
                Some(Event::Quit {..}) | Some(Event::KeyDown {
                    keycode : Some(Keycode::Escape), ..
                }) => {break 'running},

                Some(Event::KeyDown {
                    keycode : Some(Keycode::D), ..
                }) => {
                    x_offset += dx;
                    modif_flag = true;
                },

                Some(Event::KeyDown {
                    keycode : Some(Keycode::Q), ..
                }) => {
                    x_offset -= dx;
                    modif_flag = true;
                },

                Some(Event::KeyDown {
                    keycode : Some(Keycode::Z), ..
                }) => {
                    y_offset -= dy;
                    modif_flag = true;
                },

                Some(Event::KeyDown {
                    keycode : Some(Keycode::S), ..
                }) => {
                    y_offset += dy;
                    modif_flag = true;
                },

                // Zoom In
                Some(Event::KeyDown {
                    keycode : Some(Keycode::F), ..
                }) => {
                    size *= 0.9;
                    dx *= 0.9;
                    dy *= 0.9;
                    modif_flag = true;
                },

                // Megazoom, InfiniiVision !!!
                Some(Event::KeyDown {
                    keycode : Some(Keycode::G), ..
                }) => {
                    size *= 0.5;
                    dx *= 0.5;
                    dy *= 0.5;
                    modif_flag = true;
                },

                // Upscale
                Some(Event::KeyDown {
                    keycode : Some(Keycode::U), ..
                }) => {
                    nb_iterations += 10;
                    modif_flag = true;
                },


                // Mega Upscale
                Some(Event::KeyDown {
                    keycode : Some(Keycode::M), ..
                }) => {
                    nb_iterations += 50;
                    modif_flag = true;
                },

                // Downscale
                Some(Event::KeyDown {
                    keycode : Some(Keycode::J), ..
                }) => {
                    nb_iterations -= 20;
                    modif_flag = true;
                },
                _ => {}
            }

        if modif_flag {

            modif_flag = false;
            draw(&mut canvas, size, x_offset, y_offset, nb_iterations);
        }
    }
}

fn draw(canvas : &mut WindowCanvas, size : f32, x_offset : f32, y_offset : f32, nb_iterations : u32) {
    
    canvas.clear();

    for x in 0..WIN_X {
        for y in 0..WIN_Y {
            let re = x_offset + size * (x as f32)/(WIN_X as f32) - size/2.;
            let im = y_offset + size * (y as f32)/(WIN_Y as f32) - size/2.;
            //println!("({}, {})", re, im);

            canvas.set_draw_color(test_point(re, im, nb_iterations));
            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
        }
    }

    canvas.present();
}

fn test_point(re : f32, im : f32, iterations : u32) -> Color {

    let c = num_complex::Complex::new(re, im);
    let mut z = num_complex::Complex::new(0., 0.);
    let mut i = 0;

    while z.norm() < 2. && i <= iterations {
            z = z*z + c;
            i += 1
    }
    if i < iterations {
        Color::RGB(0, 0, 0)
    } else {
        Color::RGB(255, 255, 255)
    }
}

fn init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Mandlebrot", WIN_X as u32, WIN_Y as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let event_pump = sdl_context.event_pump().unwrap();

    let canvas = window.into_canvas().build().expect("could not make canvas");

    (canvas, event_pump)
}