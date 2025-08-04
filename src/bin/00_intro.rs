use std::io::{stdin, stdout, Write};

/// 입력 함수
fn get_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

/// 시리얼 키 생성 함수
fn generate_serial(customer_id: &str, product_id: &str) -> String {
    format!("{}{}", customer_id, product_id)
}

/// 유효성 검사 함수
fn validate_input(customer_id: &str, product_id: &str) -> bool {
    customer_id.len() == 4 && product_id.len() == 8
}

fn main() {
    println!("=== Serial Key Generator ===");
    
    // 사용자 입력 받기
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_user_input();
    println!("Please input 8-digits Product ID: ");
    let product_id = get_user_input();
    
    // 입력 유효성 검사
    if !validate_input(&customer_id, &product_id) {
        println!("Error: Customer ID must be 4 digits, Product ID must be 8 digits.");
        println!("Input values - Customer ID: {} ({} digits), Product ID: {} ({} digits)", 
                 customer_id, customer_id.len(), product_id, product_id.len());
        return;
    }
    
    // 시리얼 키 생성
    let serial = generate_serial(&customer_id, &product_id);
    println!("\n=== Generated Serial Key ===");
    println!("Serial Key: {}", serial);
    
    // 검증
    let verify_customerid = &serial[0..4];
    let verify_productid = &serial[4..12];
    println!("Verify Customer ID: {}", verify_customerid);
    println!("Verify Product ID: {}", verify_productid);
}
