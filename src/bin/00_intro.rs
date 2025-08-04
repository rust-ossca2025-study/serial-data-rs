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
fn validate_input(customer_id: &str, product_id: &str) -> Result<(), String> {
    // 길이 검사
    if customer_id.len() != 4 {
        return Err(format!("Customer ID must be 4 digits, got {} digits", customer_id.len()));
    }
    if product_id.len() != 8 {
        return Err(format!("Product ID must be 8 digits, got {} digits", product_id.len()));
    }
    
    // 빈 문자열 검사
    if customer_id.trim().is_empty() {
        return Err("Customer ID cannot be empty or whitespace only".to_string());
    }
    if product_id.trim().is_empty() {
        return Err("Product ID cannot be empty or whitespace only".to_string());
    }
    
    // 특수문자 검사 (고객 ID는 숫자만 허용)
    if !customer_id.chars().all(|c| c.is_ascii_digit()) {
        return Err("Customer ID must contain only digits (0-9)".to_string());
    }
    
    // 제품 ID는 알파벳과 숫자만 허용
    if !product_id.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err("Product ID must contain only alphanumeric characters (A-Z, a-z, 0-9)".to_string());
    }
    
    Ok(())
}

fn main() {
    println!("=== Serial Key Generator ===");
    
    // 사용자 입력 받기
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_user_input();
    println!("Please input 8-digits Product ID: ");
    let product_id = get_user_input();
    
    // 입력 유효성 검사
    match validate_input(&customer_id, &product_id) {
        Ok(()) => {
            // 유효성 검사 통과 -> 계속 진행
        }
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
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