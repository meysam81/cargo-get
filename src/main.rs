mod delimiter;
mod error;
mod version;

use cargo_toml::Manifest;
use clap::{App, AppSettings, Arg, ArgGroup, ArgMatches};
use delimiter::Delimiter;
use error::InheritanceError;
use error::InvalidSemver;
use error::NotFound;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::version::match_version;

fn main() -> Result<(), Box<dyn Error>> {
    let args = get_args();
    let app = make_app();

    let matches = app.get_matches_from(args);

    let entry_point = match matches.value_of("root") {
        Some(p) => p.parse()?,
        None => env::current_dir()?,
    };

    let entry_point_absolute =
        fs::canonicalize(entry_point).map_err(|_| "No such file or directory")?;

    let manifest_path = search_manifest_path(&entry_point_absolute).ok_or("No manifest found")?;

    let manifest = Manifest::from_path(&manifest_path)?;

    if let Err(err) = output(&matches, manifest) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}

// Remove get argument in order to make it work with or without `get` subcommand
fn get_args() -> Vec<String> {
    let mut args: Vec<_> = std::env::args().collect();

    if args.get(1) == Some(&"get".to_owned()) {
        args.remove(1);
    }

    args
}

pub fn make_app() -> App<'static, 'static> {
    App::new("cargo-get")
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::VersionlessSubcommands)
        .author("Nicolai Unrein <n.unrein@gmail.com>")
        .about("Query package info from Cargo.toml in a script-friendly way.")
        .arg(
            Arg::with_name("authors")
                .long("authors")
                .short("a")
                .hidden(true)
                .help("get package.authors"),
        )
        .arg(
            Arg::with_name("edition")
                .long("edition")
                .short("e")
                .hidden(true)
                .help("get package.edition"),
        )
        .arg(
            Arg::with_name("name")
                .long("name")
                .short("n")
                .hidden(true)
                .help("get package.name"),
        )
        .arg(
            Arg::with_name("homepage")
                .long("homepage")
                .short("o")
                .hidden(true)
                .help("get package.homepage"),
        )
        .arg(
            Arg::with_name("keywords")
                .long("keywords")
                .short("k")
                .hidden(true)
                .help("get package keywords"),
        )
        .arg(
            Arg::with_name("license")
                .long("license")
                .short("l")
                .hidden(true)
                .help("get package license"),
        )
        .arg(
            Arg::with_name("links")
                .long("links")
                .short("i")
                .hidden(true)
                .help("get package links"),
        )
        .arg(
            Arg::with_name("description")
                .long("description")
                .short("d")
                .hidden(true)
                .help("get package description"),
        )
        .arg(
            Arg::with_name("categories")
                .long("categories")
                .short("c")
                .hidden(true)
                .help("get package categories"),
        )
        .arg(
            Arg::with_name("root")
                .long("root")
                .help("optional entry point")
                .value_name("PATH"),
        )
        .arg(
            Arg::with_name("delimiter")
                .long("delimiter")
                .help("specify delimiter for values")
                .value_name("Tab | CR | LF | CRLF | String")
                .global(true),
        )
        .group(ArgGroup::with_name("version-group").requires("version"))
        .group(ArgGroup::with_name("get").required(false).args(&[
            "authors",
            "edition",
            "name",
            "homepage",
            "keywords",
            "license",
            "links",
            "description",
            "categories",
        ]))
        .subcommand(
            App::new("package.version")
                .alias("version")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::GlobalVersion)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::VersionlessSubcommands)
                .about("get package version")
                .arg(
                    Arg::with_name("full")
                        .long("full")
                        .help("get full version")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "pretty"])
                        .hidden(true),
                )
                .arg(
                    Arg::with_name("pretty")
                        .long("pretty")
                        .help("get pretty version eg. v1.2.3")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "full"]),
                )
                .arg(Arg::with_name("major").long("major").help("get major part"))
                .arg(Arg::with_name("minor").long("minor").help("get minor part"))
                .arg(Arg::with_name("patch").long("patch").help("get patch part"))
                .arg(Arg::with_name("build").long("build").help("get build part"))
                .arg(
                    Arg::with_name("pre")
                        .long("pre")
                        .help("get pre-release part"),
                ),
        )
        .subcommand(App::new("package.authors").about("get package authors"))
        .subcommand(App::new("package.categories").about("get package categories"))
        .subcommand(App::new("package.description").about("get package description"))
        .subcommand(App::new("package.edition").about("get package edition"))
        .subcommand(App::new("package.homepage").about("get package homepage"))
        .subcommand(App::new("package.keywords").about("get package keywords"))
        .subcommand(App::new("package.license").about("get package license"))
        .subcommand(App::new("package.version").about("get package version"))
        .subcommand(App::new("workspace.members").about("get workspace members"))
        .subcommand(App::new("workspace.package.authors").about("get workspace template authors"))
        .subcommand(
            App::new("workspace.package.categories").about("get workspace template categories"),
        )
        .subcommand(
            App::new("workspace.package.description").about("get workspace template description"),
        )
        .subcommand(App::new("workspace.package.edition").about("get workspace template edition"))
        .subcommand(App::new("workspace.package.homepage").about("get workspace template homepage"))
        .subcommand(App::new("workspace.package.keywords").about("get workspace template keywords"))
        .subcommand(App::new("workspace.package.license").about("get workspace template license"))
        .subcommand(
            App::new("workspace.package.version")
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::GlobalVersion)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::VersionlessSubcommands)
                .about("get workspace template version")
                .arg(
                    Arg::with_name("full")
                        .long("full")
                        .help("get full version")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "pretty"])
                        .hidden(true),
                )
                .arg(
                    Arg::with_name("pretty")
                        .long("pretty")
                        .help("get pretty version eg. v1.2.3")
                        .conflicts_with_all(&["major", "minor", "patch", "build", "pre", "full"]),
                )
                .arg(Arg::with_name("major").long("major").help("get major part"))
                .arg(Arg::with_name("minor").long("minor").help("get minor part"))
                .arg(Arg::with_name("patch").long("patch").help("get patch part"))
                .arg(Arg::with_name("build").long("build").help("get build part"))
                .arg(
                    Arg::with_name("pre")
                        .long("pre")
                        .help("get pre-release part"),
                ),
        )
}

pub fn output(matches: &ArgMatches, manifest: Manifest) -> Result<(), Box<dyn Error>> {
    let package = || manifest.package.clone().ok_or(NotFound("package"));
    let workspace = || manifest.workspace.clone().ok_or(NotFound("workspace"));
    let ws_package = || workspace().and_then(|ws| ws.package.ok_or(NotFound("workspace.package")));

    let delimiter: Delimiter = matches
        .value_of("delimiter")
        .map(|s| s.parse().unwrap())
        .unwrap_or_default();

    let delim_string = delimiter.to_string();

    if let Some(version) = matches.subcommand_matches("package.version") {
        let v: semver::Version = package()?
            .version
            .get()
            .or(Err(InheritanceError("package.version")))?
            .parse()
            .map_err(InvalidSemver)?;

        match_version(version, v, &delimiter)?;
    }

    if matches.is_present("name") {
        println!("{}", package()?.name);
    } else if matches.is_present("homepage") {
        println!(
            "{}",
            package()?
                .homepage
                .unwrap_or_default()
                .get()
                .or(Err(InheritanceError("package.homepage")))?
        );
    } else if matches.is_present("license") {
        println!(
            "{}",
            package()?
                .license
                .unwrap_or_default()
                .get()
                .or(Err(InheritanceError("package.license")))?
        );
    } else if matches.is_present("description") {
        println!(
            "{}",
            package()?
                .description
                .unwrap_or_default()
                .get()
                .or(Err(InheritanceError("package.description")))?
        );
    } else if matches.is_present("links") {
        println!("{}", package()?.links.unwrap_or_default());
    } else if matches.is_present("authors") {
        println!(
            "{}",
            package()?
                .authors
                .get()
                .or(Err(InheritanceError("package.authors")))?
                .join(&delim_string)
        )
    } else if matches.is_present("keywords") {
        println!(
            "{}",
            package()?
                .keywords
                .get()
                .or(Err(InheritanceError("package.keywords")))?
                .join(&delim_string)
        )
    } else if matches.is_present("categories") {
        println!(
            "{}",
            package()?
                .categories
                .get()
                .or(Err(InheritanceError("package.categories")))?
                .join(&delim_string)
        )
    } else if matches.is_present("edition") {
        let edition = match package()?
            .edition
            .get()
            .or(Err(InheritanceError("package.edition")))?
        {
            cargo_toml::Edition::E2015 => "2015",
            cargo_toml::Edition::E2018 => "2018",
            cargo_toml::Edition::E2021 => "2021",
        };
        println!("{}", edition);
    } else if let Some(version) = matches.subcommand_matches("workspace.package.version") {
        let v: semver::Version = ws_package()?
            .version
            .ok_or(NotFound("workspace.package.version"))?
            .parse()
            .map_err(InvalidSemver)?;

        match_version(version, v, &delimiter)?;
    }

    Ok(())
}

fn search_manifest_path(dir: &Path) -> Option<PathBuf> {
    let manifest = dir.join("Cargo.toml");

    if fs::metadata(&manifest).is_ok() {
        Some(manifest)
    } else {
        dir.parent().and_then(search_manifest_path)
    }
}
