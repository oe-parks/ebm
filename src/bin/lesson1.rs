fn main() {
    let x = 5.0;
    println!("x is {}", x);
    println!("{}", square(3.0));
    println!("{}", energy(2.0));
}

fn square(x: f64) -> f64 {
    x * x
}

fn energy(x: f64) -> f64 {
    0.5 * x * x
}

