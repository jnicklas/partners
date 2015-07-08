use config::Config;
use std::borrow::Cow;
use author_information::AuthorInformation;
use concat::IteratorConcatExt;
use author::Author;
use std::rc::Rc;

#[derive(Debug)]
pub struct Pair<'a> {
    pub config: Rc<Config>,
    pub authors: &'a [&'a Author]
}

impl<'a> AuthorInformation for &'a Pair<'a> {
    fn get_name(&self) -> Cow<str> {
        Cow::Owned(self.authors.iter().map(|a| &*a.name).connect(" and "))
    }

    fn get_nick(&self) -> Cow<str> {
        Cow::Owned(self.authors.iter().map(|a| &*a.nick).connect(&self.config.separator))
    }

    fn get_email(&self) -> Cow<str> {
        let names = self.authors.iter().map(|a| &*a.nick).connect(&self.config.separator);
        Cow::Owned(format!("{}{}{}@{}", self.config.prefix, self.config.separator, names, self.config.domain))
    }
}
