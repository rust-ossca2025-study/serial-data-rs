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
