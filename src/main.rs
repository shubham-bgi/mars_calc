use std::io;

fn main() {
    println!("Enter your weight on Earth (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let earth_weight: f32 = input.trim().parse().unwrap();
    let mars_weight: f32 = calculate_weight_on_mars(earth_weight);
    println!("Weight on mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(earth_weight: f32) -> f32 {
    let mars_gravity: f32 = 3.711;
    let earth_gravity: f32 = 9.81;
    earth_weight * mars_gravity / earth_gravity
}
