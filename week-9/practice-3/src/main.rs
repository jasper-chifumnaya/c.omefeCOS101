use std::fs;

fn main() {
    fs::remove_file("../practice-1/data.txt").expect("could not remove file");
    println!("file is removed");
}
