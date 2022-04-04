use anyhow::anyhow;
mod cli;
mod tasks;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use tasks::Task;

fn main() -> anyhow::Result<()> {
    // println!("{:#?}", cli::CommandLineArgs::from_args());
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .expect("Failed to find journal file");

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    // .expect("Failed to perform action")

    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    let journal_file_name = ".rust-todo-journal.json";

    home::home_dir().map(|mut path| {
        path.push(journal_file_name);
        path
    })
}
