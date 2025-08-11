use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use crate::payload::InputPayload;
use crate::provider::SerialNumberProvider;

pub struct MagicCryptProvider {
    key: String,
}

impl MagicCryptProvider {
    pub fn with_default_key() -> Self {
        MagicCryptProvider {
            key: "magickey".to_string(),
        }
    }
}

impl SerialNumberProvider for MagicCryptProvider {
    fn generate_from_payload(&self, payload: &dyn InputPayload) -> Result<String, String> {
        let canonical = payload.to_canonical_string();
        let mc = new_magic_crypt!(&self.key, 256);
        Ok(mc.encrypt_str_to_base64(&canonical))
    }

    fn validate(&self, serial: &str) -> bool {
        let mc = new_magic_crypt!(&self.key, 256);
        mc.decrypt_base64_to_string(serial).is_ok()
    }

    fn name(&self) -> &'static str {
        "MagicCrypt"
    }

    fn description(&self) -> String {
        "MagicCrypt AES-256 based serial number provider".to_string()
    }
}