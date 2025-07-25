pub trait ReadBytes {
    type Error;
    
    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 8]`
    /// is within bounds of the `bytes` slice.
    fn read_u64(&mut self) -> Result<u64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 8]`
    /// is within bounds of the `bytes` slice.
    fn read_i64(&mut self) -> Result<i64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 4]`
    /// is within bounds of the `bytes` slice.
    fn read_u32(&mut self) -> Result<u32, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 4]`
    /// is within bounds of the `bytes` slice.
    fn read_i32(&mut self) -> Result<i32, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 2]`
    /// is within bounds of the `bytes` slice.
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    
    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 2]`
    /// is within bounds of the `bytes` slice.
    fn read_i16(&mut self) -> Result<i16, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    /// Valid values are 0 (false) and 1 (true). Any other value results in an error.
    fn read_bool(&mut self) -> Result<bool, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    fn read_u8(&mut self) -> Result<u8, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    
    /// Reads `const N` amount of bytes.
    /// 
    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + N]`
    /// is within bounds of the `bytes` slice.
    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;

    /// Increments `self.offset` by `bytes_to_skip`.
    /// 
    /// # Safety
    /// The caller is responsible for ensuring that the `self.offset + bytes_to_skip`
    /// is within bounds of the `bytes` slice
    fn skip(&mut self, bytes_to_skip: usize);
}

/// Methods defined under this trait DO NOT increment the offset.
pub trait PeekIntoBytes {
    type Error;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 8]`
    /// is within bounds of the `bytes` slice.
    fn peek_u64(&self) -> Result<u64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 8]`
    /// is within bounds of the `bytes` slice.
    fn peek_i64(&self) -> Result<i64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 4]`
    /// is within bounds of the `bytes` slice.
    fn peek_u32(&self) -> Result<u32, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 4]`
    /// is within bounds of the `bytes` slice.
    fn peek_i32(&self) -> Result<i32, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 2]`
    /// is within bounds of the `bytes` slice.
    fn peek_u16(&self) -> Result<u16, Self::Error>;
    
    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + 2]`
    /// is within bounds of the `bytes` slice.
    fn peek_i16(&self) -> Result<i16, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    /// Valid values are 0 (false) and 1 (true). Any other value results in an error.
    fn peek_bool(&self) -> Result<bool, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    fn peek_u8(&self) -> Result<u8, Self::Error>;

    /// # Safety
    /// This function returns an error instead of panicking if the index is out of bounds.
    fn peek_i8(&self) -> Result<i8, Self::Error>;
    
    /// Peeks into `const N` amount of bytes.
    /// 
    /// # Safety
    /// The caller is responsible for ensuring that the range `[self.offset..self.offset + N]`
    /// is within bounds of the `bytes` slice.
    fn peek_bytes<const N: usize>(&self) -> Result<[u8; N], Self::Error>;
}