use config::Config;
use std::borrow::Cow;
use std::string::CowString;
use std::rc::Rc;

#[derive(Debug)]
pub struct Author {
    pub config: Rc<Config>,
    pub nick: String,
    pub name: String,
    pub email: Option<String>,
}

impl Author {
    pub fn get_email(&self) -> CowString {
        match self.email {
            Some(ref email) => Cow::Borrowed(email.as_slice()),
            None => Cow::Owned(format!("{}@{}", self.nick, self.config.domain)),
        }
    }
}

