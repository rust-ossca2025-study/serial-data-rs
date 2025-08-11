## 연습 문제

### 1. 혹시 시리얼 번호를 생성하는 프로그램을 만들어본 적이 없다면 프로그램을 디자인해보세요. 어떠한 요구사항이 있고, 어떤 설계로 프로그램을 만들 수 있을지 생각해보세요.

시리얼 번호는 고유한 식별번호를 의미하니, 암호화시켜서 정보를 얻을 수 있도록 하면 좋을 것 같다.
입력을 받을 때도 유저가 입력한 정보를 검증하는 것은 반드시 필요해보인다.

- 입력 받기
- 정보를 넣고 정해진 길이로 나누거나 `#` 와 같이 나누기
- 합쳐진 정보를 암호화시키기
- 추후 그 정보가 들어왔을 때 다시 정보를 얻을 수 있도록 하기 

### 2. 리눅스 커널의 가상 파일 시스템 (Virtual Filesystems)이 어떻게 여러개의 파일시스템(FAT32, EXT4, Btrfs)등을 동시에 지원할 수 있는지 생각해보신 적이 있나요?

VFS 레이어가 해당 파일을 확인해 파일 시스템을 식별하고, 사용자가 호출한 일관된 함수에 맞는 파일 시스템 고유의 함수를 호출한다. 이러한 추상화된 레이어로 인해 일관된 인터페이스를 사용해 파일에 접근하는 것이 가능하다.

### 3. 리눅스 커널은 모놀리틱 커널입니다. 커널이라는 프로그램은 하나의 바이너리 파일입니다. 그런데 어떻게 동적으로 하드웨어 드라이버를 설치하거나 삭제할 수 있을까요?

독립적인 커널 모듈을 통해 필요할 때 동적으로 기능을 로드하고 언로드할 수 있다. 단일 바이너리 파일로 제공되는 커널이지만 모든 기능을 커널 내부에 포함해야 되는 것은 아니다.

### 4. new_magic_crypt라는 매크로에 대해서는 소개를 안했습니다. 직접 한번 메뉴얼 페이지를 검색해서 어떤 일을 하는 매크로인지 찾아보시기 바랍니다.

`MagicCrypt` 타입의 인스턴스를 생성한다. 첫 번째 인자로는 암호화 키를 받고, 두 번째 인자로는 암호화의 비트 수를 받는다.

### 5. 보통의 시리얼 키에는 "="라는 문자가 없는데 왜 우리가 만든 시리얼 키에는 "="가 있을까요?

Base64 는 3바이트 단위로 데이터를 처리해 4개의 문자로 인코딩한다. 인코딩 대상 데이터의 길이가 3의 배수가 아닐 경우, 빈 자리를 채우기 위해 "=" 또는 "==" 를 패딩 문자로 사용한다.  

Base64 디코더는 보통 패딩이 없어도 복원이 가능하다. 하지만 일부 엄격한 디코더는 패딩 없이는 오류를 발생시키기도 한다.

### 6. ChatGPT 나 Copilot 등에 암호화에 대해 좋은 크레이트를 추천해달라고 요청해보세요.

| 크레이트                  | 설명                                   |
| --------------------- | ------------------------------------ |
| `magic-crypt`         | 간단한 대칭키 암호화. 사용하기 매우 쉬움.             |
| `ring`                | 고성능의 암호화 라이브러리. 안전성 보장됨.             |
| `rust-crypto`         | 다양한 암호 알고리즘 지원. 다소 복잡함.              |
| `aes` + `block-modes` | 직접 AES 기반 대칭키 암호화 구현 가능.             |
| `chacha20poly1305`    | 고속 스트림 암호, `ring`보다 더 현대적인 선택지 중 하나. |



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