use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::io::{stdin, stdout, Write};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SerialError {
    InvalidLength { expected: usize, actual: usize },
    InvalidValue { field: String, reason: String },
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialError::InvalidLength { expected, actual } => {
                write!(f, "length must be {} but got {}", expected, actual)
            }
            SerialError::InvalidValue { field, reason } => {
                write!(f, "{}: {}", field, reason)
            }
        }
    }
}

impl std::error::Error for SerialError {}

impl From<String> for SerialError {
    fn from(s: String) -> Self {
        SerialError::InvalidValue { 
            field: "unknown".to_string(),
            reason: s
        }
    }
}

impl From<&str> for SerialError {
    fn from(s: &str) -> Self {
        SerialError::InvalidValue { 
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
    fn put_rawdata(&mut self, raw: &str) -> Result<(), SerialError>;

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
    fn put_rawdata(&mut self, raw: &str) -> Result<(), SerialError> {
        if raw.len() != self.len {
            return Err(SerialError::InvalidLength { 
                expected: self.len, 
                actual: raw.len() 
            });
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
    fn put_rawdata(&mut self, raw: &str) -> Result<(), SerialError> {
        if raw.len() != self.len {
            return Err(SerialError::InvalidLength { 
                expected: self.len, 
                actual: raw.len() 
            });
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
    fn from_digit_str(s: &str) -> Result<Self, SerialError> {
        match s {
            "1" => Ok(CustomerKind::Business),
            "2" => Ok(CustomerKind::Student),
            "3" => Ok(CustomerKind::Company),
            _ => Err(SerialError::InvalidValue { 
                field: "CustomerType".to_string(),
                reason: "must be one of 1/2/3".to_string()
            }),
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
    fn put_rawdata(&mut self, raw: &str) -> Result<(), SerialError> {
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

    fn parse_fields(raw: &str) -> Result<(u32, u32, u32), SerialError> {
        if raw.len() != 8 {
            return Err(SerialError::InvalidLength { 
                expected: 8, 
                actual: raw.len() 
            });
        }
        let y: u32 = raw[0..4]
            .parse()
            .map_err(|_| SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "invalid year".to_string()
            })?;
        let m: u32 = raw[4..6]
            .parse()
            .map_err(|_| SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "invalid month".to_string()
            })?;
        let d: u32 = raw[6..8]
            .parse()
            .map_err(|_| SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "invalid day".to_string()
            })?;

        if y < 2025 {
            return Err(SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "year must be >= 2025".to_string()
            });
        }
        if !(1..=12).contains(&m) {
            return Err(SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "month must be 1..=12".to_string()
            });
        }
        if !(1..=31).contains(&d) {
            return Err(SerialError::InvalidValue { 
                field: "ExpireDate".to_string(),
                reason: "day must be 1..=31".to_string()
            });
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
    fn put_rawdata(&mut self, raw: &str) -> Result<(), SerialError> {
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