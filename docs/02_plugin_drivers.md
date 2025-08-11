# 플러그인 아키텍처 문서

## 아키텍처 개요

시리얼 번호 생성기는 플러그인 기반 아키텍처로 설계되어 데이터 수집과 암호화를 분리합니다.

```
사용자 입력 → [Plugins Layer] → [Providers Layer] → 시리얼 번호
              (데이터 수집)      (암호화/해시)
```

## 프로젝트 구조

```
src/
├── main.rs              # 메인 진입점과 메뉴 시스템
├── payload.rs           # InputPayload 트레이트
├── provider.rs          # SerialNumberProvider 트레이트
├── service.rs           # SerialNumberService
├── plugins/
│   ├── mod.rs          # GenSerialData 트레이트와 PluginRegistry
│   ├── customer_id.rs  # Customer ID 플러그인 (4자리)
│   └── product_id.rs   # Product ID 플러그인 (8자리)
└── providers/
    ├── mod.rs                    
    ├── magic_crypt_provider.rs   # AES-256 암호화
    └── sha256_provider.rs        # SHA-256 해시
```

## 핵심 트레이트

### 1. GenSerialData - 데이터 수집 인터페이스
```rust
pub trait GenSerialData {
    fn get_input_from_user(&mut self);     // 사용자 입력 받기
    fn verify(&self, data: &str) -> bool;  // 입력 검증
    fn get_rawdata(&self) -> String;        // 수집된 데이터 반환
    fn get_name(&self) -> String;           // 플러그인 이름
}
```

### 2. Plugin - 플러그인 마커
```rust
pub trait Plugin: GenSerialData {}
```

### 3. InputPayload - 데이터 표준화
```rust
pub trait InputPayload {
    fn to_canonical_string(&self) -> String;
    fn from_canonical_string(s: &str) -> Result<Self, String> where Self: Sized;
}
```

### 4. SerialNumberProvider - 암호화 인터페이스
```rust
pub trait SerialNumberProvider: Send + Sync {
    fn generate_from_payload(&self, payload: &dyn InputPayload) -> Result<String, String>;
    fn validate(&self, serial: &str) -> bool;
    fn name(&self) -> &'static str;
    fn description(&self) -> String;
}
```

## 주요 컴포넌트

### PluginRegistry
플러그인들을 관리하는 컨테이너
```rust
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}
```

### SerialNumberService  
Provider들을 관리하고 시리얼 번호 생성/검증 제공
```rust
pub struct SerialNumberService {
    providers: HashMap<&'static str, Arc<dyn SerialNumberProvider>>,
}
```

## 새로운 플러그인 추가하기

### 데이터 수집 플러그인
```rust
// 1. plugins/batch_code.rs 생성
use super::{GenSerialData, Plugin, get_user_input};

pub struct BatchCode {
    code: Option<String>,
}

impl GenSerialData for BatchCode {
    fn get_input_from_user(&mut self) {
        let input = get_user_input("Batch Code: ");
        if self.verify(&input) {
            self.code = Some(input);
        }
    }
    
    fn verify(&self, data: &str) -> bool {
        !data.is_empty() && data.len() <= 10
    }
    
    fn get_rawdata(&self) -> String {
        self.code.clone().unwrap_or_default()
    }
    
    fn get_name(&self) -> String {
        "Batch Code".to_string()
    }
}

impl Plugin for BatchCode {}

// 2. plugins/mod.rs에 모듈 추가
pub mod batch_code;

// 3. main.rs에서 사용
registry.register(Box::new(BatchCode::new()));
```

### 암호화 Provider
```rust
// 1. providers/custom_provider.rs 생성
use crate::payload::InputPayload;
use crate::provider::SerialNumberProvider;

pub struct CustomProvider;

impl SerialNumberProvider for CustomProvider {
    fn generate_from_payload(&self, payload: &dyn InputPayload) -> Result<String, String> {
        let data = payload.to_canonical_string();
        // 커스텀 암호화 로직
        Ok(format!("CUSTOM-{}", data))
    }
    
    fn validate(&self, serial: &str) -> bool {
        serial.starts_with("CUSTOM-")
    }
    
    fn name(&self) -> &'static str {
        "Custom"
    }
    
    fn description(&self) -> String {
        "Custom encryption provider".to_string()
    }
}

// 2. providers/mod.rs에 추가
pub mod custom_provider;

// 3. main.rs에서 사용
service.register(Arc::new(CustomProvider::new()));
```

## 동작 흐름

1. **플러그인 등록**: 각 데이터 수집 플러그인을 PluginRegistry에 등록
2. **Provider 등록**: 암호화 Provider를 SerialNumberService에 등록
3. **데이터 수집**: 플러그인들이 사용자로부터 데이터 입력받음
4. **페이로드 생성**: 수집된 데이터를 StandardInput으로 변환
5. **시리얼 생성**: 선택된 Provider가 페이로드를 암호화/해시
6. **검증**: Provider의 validate 메서드로 시리얼 번호 유효성 확인

## 설계 원칙

- **관심사 분리**: 데이터 수집과 암호화를 별도 레이어로 분리
- **개방-폐쇄 원칙**: 새로운 플러그인/Provider 추가 시 기존 코드 수정 불필요
- **의존성 역전**: 구체적 구현이 아닌 트레이트에 의존
- **단일 책임**: 각 컴포넌트는 하나의 책임만 가짐

## 사용 예시

```rust
// main.rs
fn main() {
    // 서비스 초기화
    let mut service = SerialNumberService::new();
    service.register(Arc::new(MagicCryptProvider::with_default_key()));
    service.register(Arc::new(Sha256Provider::with_default_salt()));
    
    // 데이터 수집
    let mut registry = PluginRegistry::new();
    registry.register(Box::new(CustomerID::new()));
    registry.register(Box::new(ProductID::new()));
    
    // 사용자 입력
    for plugin in registry.get_plugins_mut() {
        plugin.get_input_from_user();
    }
    
    // 시리얼 생성
    let input = StandardInput::from_plugins(registry.get_plugins());
    let serial = service.create_serial("MagicCrypt", &input)?;
    println!("Generated: {}", serial);
}