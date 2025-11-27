// use std::io::Read;
use std::io::Write;

fn main() {
    let student_name = [
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric_number = [
        "ACC10211",
        "ECO10110",
        "CSC10328",
        "EEE110202",
        "MEE102020",
    ];

    let department = [
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let level = [
        "300",
        "100",
        "200",
        "200",
        "100",
    ];

    // Build table header
    let mut content = String::new();    
    content.push_str("|                                  PAU SMIS                          \n");
    content.push_str("| Student Name       | Matric. Number      | Department      | Level\n");
    content.push_str("|--------------------|---------------------|-----------------|---------------\n");

    // Build table rows
    for i in 0..student_name.len() {
        content.push_str(&format!(
            "| {:18} | {:19} | {:15} | {:13} |\n",
            student_name[i], matric_number[i], department[i], level[i]
        ));
    }

    // Create file and write to it
    let mut file = std::fs::File::create("Student_Management_Information_System.txt")
        .expect("Failed to create file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}