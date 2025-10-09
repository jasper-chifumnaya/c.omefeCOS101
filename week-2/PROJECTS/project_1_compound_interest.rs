fn main() {
    // Given values
    let principal: f64 = 520_000_000.0; // ₦520,000,000
    let rate: f64 = 10.0;               // 10%
    let years: u32 = 5;                 // 5 years

    // Calculate Amount
    let amount = principal * (1.0 + rate / 100.0).powi(years as i32);

    // Calculate Compound Interest
    let compound_interest = amount - principal;

    // Display results
    println!("🪙🪙🪙🪙🪙🪙🪙🪙🪙🪙🪙");
    println!("Principal (₦): {}", principal);
    println!("                           ");

    println!("💹💹💹💹💹💹💹💹💹💹💹");
    println!("Rate (%): {}", rate);
    println!("                           ");

    println!("⌚⌚⌚⌚⌚⌚⌚⌚⌚⌚⌚");
    println!("Time (years): {}", years);
    println!("                           ");

    println!("💰💰💰💰💰💰💰💰💰💰💰");
    println!("Total Amount (₦): {:.2}", amount);
    println!("                           ");
    
    println!("💸💸💸💸💸💸💸💸💸💸💸");
    println!("Compound Interest (₦): {:.2}", compound_interest);
}
