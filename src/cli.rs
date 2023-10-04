use clap::{command, Arg, ArgAction, Command};

use crate::actions::action::Action;

pub fn setup_command() -> Command {
    command!()
        .subcommand(
            Command::new("sort")
                .about("Sort paths in most recently used order")
                .arg(Arg::new("paths").required(true).action(ArgAction::Append)),
        )
        .subcommand(
            Command::new("cache")
                .about("Cache path")
                .arg(Arg::new("path").required(true)),
        )
}

pub fn get_args(command: Command) -> (Action, Vec<String>) {
    let arg_matches = command.get_matches();

    let (subcommand, arg_matches) = arg_matches.subcommand().unwrap();

    match subcommand {
        "cache" => {
            let path = arg_matches.get_one::<String>("path").unwrap().clone();
            let args = vec![path.to_owned()];

            (Action::Cache, args)
        }

        "sort" => {
            let args = arg_matches
                .get_many::<String>("paths")
                .unwrap()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();

            (Action::Sort, args)
        }

        _ => todo!(),
    }
}
