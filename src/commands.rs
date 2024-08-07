use clap::{command, Parser, Subcommand};
use super::files::license::License;
use std::{env, io::Error, io::ErrorKind, fs, path::Path};
use git2::Repository;
use super::{files,languages,utils};

#[derive(Parser)]
#[command(name = "OMPM")]
#[command(version = "1.0")]
#[command(about = "Initialize", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project with a Git repository
    Init {
        name: String,

        #[arg(long)]
        language: Option<String>,

        #[arg(short, long, default_value_t = false)]
        full: bool,

        #[arg(long)]
        license: Option<License>,

    },

    /// Add a file to the project
    Add { file : String },

    /// Merge two projects into a new one
    Merge {
        project_to_merge: String,

        project_to_be_merged: String },

    /// Clean the project by removing unnecessary files
    Clean
}



pub fn init_project(name: &str, language: Option<String>, full: bool, license: Option<License>) {

    let path = Path::new(name);
    if path.exists() {
        eprintln!("Error: Directory {} already exists!", name);
        return;
    }
    // Create the project directory
    fs::create_dir(path).expect("Failed to create project directory");

    // Initialize a Git repository
    match Repository::init(path) {
        Ok(_) => println!("Initialized empty Git repository in {}", path.display()),
        Err(e) => eprintln!("Failed to initialize Git repository: {}", e),
    }

    if full {
        // Create common project files
        files::create_readme(path, name);
        files::create_gitignore(path, "target");
        files::create_contribute(path, name);
        files::create_code_of_conduct(path);
        files::license::create_license(path, if let Some(lic) = license {lic } else {License::MIT }); // If license is specified, then ... if not, then MIT
    }

    // Ensure language is present and convert to lowercase
    if let Some(lang) = language {
        match lang.to_lowercase().as_str() {
            "rust" => {
                languages::create_rust_project_files(path, full);
            }
            "python" => {
                languages::create_python_project_files(path, full);
            }
            "node" | "javascript" => {
                languages::create_node_project_files(path, full);
            }
            _ => {
                eprintln!("Unsupported language: {}", lang);
                return;
            }
        }
    } else {
        return;
    }
}



pub fn add_file(file_type: &str) {
    match file_type {
        "readme" => {
            fs::write("README.md", "# Project\n").expect("Failed to create README.md");
            println!("README.md created");
        }
        "gitignore" => {
            fs::write(".gitignore", "target/\n").expect("Failed to create .gitignore");
            println!(".gitignore created");
        }
        _ => {
            eprintln!("Unknown file type: {}", file_type);
        }
    }
}

pub fn clean_project() -> std::io::Result<()> {
    let current_project = env::current_dir()?;
    let project_display = current_project.display().to_string();
    fs::remove_dir_all(current_project)?;
    println!("Project {} deleted", project_display);
    Ok(())
}

pub fn merge_projects(project_to_merge: &str, project_to_be_merged: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Define paths
    let path_to_merge = Path::new(project_to_merge);
    let path_merged = Path::new(project_to_be_merged);

    // Check if the projects exist
    if !path_merged.exists() {
        eprintln!("Error: Project {} does not exist!", project_to_be_merged);
        Err(Box::new(Error::new(ErrorKind::NotFound, format!("FileNotFoundError: {}", project_to_be_merged))))
    } else if !path_to_merge.exists() {
        eprintln!("Error: Project {} does not exist!", project_to_merge);
        Err(Box::new(Error::new(ErrorKind::NotFound, format!("FileNotFoundError: {}", project_to_merge))))
    } else {
        for entry in fs::read_dir(path_merged)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dest_path = path_to_merge.join(file_name);

            if entry_path.is_file() {
                fs::copy(&entry_path, &dest_path)?;
            } else if entry_path.is_dir() {
                // Recursively copy directories
                utils::copy_dir_all(&entry_path, &dest_path).expect("Cannot copy directory");
            }
        }

        Ok(())
    }
}
