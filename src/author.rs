use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Author {
    pub nick: String,
    pub name: String,
    pub email: Option<String>,
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

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:", self.nick)?;
        writeln!(f, "  Name:  {}", self.name)?;
        if let Some(ref email) = self.email.as_ref() {
            writeln!(f, "  Email: {}", email)?;
        }
        Ok(())
    }
}