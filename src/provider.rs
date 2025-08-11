use crate::payload::InputPayload;

pub trait SerialNumberProvider: Send + Sync {
    fn generate_from_payload(&self, payload: &dyn InputPayload) -> Result<String, String>;
    fn validate(&self, serial: &str) -> bool;
    fn name(&self) -> &'static str;
    fn description(&self) -> String {
        format!("{} Serial Number Provider", self.name())
    }
}