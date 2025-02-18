mod bitstring;
mod boolean;
mod enumerated;
mod integer;
mod traits;
mod unsigned_integer;

#[inline]
pub fn is_highest_bit_set(bytes: &[u8]) -> bool {
    bytes
        .first()
        .map(|byte| byte & 0b10000000 != 0)
        .unwrap_or(false)
}
