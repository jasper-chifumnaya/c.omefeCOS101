use std::io;

fn main() {
    println!("\nStudent Information Management System!");

    //input name
    println!("\nPlease enter your name");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("failed to read input");
    println!("Your name is: {}", name);

    //input age
     println!("\nPlease enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 = age.trim().parse().expect("input is not an integer");
        println!("Your name is: {}", age);

}


