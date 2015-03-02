use config::Config;
use std::borrow::Cow;
use std::rc::Rc;
use std::cmp::Ordering;
use super::AuthorInformation;

#[derive(Debug)]
pub struct Author {
    pub config: Rc<Config>,
    pub nick: String,
    pub name: String,
    pub email: Option<String>,
}

impl<'a> AuthorInformation for &'a Author {
    fn get_name(&self) -> Cow<str> {
        Cow::Borrowed(&self.name)
    }

    fn get_nick(&self) -> Cow<str> {
        Cow::Borrowed(&self.nick)
    }

    fn get_email(&self) -> Cow<str> {
        match self.email {
            Some(ref email) => Cow::Borrowed(&email),
            None => Cow::Owned(format!("{}@{}", self.nick, self.config.domain)),
        }
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Author) -> bool {
        self.nick.eq(&other.nick)
    }
}

impl PartialOrd for Author {
    fn partial_cmp(&self, other: &Author) -> Option<Ordering> {
        self.nick.partial_cmp(&other.nick)
    }
}

impl Eq for Author {}

impl Ord for Author {
    fn cmp(&self, other: &Author) -> Ordering {
        self.nick.cmp(&other.nick)
    }
}
