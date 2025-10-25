use std::io;

fn main (){
    
    println!("Enter value for a");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Cannot read Input");
    let a:f64 = a.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

    println!("Enter value for b");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Cannot read Input");
    let b:f64 = b.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

    println!("Enter value for c");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Cannot read Input");
    let c:f64 = c.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

//Compute and show the user value of discrimant
    let discriminant = b.powf(2.0) - 4.0 * a * c;

    println!("The discriminant is: {}", discriminant );

//Calculation for if discrimant is positive
    if discriminant > 0.0 {
        let firstx = (-b + discriminant.sqrt()) / (2.0 * a);
        let secondx = (-b - discriminant.sqrt()) / (2.0 * a);

        println!("Discriminant is greater than 0, therefore x has two roots: x = {} and x = {}", firstx, secondx);
    } else if discriminant == 0.0 {
        let x = -b / (2.0 * a);

        println!("Discriminant is 0, therefore x has one root\nx = {}", x);
    } else if discriminant < 0.0 {
        let rx = -b / (2.0 * a);
        let ix = (-discriminant).sqrt() / (2.0 * a);

        println!("Discriminant is less than 0, therefore it has no real root\nroot = {} + {}i \nimaginary root = {} + {}i", rx, ix, rx, ix);
    }
}

// use std::io;

// fn main() {

//     //Create co-efficients for a, b, c using quadractic equation formula x = -b ± (√b² -√4ac) / 2a

//     println!("Enter values for a, b, c");
//     let mut a = String::new();
//     let mut b = String::new();
//     let mut c = String::new();

//         io::stdin().read_line(&mut a).expect("Cannot read input");
//         io::stdin().read_line(&mut b).expect("Cannot read input");
//         io::stdin().read_line(&mut c).expect("Cannot read input");

//      //Check if values were recieved

//         let a:f64 = a.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");
//         let b:f64 = b.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");
//         let c:f64 = c.trim().parse().expect("Failed to read input, check if you inputed a fraction or letter");

//     //equation part
//         let sqrb = b * b;
//         let fac = 4 * (a * c);
//         let minus = sqrb - fac;
//         let rootpart = minus.isqrt();


//         let firstx = (-b - rootpart) / 2 * a;
//         let secondx = (-b + rootpart) / 2 * a;

//     //print results
//     println!("         ");
//     println!("Your values are: {} {} and {} \n Get ready for the magic 🧙🏽‍♀️🪄", a, b, c);
//     println!("         ");

//     println!("🟢 x is: {}", firstx );

//     println!("Or x is:");

//     println!("🟢 x is: {}", secondx );

// }