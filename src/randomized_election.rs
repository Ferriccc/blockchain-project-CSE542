use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use sha2::{Digest, Sha256};

fn get_deterministic_random(seed: &str, l: u64, r: u64) -> u64 {
    let hash = Sha256::digest(seed.as_bytes());
    let seed: [u8; 32] = hash.into();
    let mut rng = ChaChaRng::from_seed(seed);
    rng.random_range(l..r)
}

const M: u64 = 2;

pub fn is_elected(id: &str, block_hash: &str, total_nodes: u64) -> bool {
    let seed = id.to_owned() + block_hash;

    let rn = get_deterministic_random(&seed, 0, total_nodes);

    rn <= (total_nodes + M - 1) / M
}
