use config::File;

#[derive(Clone, Debug, PartialEq)]
pub struct ServerConfig {
    pub path: String,
    pub token: String,
    pub url: String,
    pub port: u16,
    pub ip: String,
}

pub fn parse_config() -> ServerConfig {
    let settings = config::Config::builder()
        .add_source(File::with_name("/etc/media-upload.toml"))
        .build()
        .expect("Error loading config, make sure it is in /etc/media-upload.toml");

    ServerConfig {
        path: settings.get::<String>("path").unwrap(),
        token: settings.get::<String>("token").unwrap(),
        url: settings.get::<String>("url").unwrap(),
        port: settings.get::<u16>("port").unwrap(),
        ip: settings.get::<String>("ip").unwrap(),
    }
}
