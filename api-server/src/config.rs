use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    http: Option<Http>,
    pub database: Database,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Http {
    pub port: u16,
}

impl Default for Http {
    fn default() -> Self {
        Http { port: 8080 }
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    server: String,
    database: String,
    username: String,
    password: String,
    pub pool_size: u32,
}

impl Database {
    fn config(&self) -> tiberius::Config {
        let mut config = tiberius::Config::new();

        config.host(&self.server);
        config.database(&self.database);
        config.authentication(tiberius::AuthMethod::sql_server(
            &self.username,
            &self.password,
        ));
        config.trust_cert();

        config
    }
}

impl Config {
    pub fn parse() -> crate::Result<Self> {
        // read file
        let contents = std::fs::read_to_string("config.toml")?;
        log::trace!("Config file contents: {}", contents);

        let config = toml::from_str(&contents)?;
        log::debug!("Parsed config: {:#?}", config);

        log::info!("Config loaded successfully");
        Ok(config)
    }

    pub fn database(&self) -> tiberius::Config {
        self.database.config()
    }

    pub fn http(&mut self) -> &Http {
        if self.http.is_none() {
            log::warn!("HTTP configuration not found, using default");
            self.http = Some(Http::default());
        }

        self.http.as_ref().unwrap()
    }
}
