use git::Config as GitConfig;

#[derive(Debug)]
pub struct Config {
    pub git: GitConfig,
    pub domain: String,
    pub prefix: String,
    pub separator: String,
}

impl Config {
    pub fn from_git(git: GitConfig) -> Config {
        Config {
            domain: git.get("config.domain").unwrap_or_else(|_| "example.com".to_string()),
            prefix: git.get("config.prefix").unwrap_or_else(|_| "dev".to_string()),
            separator: git.get("config.separator").unwrap_or_else(|_| "+".to_string()),
            git: git,
        }
    }

}
