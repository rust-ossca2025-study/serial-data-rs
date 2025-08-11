use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{stdin, stdout, Write};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SerialError {
    InvalidLength { expected: usize, actual: usize },
    InvalidFormat { field: String, reason: String },
    InvalidDate { year: u32, month: u32, day: u32, reason: String },
    InvalidCustomerType { value: String },
    ParseError { field: String, value: String },
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialError::InvalidLength { expected, actual } => {
                write!(f, "length must be {} but got {}", expected, actual)
            }
            SerialError::InvalidFormat { field, reason } => {
                write!(f, "{} format error: {}", field, reason)
            }
            SerialError::InvalidDate { year, month, day, reason } => {
                write!(f, "invalid date {}-{:02}-{:02}: {}", year, month, day, reason)
            }
            SerialError::InvalidCustomerType { value } => {
                write!(f, "invalid customer type '{}': must be one of 1/2/3", value)
            }
            SerialError::ParseError { field, value } => {
                write!(f, "{} parse error: cannot parse '{}'", field, value)
            }
        }
    }
}

impl std::error::Error for SerialError {}

impl From<String> for SerialError {
    fn from(s: String) -> Self {
        SerialError::InvalidFormat { 
            field: "unknown".to_string(),
            reason: s
        }
    }
}

impl From<&str> for SerialError {
    fn from(s: &str) -> Self {
        SerialError::InvalidFormat { 
            field: "unknown".to_string(),
            reason: s.to_string()
        }
    }
}

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


trait GenSerialData {
    fn length(&self) -> usize;
    fn name(&self) -> &str;
    fn serialize(&self) -> String;
    fn put_rawdata(&mut self, raw: &str) -> Result<(), String>;

    fn get_input_from_user(&mut self) {
        loop {
            println!(
                "Please input {}-digits for {}:",
                self.length(),
                self.name()
            );
            let input = get_user_input();
            if input.len() != self.length() {
                println!(
                    "Invalid length: expected {}, got {}",
                    self.length(),
                    input.len()
                );
                continue;
            }
            match self.put_rawdata(&input) {
                Ok(_) => break,
                Err(e) => println!("Invalid value: {}", e),
            }
        }
    }

    fn verify(&self, segment: &str) -> bool {
        segment.len() == self.length() && segment == self.serialize()
    }
}

struct CustomerId {
    raw: Option<String>,
    len: usize,
}

impl CustomerId {
    fn new(len: usize) -> Self {
        Self { raw: None, len }
    }
}

impl GenSerialData for CustomerId {
    fn length(&self) -> usize {
        self.len
    }
    fn name(&self) -> &str {
        "UserID"
    }
    fn serialize(&self) -> String {
        self.raw.clone().unwrap_or_default()
    }
    fn put_rawdata(&mut self, raw: &str) -> Result<(), String> {
        if raw.len() != self.len {
            return Err(format!("length must be {}", self.len));
        }
        self.raw = Some(raw.to_string());
        Ok(())
    }
}

struct ProductId {
    raw: Option<String>,
    len: usize,
}

impl ProductId {
    fn new(len: usize) -> Self {
        Self { raw: None, len }
    }
}

impl GenSerialData for ProductId {
    fn length(&self) -> usize {
        self.len
    }
    fn name(&self) -> &str {
        "ProductID"
    }
    fn serialize(&self) -> String {
        self.raw.clone().unwrap_or_default()
    }
    fn put_rawdata(&mut self, raw: &str) -> Result<(), String> {
        if raw.len() != self.len {
            return Err(format!("length must be {}", self.len));
        }
        self.raw = Some(raw.to_string());
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum CustomerKind {
    Business,
    Student,
    Company,
}

impl From<&CustomerKind> for usize {
    fn from(k: &CustomerKind) -> usize {
        match k {
            CustomerKind::Business => 1,
            CustomerKind::Student => 2,
            CustomerKind::Company => 3,
        }
    }
}

impl CustomerKind {
    fn from_digit_str(s: &str) -> Result<Self, String> {
        match s {
            "1" => Ok(CustomerKind::Business),
            "2" => Ok(CustomerKind::Student),
            "3" => Ok(CustomerKind::Company),
            _ => Err("must be one of 1/2/3".to_string()),
        }
    }
    fn as_digit_str(&self) -> String {
        let d: usize = self.into();
        d.to_string()
    }
}

struct CustomerType {
    kind: Option<CustomerKind>,
}

impl CustomerType {
    fn new() -> Self {
        Self { kind: None }
    }
}

impl GenSerialData for CustomerType {
    fn length(&self) -> usize {
        1
    }
    fn name(&self) -> &str {
        "CustomerType"
    }
    fn serialize(&self) -> String {
        self
            .kind
            .as_ref()
            .map(|k| k.as_digit_str())
            .unwrap_or_else(|| "0".to_string())
    }
    fn put_rawdata(&mut self, raw: &str) -> Result<(), String> {
        let k = CustomerKind::from_digit_str(raw)?;
        self.kind = Some(k);
        Ok(())
    }

    fn get_input_from_user(&mut self) {
        loop {
            println!(
                "Please input customer type: 1-{business}, 2-{student}, 3-{company}",
                business = format!("{:?}", CustomerKind::Business),
                student = format!("{:?}", CustomerKind::Student),
                company = format!("{:?}", CustomerKind::Company),
            );
            let input = get_user_input();
            if input.len() != self.length() {
                println!("Invalid length: expected 1, got {}", input.len());
                continue;
            }
            match self.put_rawdata(&input) {
                Ok(_) => break,
                Err(e) => println!("Invalid value: {}", e),
            }
        }
    }
}

struct ExpireDate {
    year: u32,
    month: u32,
    day: u32,
}

impl ExpireDate {
    fn new() -> Self {
        Self { year: 0, month: 0, day: 0 }
    }

    fn parse_fields(raw: &str) -> Result<(u32, u32, u32), String> {
        if raw.len() != 8 {
            return Err("length must be 8 (YYYYMMDD)".to_string());
        }
        let y: u32 = raw[0..4]
            .parse()
            .map_err(|_| "invalid year".to_string())?;
        let m: u32 = raw[4..6]
            .parse()
            .map_err(|_| "invalid month".to_string())?;
        let d: u32 = raw[6..8]
            .parse()
            .map_err(|_| "invalid day".to_string())?;

        if y < 2025 {
            return Err("year must be >= 2025".to_string());
        }
        if !(1..=12).contains(&m) {
            return Err("month must be 1..=12".to_string());
        }
        if !(1..=31).contains(&d) {
            return Err("day must be 1..=31".to_string());
        }
        Ok((y, m, d))
    }
}

impl GenSerialData for ExpireDate {
    fn length(&self) -> usize {
        8
    }
    fn name(&self) -> &str {
        "ExpireDate"
    }
    fn serialize(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month, self.day)
    }
    fn put_rawdata(&mut self, raw: &str) -> Result<(), String> {
        let (y, m, d) = Self::parse_fields(raw)?;
        self.year = y;
        self.month = m;
        self.day = d;
        Ok(())
    }

    fn get_input_from_user(&mut self) {
        loop {
            println!("Please input the expiration date (YYYYMMDD) (e.g. 20250123):");
            let input = get_user_input();
            match self.put_rawdata(&input) {
                Ok(_) => break,
                Err(e) => println!("Invalid date: {}", e),
            }
        }
    }
}

fn collect_data(items: &mut [Box<dyn GenSerialData>]) {
    for item in items.iter_mut() {
        item.get_input_from_user();
    }
}

fn generate_plain_serial(items: &mut [Box<dyn GenSerialData>]) -> String {
    let mut out = String::new();
    for item in items.iter_mut() {
        out.push_str(&item.serialize());
    }
    out
}

fn main() {
    println!("=== Serial Key Generator (More Plugins) ===");

    let user = CustomerId::new(4);
    let product = ProductId::new(8);
    let cust_type = CustomerType::new();
    let expire = ExpireDate::new();

    let mut items: Vec<Box<dyn GenSerialData>> =
        vec![Box::new(user), Box::new(product), Box::new(cust_type), Box::new(expire)];

    collect_data(&mut items);

    let plain_serial = generate_plain_serial(&mut items);
    println!("Plain serial: {}", plain_serial);

    let mc = new_magic_crypt!("magickey", 256);
    let encrypted = mc.encrypt_str_to_base64(&plain_serial);
    println!("Encrypted serial: {}", encrypted);

    let decrypted = mc
        .decrypt_base64_to_string(&encrypted)
        .expect("Failed to decrypt serial");
    println!("Decrypted serial: {}", decrypted);

    let mut offset = 0;
    for item in items.iter() {
        let len = item.length();
        let segment = &decrypted[offset..offset + len];
        println!("Verify {}: {} -> {}", item.name(), segment, item.verify(segment));
        offset += len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_type_roundtrip() {
        let mut ct = CustomerType::new();
        ct.put_rawdata("2").unwrap();
        assert_eq!(ct.serialize(), "2");
        assert!(ct.verify("2"));
        assert!(!ct.verify("1"));
    }

    #[test]
    fn test_expire_date_parse_and_serialize() {
        let mut ed = ExpireDate::new();
        ed.put_rawdata("20251231").unwrap();
        assert_eq!(ed.serialize(), "20251231");
        assert!(ed.verify("20251231"));
    }

    #[test]
    fn test_generate_plain_serial_with_plugins() {
        let mut user = CustomerId::new(4);
        let mut prod = ProductId::new(8);
        let mut ct = CustomerType::new();
        let mut ed = ExpireDate::new();

        user.put_rawdata("1234").unwrap();
        prod.put_rawdata("qwerasdf").unwrap();
        ct.put_rawdata("3").unwrap();
        ed.put_rawdata("20250123").unwrap();

        let mut items: Vec<Box<dyn GenSerialData>> =
            vec![Box::new(user), Box::new(prod), Box::new(ct), Box::new(ed)];
        let plain = generate_plain_serial(&mut items);
        assert_eq!(plain, "1234qwerasdf3".to_string() + "20250123");
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let mc = new_magic_crypt!("testkey", 256);
        let plain = "1234qwerasdf320250123";
        let enc = mc.encrypt_str_to_base64(plain);
        let dec = mc.decrypt_base64_to_string(&enc).unwrap();
        assert_eq!(dec, plain);
    }
} 