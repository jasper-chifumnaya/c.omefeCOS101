fn main() {
   
let name = vec!["Mary","Sam", "May","Sun","Mark","Sally", "Joy","Flo",];

let age = vec![16,17,19,22,20,21,18,23];

println!("\nAge allocation:\n");

for i in 0..age.len(){
    println!("{} is {} years old\n",name[i],age[i] );
}

}
