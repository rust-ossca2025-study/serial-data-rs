불use crate::plugins::Plugin;

pub trait InputPayload {
    fn to_canonical_string(&self) -> String;
    fn from_canonical_string(s: &str) -> Result<Self, String> where Self: Sized;
}

pub struct StandardInput {
    components: Vec<(String, String)>,
}

impl StandardInput {
    pub fn new() -> Self {
        StandardInput {
            components: Vec::new(),
        }
    }

    pub fn from_plugins(plugins: &[Box<dyn Plugin>]) -> Self {
        let components = plugins
            .iter()
            .map(|p| (p.get_name(), p.get_rawdata()))
            .collect();
        
        StandardInput { components }
    }

    pub fn add_component(&mut self, name: String, value: String) {
        self.components.push((name, value));
    }

}

impl InputPayload for StandardInput {
    fn to_canonical_string(&self) -> String {
        self.components
            .iter()
            .map(|(_, value)| value.clone())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn from_canonical_string(s: &str) -> Result<Self, String> {
        let parts: Vec<String> = s.split('-').map(|p| p.to_string()).collect();
        
        let mut input = StandardInput::new();
        for (i, part) in parts.iter().enumerate() {
            input.add_component(format!("component_{}", i), part.clone());
        }
        
        Ok(input)
    }
}