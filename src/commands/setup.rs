use git::Config;
use Result;
use helpers;
use clap::ArgMatches;

pub fn setup(partners_config: &Config, matches: &ArgMatches) -> Result<()> {
    let domain = helpers::arg_or_query_with_default(matches, "domain", "Email Domain", &partners_config.domain())?;
    partners_config.set_domain(&domain)?;

    let prefix = helpers::arg_or_query_with_default(matches, "prefix", "Email Prefix", &partners_config.prefix())?;
    partners_config.set_prefix(&prefix)?;

    let separator = helpers::arg_or_query_with_default(matches, "separator", "Separator", &partners_config.separator())?;
    partners_config.set_separator(&separator)?;

    Ok(())
}