mod cli;
mod store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use store::Store;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut store = Store::load(&cli.file)?;

    match cli.command {
        Command::Add { title, priority } => {
            let t = store.add(&title.join(" "), priority);
            println!("Added #{}: {} (p{})", t.id, t.title, t.priority);
        }
        Command::List { all } => {
            let mut shown: Vec<_> = store.tasks.iter().filter(|t| all || !t.done).collect();
            shown.sort_by_key(|t| (t.done, t.priority, t.id));
            if shown.is_empty() {
                println!("Nothing to do!");
            }
            for t in shown {
                let mark = if t.done { "x" } else { " " };
                println!("[{mark}] #{:<3} p{}  {}", t.id, t.priority, t.title);
            }
        }
        Command::Done { id } => {
            let t = store.complete(id)?;
            println!("Completed #{}: {}", t.id, t.title);
        }
        Command::Remove { id } => {
            let t = store.remove(id)?;
            println!("Removed #{}: {}", t.id, t.title);
        }
    }
    store.save(&cli.file)
}
