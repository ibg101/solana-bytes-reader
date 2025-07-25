use solana_program_error::ProgramError;


/// # Safety
/// The caller is responsible for ensuring that the range `[start..start + std::mem::size_of::<T>()]`
/// is within bounds of the `data` slice.
pub fn read_pod_slice<T: bytemuck::Pod>(data: &[u8], start: usize) -> Result<T, ProgramError> {
    let bytes_slice: &[u8] = &data[start..start + std::mem::size_of::<T>()];
    
    Ok(
        bytemuck::try_from_bytes::<T>(bytes_slice)
            .map_err(|_| ProgramError::InvalidInstructionData)?
            .clone()
    )
}