mod commands;
mod models;
mod storage;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "listman")]
#[command(about = "A beautiful CLI list manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        name: String,
    },

    Add {
        list: String,
        item: String,
    },

    View,
}

fn main() {
    let cli = Cli::parse();

    let mut lists = storage::load_lists();

    match cli.command {
        Commands::Create { name } => {
            commands::create_list(&mut lists, name);
        }

        Commands::Add { list, item } => {
            commands::add_item(&mut lists, list, item);
        }

        Commands::View => {
            commands::view_lists(&lists);
        }
    }

    storage::save_lists(&lists);
}
