#[derive(Debug)]
pub struct Config {
    pub domain: String,
    pub prefix: String,
    pub separator: String,
}

impl Config {
    pub fn from_git(config: &::git2::Config) -> Config {
        Config {
            domain: config.get_string("config.domain").unwrap_or_else(|_| "example.com".to_string()),
            prefix: config.get_string("config.prefix").unwrap_or_else(|_| "dev".to_string()),
            separator: config.get_string("config.separator").unwrap_or_else(|_| "+".to_string()),
        }
    }

}
