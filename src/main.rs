

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

    },

    Clean {
        #[arg(short,long)]
        hard: Option<String>
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { name, language, full } => init_project(name, language, *full),
        Commands::Clean { hard } => clean_project(hard),
    }
}