use std::io;

#[derive(Debug, Error)] pub enum PartnersError {
    RandomError,
    NoSuchCommand,
    HomeDirectoryNotFound,
    NoAuthorSpecified,
    NoGitNick,
    NoGitName,
    NotManagedByPartners,
    #[error(msg_embedded, non_std, no_from)]
    AuthorNotFound(String),
    #[error(msg_embedded, non_std, no_from)]
    GitError(String),
    IoError(io::Error),
    UTF8Error(::std::string::FromUtf8Error),

    CannotProcede
}