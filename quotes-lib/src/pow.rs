use hex;
use log::trace;
use sha3::{Digest, Keccak256};
use std::borrow::Borrow;

pub struct PowCalculator {}

impl PowCalculator {
    pub fn new() -> PowCalculator {
        PowCalculator {}
    }

    pub fn compute_bump_seed(&self, nonce: u64, password: &String) -> (u64, [u8; 32]) {
        let orig_hasher = self.construct_hasher(nonce, password);
        let mut it = 0..;
        let mut hash = [1; 32];
        let mut bump_seed: u64 = 0;
        while !check_prof_of_work(&hash) {
            bump_seed = it.next().unwrap();
            hash = self.compute_hash_with_seed(bump_seed, &orig_hasher);
            trace!("{:0x?}", hex::encode(hash));
        }
        (bump_seed, hash)
    }

    fn construct_hasher(&self, nonce: u64, password: &String) -> Keccak256 {
        let mut hasher = Keccak256::default();
        let nonce_bytes = nonce.borrow().to_be_bytes();
        hasher.update(&nonce_bytes);
        hasher.update(password);
        hasher
    }

    fn compute_hash_with_seed(&self, seed: u64, orig_hasher: &Keccak256) -> [u8; 32] {
        let mut hasher = orig_hasher.clone();
        let nonce_buf = seed.borrow().to_be_bytes();
        hasher.update(&nonce_buf);
        let hash = hasher.finalize().into();
        hash
    }
}

pub fn check_auth_and_pow(
    nonce: u64,
    crap_secret: &String,
    bump_seed: u64,
) -> Result<[u8; 32], ()> {
    let calculator = PowCalculator::new();
    let hasher = calculator.construct_hasher(nonce, crap_secret);
    let hash = calculator.compute_hash_with_seed(bump_seed, &hasher);
    if !check_prof_of_work(&hash) {
        return Err(());
    }
    Ok(hash)
}

pub fn check_prof_of_work(hash: &[u8; 32]) -> bool {
    hash[0..2] == [0u8; 2]
}
