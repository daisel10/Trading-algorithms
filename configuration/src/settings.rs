use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub okx: OkxConfig,
}

#[derive(Debug, Deserialize)]
pub struct OkxConfig {
    pub api_key: String,
    pub api_secret: String,
}

impl Settings {
    pub fn load() -> Self {
        // Lee RUST_ENV=dev | prod | test (por defecto: dev)
        let environment = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
        let dotenv_path = format!(".env.{}", environment);

        // Carga el archivo .env correspondiente
        dotenvy::from_path(dotenv_path).expect("No se pudo cargar el archivo .env.<env>");

        // Usa config para leer las variables de entorno ya cargadas
        config::Config::builder()
            .add_source(config::Environment::default().separator("__"))
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
