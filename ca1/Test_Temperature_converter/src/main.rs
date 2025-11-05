use std::io;

fn main() {
    //collect input
    println!("Hello, please input temperature in celsius");
    let mut celtemp = String::new();
    io::stdin().read_line(&mut celtemp).expect("Invalid input");
    let celtemp: f32 = celtemp.trim().parse().expect("Failed to read input");

    //convert
    let celtofah = (9.0 / 5.0) * celtemp + 32.0;
    let celtokel = celtemp + 273.15;


    //display values and range
    if celtemp < -273.0 {
        println!("You have entered an Invalid input");
    }else if celtemp <= 0.0 {
        println!("Your temperature in celsius is {} ,it is Freezing \n This temperature in converted to fahrenheit
        = {}\n This temperature in celsius converted to kelvin = {}", celtemp, celtofah, celtokel);
    }else if celtemp <= 30.0 {
        println!("Your temperature in celsius is {} ,it is Normal Range \n This temperature in converted to fahrenheit
        = {}\n This temperature in celsius converted to kelvin = {}", celtemp, celtofah, celtokel);
    }else if celtemp > 30.0 {
        println!("Your temperature in celsius is {} ,it is a Hot Temperature \n This temperature in converted to fahrenheit
        = {}\n This temperature in celsius converted to kelvin = {}", celtemp, celtofah, celtokel);
    }
}
