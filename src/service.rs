use std::collections::HashMap;
use std::sync::Arc;
use crate::payload::InputPayload;
use crate::provider::SerialNumberProvider;

pub struct SerialNumberService {
    providers: HashMap<&'static str, Arc<dyn SerialNumberProvider>>,
}

impl SerialNumberService {
    pub fn new() -> Self {
        SerialNumberService {
            providers: HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Arc<dyn SerialNumberProvider>) {
        let name = provider.name();
        self.providers.insert(name, provider);
    }

    pub fn create_serial(&self, provider_name: &str, payload: &dyn InputPayload) -> Result<String, String> {
        self.providers
            .get(provider_name)
            .ok_or_else(|| format!("Provider '{}' not found", provider_name))
            .and_then(|p| p.generate_from_payload(payload))
    }

    pub fn check_validity(&self, provider_name: &str, serial: &str) -> Result<bool, String> {
        self.providers
            .get(provider_name)
            .ok_or_else(|| format!("Provider '{}' not found", provider_name))
            .map(|p| p.validate(serial))
    }

    pub fn list_providers(&self) -> Vec<(&'static str, String)> {
        self.providers
            .iter()
            .map(|(name, provider)| (*name, provider.description()))
            .collect()
    }

}