# intro

## 연습문제

1. 혹시 시리얼 번호를 생성하는 프로그램을 만들어본 적이 없다면 프로그램을 디자인해보세요. 어떠한 요구사항이 있고, 어떤 설계로 프로그램을 만들 수 있을지 생각해보세요.

- use trim_end instead of trim

```rust
buf.trim_end().to_string()
```

- validate

```rust
type Validate = fn(&str) -> Result<String, Box<dyn Error>>;
```

2. 리눅스 커널의 가상 파일 시스템 (Virtual Filesystems)이 어떻게 여러개의 파일시스템(FAT32, EXT4, Btrfs)등을 동시에 지원할 수 있는지 생각해보신 적이 있나요? 관련 자료를 찾아서 읽어보세요.

```
다중 파일시스템 지원 메커니즘
1. 공통 인터페이스 정의
익숙한 파일시스템들인 ext4, NFS, /proc 등은 모두 file_operations라는 C 언어 데이터 구조에서 주요 3개 함수의 정의를 제공합니다. 또한, 특정 파일시스템들은 친숙한 객체지향 방식으로 VFS 함수를 확장하고 오버라이드합니다. The Linux Kernel's VFS Layer
각 파일시스템은 다음과 같은 표준 인터페이스를 구현해야 합니다:

super_operations: 슈퍼블록 관련 작업
inode_operations: inode 관련 작업
file_operations: 파일 I/O 작업
dentry_operations: 디렉토리 엔트리 작업

2. 파일시스템 등록 및 마운트
파일시스템 타입을 구현하는 모듈은 사용될 수 있도록 자신의 존재를 알려야 합니다. 이는 커널 초기화 시간이나 모듈이 삽입될 때 register_filesystem() 호출을 통해 이루어집니다. File system drivers (Part 1) — The Linux Kernel documentation
3. 통일된 시스템 콜 처리
VFS의 추상화는 리눅스 사용자가 내부 데이터 형식에 대해 걱정하지 않고 외부 운영체제나 파이프 같은 추상 개체로부터 파일을 복사할 수 있게 해줍니다. 사용자공간을 대신하여, 시스템 콜을 통해 프로세스는 한 파일시스템의 read() 메소드로 파일에서 커널의 데이터 구조로 복사한 다음, 다른 종류의 파일시스템의 write() 메소드를 사용하여 데이터를 출력할 수 있습니다.
```

3. 리눅스 커널은 모놀리틱 커널입니다. 커널이라는 프로그램은 하나의 바이너리 파일입니다. 그런데 어떻게 동적으로 하드웨어 드라이버를 설치하거나 삭제할 수 있을까요? 관련 자료를 찾아서 읽어보세요. 추후에 각자의 현업에 적용할 수 있는 아이디어를 얻을 수 있을지도 모릅니다.

```
리눅스는 **"동적 확장 가능한 모놀리틱 커널"**입니다. 베이스 커널은 여전히 하나의 주소 공간에서 실행되는 모놀리틱 구조이지만, LKM 시스템을 통해 런타임에 기능을 동적으로 추가하거나 제거할 수 있습니다. 이는 마이크로커널의 유연성과 모놀리틱 커널의 성능을 모두 제공하는 혁신적인 설계입니다.
```

## 최종 결과물

```rust
use std::error::Error;

type Validate = fn(&str) -> Result<String, Box<dyn Error>>;

fn get_input(validate: Validate) -> Result<String, Box<dyn Error>> {
    let buf = &mut String::new();
    std::io::stdin().read_line(buf)?;

    validate(buf.trim())?;

    Ok(buf.trim_end().to_string())
}

fn main() {
    println!("Please input 4-digits Customer ID: ");
    let customer_id = get_input(|s| {
        if s.len() != 4 || !s.chars().all(char::is_numeric) {
            return Err("Customer ID must be exactly 4 digits long.".into());
        }
        Ok(s.to_string())
    })
    .expect("Failed to read Customer ID");

    println!("Please input 8-alphanumeric Product ID: ");
    let product_id = get_input(|s| {
        if s.len() != 8 || !s.chars().all(|c| c.is_alphanumeric()) {
            return Err("Product ID must be exactly 8 alphanumeric characters.".into());
        }
        Ok(s.to_string())
    })
    .expect("Failed to read Product ID");

    let plain_serial = format!("{}{}", customer_id, product_id);
    println!("Plain serial: {}", plain_serial);

    let verify_customer_id = &plain_serial[0..4];
    let verify_product_id = &plain_serial[4..12];
    println!("Verify Customer ID: {}", verify_customer_id);
    println!("Verify Product ID: {}", verify_product_id);
}
```

# encrypt

```rust
cargo add magic-crypt
```

## 연습문제

1. 참고로 트레이트의 메뉴얼을 찾아보는 과정에 대해서 소개했습니다만 new_magic_crypt라는 매크로에 대해서는 소개를 안했습니다. 직접 한번 메뉴얼 페이지를 검색해서 어떤 일을 하는 매크로인지 찾아보시기 바랍니다. 어떤 타입의 객체를 생성하는 것인지, 2개의 인자는 각각 어떤 의미를 갖는지를 확인해보시면, 나중에 좀 더 다양한 옵션을 사용하는데 도움이 될 것입니다.

- 이 매크로는 `MagicCrypt<bits>` 인스턴스 또는 `MagicCrypt` 인스턴스를 생성하는 편리한 방법을 제공합니다.
- 첫 번째 인자 ($key:expr):
  - 암호화/복호화에 사용할 키(비밀번호)를 나타내는 문자열
  - 예: "magickey"
  - 내부적으로 SHA256 해시를 통해 실제 암호화 키로 변환됩니다
- 두 번째 인자 (숫자):
  - 키 길이를 비트 단위로 지정 (64, 128, 192, 256)
  - 64비트: DES 알고리즘 사용
  - 128/192/256비트: AES 알고리즘 사용
  - 기본값은 128비트이며, 키 길이가 클수록 보안이 강화됩니다.
- 세 번째 인자 ($iv:expr) (선택사항):
  - Initialization Vector(IV)를 설정할 수 있습니다.
  - CBC 모드에서 사용되는 초기화 벡터

2. BASE64에 대해서도 조사해보세요. 특히 위 예제에서 생성한 시리얼 키 GPghOzaNUn7G7FKiAkhKQQ==에서 마지막에 있는 "=="가 어떤 의미인지를 확인해보시기 바랍니다. 보통의 시리얼 키에는 "="라는 문자가 없는데 왜 우리가 만든 시리얼 키에는 "="가 있을까요? 사실은 "=="를 생략해도 괸찮습니다만 왜 그럴까요?

- BASE64는 바이너리 데이터를 텍스트 형태로 변환하는 인코딩 방식으로, 64개의 인쇄 가능한 문자만을 사용하여 데이터를 표현합니다. 바이너리 데이터를 6비트 청크로 나누고, 이 6비트 값들을 BASE64 테이블을 사용해 인쇄 가능한 문자로 변환합니다.
- 패딩이 필요한 이유
  BASE64는 3바이트(24비트)를 4개의 문자로 변환하는 규칙을 따릅니다. 각 3바이트의 입력에 대해 BASE64는 정확히 4개의 문자를 생성합니다. 하지만 데이터의 길이가 항상 3의 배수는 아니므로 패딩이 필요합니다.
- 패딩 규칙
  - 24비트 버퍼에서 패딩된 제로가 2옥텟이면 두 개의 "=" 문자가 추가되고, 1옥텟이 패딩된 제로로 채워지면 하나의 "=" 문자가 추가됩니다.

```
데이터 길이가 3의 배수: 패딩 없음
데이터 길이가 3n+1: "==" (2개 패딩)
데이터 길이가 3n+2: "=" (1개 패딩)
```

3. 저는 구글에서 암호화 크레이트를 검색해봤습니다만 사실 요즘에는 더 좋은 개발 보조 도구가 있습니다. ChatGPT나 Copilot등에 암호화에 대해 좋은 크레이트를 추천해달라고 요청해보세요. 예제 코드도 만들어달라고하면 좋은 예제를 보여주고, 예제에 대한 상세한 설명도 해줍니다.

- chacha20poly1305

```rust
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 키와 암호화 객체 생성
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    // 2. 암호화
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, b"hello world!".as_ref())?;

    // 3. 복호화
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;

    println!("원본: hello world!");
    println!("복호화: {}", String::from_utf8(plaintext)?);

    Ok(())
}
```
