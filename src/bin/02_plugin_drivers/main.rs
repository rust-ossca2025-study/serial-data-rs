mod domain;

use crate::domain::{
    customer_id::CustomerID, customer_type::CustomerType, ext::GenSerialDataVecExt,
    product_id::ProductID, traits::GenSerialData,
};
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

fn main() {
    let productid = ProductID::new(8);
    let customerid = CustomerID::new(4);
    let customer_type = CustomerType::new();

    let mut items: Vec<Box<dyn GenSerialData>> =
        vec![Box::new(customerid), Box::new(productid), Box::new(customer_type)];

    items.collect_data();
    let plain_serial = items.generate_serial();

    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    items.validate_serialized_data(&dec);
}
