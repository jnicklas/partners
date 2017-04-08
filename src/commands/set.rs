use git::Config;
use author::Author;
use author_selection::AuthorSelection;
use error::PartnersError;
use Result;
use clap::ArgMatches;

fn filter_authors<'a>(authors: &'a [Author], nicks: &[&str]) -> Result<Vec<Author>> {
    nicks.iter().map(|nick| {
        match authors.iter().find(|a| &a.nick == nick) {
            Some(author) => Ok(author.clone()),
            None => Err(PartnersError::AuthorNotFound(format!("unable to find author with nick: {}", nick)))
        }
    }).collect()
}

pub fn set<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nicks: Vec<&str> = matches.values_of("nicks").unwrap().collect();

    let authors = partners_config.authors()?;

    let filtered_authors = filter_authors(&authors, &nicks)?;
    
    let selection = AuthorSelection::new(partners_config, &filtered_authors)?;

    Config::Global.set_current_author(&selection)?;
            
    println!("{}", Config::Local.current_author()?);

    Ok(())
}