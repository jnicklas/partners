use config::Config;
use std::borrow::Cow;
use std::rc::Rc;
use std::cmp::Ordering;
use author_information::AuthorInformation;
use std::fmt;

#[derive(Debug)]
pub struct Author {
    pub config: Rc<Config>,
    pub nick: String,
    pub name: String,
    pub email: Option<String>,
}

impl Author {
    pub fn new<T1, T2, T3>(
        config: &Rc<Config>, nick: T1, name: T2, email: Option<T3>
    ) -> Author where T1: Into<String>, T2: Into<String>, T3: Into<String> {
        Author {
            config: config.clone(),
            nick: nick.into(),
            name: name.into(),
            email: email.map(|e| e.into())
        }
    }
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(self.nick.fmt(f));
        try!(":\n  Name:  ".fmt(f));
        try!(self.name.fmt(f));
        try!("\n  Email: ".fmt(f));
        try!(self.get_email().fmt(f));
        Ok(())
    }
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
