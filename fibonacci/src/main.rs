use std::f32;

fn main() {
    println!("Hello, world!");
    println!("Square root of five => {}", find_fibonacci_nth(9));
}

fn find_fibonacci_nth(n: i32) -> f32 {
    let sqrt_five = (5. as f32).sqrt();

    let first_parenthesis = (((1. + sqrt_five) / 2.) as f32).powi(n);
    let second_parenthesis = (((1. - sqrt_five) / 2.) as f32).powi(n);
    
    // 1./sqrt_five * (first_parenthesis * second_parenthesis)
    (1./sqrt_five * first_parenthesis) - (1./sqrt_five * second_parenthesis)
}
