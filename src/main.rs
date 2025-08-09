use crate::{
    data::{CustomerID, CustomerType, ExpireDate, GenSerialData, ProductID},
    encrypt::{decrypt_serial, encrypt_serial},
};

mod data;
mod encrypt;
mod input;

fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input_from_user();
    }
}

fn main() {
    let product_id = ProductID::new(8);
    let customer_id = CustomerID::new(4);
    let expire_date = ExpireDate::new();
    let customer_type = CustomerType::new();

    let mut items: Vec<Box<dyn GenSerialData>> = vec![
        Box::new(customer_id),
        Box::new(product_id),
        Box::new(expire_date),
        Box::new(customer_type),
    ];

    collect_data(&mut items);

    let serial = encrypt_serial(&mut items);
    println!("Encrypted serial: {serial}");

    let decrypted_serial = decrypt_serial(serial, &mut items);
    for serial_data in decrypted_serial {
        println!("{}:{}", serial_data.name, serial_data.digit);
    }
}
