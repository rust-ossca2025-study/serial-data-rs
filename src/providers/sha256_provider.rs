use sha2::{Sha256, Digest};
use hex;
use crate::payload::InputPayload;
use crate::provider::SerialNumberProvider;

pub struct Sha256Provider {
    salt: String,
}

impl Sha256Provider {
    pub fn with_default_salt() -> Self {
        Sha256Provider {
            salt: "serial_salt_2024".to_string(),
        }
    }
}

impl SerialNumberProvider for Sha256Provider {
    fn generate_from_payload(&self, payload: &dyn InputPayload) -> Result<String, String> {
        let canonical = payload.to_canonical_string();
        let salted = format!("{}{}", canonical, self.salt);
        
        let mut hasher = Sha256::new();
        hasher.update(salted.as_bytes());
        let result = hasher.finalize();
        
        let hex_hash = hex::encode(result);
        Ok(format!("SHA-{}", &hex_hash[..16].to_uppercase()))
    }

    fn validate(&self, serial: &str) -> bool {
        serial.starts_with("SHA-") && serial.len() == 20
    }

    fn name(&self) -> &'static str {
        "SHA256"
    }

    fn description(&self) -> String {
        "SHA-256 hash based serial number provider".to_string()
    }
}