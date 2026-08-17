//simple gradient descent
fn main() {
    let mut x = 5.0;
    let learning_rate = 0.1;
    
    for step in 0..20 {
        let g = gradient(x);
        x = x - learning_rate * g;
        println!("step {}: x = {:.4}, energy = {:.4}", step, x, energy(x));
    }
}

//energy function
fn energy(x: f64) -> f64 {
    0.5 * x * x
}


//slope of the function or gradient
fn gradient(x: f64) -> f64 {
    x
}


