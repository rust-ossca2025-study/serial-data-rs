## 1. 모듈화

- **data**
    - `customer_id.rs`
    - `customer_kind.rs`
    - `expire_date.rs`
    - `gen_serial_data.rs`
    - `product_id.rs`
- **encrypt**
- **input**

**원본**: 
- plain to encrypt, decrypt and debugging 처럼 통합될 수 있는 과정들이 통합되지 않았어요. 

```rust
let plain_serial = generate_serial(&mut items);
println!("Plain serial: {}", plain_serial);

let mc = new_magic_crypt!("magickey", 256);
let serial = mc.encrypt_str_to_base64(&plain_serial);

let dec = mc.decrypt_base64_to_string(serial).unwrap();
println!("Decrypted serial: {}", dec);

let mut offset = 0;
for item in items.iter_mut() {
    let len = item.get_length();
    let rawdata = &dec[offset..offset + len];
    println!("Verify {}: {}", item.get_name(), rawdata);
    println!("Verify result: {}", item.verify(rawdata));
    offset += len;
}
```

**개선**: 
- encrypt_serial 에서 plain text 를 인코딩(encoding)해요.
- decrypt_serial 에서 디버깅이 편한 `DecryptedSerialData` 로 반환해요.
- `encrypt.rs` 로 `encrypt_serial`, `decrypt_serial` 을 옮겼어요.

```rust
let serial = encrypt_serial(&mut items);
println!("Encrypted serial: {serial}");

let decrypted_serial = decrypt_serial(serial, &mut items);
for serial_data in decrypted_serial {
    println!("{}:{}", serial_data.name, serial_data.digit);
}
```

## 2. 날짜 파싱

**원본**: 
- `put_rawdata` 메소드가 구현되지 않았어요.
- 파싱 오류가 `unwrap` 으로 처리되어 파싱 시 오류가 발생하면 그대로 panic 되어 종료되요.

```rust
fn verify(&mut self, data: &str) -> bool {
    let year = data[0..4].parse().unwrap();
    let month = data[4..6].parse().unwrap();
    let day = data[6..8].parse().unwrap();

    self.year == year && self.month == month && self.day == day
}

// ...

fn put_rawdata(&mut self, _data: String) {
    unimplemented!()
}
```

**개선**:
- `Result` 를 사용해 파싱 시 발생하는 오류를 처리해요.
- 입력받는 날짜가 유효한 날짜인지 검증해요.
- `parse_to_date` 함수를 통해 로직을 분리해요.


```rust
fn parse_to_date(s: &str) -> Result<(u16, u8, u8), Box<dyn Error>> {
    let (year, month, day): (u16, u8, u8) = (s[0..4].parse()?, s[4..6].parse()?, s[6..8].parse()?);
    if year < 2025 {
        return Err("The year must be 2025 or later.".into());
    }
    if !(1..=12).contains(&month) {
        return Err("The month must be between 1 and 12.".into());
    }
    if !(1..=31).contains(&day) {
        return Err("The day must be between 1 and 31.".into());
    }
    Ok((year, month, day))
}

// ...

fn put_rawdata(&mut self, data: String) {
    let (year, month, day) = parse_to_date(&data).unwrap();
    self.year = year;
    self.month = month;
    self.day = day;
}
```

## 3. 입력 검증

**원본**:
- `assert` 를 통해 검증했어요, 그래서 검증이 실패하면 panic 이 발생해 프로그램이 종료됬죠.

```rust
fn get_input_from_user(&mut self) {
    println!("Please input the expiration date (YYYYMMDD) (e.g. 20250123) : ",);
    let rawdata = get_user_input();
    assert_eq!(rawdata.len(), 8);

    self.year = rawdata[0..4].parse().unwrap();
    assert!(self.year >= 2021, "The year must be 2021 or later.");
    self.month = rawdata[4..6].parse().unwrap();
    assert!(
        self.month >= 1 && self.month <= 12,
        "The month must be between 1 and 12."
    );
    self.day = rawdata[6..8].parse().unwrap();
    assert!(
        self.day >= 1 && self.day <= 31,
        "The day must be between 1 and 31."
    );
}
```

**개선**:
- 입력을 받아오는 함수가 기본적으로 길이와, 알파벳, 숫자 여부를 검증해요.
- 커스텀 검증 로직을 사용할 수 있는 `get_user_input_custom` 을 사용해 길이, 숫자, 날짜를 검증했어요.
- 검증이 실패할 시 프로그램이 종료되지 않고 다시 입력을 받아와요.

```rust
fn get_input_from_user(&mut self) {
    println!("Please input the expiration date (YYYYMMDD) (e.g. 20251223): ",);
    let rawdata = get_user_input_custom(|s| {
        if s.len() != 8 {
            return Err("date must be 8 digits".into());
        }
        if !s.chars().all(|c| c.is_numeric()) {
            return Err("date must be numberic".into());
        }
        let _ = parse_to_date(s)?;
        Ok(s.to_owned())
    });
    self.put_rawdata(rawdata);
}
```

## 4. 입력값 처리

**원본**:
- 커스텀 검증 로직을 넣을 수 없어서 `get_input_from_user` 에서 검증을 처리해야 했어요.
- 검증 실패시 유저가 다시 입력할 수 있게 하려면 `get_input_from_user` 에서 입력 로직을 조작해야 했어요.

```rust
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

fn get_input_from_user(&mut self) {
    println!("Please input the expiration date (YYYYMMDD) (e.g. 20250123) : ",);
    let rawdata = get_user_input();
    assert_eq!(rawdata.len(), 8);

    self.year = rawdata[0..4].parse().unwrap();
    assert!(self.year >= 2021, "The year must be 2021 or later.");
    self.month = rawdata[4..6].parse().unwrap();
    assert!(
        self.month >= 1 && self.month <= 12,
        "The month must be between 1 and 12."
    );
    self.day = rawdata[6..8].parse().unwrap();
    assert!(
        self.day >= 1 && self.day <= 31,
        "The day must be between 1 and 31."
    );
}
```

**개선**:
- 커스텀 검증 로직을 통해 유연한 처리가 가능해요.
- 모든 입력 관련 사항은 입력값을 받아오는 함수에서 맡아요. 

```rust
pub fn get_user_input(name: &str, expected_len: usize) -> String {
    loop {
        let input = get_input();
        match validate_input(&input, expected_len, name) {
            Ok(_) => return input,
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}

pub fn get_user_input_custom(validate_fn: Validate) -> String {
    loop {
        let input = get_input();
        match validate_fn(&input) {
            Ok(_) => return input,
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
    }
}
```