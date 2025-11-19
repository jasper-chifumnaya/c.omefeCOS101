fn main() {
    
    let b:(i32,bool,f64) = (30,true,4.9);
    print!("{:?}", b);
}


fn print(x:(i32,bool,f64)){

    println!("Inside print method");
    let (age,is_female,cgpa) = x;
    println!("Age is {}, is_female {}, cgpa is {}", age,is_female,cgpa);
}