use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the current directory where build.rs is located
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let current_dir = Path::new(&current_dir);

    // Define the path to the CSV files in your project's "data" directory
    let csv_files = vec!["./filtered_data/equities.csv", "./filtered_data/etfs.csv"];

    // Copy the CSV files to the "target/release" directory
    let target_dir = current_dir.join("target").join("release");
    for csv_file in csv_files {
        let src_path = current_dir.join(csv_file);
        let dest_path = target_dir.join(Path::new(csv_file).file_name().unwrap());
        fs::copy(&src_path, &dest_path).expect("Failed to copy CSV file.");
    }
}