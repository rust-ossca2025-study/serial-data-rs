use std::error::Error;

type Validate = fn(&str) -> Result<String, Box<dyn Error>>;

fn get_input(validate: Validate) -> Result<String, Box<dyn Error>> {
    let buf = &mut String::new();
    std::io::stdin().read_line(buf)?;

    validate(buf.trim())?;

    Ok(buf.trim_end().to_string())
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_input(|s| {
        if s.len() != 4 || !s.chars().all(char::is_numeric) {
            return Err("Customer ID must be exactly 4 digits long.".into());
        }
        Ok(s.to_string())
    })
    .expect("Failed to read Customer ID");

    println!("Please input 8-alphanumeric Product ID: ");
    let product_id = get_input(|s| {
        if s.len() != 8 || !s.chars().all(|c| c.is_alphanumeric()) {
            return Err("Product ID must be exactly 8 alphanumeric characters.".into());
        }
        Ok(s.to_string())
    })
    .expect("Failed to read Product ID");

    let plain_serial = format!("{}{}", customer_id, product_id);
    println!("Plain serial: {}", plain_serial);

    let verify_customer_id = &plain_serial[0..4];
    let verify_product_id = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customer_id);
    println!("Verify Product ID: {}", verify_product_id);
}
