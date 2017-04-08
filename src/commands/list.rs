use git::Config;
use clap::ArgMatches;
use Result;

pub fn list<'a>(partners_config: &'a Config, _matches: &ArgMatches) -> Result<()> {
    let authors = partners_config.authors()?;

    for author in authors.iter() {
        println!("{}", author);
    }

    Ok(())
}