use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use sha2::{Digest, Sha256};

pub fn get_deterministic_random(seed: u64, param: u64) -> u64 {
    let combined_seed = seed ^ param;
    let mut rng = ChaChaRng::seed_from_u64(combined_seed);
    rng.random::<u64>()
}

pub fn compute_file_hash(file_data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(file_data);
    format!("{:x}", hasher.finalize())
}
