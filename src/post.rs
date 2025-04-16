use rand::Rng;
use sha2::{Digest, Sha256};

pub fn generate_new_challenge(file_size: usize) -> (usize, usize) {
    let mut rng = rand::rng();
    let start = rng.random_range(0..file_size - 100);
    let end = start + 100;

    (start, end)
}

pub fn validate(file: &Vec<u8>, start: usize, end: usize, hash: &str) -> bool {
    // Check if range is valid
    if start >= file.len() || end > file.len() || start >= end {
        return false;
    }

    // Extract segment from file
    let segment = &file[start..end];

    // Compute SHA-256 hash of the segment
    let mut hasher = sha2::Sha256::new();
    hasher.update(segment);
    let result = hasher.finalize();

    // Convert hash to hex string
    let computed_hash = format!("{:x}", result);

    // Compare computed hash with provided hash
    computed_hash == hash
}
