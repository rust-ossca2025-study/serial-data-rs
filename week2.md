# Week2

## 플러그인

### 공통 특성

- 플러그를 만들 때 가장 먼저 할 일은 사용할 각 입력 데이터의 공통적인 특성을 뽑아 내는 것
- 발견된 특성 나열
  - 사용자 ID
    - 길이 4글자
    - 숫자나 알파벳으로 이루어짐
  - 제품 ID
    - 길이 8글자
    - 숫자나 알파벳으로 이루어짐
- 공통 특성 추출
  - 데이터의 특성
    - alphanumeric 으로 이루어짐
    - 길이 값이 다르더라도, 길이가 미리 정해져 있다는 것 자체가 중요한 공통 특성
  - 데이터 를 처리하는 방법
    - 입력받는 방법이 같음

### 구현

### 정리

- plugin(driver) 를 사용해야 하는 상황 식별
- 처리할 데이터와 처리하는 특성 정의
- trait 은 plugin 구현의 핵심 역할을 함
- 구체적인 구현은 관심 없고 표준 인터페이스만 제공
- 구체 struct 를 plugin, trait 을 framework 라고 부르기도 함

### 연습문제

CustomerID와 ProductID, 2개의 입력 데이터를 만들어봤습니다. 그 외에 시리얼 번호를 생성하기 위해서 어떤 입력 데이터가 있을 수 있을지 한번 생각해보세요. 그리고 한번 구현까지 해보세요. 다음장에서 2개의 입력데이터를 추가해볼 예정입니다만 그 전에 직접 고민해보는 것을 추천합니다. 다음 장을 읽어보면서 제가 제시한 방법보다 더 좋은 방법을 생각해보시기 바랍니다.

## CustomerType 추가

- override get_input_from_user
- from takes CustomerKind not ref => should clone
- if let used to use ref not to take ownership before the main logic

## ExpireDate 추가

- override get_input_from_user, verify
- set unimplemented on unused `put_rawdata()`

## 연습문제

- get_rawdata에서 `(*kind).clone()`과 같이 clone메소드를 호출하지 않도록 고쳐보세요. From트레이트 구현의 어디를 바꾸면 될지 생각해보세요.

- get_rawdata 메소드의 구현에서 if let Some(kind) = self.customer_type와 같이 소유권이 이동되도록 구현을 바꿔보세요. 어디에서 어떤 에러가 나는지 확인해보고 그 의미를 생각해보세요.

```rust
fn get_rawdata(&self) -> String {
    if let Some(kind) = &self.customer_type {
        return format!("{}", usize::from((kind)));
    } else {
        return "0".to_owned();
    }
}
```

### Issue

1. `kind` 에 값 할당 시도
2. 할당 방법 결정
3. Copy?: CustomerKind에 구현 안됨
4. Move 시도
5. &self 뒤에 데이터는 Move 불가함

### 해결책

- 원래대로 & 사용
- ref 사용
- Copy 구현

# Comment

## & 와 ref

### & - 참조 연산자

- 값에서 참조를 생성
- 표현식에서 사용

### ref - 패턴 매칭 키워드

- 패턴 매칭에서 참조를 생성
- let, match, if let 등에서 사용

## from includes into

- [It is the reciprocal of Into](https://doc.rust-lang.org/std/convert/trait.From.html)

````rust
struct Person {
    name: String,
}

// From만 구현
impl From<String> for Person {
    fn from(name: String) -> Self {
        Person { name }
    }
}

fn main() {
    let name = String::from("Alice");

    // From 사용
    let person1 = Person::from(name.clone());

    // Into 자동으로 사용 가능!
    let person2: Person = name.into();
}

### TryFrom
- Rust 에서 Try 가 붙은 경우 Result return 타입에 대한 구현을 의미

```rust
use std::convert::TryFrom;

struct PositiveNumber(u32);

impl TryFrom<i32> for PositiveNumber {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err("Number must be positive")
        }
    }
}

fn main() {
    // TryFrom 사용
    let pos1 = PositiveNumber::try_from(5).unwrap();

    // TryInto 자동으로 사용 가능
    let pos2: Result<PositiveNumber, _> = 10.try_into();
}
````

## copy vs clone

- Copy 는 암시적으로 데이터 복사를 자동으로 수행해줌
- Copy 는 단순히 메모리 복사
- Primitive 타입은 Copy 트레이트를 구현
- Custom Type 이 Copy 구현하고 싶은 경우, 하위 모든 필드가 Copy trait 을 구현해야 함

```rs
#[derive(Copy)]
struct SomeType {
    a: i32,
    b: i32,
}
```

- Clone 은 명시적으로 데이터 복사를 수행
- resource 소비 큼

1. .clone() 메서드 호출 (함수 호출 오버헤드)
2. 새로운 힙 메모리 할당
3. 데이터 복사
4. 메타데이터(길이, 용량) 설정

- Custom logic 추가 가능

```rs
#[derive(Clone)]
struct Database {
    connections: Vec<Connection>,
    cache: HashMap<String, Data>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        println!("데이터베이스 복제 시작...");

        // 1. 연결 풀 새로 생성
        let new_connections = self.connections
            .iter()
            .map(|conn| conn.duplicate())  // 각 연결을 새로 생성
            .collect();

        // 2. 캐시는 비우기 (성능상 이유)
        let empty_cache = HashMap::new();

        println!("데이터베이스 복제 완료");
        Database {
            connections: new_connections,
            cache: empty_cache,  // 원본과 다른 상태!
        }
    }
}
```
