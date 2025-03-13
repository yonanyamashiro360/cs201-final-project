import crypto::{digest::Digest, sha3_512::Sha3512};

fn main() {
    let mut hasher = Sha3512::new();
    hasher.update(b"Hello, world!");
    println!("{:x}", hasher.finalize());
}