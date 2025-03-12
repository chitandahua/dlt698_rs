use super::traits::{FromAxdr, ToAxdr};
use super::unsigned_integer::UnsignedInteger;
use crate::{Error, ParseResult, Result, SerializeResult};
use std::array::TryFromSliceError;
use std::borrow::Cow;

// 定长BitString
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixedBitString<const N: usize, const BYTES: usize>([u8; BYTES]);

impl<const N: usize, const BYTES: usize> FixedBitString<N, BYTES> {
    pub fn new(data: [u8; BYTES]) -> Self {
        Self(data)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, TryFromSliceError> {
        Ok(Self(<[u8; BYTES]>::try_from(slice)?))
    }

    pub fn from_vec(vec: Vec<u8>) -> Result<Self, TryFromSliceError> {
        Self::from_slice(&vec)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn into_array(self) -> [u8; BYTES] {
        self.0
    }
}

impl<const N: usize, const BYTES: usize> AsRef<[u8]> for FixedBitString<N, BYTES> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize, const BYTES: usize> FromAxdr<'_> for FixedBitString<N, BYTES> {
    fn from_axdr(bytes: &[u8]) -> ParseResult<Self> {
        if bytes.len() < BYTES {
            return Err(asn1_rs::Err::Error(Error::InvalidLength));
        }

        Ok((
            &bytes[BYTES..],
            FixedBitString::<N, BYTES>::new(bytes[0..BYTES].try_into().unwrap()),
        ))
    }
}

impl<const N: usize, const BYTES: usize> ToAxdr for FixedBitString<N, BYTES> {
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(BYTES)
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        writer.write(self.as_ref()).map_err(Into::into)
    }
}

// 变长BitString
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitString<'a> {
    pub unused_bits: u8,
    pub data: Cow<'a, [u8]>,
}

impl<'a> BitString<'a> {
    pub const fn new(unused_bits: u8, s: &'a [u8]) -> Self {
        BitString {
            unused_bits,
            data: Cow::Borrowed(s),
        }
    }

    /// Test if bit `bitnum` is set
    pub fn is_set(&self, bitnum: usize) -> bool {
        let byte_pos = bitnum / 8;
        if byte_pos >= self.data.len() {
            return false;
        }
        let b = 7 - (bitnum % 8);
        (self.data[byte_pos] & (1 << b)) != 0
    }
}

impl AsRef<[u8]> for BitString<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

#[inline]
fn bytes_number(bit_length: usize) -> usize {
    bit_length.div_ceil(8)
}

#[inline]
fn unused_bits(bit_length: usize) -> u8 {
    8 - (bit_length % 8) as u8
}

impl<'a> FromAxdr<'a> for BitString<'a> {
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        // TODO 最前面的Length为变长Integer的编码? 但是没有负数 没有补码。。。
        let (bytes, int) = UnsignedInteger::from_axdr(bytes)?;
        let len = int.as_u64()? as usize; // TODO BigInt?

        let bytes_num = bytes_number(len);
        let unused_bits = unused_bits(len);

        if bytes.len() < bytes_num {
            return Err(asn1_rs::Err::Error(Error::InvalidLength));
        }

        Ok((
            &bytes[bytes_num..],
            BitString::new(unused_bits, &bytes[0..bytes_num]),
        ))
    }
}

impl ToAxdr for BitString<'_> {
    fn to_axdr_len(&self) -> Result<usize> {
        let len = self.as_ref().len();
        if len > 0x7f {
            Ok(1 + bytes_number(len) + len)
        } else {
            Ok(1 + len)
        }
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        let bit_length = self.as_ref().len() * 8 - self.unused_bits as usize;
        let int = UnsignedInteger::from_u64(bit_length as u64); // TODO
        int.write_axdr(writer)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        writer.write(self.as_ref()).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitstring_decode() {
        let (_, obj) = BitString::from_axdr(&[0x0d, 0x67, 0x50]).unwrap();
        assert_eq!(obj.unused_bits, 3);
        assert_eq!(obj.data.as_ref(), &[0x67, 0x50]);

        let (_, obj) = BitString::from_axdr(&[
            0x81, 0x83, 0x12, 0x23, 0x67, 0x50, 0x35, 0x12, 0x23, 0x67, 0x50, 0x89, 0x12, 0x23,
            0x67, 0x50, 0x89, 0x12, 0x80,
        ])
        .unwrap();
        assert_eq!(obj.unused_bits, 5);
        assert_eq!(
            obj.data.as_ref(),
            &[
                0x12, 0x23, 0x67, 0x50, 0x35, 0x12, 0x23, 0x67, 0x50, 0x89, 0x12, 0x23, 0x67, 0x50,
                0x89, 0x12, 0x80
            ]
        );
    }

    #[test]
    fn test_bitstring_encode() {
        let obj = BitString::new(3, &[0x67, 0x50]);
        let bytes = obj.to_axdr_vec().unwrap();
        assert_eq!(&bytes, &[0x0d, 0x67, 0x50]);

        let obj = BitString::new(
            5,
            &[
                0x12, 0x23, 0x67, 0x50, 0x35, 0x12, 0x23, 0x67, 0x50, 0x89, 0x12, 0x23, 0x67, 0x50,
                0x89, 0x12, 0x80,
            ],
        );
        let bytes = obj.to_axdr_vec().unwrap();
        assert_eq!(
            &bytes,
            &[
                0x81, 0x83, 0x12, 0x23, 0x67, 0x50, 0x35, 0x12, 0x23, 0x67, 0x50, 0x89, 0x12, 0x23,
                0x67, 0x50, 0x89, 0x12, 0x80
            ]
        );
    }

    #[test]
    fn test_fixed_bit_string_decode() {
        let (_, obj) = FixedBitString::<20, 3>::from_axdr(&[0x0d, 0x67, 0x50]).unwrap();
        assert_eq!(obj.as_ref(), &[0x0d, 0x67, 0x50]);
    }

    #[test]
    fn test_fixed_bit_string_encode() {
        let obj = FixedBitString::<20, 3>::new([0x0d, 0x67, 0x50]);
        assert_eq!(&obj.to_axdr_vec().unwrap(), &[0x0d, 0x67, 0x50]);
    }
}
