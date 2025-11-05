use std::io;

fn main() {
    let mut grosssalary: f32 = 0.0;

    loop {
        println!("\nDo you want to calculate your pay? (Type yes or no)");
        let mut reply = String::new();
        io::stdin().read_line(&mut reply).expect("Invalid input");
        let reply: String = reply.trim().parse().expect("Failed to read input");

        if reply == "yes" {
            println!("Input employee name");
            let mut emname = String::new();
            io::stdin().read_line(&mut emname).expect("Invalid input");
            let emname: String = emname.trim().parse().expect("Failed to read input");

            println!("Enter your hours");
            let mut hours = String::new();
            io::stdin().read_line(&mut hours).expect("Invalid input");
            let hours: f32 = hours.trim().parse().expect("Failed to read input");

            if hours <= 40.0 {
                grosssalary = 3000.0 * hours;
                println!("{}, your salary is {}", emname, grosssalary);
            }if hours > 40.0 {
                grosssalary = 4500.0 * hours;
                println!("{}, your salary is {}", emname, grosssalary);
            }if grosssalary > 100000.0{
                grosssalary-=2000.0;
                println!("After tax your gross salary is {}", grosssalary);
            }
        } else if reply == "no" {
            break;
        } else {
            println!("Wrong input! Try again!");
            continue;
        }
    }

    // println!("Do you want to calculate your pay? (Type yes or no)");
    // let mut reply = String::new();
    // io::stdin().read_line(&mut reply).expect("Invalid input");
    // let reply: String = reply.trim().parse().expect("Failed to read input");

    // println!("Input employee name");
    // let mut emname = String::new();
    // io::stdin().read_line(&mut emname).expect("Invalid input");
    // let emname: String = emname.trim().parse().expect("Failed to read input");

    // println!("Enter your hours");
    // let mut hours = String::new();
    // io::stdin().read_line(&mut hours).expect("Invalid input");
    // let hours: f32 = hours.trim().parse().expect("Failed to read input");



    // let mut grosssalary: f32 = 0.0;

    // loop {
    //     if reply == "yes" && hours <= 40.0{
    //         let rate = 3000.0;
    //         grosssalary = rate * hours;
    //         println!("{}, Your Salary is grosssalary {}", emname, grosssalary);
    //     }else if reply == "yes" && hours > 40.0{
    //         let rate = 4500.0;
    //         grosssalary = rate * hours;
    //         println!("{}, Your Salary is grosssalary {}",emname, grosssalary);
    //     }else if reply == "yes" && grosssalary > 100000.0{
    //         let rate = 3000.0;
    //         grosssalary = rate * hours;
    //         grosssalary -= 2000.0;
    //         println!("{}, Your grosssalary is {}", emname, grosssalary);
    //     }else if reply == "yes" && grosssalary > 100000.0{
    //         let rate = 4500.0;
    //         grosssalary = rate * hours;
    //         grosssalary -= 2000.0;
    //         println!("{}, Your grosssalary is {}", emname, grosssalary);
    //     } else if reply == "no"{
    //         break;
    //     }else {
    //         println!("You entered an invalid input, try again");
    //         continue;
    //     }


    // }
}