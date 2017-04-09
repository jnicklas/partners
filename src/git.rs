use std::process::{Command};
use std::path::Path;
use PartnersError;
use author::Author;
use author_selection::AuthorSelection;
use Result;

#[derive(Debug)]
pub enum Config<'a> {
    File(&'a Path),
    Global,
    Local,
}

fn read_result(command: &mut Command) -> Result<String> {
    let result = command.output()?;

    if result.status.success() {
        let string = String::from_utf8(result.stdout)?;
        Ok(string.trim().to_string())
    } else {
        let string = String::from_utf8(result.stderr)?;
        Err(PartnersError::GitError(string))
    }
}

impl<'a> Config<'a> {
    fn command(&self) -> Command {
        let mut command = Command::new("git");
        match *self {
            Config::File(ref path) => command.arg("config").arg("-f").arg(path),
            Config::Global => command.arg("config").arg("--global"),
            Config::Local => command.arg("config")
        };
        command
    }

    fn get(&self, key: &str) -> Result<String> {
        let mut command = self.command();

        read_result(command.arg(key))
    }

    fn set(&self, key: &str, value: &str) -> Result<()> {
        let mut command = self.command();

        read_result(command.arg(key).arg(value))?;

        Ok(())
    }

    fn list(&self, keyexp: &str) -> Result<Vec<String>> {
        let mut command = self.command();

        let string = read_result(command.arg("--get-regexp").arg(keyexp))?;

        Ok(string.split('\n').map(ToString::to_string).collect())
    }

    pub fn domain(&self) -> String {
        self.get("config.domain").unwrap_or_else(|_| "example.com".to_string())
    }

    pub fn prefix(&self) -> String {          
        self.get("config.prefix").unwrap_or_else(|_| "dev".to_string())
    }

    pub fn separator(&self) -> String { 
        self.get("config.separator").unwrap_or_else(|_| "+".to_string())
    }

    pub fn authors(&self) -> Result<Vec<Author>> {
        let lines = self.list("author.\\w+.name")?;
        lines.iter().map(|line| {
            let mut parts = line.splitn(2, ' ');

            let nick = parts.next().expect("does not contain nick").split('.').nth(1).unwrap().to_string();
            let name = parts.next().expect("does not contain name").to_string();
            let email = self.get(&format!("author.{}.email", nick)).ok();

            Ok(Author { nick: nick, name: name, email: email })
        }).collect()
    }

    pub fn add_author(&self, author: &Author) -> Result<()> {
        self.set(&format!("author.{}.name", author.nick), &author.name)?;
        
        if let Some(ref email) = author.email {
            self.set(&format!("author.{}.email", author.nick), email)?;
        }
        Ok(())
    }

    pub fn set_current_author(&self, current: &AuthorSelection) -> Result<()> {
        self.set("partners.current", &current.nick())?;
        self.set("user.name", &current.name())?;
        self.set("user.email", &current.email())?;
        Ok(())
    }

    pub fn current_author(&self) -> Result<Author> {
        let nick = self.get("partners.current").map_err(|_| PartnersError::NoGitNick)?;
        let name = self.get("user.name").map_err(|_| PartnersError::NoGitName)?;
        let email = self.get("user.email").ok();

        Ok(Author { nick: nick, name: name, email: email })
    }

    pub fn find_authors(&'a self, nicks: &[&str]) -> Result<AuthorSelection<'a>> {
        let authors = self.authors()?;
        let authors: Result<Vec<Author>> = nicks.iter().map(|nick| {
            match authors.iter().find(|a| &a.nick == nick) {
                Some(author) => Ok(author.clone()),
                None => Err(PartnersError::AuthorNotFound(format!("unable to find author with nick: {}", nick)))
            }
        }).collect();
        AuthorSelection::new(self, authors?)
    }

    pub fn remove_author(&self, nick: &str) -> Result<()> {
        let section_name = format!("author.{}", nick);
        let mut command = self.command();

        // trying to remove a section that doesn't exist fails ungracefully, so check whether author exists first.
        if let Ok(_) = self.find_authors(&[nick]) {
            read_result(command.arg("--remove-section").arg(&section_name))?;
        } else {
            // probably already removed, do nothing
        }

        Ok(())
    }
}
