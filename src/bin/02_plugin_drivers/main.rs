use std::io::{stdin, stdout, Write};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

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

pub struct CustomerID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl CustomerID {
    pub fn new(digit: usize) -> Self {
        CustomerID {
            name: "UserID".to_owned(),
            digit,
            id: None,
        }
    }
}

pub struct ProductID {
    id: Option<String>,
    digit: usize,
    name: String,
}

impl ProductID {
    pub fn new(digit: usize) -> Self {
        ProductID {
            name: "ProductID".to_owned(),
            digit,
            id: None,
        }
    }
}

trait GenSerialData {
    fn get_input_from_user(&mut self) {
        let input: String;
        println!(
            "Please input {}-digits for {}: ",
            self.get_length(),
            self.get_name(),
        );
        input = get_user_input();
        assert_eq!(input.len(), self.get_length());
        self.put_rawdata(input);
    }


    fn verify(&mut self, data: &str) -> bool {
        self.get_length() == data.len() && self.get_rawdata() == data
    }

    fn get_length(&mut self) -> usize;
    fn get_rawdata(&self) -> String;
    fn get_name(&self) -> String;
    fn put_rawdata(&mut self, _data: String);
}

impl GenSerialData for CustomerID {
    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        self.id = Some(data);
    }
}

impl GenSerialData for ProductID {
    fn get_length(&mut self) -> usize {
        self.digit
    }

    fn get_rawdata(&self) -> String {
        self.id.clone().unwrap()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn put_rawdata(&mut self, data: String) {
        self.id = Some(data);
    }
}

fn collect_data(items: &mut Vec<Box<dyn GenSerialData>>) {
    for item in items.iter_mut() {
        item.get_input_from_user();
    }
}

fn generate_serial(items: &mut Vec<Box<dyn GenSerialData>>) -> String {
    let mut data = String::new();
    for item in items.iter_mut() {
        data.push_str(&item.get_rawdata());
    }
    data
}


fn main() {
    let productid = ProductID::new(8);
    let customerid = CustomerID::new(4);
    let mut items: Vec<Box<dyn GenSerialData>> = vec![Box::new(customerid), Box::new(productid)];

    collect_data(&mut items);
    let plain_serial = generate_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256); // AES256 알고리즘을 사용하는 MagicCrypt256타입의 객체 생성
    let serial = mc.encrypt_str_to_base64(&plain_serial); // 암호화 후 BASE64로 인코딩
    println!("Encrypted serial: {}", serial);

    let dec = mc.decrypt_base64_to_string(serial).unwrap(); // BASE64로 인코딩된 데이터를 디코딩 후 암호 해제
    println!("Decrypted serial: {}", dec);

    let mut offset = 0;
    for item in items.iter_mut() {
        let len = item.get_length();
        let rawdata = &dec[offset..offset + len];
        println!("Verify {}: {}", item.get_name(), rawdata);
        println!("Verify result: {}", item.verify(rawdata));
        offset += len;
    }
}
