mod bitstring;
mod boolean;
mod choice;
mod enumerated;
mod integer;
mod octetstring;
mod strings;
mod tag;
mod traits;
mod unsigned_integer;

#[inline]
pub fn is_highest_bit_set(bytes: &[u8]) -> bool {
    bytes
        .first()
        .map(|byte| byte & 0b10000000 != 0)
        .unwrap_or(false)
}
