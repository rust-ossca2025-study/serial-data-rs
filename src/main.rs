use data::{CUSTOMERID_LENGTH, PRODUCTID_LENGTH, SerialData, decrypt_serialdata};
use input::get_user_input;

mod data;
mod input;

fn get_input() -> SerialData {
    let customerid = get_user_input("Customer ID", CUSTOMERID_LENGTH);
    let productid = get_user_input("Product ID", PRODUCTID_LENGTH);

    SerialData {
        customerid,
        productid,
    }
}

fn main() {
    let input = get_input();
    println!("Plain serial: {}", input.concat());

    let serial = input.encrypt();
    println!("Encrypted serial: {serial}");

    let dec = decrypt_serialdata(serial);
    println!("Decrypted serial: {dec}");

    let serialdata = SerialData::from_string(dec);
    serialdata.print();
}
