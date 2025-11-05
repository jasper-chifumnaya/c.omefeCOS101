fn main(){
    let fullname = "Pan-Athlantic University";
    println!();
    println!("Nmae: {}", fullname);
    println!();
    println!("Before trim ");
    println!("length is {}",fullname.len());
    println!();
    println!("After trim ");
    println!("length is {}", fullname.trim().len());
}