use std::io;
use std::process;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { return a; }
    return gcd(b, a % b);
}

fn main() {
    println!("Please input two numeric.");
    let mut argv = String::new();
    io::stdin().read_line(&mut argv)
        .ok()
        .expect("Failed to read line");

    let numbers: Vec<&str> = argv.trim().split(' ').collect();
    match numbers.len() {
        2 => println!("Valid input."),
        _ => {
            println!("Invalid input.");
            process::exit(1); // todo
        }
    };
    let answer = gcd(numbers[0].parse::<i32>().unwrap(), numbers[1].parse::<i32>().unwrap());
    println!("Greatest common divisor => {}", answer);
}
