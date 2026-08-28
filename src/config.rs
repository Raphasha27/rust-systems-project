pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost:5432/app".to_string()),
        }
    }
}
