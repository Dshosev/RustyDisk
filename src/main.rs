use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::Instant;
use std::env;

const DEFAULT_FILE_SIZE_MB: usize = 100; // Standardgröße in MB
const BUFFER_SIZE: usize = 1024 * 1024; // 1 MB

fn write_test(file_path: &str, size_mb: usize) {
    let total_bytes = size_mb * 1024 * 1024;
    let buffer = vec![b'A'; BUFFER_SIZE];

    println!("Starting write test: {} MB to {}...", size_mb, file_path);
    let start = Instant::now();

    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        },
    };

    let mut written_bytes = 0;
    while written_bytes < total_bytes {
        let to_write = (total_bytes - written_bytes).min(BUFFER_SIZE);
        if let Err(e) = file.write_all(&buffer[..to_write]) {
            eprintln!("Error writing to file: {}", e);
            return;
        }
        written_bytes += to_write;
    }

    let duration = start.elapsed();
    println!(
        "Write completed: {:.2} MB/s",
        (total_bytes as f64 / 1_000_000.0) / duration.as_secs_f64()
    );
}

fn read_test(file_path: &str, size_mb: usize) {
    let total_bytes = size_mb * 1024 * 1024;
    let mut buffer = vec![0; BUFFER_SIZE];

    println!("Starting read test: {} MB from {}...", size_mb, file_path);
    let start = Instant::now();

    let mut file = match OpenOptions::new().read(true).open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file for reading: {}", e);
            return;
        },
    };

    let mut read_bytes = 0;
    while read_bytes < total_bytes {
        let to_read = (total_bytes - read_bytes).min(BUFFER_SIZE);
        match file.read(&mut buffer[..to_read]) {
            Ok(0) => break, // EOF
            Ok(n) => read_bytes += n,
            Err(e) => {
                eprintln!("Error reading from file: {}", e);
                return;
            },
        }
    }

    let duration = start.elapsed();
    println!(
        "Read completed: {:.2} MB/s",
        (total_bytes as f64 / 1_000_000.0) / duration.as_secs_f64()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file_path = String::from("testfile");
    let mut size_mb = DEFAULT_FILE_SIZE_MB;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-f" => {
                if i + 1 < args.len() {
                    file_path = args[i + 1].clone();
                }
            }
            "-s" => {
                if i + 1 < args.len() {
                    size_mb = args[i + 1].parse().unwrap_or(DEFAULT_FILE_SIZE_MB);
                }
            }
            "-h" => {
                println!("Usage: disk_speed_test [-f <file_path>] [-s <size_mb>] [-h]");
                return;
            }
            _ => {}
        }
    }

    write_test(&file_path, size_mb);
    read_test(&file_path, size_mb);

    if std::fs::remove_file(&file_path).is_ok() {
        println!("Temporary file removed.");
    } else {
        eprintln!("Error removing temporary file.");
    }
}
