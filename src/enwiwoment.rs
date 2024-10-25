use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuwation {
    #[serde(default = "default_address")]
    pub address: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_methods")]
    pub methods: Vec<String>,
}

impl Configuwation {
    pub fn from_env() -> Result<Configuwation, envy::Error> {
        envy::prefixed("CAT_").from_env()
    }
}

fn default_address() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    8080
}
fn default_methods() -> Vec<String> {
    vec!["MEOW".to_string()]
}
