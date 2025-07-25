use solana_program_error::ProgramError;


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

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 4]`
/// is within bounds of the `data` slice.
pub fn read_i32_slice(data: &[u8], start: usize) -> Result<i32, ProgramError> {
    Ok(
        i32::from_le_bytes(data[start..start + 4]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 2]`
/// is within bounds of the `data` slice.
pub fn read_u16_slice(data: &[u8], start: usize) -> Result<u16, ProgramError> {
    Ok(
        u16::from_le_bytes(data[start..start + 2]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}

/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + 2]`
/// is within bounds of the `data` slice.
pub fn read_i16_slice(data: &[u8], start: usize) -> Result<i16, ProgramError> {
    Ok(
        i16::from_le_bytes(data[start..start + 2]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
        )
    )
}

/// # Safety
/// This function returns an error instead of panicking if the index is out of bounds.
/// Valid values are 0 (false) and 1 (true). Any other value results in an error.
pub fn read_bool_slice(data: &[u8], start: usize) -> Result<bool, ProgramError> {
    Ok(
        match data.get(start).copied() {
            Some(0) => false,
            Some(1) => true,
            _ => return Err(ProgramError::InvalidInstructionData)
        }
    )
}

/// If this is a standalone function, consider simply using `bytes[i]`,
/// because in that case it's just a unneccessary indirection.
///
/// # Safety
/// This function returns an error instead of panicking if the index is out of bounds.
pub fn read_u8_slice(data: &[u8], start: usize) -> Result<u8, ProgramError> {
    data.get(start).map(|&i| i).ok_or(ProgramError::InvalidInstructionData)
}

/// # Safety
/// This function returns an error instead of panicking if the index is out of bounds.
pub fn read_i8_slice(data: &[u8], start: usize) -> Result<i8, ProgramError> {
    data.get(start).map(|&i| i as i8).ok_or(ProgramError::InvalidInstructionData)
}

/// Reads `const N` amount of bytes.
/// 
/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + N]`
/// is within bounds of the `data` slice.
pub fn read_bytes_slice<const N: usize>(data: &[u8], start: usize) -> Result<[u8; N], ProgramError> {
    Ok(
        data[start..start + N]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?
    )
}