use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
//use std::{thread, time};
use rayon::prelude::*;

const WIN_X: u32 = 700;
const WIN_Y: u32 = WIN_X;

fn main() {
    //Initialize window and settings
    let (mut canvas, mut event_pump) = init();
    let mut settings = Settings::new();
    let mut modif_flag = true;

    'running: loop {
        let event = event_pump.poll_event();
        match event {
            //Exit
            Some(Event::Quit { .. })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            }) => break 'running,

            //Print settings
            Some(Event::KeyDown {
                keycode: Some(Keycode::N),
                ..
            }) => {
                settings.print_settings();
                modif_flag = true;
            }

            //Switch Fractal
            Some(Event::KeyDown {
                keycode: Some(Keycode::Tab),
                ..
            }) => {
                settings.switch_fractal();
                modif_flag = true;
            }

            //Tweak Julia parameter
            Some(Event::KeyDown {
                keycode: Some(Keycode::T),
                ..
            }) => {
                settings.tweak();
                modif_flag = true;
            }

            // Right
            Some(Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            }) => {
                settings.right();
                modif_flag = true;
            }

            // Left
            Some(Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            }) => {
                settings.left();
                modif_flag = true;
            }

            // Up
            Some(Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            }) => {
                settings.down();
                modif_flag = true;
            }

            // Down
            Some(Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            })
            | Some(Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            }) => {
                settings.up();
                modif_flag = true;
            }

            // In zoom
            Some(Event::KeyDown {
                keycode: Some(Keycode::I),
                ..
            }) => {
                settings.zoom(2.);
                modif_flag = true;
            }

            // Out zoom
            Some(Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            }) => {
                settings.zoom(0.5);
                modif_flag = true;
            }

            // Upscale
            Some(Event::KeyDown {
                keycode: Some(Keycode::U),
                ..
            }) => {
                settings.upscale(50, settings.nb_divisions / 2);
                modif_flag = true;
            }

            // Downscale
            Some(Event::KeyDown {
                keycode: Some(Keycode::P),
                ..
            }) => {
                settings.downscale(50, settings.nb_divisions / 3);
                modif_flag = true;
            }

            // Small In zoom
            Some(Event::KeyDown {
                keycode: Some(Keycode::K),
                ..
            }) => {
                settings.zoom(1.1);
                modif_flag = true;
            }

            // Small Upscale
            Some(Event::KeyDown {
                keycode: Some(Keycode::J),
                ..
            }) => {
                settings.upscale(20, settings.nb_iterations / 5);
                modif_flag = true;
            }

            _ => {}
        }

        if modif_flag {
            modif_flag = false;
            draw(&mut canvas, &settings);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Fractal {
    Mandelbrot,
    Julia(num_complex::Complex<f64>),
    Newton(
        fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
        fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
        num_complex::Complex<f64>,
    ),
}

#[derive(Debug)]
struct Settings {
    c: num_complex::Complex<f64>,
    a: num_complex::Complex<f64>,
    f: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
    fp: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
    fractal: Fractal,
    size: f64,
    x_offset: f64,
    y_offset: f64,
    dx: f64,
    dy: f64,
    dc: f64,
    nb_iterations: u32,
    nb_divisions: u32,
    colors: Vec<[u8; 3]>,
    grad: Vec<[u8; 3]>,
}

impl Settings {
    fn new() -> Settings {
        let colors = vec![[0, 0, 0], /* [53, 12, 166], [250, 151, 22],*/ [255, 255, 255]];

        let grad = colorscale::multi_lin_grad(&colors, 100);
        fn default_f(z: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
            z.cos()
        }
        fn default_fp(z: num_complex::Complex<f64>) -> num_complex::Complex<f64> {
            -z.sin()
        }
        Settings {
            c: num_complex::Complex::new(-0.4, 0.6),
            a: num_complex::Complex::new(1., 0.),
            f: default_f,
            fp: default_fp,
            fractal: Fractal::Newton(default_f, default_fp, num_complex::Complex::new(1., 0.)),
            size: 4.0,
            x_offset: 0.,
            y_offset: 0.,
            dx: 0.7,
            dy: 0.7,
            dc: 0.1,
            nb_iterations: 100,
            nb_divisions: 100,
            colors,
            grad,
        }
    }

    // Get ownership of numbers for easier calculations
    fn get_values(
        &self,
    ) -> (
        num_complex::Complex<f64>,
        num_complex::Complex<f64>,
        fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
        fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
        f64,
        f64,
        f64,
        u32,
    ) {
        (
            self.c,
            self.a,
            self.f,
            self.fp,
            self.size,
            self.x_offset,
            self.y_offset,
            self.nb_iterations,
        )
    }

    fn print_settings(&self) {
        match self.fractal {
            Fractal::Mandelbrot => println!("============================ \n Fractal : Mandelbrot \n position : ({}, {}) \n image size : {}x{} \n ============================", self.x_offset, self.y_offset, self.size, self.size),
            Fractal::Julia(c) => println!( " ============================ \n Fractal : Julia \n c parameter : {} \n position : ({}, {}) \n image size : {}x{} \n ============================", c, self.x_offset, self.y_offset, self.size, self.size),
            Fractal::Newton(..) => println!( " ============================ \n Fractal : Newton \n position : ({}, {}) \n image size : {}x{} \n  ============================", self.x_offset, self.y_offset, self.size, self.size),
        };
    }

    // TODO: struct decomp

    fn switch_fractal(&mut self) {
        let c = self.c;
        let a = self.a;
        let f = self.f;
        let fp = self.fp;
        let fr = match self.fractal {
            Fractal::Mandelbrot => Fractal::Julia(self.c),
            Fractal::Julia(..) => Fractal::Newton(self.f, self.fp, self.a),
            Fractal::Newton(..) => Fractal::Mandelbrot,
        };

        *self = Settings::new();
        self.c = c;
        self.a = a;
        self.f = f;
        self.fp = fp;
        self.fractal = fr;
    }

    fn tweak(&mut self) {
        match self.fractal {
            Fractal::Mandelbrot => (),
            Fractal::Newton(..) => {
                self.a *= num_complex::Complex::from_polar(1., self.dc);
                self.fractal = Fractal::Newton(self.f, self.fp, self.a);
            }
            Fractal::Julia(_) => {
                self.c *= num_complex::Complex::from_polar(1., self.dc);
                self.fractal = Fractal::Julia(self.c);
            }
        }
    }

    fn zoom(&mut self, factor: f64) {
        let f = factor.recip();
        self.size *= f;
        self.dx *= f;
        self.dy *= f;
        self.dc *= f;
    }

    fn upscale(&mut self, iterations: u32, divisions: u32) {
        self.nb_iterations += iterations;
        self.nb_divisions += divisions;
        self.grad = colorscale::multi_lin_grad(&self.colors, self.nb_divisions);
    }

    fn downscale(&mut self, iterations: u32, divisions: u32) {
        self.nb_iterations -= iterations;
        self.nb_divisions -= divisions;
        self.grad = colorscale::multi_lin_grad(&self.colors, self.nb_divisions);
    }

    fn right(&mut self) {
        self.x_offset += self.dx
    }
    fn left(&mut self) {
        self.x_offset -= self.dx
    }
    fn up(&mut self) {
        self.y_offset += self.dy
    }
    fn down(&mut self) {
        self.y_offset -= self.dy
    }
}

fn draw(canvas: &mut WindowCanvas, settings: &Settings) {
    let Settings { grad, fractal, .. } = settings;
    let (c, a, f, fp, size, x_offset, y_offset, nb_iterations) = settings.get_values();

    canvas.clear();

    let mut pixels: Vec<_> = (0..WIN_X)
        .flat_map(|x| (0..WIN_Y).map(move |y| (x, y, Color::RGB(0, 0, 0))))
        .collect();

    pixels.par_iter_mut().for_each(|t| {
        let (x, y, color) = t;
        let re = x_offset + size * (*x as f64) / (WIN_X as f64) - size / 2.;
        let im = y_offset + size * (*y as f64) / (WIN_Y as f64) - size / 2.;
        //println!("({}, {})", re, im);

        let i = match fractal {
            Fractal::Mandelbrot => test_mandelbrot(re, im, nb_iterations),
            Fractal::Julia(..) => test_julia(re, im, c, nb_iterations),
            Fractal::Newton(..) => test_newton(re, im, f, fp, a, nb_iterations, 0.0001),
        };

        *color = if i as u32 == nb_iterations {
            Color::RGB(0, 0, 0)
        } else {
            let [r, g, b] = grad[i];
            Color::RGB(r, g, b)
        };
    });

    pixels.into_iter().for_each(|(x, y, color)| {
        canvas.set_draw_color(color);
        canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
    });

    canvas.present();
}

fn test_mandelbrot(re: f64, im: f64, iterations: u32) -> usize {
    let c = num_complex::Complex::new(re, im);
    let mut z = num_complex::Complex::new(0., 0.);
    let mut i = 0;

    while z.norm() < 2. && i < iterations {
        z = z * z + c;
        i += 1;
    }

    i as usize
}

fn test_julia(re: f64, im: f64, c: num_complex::Complex<f64>, iterations: u32) -> usize {
    let mut z = num_complex::Complex::new(re, im);
    let mut i = 0;

    while z.norm() < 2. && i < iterations {
        z = z * z + c;
        i += 1;
    }

    i as usize
}

fn test_newton(
    re: f64,
    im: f64,
    f: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
    fp: fn(num_complex::Complex<f64>) -> num_complex::Complex<f64>,
    a: num_complex::Complex<f64>,
    iterations: u32,
    eps: f64,
) -> usize {
    let mut z = num_complex::Complex::new(re, im);
    let mut i = 0;

    while f(z).norm() > eps && i < iterations {
        z = z - a * f(z) / fp(z);
        i += 1;
    }

    i as usize
}

fn init() -> (WindowCanvas, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mandelbrot", WIN_X as u32, WIN_Y as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let event_pump = sdl_context.event_pump().unwrap();

    let canvas = window.into_canvas().build().expect("could not make canvas");

    (canvas, event_pump)
}
