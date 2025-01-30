use std::env;
use std::fs;
use std::path::Path;
use rand::Rng;
use rand::SeedableRng; // Import the SeedableRng trait
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    let use_stdout_only = args.contains(&"--bare".to_string());

    if args.len() < 3 || (args.len() > 3 && !use_stdout_only) || (args.len() > 4 && use_stdout_only) {
        eprintln!("Usage: {} [--bare] <path> <file_extension>", args[0]);
        std::process::exit(1);
    }

    let path = if use_stdout_only { &args[2] } else { &args[1] };
    let mut extension = if use_stdout_only { args[3].clone() } else { args[2].clone() };

    // Normalize the extension by removing the leading dot if it exists
    if extension.starts_with('.') {
        extension = extension[1..].to_string();
    }

    // Seed the random number generator with the current time
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Reservoir sampling
    let mut chosen_file = None;
    let mut count = 0;

    collect_files(Path::new(path), &extension, &mut |file| {
        count += 1;
        if count == 1 {
            chosen_file = Some(file.to_path_buf());
        } else {
            let j = rng.gen_range(0..count);
            if j == 0 {
                chosen_file = Some(file.to_path_buf());
            }
        }
    });

    match chosen_file {
        Some(file) => {
            eprintln!("Chosen file: {}", file.display());
            if use_stdout_only {
                println!("{}", file.display());
            }
        }
        None => eprintln!("No files with extension {} found in {}", extension, path),
    }
}

// Recursively collect files with the given extension
fn collect_files<F>(dir: &Path, extension: &str, callback: &mut F)
where
    F: FnMut(&std::path::Path),
{
    if dir.is_symlink() {
        return; // Avoid following symlinks
    }

    if dir.is_dir() {
        // Read the directory contents
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        // Call the callback function for each matching directory
                        if path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
                            callback(&path);
                        }
                        // Recursively collect files from subdirectories
                        collect_files(&path, extension, callback);
                    } else if path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
                        // Call the callback function for each matching file
                        callback(&path);
                    }
                }
            }
        }
    }
}