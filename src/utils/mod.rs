use rand::prelude::*;
use rand::rngs::adapter::ReseedingRng;
use rand::rngs::OsRng;
use rand_chacha::ChaCha20Core;

mod u256;

pub fn get_random_u256() -> u256::U256 {
    let prng = ChaCha20Core::from_entropy();
    let mut rng = ReseedingRng::new(prng, 0, OsRng);

    let mut slices = [0; 2];

    for item in &mut slices {
        *item = rng.gen::<u128>();
    }
    u256::U256::new(slices)
}

pub fn prefixed_hex(hex: &str) -> String {
    let prefix = String::from("0x");
    prefix + hex
}
