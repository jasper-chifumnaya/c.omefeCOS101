use std::io;

fn main() {

    //Create co-efficients for a, b, c using quadractic equation formula x = -b ± (√b² -√4ac) / 2a

    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? Type Yes or No:");
    io::stdin().read_line(&mut experience).expect("Cannot read input");
    let experience: &str = experience.trim();

    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Cannot read input");
    let age:i32 = age.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

    //conditions
    if age >= 40 && experience == "yes" {
        println!("Your Incentive is ₦1,560,000 😁");
    } else if age >= 30 && experience == "yes"  {
        println!("Your Incentive is ₦1,480,000 😃");
    } else if age <=17 {
        println!("You are not legally allowed to work, you have no incentive ⛓️‍💥👮🏽‍♀️🤨");
    } else if age <= 28 && experience == "yes" {
        println!("Your Incentive is ₦1,300,000 😊");
    } else if experience == "no" {
        println!("You have no experience, lets start you on ₦100,000 🙄");
    }

}