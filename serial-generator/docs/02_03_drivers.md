## 개선

파일을 모듈화하고 lib.rs에서 통합 관리해 임포트 경로를 단순화하였습니다.

clone() 제거 : `impl From<CustomerKind> for usize` -> `impl From<&CustomerKind> for usize로 수정`



`customer_kind.rs` 에서 `get_rawdata(&self)` 메서드는 조회용이므로 소유권 없이 이동하도록 수정
``` rust

    fn get_rawdata(&self) -> String {
        if let Some(kind) =self.customer_type.as_ref() {
            return  format!("{}", usize::from(kind));
        } else {
            return "0".to_owned();
        }
    }
```




## 기타

`to_owned()` :  참조 타입(&str, &T 등)을 소유하는 타입(String, T 등)으로 복사(clone)하는 메서드

`#[derive(Clone, Debug)]` : Clone 트레이트와 Debug 트레이트의 구현을 자동으로 생성

`Vec<T>` : 가변 길이의 동일 타입 요소를 순차적으로 저장하는 컬렉션 타입

`Box<T>` : 값을 힙에 저장하고 해당 값의 포인터를 스택에 보관하는 스마트 포인터. 크기가 컴파일 시점에 고정되지 않는 타입이나 재귀 자료구조, 트레이트 객체 등을 다룰 때 사용됨.