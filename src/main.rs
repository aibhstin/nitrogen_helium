use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::fs::File;
use std::error::Error;

use walkdir::WalkDir;

use data_encoding::HEXUPPER;

use ring::digest::{Context, Digest, SHA256};

fn main() {
    let mut hash_map = HashMap::<String, String>::new();
    // This HashMap is used to store file hashes and the filepaths
    // of the first instance of the hash.

    let mut duplicate_filepaths = HashMap::<String, String>::new();
    
    for entry in WalkDir::new(".")
        .into_iter()
        .map(|e| e.expect("Error"))
        .filter(|e| !e.file_type().is_dir()) {
        let f_path = String::from(entry.path().to_string_lossy());

        let input = File::open(&f_path)
            .expect("Unable to open file!");
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader);

        let hash = HEXUPPER.encode(digest.as_ref());

        //println!("SHA-256 digest of {} is {}", f_path, hash);

        if hash_map.contains_key(&hash) {
            let original_filepath = hash_map
                .get(&hash)
                .expect("Error");
            duplicate_filepaths
                .insert(f_path, original_filepath.to_string());
        } else {
            hash_map.insert(hash, f_path);
        }
    }

    println!("Duplicate filepaths: ");

    for (duplicate_file, original_file) in &duplicate_filepaths {
        println!("{}: {}", duplicate_file, original_file);
    }

}

fn sha256_digest<R: Read>(mut reader: R) -> Digest {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)
            .expect("Unable to read into buffer");
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    context.finish()
}
