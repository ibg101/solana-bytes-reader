use solana_program_error::ProgramError;


pub trait ReadBytes {
    type Error;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[start..start + 8]`
    /// is within bounds of the `data` slice.
    fn read_u64(&self, start: usize) -> Result<u64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[start..start + 8]`
    /// is within bounds of the `data` slice.
    fn read_i64(&self, start: usize) -> Result<i64, Self::Error>;

    /// # Safety
    /// The caller is responsible for ensuring that the range `[start..start + 4]`
    /// is within bounds of the `data` slice.
    fn read_u32(&self, start: usize) -> Result<u32, Self::Error>;
}

/// It's recommended to initialize `Reader` and use it's methods if there are more than 2 method calls.
/// 
/// Otherwise there's no need in this struct and functions can be used instead.
pub struct Reader<'a> {
    pub bytes: &'a [u8]
}

impl<'a> From<&'a [u8]> for Reader<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl ReadBytes for Reader<'_> {
    type Error = ProgramError;

    fn read_u64(&self, start: usize) -> Result<u64, Self::Error> {
        read_u64_slice(self.bytes, start)
    }

    fn read_i64(&self, start: usize) -> Result<i64, Self::Error> {
        read_i64_slice(self.bytes, start)
    }

    fn read_u32(&self, start: usize) -> Result<u32, Self::Error> {
        read_u32_slice(self.bytes, start)
    }
}

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 8]`
/// is within bounds of the `data` slice.
pub fn read_u64_slice(data: &[u8], start: usize) -> Result<u64, ProgramError> {
    Ok(
        u64::from_le_bytes(data[start..start + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 8]`
/// is within bounds of the `data` slice.
pub fn read_i64_slice(data: &[u8], start: usize) -> Result<i64, ProgramError> {
    Ok(
        i64::from_le_bytes(data[start..start + 8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 4]`
/// is within bounds of the `data` slice.
pub fn read_u32_slice(data: &[u8], start: usize) -> Result<u32, ProgramError> {
    Ok(
        u32::from_le_bytes(data[start..start + 4]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}