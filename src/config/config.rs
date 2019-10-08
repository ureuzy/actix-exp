#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "API_KEY", default = "")]
    pub api_key: String,

    #[envconfig(from = "API_SECRET", default = "")]
    pub api_secret: String,
}