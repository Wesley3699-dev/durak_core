use rand::seq::SliceRandom;
use rand::{SeedableRng, TryRng};
use rand::rngs::SysRng;
use rand_xoshiro::Xoshiro256StarStar;


pub fn seed_generation() -> [u8; 32] {
    let mut seed = [0u8; 32];
    SysRng.try_fill_bytes(&mut seed).unwrap();
    seed
}

pub fn shuffle_deck(slice: &mut [u8]){
    let seed = seed_generation();
    let mut rng = Xoshiro256StarStar::from_seed(seed);
    slice.shuffle(&mut rng);
}

