use git::Config;
use author::Author;
use error::PartnersError;
use Result;

pub struct AuthorSelection<'a> {
    config: &'a Config<'a>,
    authors: Vec<Author>,
}

impl<'a> AuthorSelection<'a> {
    pub fn new(config: &'a Config<'a>, mut authors: Vec<Author>) -> Result<AuthorSelection<'a>> {
        if authors.len() == 0 {
            Err(PartnersError::NoAuthorSpecified)
        } else {
            authors.sort();
            Ok(AuthorSelection { config: config, authors: authors })
        }
    }

    pub fn name(&self) -> String {
        if self.authors.len() == 1 {
            self.authors[0].name.to_string()
        } else {
            self.authors.iter().map(|a| a.name.to_string()).collect::<Vec<_>>().join(", ")
        }
    }

    pub fn nick(&self) -> String {
        if self.authors.len() == 1 {
            self.authors[0].nick.to_string()
        } else {
            self.authors.iter().map(|a| a.nick.to_string()).collect::<Vec<_>>().join(&self.config.separator())
        }
    }

    pub fn email(&self) -> String {
        if self.authors.len() == 1 {
            if let Some(ref email) = self.authors[0].email.as_ref() {
                email.to_string()
            } else {
                format!("{}@{}", self.nick(), self.config.domain())
            }
        } else {
            format!("{}{}{}@{}", self.config.prefix(), self.config.separator(), self.nick(), self.config.domain())
        }
    }
}