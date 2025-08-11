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

# Comment

- from includes into
