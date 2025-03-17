pub type SequenceOf<T> = Vec<T>;

use crate::traits::{FromAxdr, ToAxdr};
use crate::{ParseResult, Result, SerializeResult};

impl<'a, T, const N: usize> FromAxdr<'a> for [T; N]
where
    T: FromAxdr<'a>,
{
    fn from_axdr(bytes: &'a [u8]) -> ParseResult<'a, Self> {
        let mut array = Vec::with_capacity(N);
        let mut bytes = bytes;

        while array.len() < N {
            let (b, t) = T::from_axdr(bytes)?;
            array.push(t);
            bytes = b;
        }
        Ok((
            bytes,
            array
                .try_into()
                .map_err(|_| asn1_rs::Error::InvalidLength)?,
        ))
    }
}

impl<T, const N: usize> ToAxdr for [T; N]
where
    T: ToAxdr,
{
    fn to_axdr_len(&self) -> Result<usize> {
        Ok(self[0].to_axdr_len()? * N)
    }

    fn write_axdr_header(&self, _writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        Ok(0)
    }

    fn write_axdr_content(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        let mut num_bytes = 0;
        for t in self.iter() {
            num_bytes += t.write_axdr(writer)?;
        }
        Ok(num_bytes)
    }
}

mod tests {
    use super::*;
    use crate::traits::{FromAxdr, ToAxdr};

    #[test]
    fn test_array_to_axdr() {
        let v: [u8; 3] = [1, 2, 3];
        assert_eq!(v.to_axdr_vec().unwrap(), vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_array_from_axdr() {
        let axdr: [u8; 3] = [0x01, 0x02, 0x03];
        let (_, v) = <[u8; 3]>::from_axdr(&axdr).unwrap();
        assert_eq!(v, [1, 2, 3]);
    }
}
