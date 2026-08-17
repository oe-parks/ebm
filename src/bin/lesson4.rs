//simple ebm with random component
use rand_distr::{Distribution, Normal};

fn energy(x: f64) -> f64 {
    0.5 * x * x
}

fn gradient(x: f64) -> f64 {
    x
}

fn main() {
    let mut rng = rand::rng();
    let normal = Normal::new(0.0,1.0).unwrap();

    let step_size: f64 = 0.1;
    let temperature: f64 = 1.0;
    let noise_scale = (2.0 * step_size * temperature).sqrt();

    let mut x = 5.0;

    for step in 0..50 {
        let noise = normal.sample(&mut rng);
        x = x - step_size * gradient(x) + noise_scale * noise;
        println!("step {:>2}: x = {:>7.4}, E = {:.4}", step, x, energy(x));
    }
}




