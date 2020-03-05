#[derive(Clone)]
pub struct Config {
    pub region: String,
}

impl Config {
    pub fn from_env() -> Self {
        let region = std::env::var("AWS_REGION").unwrap_or("eu-north-1".to_string());

        Config { region }
    }
}
