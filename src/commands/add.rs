use git::Config;
use clap::ArgMatches;
use author::Author;
use helpers;
use Result;

pub fn add<'a>(partners_config: &'a Config, matches: &ArgMatches) -> Result<()> {
    let nick = helpers::arg_or_query_required(matches, "nick", "Nick")?;
    let name = helpers::arg_or_query_required(matches, "name", "Name")?;
    let email = helpers::arg_or_query_optional(matches, "email", "Email")?;

    let author = Author { nick: nick, name: name, email: email };
    partners_config.add_author(&author)?;

    Ok(())
}