# 2주차
2주차부터는 **기존 코드에 대한 이해를 바탕**으로, **리팩토링 할 수 있는 부분은 없는지**에 대해 정리합니다.

# Good Point
## [1] GenSerialData Trait를 사용하는 것은 좋은 선택일까?
- Serial Number를 구성하는 종류가 다양해질 수 있는 상황에서, Serial Number를 구성할 수 있는 아이템이 반드시 준수해야 할 인터페이스를 Trait로 묶는 것은 좋다고 생각함.

## [2] Serial Number를 구성하는 각 아이템을 Box에 넣고, Vec로 관리하는 것은 좋은 선택일까?
- 아이템의 개수가 몇 개가 될지 모르는 상황이기 떄문에, 좋다고 생각함.
- Vec의 아이템은 컴파일 타임에 크기가 결정되어야 함. 그런데 Trait의 구현체에 대한 크기를 알 수가 없음. 따라서, Box에 구체의 포인터를 담는 것은 좋은 선택이라고 봄. 이게 일반적인 패턴인 듯?
- 아니 근데, vec![Box::new(customerid), Box::new(productid)]; 이런 식으로 선언했으니까 컴파일 타임에 Box가 없어도 아이템의 크기를 알 수 있는 것 아닌가?

# 개선 point
## [1] main.rs에 모든 코드가 있음. 디렉토리 구조 잡기
- main.rs 은 최대한 얇게 유지하자
- 그러기 위해 trait와 struct를 별도 모듈로 빼기로 함.
- trait, struct 모두 하나의 domain으로 볼 수 있다고 생각해서, domain 모듈 하위에 위치시킴.
- GenSerialData trait나 CustomerID, ProductID와 같은 struct의 로직이 복잡하지 않아서, domain 안에 서브디렉토리에 위치시키지는 않고 개별 파일로 구현함.

## [2] GenSerialData 모듈화
- domain/traits.rs에 GenSerialData를 위치시킴
- GenSerialData의 get_input_from_user 메소드에서 사용하는 get_user_input는 traits.rs에 같이 위치시킴. Rust에서는 트레이트 내부에 private 메소드를 둘 수 없음. 왜냐면 트레이트의 모든 메소드를 계약의 일부로 보기 때문임.
- 따라서, 권장 패턴은 트레이트와 같은 파일에 private function으로 두는 것임.

## [3] CustomerID, ProductID와 모듈화
- domain 디렉토리 하위에 customder_id.rs와 product_id.rs로 분리함.
- 각 파일에서 domain/traits/GenSerialData를 참조하기 위해서 use super를 사용함. super를 사용하면 파일의 상대적인 위치가 고정되어야 한다는 제약이 있음. 파일의 위치가 크게 변경될 일은 없어 보이고, use 구문을 간단하게 사용할 수 있기 때문에 super를 사용함.


## [4] main에 있는 collect_data, generate_serial 함수를 어디에 위치시킬까?
- helpers.rs에 위치시킴 -> GenSerialData에 의존성이 있는 함수들이라 여기에 있는게 애매한 느낌
- Vec<Box<dyn GenSerialData>> 타입에 대해 확장 트레이트(extenstion trait)를 적용해서, collect_data() 와 generate_serial() 에 대한 코드의 응집도를 높임.
