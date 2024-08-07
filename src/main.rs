use clap::Parser;
use ompm::commands::{self, Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { name, language, full, license } => commands::init_project(&name,language.to_owned(), full.to_owned(),license.to_owned()),
        Commands::Add { file } => commands::add_file(&file),
        Commands::Clean => commands::clean_project().expect("Failed to clean the project"),
        Commands::Merge { project_to_merge, project_to_be_merged } => {
            match commands::merge_projects(project_to_merge, project_to_be_merged) {
                Ok(()) => println!("Projects merged successfully!"),
                Err(e) => eprintln!("Error merging projects: {}", e),
            }

        },
    }
}