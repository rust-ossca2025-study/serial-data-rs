fn get_input() -> String {
    let buf = &mut String::new();
    std::io::stdin()
        .read_line(buf)
        .expect("Failed to read line"); // TODO: write error handling

    // chomp NF and CR
    buf.trim_end().to_string()
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_input();

    println!("Please input 8-digits Product ID: ");
    let product_id = get_input();

    let plain_serial = format!("{}{}", customer_id, product_id);
    println!("Plain serial: {}", plain_serial);

    let verify_customer_id = &plain_serial[0..4];
    let verify_product_id = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customer_id);
    println!("Verify Product ID: {}", verify_product_id);
}
