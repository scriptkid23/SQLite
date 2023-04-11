mod contants;
mod table;
mod vm;

use clap::Parser;
use table::{Statement, Row};
use std::{fs::File};


#[derive(Parser)]
#[command(name = "SQLite")]
#[command(author = "Hoan Do. <hoando.dev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Clone Sqlite and research B-tree", long_about = None)]
struct Cli {
    /// Optional database name to activity
    database: Option<String>,

    /// Create a database in current folder
    #[arg(short, long, value_name = "database")]
    create: Option<String>,

    #[arg(short, long, value_name = "database")]
    dbinfo: Option<String>,

    #[arg(short, long, value_name = "query")]
    query: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    // You can check the value provided by positional arguments, or option arguments
    if let Some(database) = cli.database.as_deref() {
        let query = cli.query.as_deref();
        if query == None {
            return;
        }
        let table = table::Table::db_open(database);
        let statement = Statement {
          
        };
        vm::execute_insert(, &mut table)
        //TODO: query here!


        
        

    }

    if let Some(database) = cli.create.as_deref() {
        File::create(database).expect("create database failed");
        println!("Executed done!");
    }

    if let Some(database) = cli.dbinfo.as_deref() {}
}
