use clap::{ArgAction, Command, arg, command, value_parser};

pub mod subcommands {
    use clap::Arg;

    use crate::ProjectSource;

    use super::*;

    pub fn new() -> Command {
        Command::new("new")
            .about("Creates a new project from <TEMPLATE> in a subfolder.")
            .arg(
                arg!([TEMPLATE] "Template specifier: a local file path, a git URL (ie https://github.com/user/repo.git), or a shorthand for a Github repository (USER/REPO). Required if --local, --git, or --github aren't specified.")
                .value_parser(value_parser!(ProjectSource))
            )
            .arg(arg!(--local <SOURCE> "Loads a template from a local file path").value_parser(value_parser!(std::path::PathBuf)))
            .arg(arg!(--git <SOURCE> "Loads a template from a git repository URL").value_parser(value_parser!(url::Url)))
            .arg(arg!(--github <SOURCE> "Loads a template from a Github repo, with the format \"USER/REPO\"."))
            .arg(Arg::new("overwrite").long("overwrite").action(ArgAction::SetTrue).help("Allows overwriting of the selected project directory, if it exists."))
    }

    pub fn init() -> Command {
        Command::new("init")
            .about("Creates a new project from <TEMPLATE> in the current directory.")
            .arg(
                arg!([TEMPLATE] "Template specifier: a local file path, a git URL (ie https://github.com/user/repo.git), or a shorthand for a Github repository (USER/REPO). Required if --local, --git, or --github aren't specified.")
                .value_parser(value_parser!(ProjectSource))
            )
            .arg(arg!(--local <SOURCE> "Loads a template from a local file path").value_parser(value_parser!(std::path::PathBuf)))
            .arg(arg!(--git <SOURCE> "Loads a template from a git repository URL").value_parser(value_parser!(url::Url)))
            .arg(arg!(--github <SOURCE> "Loads a template from a Github repo, with the format \"USER/REPO\"."))
            .arg(Arg::new("overwrite").long("overwrite").action(ArgAction::SetTrue).help("Allows overwriting of files if the current directory is not empty."))
    }

    pub fn schema() -> Command {
        Command::new("schema")
            .about("Generates JSON schemas for configuration types, outputting to the current directory by default.")
            .arg(Arg::new("config").long("config").short('c').help("Output file for TemplateConfig schema").value_name("FILE").value_parser(value_parser!(std::path::PathBuf)).default_value("femto-template.schema.json"))
            .arg(Arg::new("project").long("project").short('p').help("Output file for ProjectConfig schema").value_name("FILE").value_parser(value_parser!(std::path::PathBuf)).default_value("femto-project.schema.json"))
    }
}

pub fn femto_cli() -> Command {
    command!()
        .arg(arg!(-v --verbose ... "Verbosity level, up to -vvv"))
        .subcommand(subcommands::new())
        .subcommand(subcommands::init())
        .subcommand(subcommands::schema())
}
