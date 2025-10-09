fn main() {
    // Values
    let principal: f64 = 210_000.0; // ₦210,000
    let rate: f64 = 5.0;               // 5%
    let years: u32 = 3;                 // 3 years

    // Calculate Amount
    let amount = principal * (1.0 - rate / 100.0).powi(years as i32);

    // Calculate Compound Interest
    let compound_depreciation = amount - principal;

    // Display results
    println!("                           ");
    println!("🪙🪙🪙🪙🪙🪙🪙🪙🪙🪙🪙");
    println!("Principal (₦): {}", principal);
    println!("                           ");

    println!("📉📉📉📉📉📉📉📉📉📉📉");
    println!("Rate (%): {}", rate);
    println!("                           ");

    println!("⌚⌚⌚⌚⌚⌚⌚⌚⌚⌚⌚");
    println!("Time (years): {}", years);
    println!("                           ");

    println!("💰💰💰💰💰💰💰💰💰💰💰");
    println!("Total Amount (₦): {:.2}", amount);
    println!("                           ");

    println!("💳💳💳💳💳💳💳💳💳💳💳");
    println!("Depreciation (₦): {:.2}", compound_depreciation);
}