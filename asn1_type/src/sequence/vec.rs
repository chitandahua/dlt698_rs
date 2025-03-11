use crate::traits::{FromAxdr, ToAxdr};
use crate::unsigned_integer::UnsignedInteger;
use asn1_rs::{Error, SerializeResult};

impl<'a, T> FromAxdr<'a> for Vec<T> where T: FromAxdr<'a> {
    fn from_axdr(bytes: &'a [u8]) -> asn1_rs::ParseResult<'a, Self> {
        let (bytes, int) = UnsignedInteger::from_axdr(bytes)?;
        let len = int.as_u64()? as usize; // TODO BigInt?

        // 不能简单的用std::mem::size_of 比如Data...
        //if bytes.len() < len * std::mem::size_of::<T>()   {
        //    return Err(asn1_rs::Err::Error(Error::InvalidLength));
        //}

        let mut vec = Vec::new();
        let mut bytes = bytes;
        while vec.len() < len {
            let (b, t) = T::from_axdr(bytes)?;
            vec.push(t);
            bytes = b;
        }
        Ok((bytes, vec))
    }
}

impl<T> ToAxdr for Vec<T> where T: ToAxdr {
    fn to_axdr_len(&self) -> asn1_rs::Result<usize> {
        // 取第一个的长度  若空则为0
        let len: usize = if self.is_empty() { 0 } else { self[0].to_axdr_len()? } * self.len();
        Ok(UnsignedInteger::from_u64(self.len() as u64).to_axdr_len()? + len)
    }

    fn write_axdr_header(&self, writer: &mut dyn std::io::Write) -> SerializeResult<usize> {
        UnsignedInteger::from_u64(self.len() as u64).write_axdr(writer)
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
    fn test_vec_to_axdr() {
        let v: Vec<u8> = vec![1, 2, 3];
        assert_eq!(v.to_axdr_vec().unwrap(), vec![0x03, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_vec_from_axdr() {
        let axdr = vec![0x03, 0x01, 0x02, 0x03];
        let (_, v) = Vec::<u8>::from_axdr(&axdr).unwrap();
        assert_eq!(v, vec![1, 2, 3]);
    }

}