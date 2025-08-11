# Week2
## 02_plugin_drivers
### 초기 구조 - 고추상화 (Trait 중심)
#### trait란
Rust에서 (반드시 구현되어야하는) 동작 규격을 정의하는 도구\
(다른 언어에서 interface, abstract class와 비슷하다고 느낌)

#### 초기 코드 분석
**trait GenSerialData** 에서 length, name, serialize 등의 메서드를 가지고 있었고, ProductId, CustomerId가 모두 이 trait를 impl해서 사용하고 있었다.

이 구조 대신 trait을 없애서 자유 함수로 분해하는 작업을 해봤다.

#### 구조 단순화
먼저 기존에 다른 구조체로 나뉘어있었던 ProductId와 CustomerId를 하나의 구조체로 만들어봤다\
그리고 위 trait에 있는 메소드를 새로 만든 단일 구조체 필드와 자유 함수로 분해를 해봤다.

| 원래 trait 메서드             | 변경 후 형태                          |
| ------------------------ | -------------------------------- |
| `fn length(&self)`       | `user_len`, `product_len` 필드 참조  |
| `fn name(&self)`         | 하드코딩된 문자열 출력                     |
| `fn serialize(&self)`    | `plain_serial(&data)` 자유 함수      |
| `fn put_rawdata`         | `prompt_inputs(&mut data)` 내부 로직 |
| `fn get_input_from_user` | `prompt_inputs`에서 직접 호출          |
| `fn verify`              | `verify_decrypted` 자유 함수         |

이런 식으로 구조를 바꾸니까 묶여있던 모듈들을 다 풀면서 코드 흐름과 구조가 단순해졌다. 그런데 하드코딩되는 부분도 생기고 새로운 필드를 추가할 때는 해당 관련 문자열들이랑 로직들을 여러 곳에서 직접 찾아서 수정해야된다는 점이 생겼다. 또 모듈화를 줄이면서 다형성이 사라져서 확장성이 떨어졌다.

## 03_more_drivers
### 오류 모듈화
다른 모듈화가 된 것들을 보고 입력 검증·형식 오류·길이 불일치와 같은 로직도 한 곳에 모아야겠다는 생각이 들어서 SerialError 타입을 추가했다. 지금 있는 오류 패턴에 맞게
- SerialError enum
    - InvalidLength { expected, actual }
    - InvalidValue { field, reason }

크게 2개를 만들었다. 원래 기존에는
- InvalidFormat { field: String, reason: String },
- InvalidDate { year: u32, month: u32, day: u32, reason: String },
- InvalidCustomerType { value: String },
- ParseError { field: String, value: String }

이렇게 InvalidValue 대신 4개가 있었는데 이게 거의 date를 위한 것들이여서 그냥 하나로 묶어서 InvalidValue로 만들어버렸다.

#### \#[derive(..)]
구조체나 열거형에서 반복적으로 구현해야 하는 표준 기능들을 컴파일러가 자동으로 만들어 주는 기능이다. 예를 들어 값 비교, 복제, 디버깅 출력처럼 자주 쓰이는 기능들을 일일이 작성하지 않고, derive 속성에 해당 기능 이름을 적어주면 컴파일러가 그 구현을 대신 생성함

#### impl fmt
사람이 보기 좋게 값(객체)을 문자열로 표현하는 방법을 직접 정의

#### impl From<...>
타입 변환 규칙 정의 : impl From\<T> for U를 만들면, T를 U로 자동 변환 가능
