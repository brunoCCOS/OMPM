use clap::{command, Parser, Subcommand};
use ompm::{init_project, add_file, clean_project,files::license::License};

#[derive(Parser)]
#[command(name = "OMIC")]
#[command(version = "1.0")]
#[command(about = "Initialize", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project with a Git repository
    Init {
        name: String,

        #[arg(short, long)]
        language: Option<String>,

        #[arg(short, long)]
        full: bool,

        #[arg(value_enum)]
        license: License,

    },

    Add { file : String },


    Clean
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { name, language, full, license } => init_project(name, language, *full,*license),
        Commands::Add { file } => add_file(file),
        Commands::Clean => clean_project().expect("Failed to clean the project"),
    }
}