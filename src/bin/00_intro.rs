use std::io::stdin;

fn get_user_input(digit: usize) -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");

    let trimmed = s.trim();

    // Check digit count
    if trimmed.len() != digit {
        eprintln!("Digit count mismatch: expected {} digits.", digit);
        std::process::exit(1);
    }

    // Ensure all characters are digits
    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        eprintln!("Input must consist only of digits.");
        std::process::exit(1);
    }

    trimmed.to_string()
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_user_input(4);

    println!("Please input 8-digits Product ID: ");
    let product_id = get_user_input(8);

    let plain_serial = format!("{}{}", customer_id, product_id);
    println!("Plain serial: {}", plain_serial);

    let verify_customer_id = &plain_serial[0..4];
    let verify_product_id = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customer_id);
    println!("Verify Product ID: {}", verify_product_id);
}
