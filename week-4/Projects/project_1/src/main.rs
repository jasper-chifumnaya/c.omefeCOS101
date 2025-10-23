use std::io;

fn main() {

    //Create co-efficients for a, b, c using quadractic equation formula x = -b ± (√b² -√4ac) / 2a

    println!("Enter values for a, b, c");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

        io::stdin().read_line(&mut a).expect("Cannot read input");
        io::stdin().read_line(&mut b).expect("Cannot read input");
        io::stdin().read_line(&mut c).expect("Cannot read input");

     //Check if values were recieved

        let a:i32 = a.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");
        let b:i32 = b.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");
        let c:i32 = c.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

    //equation part
        let sqrb = b * b;
        let fac = 4 * (a * c);
        let minus = sqrb - fac;
        let rootpart = minus.isqrt();


        let firstx = (-b - rootpart) / 2 * a;
        let secondx = (-b + rootpart) / 2 * a;

    //print results
    println!("         ");
    println!("Your values are: {} {} and {} \n Get ready for the magic 🧙🏽‍♀️🪄", a, b, c);
    println!("         ");

    println!("🟢 x is: {}", firstx );

    println!("Or x is:");

    println!("🟢 x is: {}", secondx );

}





















