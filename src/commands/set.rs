use git::Config;
use Result;
use clap::ArgMatches;

pub fn set<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nicks: Vec<&str> = matches.values_of("nicks").unwrap().collect();

    let selection = partners_config.find_authors(&nicks)?;

    Config::Global.set_current_author(&selection)?;

    println!("{}", Config::Local.current_author()?);

    Ok(())
}