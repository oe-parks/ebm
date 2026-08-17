//imports everything from macroquad (* = everything)
use macroquad::prelude::*;
//imports specific functioin from macroquad
use macroquad::rand::gen_range;
use rand_distr::{Distribution, Normal};

//defines a flake as two 32-bit floats
struct Flake {
    x: f32,
    y: f32,
}

//
#[macroquad::main("Snow")]
async fn main() {
    let mut rng = ::rand::rng();
    let normal = Normal::new(0.0f32,1.0f32).unwrap();

    let mut flakes: Vec<Flake> = Vec::new();
    for _ in 0..400 {
        flakes.push(Flake {
            x: gen_range(0.0, screen_width()),
            y: gen_range(0.0, screen_height()),
        });
    }

    loop {
        clear_background(BLACK);

        for f in flakes.iter_mut() {
            f.y += 1.0;
            f.x += 1.5 * normal.sample(&mut rng);
        
            if f.y > screen_height() {
                f.y = 0.0;
                f.x = gen_range(0.0, screen_width());
            }

            draw_circle(f.x, f.y, 2.0, WHITE);
        }
    
        next_frame().await
    }
}

