//simple energy function
fn energy(x: f64) -> f64 {
    0.5 * x * x
}

fn main() {
    let xs = vec![-3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0];

    let mut unnormalized = Vec::new();
    for x in &xs {
        unnormalized.push((-energy(*x)).exp());
    }

    let mut z = 0.0;
    for u in &unnormalized {
        z += u;
    }

    for i in 0..xs.len() {
        let p = unnormalized[i] / z;
        println!("x = {:>5.1}   E = {:.3}   p = {:.4}", xs[i], energy(xs[i]),p);
    }
}



