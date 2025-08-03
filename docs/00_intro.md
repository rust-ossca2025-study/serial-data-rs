# Intro: Serial Number Generator

## [Linux VFS(Virtual File System)](https://www.kernel.org/doc/html/latest/filesystems/vfs.html) - Adapter Pattern

Linux의 VFS(가상 파일 시스템)는 애플리케이션과 구체적인 파일 시스템(EXT4, FAT32, NTFS 등) 사이의 추상화 계층 역할을 합니다. 애플리케이션은 open(), read(), write() 와 같은 표준화된 함수만 호출하면 됩니다. 어떤 저장 장치에 어떤 파일 시스템이 사용되었는지는 신경 쓸 필요가 없습니다.

VFS는 모든 파일 시스템이 따라야 할 **공통 인터페이스(어댑터 패턴)**를 정의합니다. 각 파일 시스템 드라이버는 이 인터페이스에 맞춰 자신의 고유한 기능을 구현하고 VFS에 등록합니다. VFS는 요청이 들어오면 해당 파일에 맞는 실제 파일 시스템 드라이버의 구현체를 호출해주는 '어댑터' 역할을 수행합니다.

핵심 아이디어: "무엇을 할지(What)는 표준으로 정의하고, 어떻게 할지(How)는 각 구현체에 위임한다."

## Linux Monolithic Kernel - Modular Architecture

Linux는 핵심 기능이 하나의 거대한 바이너리로 통합된 모놀리틱 커널이지만, 적재 가능 커널 모듈(LKM) 메커니즘을 통해 놀라운 유연성을 보여줍니다.

커널은 핵심 API(심볼)만 외부에 노출하고, 하드웨어 드라이버나 특정 파일 시스템 같은 부가 기능은 .ko 라는 확장자를 가진 모듈로 만듭니다. 이 모듈들은 필요할 때마다 실행 중인 커널에 동적으로 연결(적재)되거나 분리(제거)될 수 있습니다.

핵심 아이디어: "안정적인 코어(Core)는 최소한의 기능만 유지하고, 확장 기능은 플러그인(Plug-in)처럼 붙였다 떼었다 할 수 있게 만든다."

## Serial Number Generator Architecture

### 목표

- 알고리즘 교체 용이성: 새로운 시리얼 번호 생성 알고리즘(예: 더 강력한 암호화 방식)을 기존 코드 수정 없이 쉽게 추가할 수 있어야 한다.

- 다양한 정책 지원: 제품 라인별, 고객 등급별로 각기 다른 시리얼 번호 정책을 동시에 지원할 수 있어야 한다.

- 단순한 사용: 생성기를 사용하는 클라이언트 코드는 내부의 복잡한 알고리즘에 대해 알 필요 없이 간단하게 사용할 수 있어야 한다.

### 설계 구조

#### 1. InputPayload Trait

- 시리얼 번호 생성에 필요한 모든 입력 데이터가 따라야 할 표준 규약을 정의
- 이를 통해 다양한 형태의 입력 데이터를 일관된 방식으로 처리

```
trait InputPayload {
    fn to_canonical_string(&self) -> String;
}

struct StandardInput { customer_id: u64, product_id: u64, tier: String }
impl InputPayload for StandardInput {
    fn to_canonical_string(&self) -> String {
        format!("{}-{}-{}", self.customer_id, self.product_id, &self.tier)
    }
}
```

#### 2. SerialNumberProvider Trait

- 모든 시리얼 번호 생성 알고리즘이 구현해야 할 표준 동작을 정의

```
trait SerialNumberProvider {
    fn generate<T: InputPayload>(&self, payload: &T) -> Result<String, &'static str>;

    fn validate(&self, serial: &str) -> bool;

    fn name(&self) -> &'static str;
}
```

#### 3. Concrete Providers

- `SerialNumberProvider` trait을 실제로 구현한 구현체

```
struct Sha256Provider;
impl SerialNumberProvider for Sha256Provider {
    fn generate<T: InputPayload>(&self, payload: &T) -> Result<String, &'static str> {
        // 시리얼 번호를 생성
    }
    // validate, name 구현
}

struct EccProvider;
// ...
```

#### 4. SerialNumberService

- 시리얼 번호 생성 및 검증 서비스를 제공하는 창구
- 내부적으로 어떤 Provider가 있는지 알고 있지만, 클라이언트에게는 그저 통일된 서비스 API만을 노출

```
use std::collections::HashMap;
use std::sync::Arc;

pub struct SerialNumberService {
    providers: HashMap<&'static str, Arc<dyn SerialNumberProvider>>,
}

impl SerialNumberService {
    pub fn new() -> Self {
        Self { providers: HashMap::new() }
    }

    // 새로운 Provider를 서비스에 등록
    pub fn register(&mut self, provider: Arc<dyn SerialNumberProvider>) {
        self.providers.insert(provider.name(), provider);
    }

    // 이름으로 특정 Provider를 찾아 시리얼 번호 생성을 요청
    pub fn create_serial<T: InputPayload>(&self, provider_name: &str, payload: &T) -> Option<Result<String, &'static str>> {
        self.providers.get(provider_name).map(|p| p.generate(payload))
    }

    // 이름으로 특정 Provider를 찾아 유효성 검증을 요청
    pub fn check_validity(&self, provider_name: &str, serial: &str) -> Option<bool> {
        self.providers.get(provider_name).map(|p| p.validate(serial))
    }
}
```
