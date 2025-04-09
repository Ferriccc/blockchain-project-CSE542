use rand::Rng;

pub fn generate_new_challenge(file_size: usize) -> (usize, usize) {
    let mut rng = rand::rng();
    let start = rng.random_range(0..file_size - 100);
    let end = start + 100;

    (start, end)
}
