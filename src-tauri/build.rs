use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=migrations");

    // Get the path to the Cargo.toml manifest directory (src-tauri/)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = PathBuf::from(manifest_dir);

    // Construct the absolute path to the data directory
    // This assumes 'data' is a sibling of 'src-tauri'
    let project_root = manifest_path.parent().expect("Failed to get project root");
    let data_dir = project_root.join("data");
    let db_path = data_dir.join("db.sqlite");

    let db_url = format!("sqlite:{}", db_path.to_str().expect("Failed to convert path to string"));

    // Set DATABASE_URL environment variable for sqlx-cli
    env::set_var("DATABASE_URL", &db_url);

    // Run sqlx database setup and migrations
    println!("Running sqlx database setup and migrations...");

    // Create the database if it doesn't exist
    let output = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .output()
        .expect("Failed to execute sqlx database create");

    if !output.status.success() {
        eprintln!("Error creating database: {:?}", output);
        // Don't panic here, as it might already exist
    }

    // Run migrations
    let output = Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .output()
        .expect("Failed to execute sqlx migrate run");

    if !output.status.success() {
        eprintln!("Error running migrations: {:?}", output);
        panic!("Failed to run database migrations");
    }

    println!("sqlx database setup and migrations completed.");

    tauri_build::build()
}