# 2주차
2주차부터는 **기존 코드에 대한 이해를 바탕**으로, **리팩토링 할 수 있는 부분은 없는지**에 대해 정리합니다.

# Good Point
## 1. GenSerialData Trait를 사용하는 것은 좋은 선택일까?
- Serial Number를 구성하는 종류가 다양해질 수 있는 상황에서, Serial Number를 구성할 수 있는 아이템이 반드시 준수해야 할 인터페이스를 Trait로 묶는 것은 좋다고 생각함.

## 2. Serial Number를 구성하는 각 아이템을 Box에 넣고, Vec로 관리하는 것은 좋은 선택일까?
- 아이템의 개수가 몇 개가 될지 모르는 상황이기 때문에 `Vec<T>` 사용은 적절하다고 봄.
- `Vec<T>`의 요소는 `Sized` 이어야 함. `dyn Trait`은 `?Sized`라 그대로는 담을 수 없음. 따라서 `Box<dyn GenSerialData>`와 같은 간접 참조로 크기를 고정하는 것이 일반적인 패턴임.

# 개선한 것들
## 1. main.rs에 모든 코드가 있음. 디렉토리 구조 잡기
- main.rs 은 최대한 얇게 유지하자
- 그러기 위해 trait와 struct를 별도 모듈로 빼기로 함.
- trait, struct 모두 하나의 domain으로 볼 수 있다고 생각해서, domain 모듈 하위에 위치시킴.
- GenSerialData trait나 CustomerID, ProductID와 같은 struct의 로직이 복잡하지 않아서, domain 안에 서브디렉토리에 위치시키지는 않고 개별 파일로 구현함.

## 2. GenSerialData 모듈화
- domain/traits.rs에 GenSerialData를 위치시킴
- GenSerialData의 get_input_from_user 메소드에서 사용하는 get_user_input는 traits.rs에 같이 위치시킴. Rust에서는 트레이트 항목이 모두 트레이트의 공개 인터페이스를 구성하므로, 트레이트 내부에만 보이는 private 함수를 둘 수 없음.
- 대신 동일 모듈에 private 자유 함수를 두고 트레이트의 기본 메서드에서 호출하는 패턴이 권장됨. 현재 `traits.rs`의 `get_user_input`이 그 예임.

## 3. CustomerID, ProductID와 모듈화
- domain 디렉토리 하위에 `customer_id.rs`와 `product_id.rs`로 분리함.
- 각 파일에서 `GenSerialData`를 참조할 때 현재는 `use super::traits::GenSerialData`를 사용함. 모듈 재배치 내성을 높이려면 `use crate::domain::traits::GenSerialData`처럼 절대 경로를 권장.


## 4. main에 있는 collect_data, generate_serial 함수를 어디에 위치시킬까?
- helpers.rs에 위치시킴 -> GenSerialData에 의존성이 있는 함수들이라 여기에 있는게 애매한 느낌
- `Vec<Box<dyn GenSerialData>>` 타입에 대해 확장 트레이트(extension trait)를 적용해서, `collect_data()`와 `generate_serial()`의 응집도를 높임.


## 추가로 개선하면 좋을 것들
- 메서드 시그니처의 가변성 최소화: 실제로 변경이 없는 메서드는 `&mut self` 대신 `&self`로 받도록 수정하기. e.g., `get_length(&self)`, `get_name(&self)`, `verify(&self, data: &str)`.
