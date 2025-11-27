use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let table = "Lager\t\tStout\t\tNon-alcoholic\n33 Export\t\tLegend\t\tMaltina";

    let mut file = OpenOptions::new().append(true).open("../practice-2/welcome_message.txt").expect("cannot open file");
    file.write_all("\nHello Class".as_bytes()).expect("write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");

    println!("file append success");
}
