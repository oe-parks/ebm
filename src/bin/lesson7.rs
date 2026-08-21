use macroquad::prelude::*;
use macroquad::rand::gen_range;
use rand_distr::{Distribution, Normal};

struct Particle {
    x: f32,
    y: f32,
}

#[macroquad::main("EBM Training")]
async fn main() {
    // - - - - - - setup: runs ONCE - - - - - - - - 
    let mut rng = ::rand::rng();
    let normal = Normal::new(0.0f32, 1.0f32).unwrap();

    let step: f32 = 0.005;
    let noise_scale = (2.0 * step * 1.0).sqrt();
    
    // learnable parameters
    let mut mx: f32 = 0.0;
    let mut my: f32 = 0.0;
    let mut wx: f32 = 1.0;
    let mut wy: f32 = 1.0;
    let lr: f32 = 0.02;

    // the data - a cloud the current model (lesson 6) hasn't seen,
    // centered at (1.2, -0.8)
    let mut data: Vec<Particle> = Vec::new();
    for _ in 0..400 {
        data.push(Particle  {
            x: 1.2 + 0.45 * normal.sample(&mut rng),
            y: -0.8 + 0.25 * normal.sample(&mut rng),
        });
    }
    
    loop {
        clear_background(BLACK);

        let scale = screen_width() / 6.0;
        let cx = screen_width() / 2.0;
        let cy = scree_height() / 2.0;


    }



}
