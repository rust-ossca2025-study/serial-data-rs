# 1주차 후기

## Intro
스터디의 목표를 두 가지로 설정했다.
1. Rust 문법과 친해지자
2. (Optional) low-level에 대해 더 이해하자

따라서, 다음의 원칙을 세웠다.
1. 예제에 있는 코드를 그대로 복붙하지 않는다.
2. AI 자동 완성 기능을 사용하지 않는다.
3. Rust 문법과 관련해서 모르는 키워드가 등장하면 공부해서 정리하자.

## 1주차 범위
- [intro.md](https://github.com/gurugio/quick-guide-rust-programming/blob/master/text/09_tiny_project/09_00_intro.md)
- [encrypt.md](https://github.com/gurugio/quick-guide-rust-programming/blob/master/text/09_tiny_project/09_01_encrypt.md)

## Goal
***Note: 기존 자료에서 나에게 좀 더 맞는 방향으로 요구 사항을 조금씩 수정해서 진행했다.***
- 유저에게 `customer_id`와 `product_id`를 입력받아서, plain serial number를 만드는 기능을 구현하자.
- `customer_id`와 `product_id`는 각각 4자리, 8자리이고 다음과 같은 조건을 만족시켜야 한다.
  - id는  ASCII 숫자로 구성되어 있어야 한다.
  - 자리수를 정확히 지켜야한다.
- `customer_id`와 `product_id`를 concat한 값을 암호화해서 serial number로 사용한다.

## What I Learned
**[1] Rust 문법과 친해지기: string slice, closure, trim, stdin**

**[2] `fn encrypt_str_to_base64<S: AsRef<str>>(&self, string: S) -> String` 함수 시그니처 이해**
- <S: AsRef<str>>: S는 제네릭 타입 매개변수이고, AsRef<str> 트레이트를 만족해야 한다는 제약(=트레이트 바운드)를 갖는다는 의미함. 해당 바운드는 "S 값을 &str 참조로 '가볍게' 변환할 수 있음을 보장"해야 한다는 것을 의미함.
- (&self, string: S): 런타임 인자로, string: S를 받음. 호출할 때는 String, &str 등 AsRef<str>를 구현한 아무 타입이나 넘길 수 있다는 것을 의미함.
- AsRef<T>는 소유권 이동 없이 &T 참조를 얻는 표준 변환 인터페이스임. 덕분에 함수 안에서는 string.as_ref() 만 호출하면 곧바로 &str을 얻어 처리할 수 있음.

**[3] AsRef<T> 트레이트의 시그니처를 보면, T: ?Sized 라고 되어 있음. `Sized`, `?Sized`는 무엇일까?**
1. `Sized`
- `컴파일 타임에 크기가 결정되는 타입`을 나타내는 트레이트(컴파일러가 자동 파생).
- 대부분의 구체 타입(스칼라, 구조체, 열거형, 고정 길이 배열, String, Vec<T> 등)이 해당됨.
2. unsized 타입
- 컴파일러 입장에서 자체로는 크기를 알 수 없어 포인터(참조·Box 등)에 붙여서만 사용할 수 있는 타입.
- 대표적으로 str, [T], 트레이트 객체 dyn Trait 등이 있음.
3. `?Sized` 표기
- 기본적으로 모든 제네릭 파라미터엔 암묵적 Sized 제약이 붙습니다.
- `T: ?Sized`라고 쓰면 “T가 Sized일 수도 있고 아닐 수도 있다”는 뜻으로, 그 암묵 제약을 해제함.
- 현재 안정된 표준 문법에서 사용되는 ‘옵셔널 트레이트 바운드’는 ?Sized가 사실상 유일함.
