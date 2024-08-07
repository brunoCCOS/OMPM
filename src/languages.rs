use std::{env, fs, path::Path};

/// Create languages functions

pub fn create_rust_project_files(path: &Path, full: bool) {
    // Create a Cargo.toml file
    let cargo_toml_path = path.join("Cargo.toml");
    fs::write(&cargo_toml_path, "[package]\nname = \"project\"\nversion = \"0.1.0\"\nedition = \"2021\"\n").expect("Failed to create Cargo.toml");

    if full {
        // Add additional files and folders for a full Rust project setup
        let src_path = path.join("src");
        fs::create_dir(&src_path).expect("Failed to create src directory");
        let main_rs_path = src_path.join("main.rs");
        fs::write(&main_rs_path, "pub fn main() {\n    println!(\"Hello, Rust project!\");\n}\n").expect("Failed to create main.rs");

        println!("Created additional Rust project files for full setup");
    }

    println!("Created Rust project files");
}

pub fn create_python_project_files(path: &Path, full: bool) {
    // Create a main.py file
    let main_py_path = path.join("main.py");
    fs::write(&main_py_path, "if __name__ == '__main__':\n    print('Hello, Python project!')\n").expect("Failed to create main.py");

    if full {
        // Add additional files and folders for a full Python project setup
        let requirements_txt_path = path.join("requirements.txt");
        fs::write(&requirements_txt_path, "# Add your project dependencies here\n").expect("Failed to create requirements.txt");

        let tests_path = path.join("tests");
        fs::create_dir(&tests_path).expect("Failed to create tests directory");
        let test_main_py_path = tests_path.join("test_main.py");
        fs::write(&test_main_py_path, "def test_example():\n    assert True\n").expect("Failed to create test_main.py");

        println!("Created additional Python project files for full setup");
    }

    println!("Created Python project files");
}

pub fn create_node_project_files(path: &Path, full: bool) {

    // Create a package.json file
    let package_json_path = path.join("package.json");
    fs::write(&package_json_path, "{\n  \"name\": \"project\",\n  \"version\": \"1.0.0\",\n  \"main\": \"index.js\"\n}\n").expect("Failed to create package.json");

    // Create an index.js file
    let index_js_path = path.join("index.js");
    fs::write(&index_js_path, "console.log('Hello, Node.js project!');\n").expect("Failed to create index.js");

    if full {
        // Add additional files and folders for a full Node.js project setup
        let src_path = path.join("src");
        fs::create_dir(&src_path).expect("Failed to create src directory");
        let index_src_js_path = src_path.join("index.js");
        fs::write(&index_src_js_path, "console.log('Hello from src directory!');\n").expect("Failed to create index.js in src");

        println!("Created additional Node.js project files for full setup");
    }

    println!("Created Node.js project files");
}
