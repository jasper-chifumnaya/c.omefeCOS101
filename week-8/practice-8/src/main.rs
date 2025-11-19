fn main() {
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 6993);

    println!("Original tuple = {:?}", mountain_heights);

    mountain_heights.2 = "Kilimanjaro";
    mountain_heights.3 = 7000;

    println!("Changed tuple = {:?}", mountain_heights);
}
