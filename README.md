**Cargo plugin to easily query information from Cargo.toml file.**

### Overview
This plugin helps querying information from a Cargo.toml file and can be used in shell scripts or CI/CD pipelines.
The plugin accepts only one flag at a time and returns a single string with the requested value.
Some queries like eq. keywords or authors return multiple values which get separated by line breaks (each value on a new line).

This plugin is inspired by [cargo-edit](https://github.com/killercup/cargo-edit) and presents itself as a lightweight wrapper around [cargo_toml](https://gitlab.com/crates.rs/cargo_toml)


### Contribution
Thanks for your interest - we gratefully welcome contributions.

Questions can be asked in issues.
To help us help you get pull requests merged quickly and smoothly, open an issue before submitted large changes. Please keep the contents of pull requests and commits short. Commit messages should include the intent of the commit.
cargo-info uses rustfmt for formatting and clippy for linting.

### Installation
Ensure that you have a fairly recent version of rust/cargo installed.
```
$ cargo install cargo-info
```

(Please check cargo's documentation to learn how cargo install works and how to set up your system so it finds binaries installed by cargo.)


### Examples

#### All Options
```
$ cargo info -h
Cargo Info
Nicolai Unrein <info@auxcontrol.io>
Query package info from Cargo.toml in a script-friendly way.

USAGE:
    cargo info <--version|--authors|--edition|--name|--homepage|--keywords|--license|--links|--description|--categories>

FLAGS:
    -v, --version        get package version
    -a, --authors        get package authors
    -e, --edition        get package edition
    -n, --name           get package name
    -o, --homepage       get package homepage
    -k, --keywords       get package keywords
    -l, --license        get package license
    -i, --links          get package links
    -d, --description    get package description
    -c, --categories     get package categories
    -h, --help           Prints help information
```

#### Get Version
```
$ cargo info -v
0.2.1

```

#### Get keywords 
```
$ cargo info -k
command-line-utilities
development-tools::cargo-plugins

```

#### Custom delimiter
```
$ cargo info -k --delimiter Tab
command-line-utilities 	development-tools::cargo-plugins

$ cargo info -k --delimiter=" -- "
command-line-utilities -- development-tools::cargo-plugins

$ cargo info -k --delimiter=";"
command-line-utilities;development-tools::cargo-plugins

```

#### Optional entry point 
```
$ cargo info -n --root="../../some/other/project/Cargo.toml"
some-other-project

$ cargo info -n --root="../../some/other/project"
some-other-project

$ cargo info -n
current-project

```


