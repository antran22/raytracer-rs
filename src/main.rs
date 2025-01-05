use std::io::{self, Write};

use vec3::Color;
mod ray;
mod vec3;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (400 as f64 / ASPECT_RATIO) as i32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

fn main() {
    print!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n");
    let stdout = &mut io::stdout();
    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {}", IMAGE_HEIGHT - j);
        io::stderr().flush().expect("Unable to flush stderr");
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH- 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let color = Color::val(r, g, 0.0);

            color.print_color(stdout).expect("cannot printout color");
        }
    }
}
