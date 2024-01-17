use rayon::prelude::*;
use std::{
    env::args,
    fs::{self, File},
    io::{self, Write},
};

const DEFAULT_SIZE: usize = 1024 * 1024 * 1024 * 1; // 1GB
const DEFAULT_FILE_FILL: char = 'a';
const DEFAULT_PART: usize = 8;

fn generate_file(file_path: &str, file_size: usize) {
    println!("generating file: {}, {} byte", file_path, file_size);
    let mut file = File::create(file_path).expect("failed to create file");
    let fill_data: Vec<u8> = vec![DEFAULT_FILE_FILL as u8; file_size];
    file.write_all(&fill_data).expect("failed to write to file");
}

fn fill_disk(file_paths: &Vec<String>, total_size: usize) {
    file_paths
        .par_iter()
        .map(|file_path| generate_file(file_path, total_size / file_paths.len()))
        .collect::<Vec<_>>();
}

fn main() {
    let total_size = args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_SIZE);

    let file_paths: Vec<String> = (0..DEFAULT_PART)
        .map(|i| format!("temp_file_{}.bin", i))
        .collect();

    // create the temporary files
    fill_disk(&file_paths, total_size);

    // wait for user to press Enter
    println!("files generated. press Enter to terminate...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    // delete the temporary files
    for file_path in &file_paths {
        fs::remove_file(file_path).expect("failed to remove file");
    }
}
