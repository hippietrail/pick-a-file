use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use rand::Rng;
use rand::SeedableRng; // Import the SeedableRng trait
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    let use_stdout_only = args.contains(&"--bare".to_string());

    // Update the condition to check for at least one extension
    if args.len() < 3 || (args.len() < 4 && use_stdout_only) {
        eprintln!("Usage: {} [--bare] <path> <file_extension1> <file_extension2> ...", args[0]);
        std::process::exit(1);
    }

    let path = if use_stdout_only { &args[2] } else { &args[1] };
    let mut extensions: Vec<String> = if use_stdout_only {
        args[3..].to_vec() // Collect all extensions passed
    } else {
        args[2..].to_vec() // Collect all extensions passed
    };

    // Normalize the extensions by removing leading dots if they exist
    for ext in &mut extensions {
        if ext.starts_with('.') {
            *ext = ext[1..].to_string();
        }
    }

    // Seed the random number generator with the current time
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    // Reservoir sampling
    let mut chosen_file = None;
    let mut count = 0;

    collect_files(Path::new(path), &extensions, &mut |file| {
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
        None => eprintln!("No files with specified extensions found in {}", path),
    }
}

// our custom function to get the extension which also works when the filename starts with a dot
fn get_ext(path: &Path) -> Option<&OsStr> {
    // first try the standard function
    let ext = path.extension();
    if ext.is_some() {
        return Some(ext.unwrap());
    }
    // if the standard function fails, check if filename starts with a dot
    let file_name = path.file_name();
    if file_name.is_some() {
        let file_name_str = file_name.unwrap().to_str();
        if file_name_str.is_some() {
            let file_name_str = file_name_str.unwrap();
            if file_name_str.starts_with('.') {
                return Some(OsStr::new(&file_name_str[1..]));
            }
        }
    }
    None
}

// Recursively collect files and directories with the given extensions
fn collect_files<F>(dir: &Path, extensions: &[String], callback: &mut F)
where
    F: FnMut(&std::path::Path),
{
    if dir.is_symlink() {
        return; // Avoid following symlinks
    }

    // Read the directory contents
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                // Check if the entry has a matching extension
                // let has_matching_extension = path.extension()
                let has_matching_extension = get_ext(&path)
                    .and_then(|e| e.to_str())
                    .map(|ext| extensions.contains(&ext.to_string()))
                    .unwrap_or(false);

                if has_matching_extension {
                    // print for debugging purpose
                    callback(&path);
                }

                // Check if the entry is a directory
                if path.is_dir() {
                    // Recursively collect files and directories from subdirectories
                    collect_files(&path, extensions, callback);
                }
            }
        }
    }
}