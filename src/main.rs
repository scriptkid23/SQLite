use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "SQLite")]
#[command(author = "Hoan Do. <hoando.dev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Clone Sqlite and research B-tree", long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Demo
    unk: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: String,
    },
}
fn main() {
    let cli = Cli::parse();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    print!("{:?}", cli.unk);

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
        Some(Commands::Test { list }) => {
            print!("{}", *list);
        }
        None => {}
    }
}
