// use std::io::Read;
use std::io::Write;

fn main() {
    let serial_number = [
        "1",
        "2",
        "3",
        "4",
        "5",
    ];

    let commisioner = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministry = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zone = [
        "South West",
        "South East",
        "South South",
        "South West",
        "South East",
    ];

    // Build table header
    let mut content = String::new();    
    content.push_str("|                                     EFCC FILES                                  \n");
    content.push_str("| S/N   | Name of Commisioner        | Ministry             | Geopolitical Zone   \n");
    content.push_str("|-------|----------------------------|----------------------|---------------------\n");

    // Build table rows
    for i in 0..serial_number.len() {
        content.push_str(&format!(
            "| {:5} | {:26} | {:20} | {:19} |\n",
            serial_number[i], commisioner[i], ministry[i], zone[i]
        ));
    }

    // Create file and write to it
    let mut file = std::fs::File::create("Information Service Department.txt")
        .expect("Failed to create file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}