## `new_magic_crypt!` macro

### 특징

- AES 기반 암호화 객체를 간편하게 생성할 수 있도록 도와주는 wrapper 매크로입니다.
- MagicCrypt::new(...) 호출을 감싸는 sugar syntax 역할을 합니다.
- 타입 명시 없이 간결하고 직관적인 코드 작성 가능합니다.
- 파라미터
  - $key:expr → 키 문자열 (예: "my-secret-key")
  - $bit:expr → 128 또는 256 같은 정수 리터럴

### 내부적으로 bit 값은 어떻게 해석될까?

1. 매크로는 전달받은 비트 수를 내부적으로 SecureBit enum의 값으로 변환하여 MagicCrypt::new 함수에 전달

   ```
   #[macro_export]
   macro_rules! new_magic_crypt {
       (wrapper $key:expr) => {
           $crate::MagicCrypt::new($key, $crate::SecureBit::Bit128, None::<String>)
       };
       (wrapper $key:expr,64) => {
           $crate::MagicCrypt::new($key, $crate::SecureBit::Bit64, None::<String>)
       };
       (wrapper $key:expr,128) => {
           $crate::MagicCrypt::new($key, $crate::SecureBit::Bit128, None::<String>)
       };
       (wrapper $key:expr,192) => {
           $crate::MagicCrypt::new($key, $crate::SecureBit::Bit192, None::<String>)
       };
       (wrapper $key:expr,256) => {
           $crate::MagicCrypt::new($key, $crate::SecureBit::Bit256, None::<String>)
       };
       ...
   ```

2. SecureBit 값은 MagicCrypt::new 내부에서 match를 통해 적절한 암호 알고리즘으로 분기된다.

   ```
   impl MagicCrypt {
       /// Create a new `MagicCrypt` instance. You may want to use the `new_magic_crypt!` macro.
       pub fn new<S: AsRef<[u8]>, V: AsRef<[u8]>>(
           key: S,
           bit: SecureBit,
           iv: Option<V>,
       ) -> MagicCrypt {
           let cipher = match bit {
               SecureBit::Bit64 => MagicCryptCipher::DES64(MagicCrypt64::new(key, iv)),
               SecureBit::Bit128 => MagicCryptCipher::AES128(MagicCrypt128::new(key, iv)),
               SecureBit::Bit192 => MagicCryptCipher::AES192(MagicCrypt192::new(key, iv)),
               SecureBit::Bit256 => MagicCryptCipher::AES256(MagicCrypt256::new(key, iv)),
           };

           MagicCrypt {
               cipher,
           }
       }
   }
   ```

## Base64와 "==" 패딩의 의미

- MagicCrypt는 암호화된 데이터를 Base64 문자열로 반환합니다.
- Base64 인코딩은 3바이트 → 4문자 단위로 변환되며, 전체 바이트 수가 3의 배수가 아닐 경우 = 또는 == 패딩 문자가 붙습니다.

## Encryption Crate

| 라이브러리      | 특징                                                                 |
| --------------- | -------------------------------------------------------------------- |
| **magic-crypt** | 사용법이 매우 간단함. 학습용 또는 경량 프로젝트에 적합               |
| **aes-gcm**     | 인증된 암호 방식 (AEAD). 민감한 데이터 처리에 적합. 무결성 보장 포함 |
| **ring**        | 고성능 보안 라이브러리. 내부적으로 안전한 기본값 사용. 상업용 적합   |
| **rust-crypto** | 다양한 알고리즘과 블록 모드 지원. 유연하지만 진입장벽이 조금 높음    |
