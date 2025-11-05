use std::io;

fn main() {
    println!("Welcome to our restaurant 🤵🏽‍♂, here's our menu:");
        println!("Code : P | Food : Poundo Yam/Edinkaioko Soup 🍲 | Price : 3,200\nCode : F | Food : Fried Rice & Chicken 🥘 | Price : 3,000\nCode : A | Food : Amala & Ewedu Soup 🍮 | Price : 2,500\nCode : E | Food : Eba & Egusi Soup 🧈 | Price : 2,000\nCode : W | Food : White Rice and Stew 🍚 | Price : 2,500");


    loop {
        let mut am: f32;

        println!("🤵🏽‍♂️Would you like to make an order? (Say Yes or No)");
        let mut re = String::new();
        io::stdin().read_line(&mut re).expect("Invalid Input");
        let re: String = re.trim().parse().expect("Failed to read input");

        if re == "yes" {
            println!("🤵🏽‍♂️📝What would you like? (Use the code for the meal)");
            let mut code = String::new();
            io::stdin().read_line(&mut code).expect("Invalid input");
            let code: char = code.trim().parse().expect("Failed to read input");

            println!("🤵🏽‍♂️How many portions would you like?");
            let mut quant = String::new();
            io::stdin().read_line(&mut quant).expect("Invalid input");
            let quant: f32 = quant.trim().parse().expect("Failed to read input");

            if code == 'p' || code == 'P' {
                am = quant * 3200.0;
            } else if code == 'f' || code == 'F' {
                am = quant * 3000.0;
            } else if code == 'a' || code == 'A' {
                am = quant * 2500.0;
            } else if code == 'e' || code == 'E' {
                am = quant * 2000.0;
            } else if code == 'w' || code == 'W' {
                am = quant * 2500.0;
            } else {
                println!("Wrong input, try again!\n");
                continue;
            }

            println!("Your bill is {}. Thank you for your purchase, enjoy your meal 🙂\n",am );

            if am > 10000.0 {
                am *= 0.95;
                println!("You got a 5% discount 🎉. Pay {} at checkout 😏\n", am);
            }

        } else if re == "no" {
            break;
        } else {
            println!("Wrong input! Try again!");
        }
    }
}