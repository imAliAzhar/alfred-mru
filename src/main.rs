mod actions;
mod cli;
mod consts;

use actions::action::Action;

fn main() {
    let command = cli::setup_command();
    let (action, args) = cli::get_args(command);

    match action {
        Action::Cache => actions::cache_path(args[0].to_owned()),

        Action::Sort => actions::sort_paths(args),
    };
}
