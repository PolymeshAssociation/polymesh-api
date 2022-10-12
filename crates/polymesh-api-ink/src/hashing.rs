use core::hash::Hasher;

use ink_env::hash::*;

pub fn blake2_128(data: &[u8]) -> [u8; 16] {
  let mut hash = <Blake2x128 as HashOutput>::Type::default(); // 128-bit buffer
  ink_env::hash_bytes::<Blake2x128>(data, &mut hash);
  hash
}

pub fn blake2_256(data: &[u8]) -> [u8; 32] {
  let mut hash = <Blake2x256 as HashOutput>::Type::default(); // 256-bit buffer
  ink_env::hash_bytes::<Blake2x256>(data, &mut hash);
  hash
}

fn twox_helper_64(seed: u64, data: &[u8]) -> [u8; 8] {
  let mut hasher = twox_hash::XxHash::with_seed(seed);
  hasher.write(data);
  hasher.finish().to_le_bytes()
}

pub fn twox_64(data: &[u8]) -> [u8; 8] {
  twox_helper_64(0, data)
}

pub fn twox_128(data: &[u8]) -> [u8; 16] {
  let mut r = [0u8; 16];
  r[0..8].copy_from_slice(&twox_helper_64(0, data));
  r[8..16].copy_from_slice(&twox_helper_64(1, data));
  r
}

pub fn twox_256(data: &[u8]) -> [u8; 32] {
  let mut r = [0u8; 32];
  r[0..8].copy_from_slice(&twox_helper_64(0, data));
  r[8..16].copy_from_slice(&twox_helper_64(1, data));
  r[16..24].copy_from_slice(&twox_helper_64(2, data));
  r[24..32].copy_from_slice(&twox_helper_64(3, data));
  r
}
