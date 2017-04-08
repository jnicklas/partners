use std::io::{self, Write};
use Result;
use termion::input::TermRead;
use std::path::Path;
use std::fs::File;

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

pub fn create_config_file(path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "")?;
    file.sync_all()?;
    Ok(())
}