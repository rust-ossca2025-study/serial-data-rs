# 1주차 후기

스터디의 목표를 두 가지로 설정했다.
1. Rust 문법과 친해지자
2. (Optional) low-level에 대해 더 이해하자

따라서, 두 가지의 원칙을 세웠다.
1. 예제에 있는 코드를 그대로 복붙하지 않는다.
2. AI 자동 완성 기능을 사용하지 않는다.

## 1. [intro.md](https://github.com/gurugio/quick-guide-rust-programming/blob/master/text/09_tiny_project/09_00_intro.md)


**목표**
- 유저에게 `customer_id`와 `product_id`를 입력받아서, plain serial number를 만드는 기능을 구현하자.
- `customer_id`와 `product_id`는 각각 4자리, 8자리이고 다음과 같은 조건을 만족시켜야 한다.
  - id는  ASCII 숫자로 구성되어 있어야 한다.
  - 자리수를 정확히 지켜야한다.

**배운 것**
1. Rust 문법과 친해지기: string slice, closure, trim, stdin
