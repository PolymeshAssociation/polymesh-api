use ink_env::hash::*;

use polymesh_extension::new_instance;

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

pub fn twox_64(data: &[u8]) -> [u8; 8] {
  let runtime = new_instance();
  runtime.twox_64(data.to_vec().into()).unwrap()
}

pub fn twox_128(data: &[u8]) -> [u8; 16] {
  let runtime = new_instance();
  runtime.twox_128(data.to_vec().into()).unwrap()
}

pub fn twox_256(data: &[u8]) -> [u8; 32] {
  let runtime = new_instance();
  runtime.twox_256(data.to_vec().into()).unwrap()
}
