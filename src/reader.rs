use solana_program_error::ProgramError;
use super::{
    utils::*,
    traits::*
};


#[derive(Debug, Clone, Copy)]
/// # Safety
/// The caller is responsible for ensuring that the bytes slice is valid.
/// 
/// # Examples
/// ```
/// use core::array;
/// use solana_bytes_reader::{Reader, ReadBytes};
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let bytes: [u8; 32] = array::from_fn(|i| if i % 4 == 0 { i as u8 } else { 0u8 });
///     
///     let mut reader: Reader = Reader::new(&bytes);
///     let first_u32: u32 = reader.read_u32()?;
///     let second_u32: u32 = reader.read_u32()?;
///     let third_u32: u32 = reader.read_u32()?;
/// 
///     assert_eq!(first_u32, 0u32);
///     assert_eq!(second_u32, 4u32);
///     assert_eq!(third_u32, 8u32);
///     
///     Ok(())
/// }
/// ```
///
/// It's recommended to initialize `Reader` and use it's methods if there are more than 2 method calls.
/// 
/// Otherwise there's no need in this struct and functions can be used instead.
pub struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize
}

impl<'a> Reader<'a> {
    /// # Panics
    /// if bytes slice is empty.
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        assert!(bytes.len() > 0);
        Self { bytes, offset: 0 }
    }

    /// # Panics 
    /// if `offset` equals/greater than bytes slice OR if bytes slice is empty.
    #[inline]
    pub fn new_with_offset(bytes: &'a [u8], offset: usize) -> Self {
        let len: usize = bytes.len();
        assert!(len > offset && len > 0);
        Self { bytes, offset }
    }

    #[inline]
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline]
    /// Returns remaining bytes.
    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    /// # Panics 
    /// if `new_offset` equals/greater than bytes slice.
    #[inline]
    pub fn set_offset(&mut self, new_offset: usize) -> () {
        assert!(self.bytes.len() > new_offset);
        self.offset = new_offset;
    }

    fn read<T, F>(&mut self, read_method: F) -> Result<T, ProgramError> 
    where
        T: Sized,
        F: Fn(&[u8], usize) -> Result<T, ProgramError>
    {
        let val: T = read_method(self.bytes, self.offset)?;
        self.offset += std::mem::size_of::<T>();
        Ok(val)
    }
}

impl ReadBytes for Reader<'_> {
    type Error = ProgramError;

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        self.read(read_u64_slice)
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        self.read(read_i64_slice)
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        self.read(read_u32_slice)
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        self.read(read_i32_slice)
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        self.read(read_u16_slice)
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        self.read(read_i16_slice)
    }

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        self.read(read_bool_slice)
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.read(read_u8_slice)
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        self.read(read_i8_slice)
    }

    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        self.read(read_bytes_slice)
    }

    fn skip(&mut self, bytes_to_skip: usize) {
        self.offset += bytes_to_skip;
    }
}

impl PeekIntoBytes for Reader<'_> {
    type Error = ProgramError;

    fn peek_u64(&self) -> Result<u64, Self::Error> {
        read_u64_slice(self.bytes, self.offset)
    }

    fn peek_i64(&self) -> Result<i64, Self::Error> {
        read_i64_slice(self.bytes, self.offset)
    }

    fn peek_u32(&self) -> Result<u32, Self::Error> {
        read_u32_slice(self.bytes, self.offset)
    }

    fn peek_i32(&self) -> Result<i32, Self::Error> {
        read_i32_slice(self.bytes, self.offset)
    }

    fn peek_u16(&self) -> Result<u16, Self::Error> {
        read_u16_slice(self.bytes, self.offset)
    }

    fn peek_i16(&self) -> Result<i16, Self::Error> {
        read_i16_slice(self.bytes, self.offset)
    }

    fn peek_bool(&self) -> Result<bool, ProgramError> {
        read_bool_slice(self.bytes, self.offset)
    }

    fn peek_u8(&self) -> Result<u8, Self::Error> {
        read_u8_slice(self.bytes, self.offset)
    }

    fn peek_i8(&self) -> Result<i8, Self::Error> {
        read_i8_slice(self.bytes, self.offset)
    }

    fn peek_bytes<const N: usize>(&self) -> Result<[u8; N], Self::Error> {
        read_bytes_slice::<N>(self.bytes, self.offset)
    }
}