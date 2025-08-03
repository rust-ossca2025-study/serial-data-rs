## 개선해본 사항

### 1. **모듈화 및 구조화**

**원본**: 모든 로직이 하나의 파일에 집중

**개선**: 기능별 모듈 분리
- **`data.rs`**: 데이터 구조 및 암호화 로직
- **`input.rs`**: 사용자 입력 처리
- **`main.rs`**: 메인 실행 흐름

### 2. **객체지향적 설계**

**원본**: 절차지향적 접근

**개선**: 구조체와 메서드 활용
```rust
#[derive(Clone, PartialEq, Debug)]
pub struct SerialData {
    pub customerid: String,
    pub productid: String,
}

impl SerialData {
    pub fn concat(&self) -> String { ... }
    pub fn encrypt(&self) -> String { ... }
    pub fn from_string(dec: String) -> SerialData { ... }
}
```

### 3. **입력 검증 시스템**

**원본**: 입력 검증 없음

**개선**: 입력 검증 추가
```rust
pub fn get_user_input(name: &str, expected_len: usize) -> String {
    loop {
        // 입력 받기
        match validate_input(&s, expected_len, name) {
            Ok(_) => return s,
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Please try again.");
            }
        }
    }
}

fn validate_input(input: &str, expected_len: usize, name: &str) -> Result<(), String> {
    if input.len() != expected_len {
        return Err(format!("{} must be {} digits", name, expected_len));
    }
    Ok(())
}
```

### 4. **전역 상태 관리**

**원본**: 매번 새로운 객체 생성
```rust
let mc = new_magic_crypt!("magickey", 256);
```

**개선**: `lazy_static`을 통한 관리
```rust
lazy_static! {
    static ref MAGIC_CRYPT_BASE: MagicCrypt256 = new_magic_crypt!(CRYPT_KEY, 256);
}
```

### 5. **에러 처리 개선**

**원본**: 단순한 `unwrap()`
```rust
let dec = mc.decrypt_base64_to_string(serial).unwrap();
```

**개선**: 구체적인 에러 처리
```rust
pub fn decrypt_serialdata(encrypted_serial: String) -> String {
    MAGIC_CRYPT_BASE
        .decrypt_base64_to_string(encrypted_serial)
        .unwrap_or_else(|e| {
            eprintln!("Decryption failed: {:?}", e);
            panic!("Invalid encrypted serial data provided: {:?}", e);
        })
}
```

### 6. **코드 재사용성**

**원본**: 하드코딩된 파싱
```rust
let verify_customerid = &dec[0..4];
let verify_productid = &dec[4..12];
```

**개선**: 메서드로 캡슐화
```rust
impl SerialData {
    pub fn from_string(dec: String) -> SerialData {
        if dec.len() < 12 {
            panic!("Serial must be at least 12 characters, got {}", dec.len());
        }
        SerialData {
            customerid: dec[0..4].to_owned(),
            productid: dec[4..12].to_owned(),
        }
    }
}
```

### 7. **테스트 추가**

**원본**: 테스트 부재

**개선**: 테스트 케이스 추가
```rust
#[cfg(test)]
mod tests {
    use super::{SerialData, decrypt_serialdata};

    #[test]
    fn concat_serialdata() { ... }
    
    #[test]
    fn get_serialdata_from_string() { ... }
    
    #[test]
    fn e2e_serialize() { ... }
}
```


## 새로 알게 된 사항

`let _ = stdout().flush()` 를 보고 `stdout().flush()` 로 바꾸다가 Rust 에서는 `stdout().flush()` 와 같이 `Result` 를 반환하는 것들은 `#[must_use]` 가 붙어 있어서 반드시 사용되어야 한다는 경고가 출력되었다.

값이 사용되지 않더라도 다른 언어와 달리 `ok()` 로 에러를 무시하거나 `let _ = ...` 과 같이 명시적으로 무시해야 한다는 것을 새로 알게 되었다.