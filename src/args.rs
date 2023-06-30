use anyhow::{anyhow, Result};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, Arg,
    Command as ClapApp,
};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::{env, fs::{self, File}, path::PathBuf};

pub struct CliArgs {
     pub theme: PathBuf,
     pub contacts: PathBuf,
     pub phone: PathBuf
 }

 pub fn process_cmdline() -> Result<CliArgs> {
     let app = app();

     let arg_matches = app.get_matches();


     Ok(CliArgs {
         theme,
         contacts,
         phone
     })
 }

fn setup_logging() -> Result<()> {
    let mut path = get_app_cache_path()?;
    path.push("contacts.log");

    println!("Logging enabled. log written to {path:?}");

    WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        File::create(path)?,
    )?;

    Ok(())
}

fn get_app_cache_path() -> Result<PathBuf> {
    let mut path = dirs_next::cache_dir()
        .ok_or_else(|| anyhow!("failed to get o/s cache dir."))?;

    path.push("contacts");
    fs::create_dir_all(&path)?;

    Ok(path)
}


fn app() -> ClapApp {
    ClapApp::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .help_template(
            "\
{before-help}contacts {version}
{author}
{about}

{usage-heading} {usage}

{all-args}{after-help}
            ",
        ).arg(
            Arg::new("theme")
                .help("Set the color theme (defaults to theme.ron)")
                .short('t')
                .long("theme")
                .value_name("THEME")
                .num_args(1),
        ).arg(
            Arg::new()
                .help("Set the file to store the contacts in")
                .short('c')
                .long("contacts")
                .value_name("CONTACTS")
                .num_args(1)
        ).arg(
            Arg::new()
                .help("Set the phone configuration file")
                .short('p')
                .long("phone")
                .value_name("PHONE")
                .num_args(1)
        ).arg(
            Arg::new()
                .help("Stores logging output into a cache directory")
                .short('l')
                .long("logging")
                .num_args(0)
        ).arg(
            Arg::new()
                .help("Hides the splash screen (enabled by default)")
                .long("no-splash")
                .num_args(0)
        )
}