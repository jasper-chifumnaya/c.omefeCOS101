use std::io;

fn read_float(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid number")
}

fn a_trapezium(h: f64, b1: f64, b2: f64) -> f64 {
    (h / 2.0) * (b1 + b2)
}

fn a_rhombus(d1: f64, d2: f64) -> f64 {
    (d1 * d2) / 2.0
}

fn a_parallelogram(b: f64, h: f64) -> f64 {
    b * h
}

fn a_cube(l: f64) -> f64 {
    6.0 * l * l
}

fn v_cylinder(r: f64, h: f64) -> f64 {
    (22.0 / 7.0) * r * r * h
}

fn handle_trapezium() {
    let h = read_float("Enter the height:");
    let b1 = read_float("Enter base 1:");
    let b2 = read_float("Enter base 2:");
    println!("Area of trapezium = {}", a_trapezium(h, b1, b2));
}

fn handle_rhombus() {
    let d1 = read_float("Enter diagonal 1:");
    let d2 = read_float("Enter diagonal 2:");
    println!("Area of rhombus = {}", a_rhombus(d1, d2));
}

fn handle_parallelogram() {
    let b = read_float("Enter base:");
    let h = read_float("Enter height:");
    println!("Area of parallelogram = {}", a_parallelogram(b, h));
}

fn handle_cube() {
    let l = read_float("Enter side length:");
    println!("Area of cube = {}", a_cube(l));
}

fn handle_cylinder() {
    let r = read_float("Enter radius:");
    let h = read_float("Enter height:");
    println!("Volume of cylinder = {}", v_cylinder(r, h));
}

fn main() {
    println!("Area of trapezium (t)");
    println!("Area of rhombus (r)");
    println!("Area of parallelogram (p)");
    println!("Area of cube (c)");
    println!("Volume of cylinder (v)
");
    println!("Which formula would you like to use?:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    let choice: char = choice.trim().parse().expect("Invalid character");

    match choice {
        't' => handle_trapezium(),
        'r' => handle_rhombus(),
        'p' => handle_parallelogram(),
        'c' => handle_cube(),
        'v' => handle_cylinder(),
        _ => println!("No option available for that."),
    }
}
