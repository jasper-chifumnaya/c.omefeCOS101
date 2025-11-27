// use std::io::Read;
use std::io::Write;

fn main() {
    let lager = [
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = [
        "Legend",
        "Turbo King",
        "Williams",
        "",
        "",
        "",
    ];

    let non_alcoholic = [
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
        "",
        "",
    ];

    // Build table header
    let mut content = String::new();
    content.push_str("| Lager       | Stout       | Non-Alcoholic |\n");
    content.push_str("|-------------|-------------|----------------|\n");

    // Build table rows
    for i in 0..lager.len() {
        content.push_str(&format!(
            "| {:11} | {:11} | {:14} |\n",
            lager[i], stout[i], non_alcoholic[i]
        ));
    }

    // Create file and write to it
    let mut file = std::fs::File::create("Nigerian_Breweries_Portfolio.txt")
        .expect("Failed to create file");

    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}