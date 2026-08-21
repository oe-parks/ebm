//imports everything from macroquad (* = everything)
use macroquad::prelude::*;
//imports specific functioin from macroquad
use macroquad::rand::gen_range;
use rand_distr::{Distribution, Normal};

//Makes the energy landscape with two valleys at x = -1 and x = +1, one valley in y at 0.
fn energy(x: f32, y: f32) -> f32 {
    0.8 * (x * x - 1.0) * (x * x - 1.0) + 0.5 * y * y
}

//Slope in each directioin (derivations of 'energy')
fn grad_x(x: f32) -> f32 {
    3.2 * x * (x * x - 1.0)
}

fn grad_y(y: f32) -> f32 {
    y
}

struct Particle {
    x: f32,
    y: f32,
}


#[macroquad::main("EBM")]
async fn main() {
    let mut rng = ::rand::rng();
    let normal = Normal::new(0.0f32,1.0f32).unwrap();

    let step: f32 = 0.005;
    let temperature: f32 = 1.0;
    let noise_scale = (2.0 * step * temperature).sqrt();

    let mut particles: Vec<Particle> = Vec::new();
    for _ in 0..2000 {
        particles.push(Particle {
            x: gen_range(-2.5f32, 2.5f32),
            y: gen_range(-2.5f32, 2.5f32),
        });
    }

    loop {
        clear_background(BLACK);
        
        let scale = screen_width() / 6.0;
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        for p in particles.iter_mut() {
            p.x += -step * grad_x(p.x) + noise_scale * normal.sample(&mut rng);
            p.y += -step * grad_y(p.y) + noise_scale * normal.sample(&mut rng);

            let sx = cx + p.x * scale;
            let sy = cy + p.y * scale;

            //color by energy: low = blue, high = white
            let e = energy(p.x, p.y).min(3.0) / 3.0;
            draw_circle(sx, sy, 1.5, Color::new(0.4 + e * 0.6, 0.6, 1.0, 0.6));
        }
        
        draw_text(
            &format!("particle: {}  T =  {:.2}", particles.len(),temperature),
            20.0, 30.0, 24.0, GRAY,
        );

        next_frame().await

    }
}

