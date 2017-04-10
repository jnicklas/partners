use std::io::{self, Write};
use Result;
use termion::input::TermRead;
use std::path::Path;
use std::fs::File;
use clap::ArgMatches;

pub fn confirm(prompt: &str) -> Result<bool> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    write!(&mut stdout, "{} (Y/n) ", prompt)?;

    stdout.flush()?;

    match TermRead::read_line(&mut stdin) {
        Ok(Some(ref line)) => {
            Ok(line == "" || line == "y" || line == "y")
        },
        _ => Ok(false)
    }
}

pub fn query(prompt: &str) -> Result<Option<String>> {
    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    write!(&mut stdout, "{}: ", prompt)?;

    stdout.flush()?;

    match TermRead::read_line(&mut stdin)? {
        Some(value) => {
            if value == "" {
                Ok(None)
            } else {
                Ok(Some(value))
            }
        },
        None => Ok(None)
    }
}

pub fn query_required(prompt: &str) -> Result<String> {
    loop {
        match query(prompt)? {
            Some(value) => return Ok(value),
            None => println!("ERROR: {} can't be blank", prompt),
        }
    }
}

pub fn query_optional(prompt: &str) -> Result<Option<String>> {
    let prompt = format!("{} [optional]", prompt);
    query(&prompt)
}

pub fn query_with_default(prompt: &str, default: &str) -> Result<String> {
    let prompt = format!("{} ({})", prompt, default);
    let result = query(&prompt)?.unwrap_or_else(|| String::from(default));
    Ok(result)
}

pub fn arg_or_query_required(matches: &ArgMatches, name: &str, prompt: &str) -> Result<String> {
    match matches.value_of(name) {
        Some(value) => Ok(String::from(value)),
        None => query_required(prompt),
    }
}

pub fn arg_or_query_optional(matches: &ArgMatches, name: &str, prompt: &str) -> Result<Option<String>> {
    match matches.value_of(name) {
        Some(nick) => Ok(Some(String::from(nick))),
        None => query_optional(prompt),
    }
}

pub fn arg_or_query_with_default(matches: &ArgMatches, name: &str, prompt: &str, default: &str) -> Result<String>
{
    match matches.value_of(name) {
        Some(value) => Ok(String::from(value)),
        None => query_with_default(prompt, default),
    }
}

pub fn create_config_file(path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "")?;
    file.sync_all()?;
    Ok(())
}