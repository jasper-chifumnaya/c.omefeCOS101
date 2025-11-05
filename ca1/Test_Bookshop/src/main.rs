use std::io;

fn main() {
    println!("Here's our menu");
    println!("Code : R | Book Title : Rust for Beginners | Price : 15000\nCode : A | Book Title : AI Basics | Price : 12500\nCode : D | Book Title : Data Structures in Rust | Price : 20000\nCode : N | Book Title : Networking Essentials | Price : 18000");

    loop {
        let mut am: f32;

        println!("Do you want to buy a book? Yes or No");
        let mut re = String::new();
        io::stdin().read_line(&mut re).expect("Invalid input");
        let re: String = re.trim().parse().expect("Failed to read input");

        if re == "yes" {
            println!("Book Code");
            let mut code = String::new();
            io::stdin().read_line(&mut code).expect("Invalid input");
            let code: char = code.trim().parse().expect("Failed to read input");

            println!("Enter Quantity");
            let mut quant = String::new();
            io::stdin().read_line(&mut quant).expect("Invalid input");
            let quant: f32 = quant.trim().parse().expect("Failed to read input");

            if code == 'r' || code == 'R' {
                am = quant * 15000.0;
            } else if code == 'a' || code == 'A' {
                am = quant * 12500.0;
            } else if code == 'd' || code == 'D' {
                am = quant * 20000.0;
            } else if code == 'n' || code == 'N' {
                am = quant * 18000.0;
            } else {
                println!("Wrong input, try again!");
                continue;
            }

            if am > 100000.0 {
                am *= 0.9;
            }

            println!("Pay {} at checkout 😏", am);
        } else if re == "no" {
            break;
        } else {
            println!("Wrong input! Try again!");
        }
    }

    // println!("Do you want to buy a book? Yes or No");
    // let mut re = String::new();
    // io::stdin().read_line(&mut re).expect("Invalid input");
    // let re: String = re.trim().parse().expect("Failed to read input");

    // println!("Book Code");
    // let mut code = String::new();
    // io::stdin().read_line(&mut code).expect("Invalid input");
    // let code: char = code.trim().parse().expect("Failed to read input");

    // println!("Enter Quantity");
    // let mut quant = String::new();
    // io::stdin().read_line(&mut quant).expect("Invalid input");
    // let quant: u32 = quant.trim().parse().expect("Failed to read input");


    // loop {
    //     if re == "no"{
    //         break;
    //     }else if re == "yes" && code == 'r' {
    //         let amount = quant * 15000;
    //         println!("Pay {}", amount);
    //     }else if re == "yes" && code == 'a'{
    //         let am = quant * 12500;
    //         println!("Pay {}", am);
    //     }else if re == "yes" && code == 'd'{
    //         let amo = quant * 20000;
    //         println!("Pay {}", amo);
    //     }else if re == "yes" && code == 'n'{
    //         let amou = quant * 18000;
    //         println!("Pay {}", amou);
    //     }else{
    //         println!("Invalid Input, try again");
    //         continue;
    //     }
    // }
}
