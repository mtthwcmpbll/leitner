use std::path::PathBuf;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde_json;
use clap::{Parser, Subcommand};
use leitner_core::{FactRepository, InMemoryFactRepository, Fact};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Add {
        #[clap(short, long, action)]
        question: String,
        #[clap(short, long, action)]
        answer: String,
    },
}

fn connect_repository(repo_path: &PathBuf) -> Result<InMemoryFactRepository, Box<dyn Error>> {
    let repo: InMemoryFactRepository;
    if !repo_path.exists() {
        // create a new empty repository
        repo = InMemoryFactRepository::new();
    } else {
        let mut options = OpenOptions::new();
        let mut repo_file = options.read(true).open(repo_path.as_os_str())?;
        let mut repo_json = String::new();
        repo_file.read_to_string(&mut repo_json)?;
        repo = serde_json::from_str(repo_json.as_str())?;
    }
    return Ok(repo);
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // load the repository
    let repo_path = PathBuf::from("./leitner.json");
    let mut repo = connect_repository(&repo_path)?;

    match cli.command {
        Some(Commands::Add { question, answer }) => {
            println!("{}", question);
            println!("{}", answer);

            repo.add_fact(Fact::new(question, answer));
        }
        None => println!("No command given"),
    }

    // match &cli.command {
    //     Commands::Add { question, answer} => {
    //         println!("'myapp add' was used, name is: {:?}, {:?}", question, answer)
    //     }
    // }

    // save the repository
    let repo_json: String = serde_json::to_string(&repo)?;
    println!("{}", repo_json);
    let mut options = OpenOptions::new();
    let mut repo_file = options.write(true).create(true).open(repo_path)?;
    repo_file.write_all(repo_json.as_bytes())?;

    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}