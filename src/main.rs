use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "SQLite")]
#[command(author = "Hoan Do. <hoando.dev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Clone Sqlite and research B-tree", long_about = None)]
struct Cli {
    /// Optional database name to activity
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(long, value_name="Hello")]
    create_database: Option<String>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    Create {
        /// lists test values
        name: String,
    },
}
fn main() {
    let cli = Cli::parse();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    print!("{:?}", cli.create_database.as_deref());


    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    println!("{:?}", cli.debug);
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    println!("{:?}", cli.command);
    match &cli.command {
        Some(Commands::Create { name }) => {
            print!("{}", *name);
        }
        None => {}
    }
}
