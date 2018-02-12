use git::Config;
use helpers;
use author::Author;
use author_selection::AuthorSelection;
use xdg::BaseDirectories;
use Result;
use CannotProcede;

pub fn initial() -> Result<Config> {
    let xdg_dirs = BaseDirectories::with_prefix("partners")?;

    let config_path = xdg_dirs.place_config_file("partners.cfg")?;

    if !config_path.exists() {
        println!("config file not found at {:?}", config_path);

        if helpers::confirm("do you want to create it?")? {
            helpers::create_config_file(&config_path)?;
        } else {
            Err(CannotProcede)?;
        }
    }

    let partners_config = Config::File(config_path);

    let author = match Config::Local.current_author() {
        Ok(author) => author,
        Err(_) => {
            println!("It seems like the current git author is not known to partners");

            let nick = Config::Local.nick().or_else(|_| {
                println!("Please enter a nickname you would like to use");
                helpers::query_required("Nick")
            })?;

            let name = Config::Local.user_name().or_else(|_| {
                println!("Unable to determine your name from git configuration");
                helpers::query_required("Name")
            })?;

            let email = Config::Local.user_email().ok().or_else(|| {
                println!("Unable to determine your email address from git configuration");
                helpers::query_optional("Email").ok().and_then(|v| v)
            });

            let author = Author { nick: nick, name: name, email: email };

            let selection = AuthorSelection::new(&partners_config, vec![author.clone()])?;
            Config::Global.set_current_author(&selection)?;

            author
        }
    };

    if partners_config.find_author(&author.nick).is_none() {
        partners_config.add_author(&author)?;
    }

    Ok(partners_config)
}