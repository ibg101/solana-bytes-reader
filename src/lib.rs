pub mod reader;
pub mod traits;
pub mod utils;
#[cfg(feature = "bytemuck")]
pub mod pod;
pub mod prelude;

pub use prelude::*;


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_primitive() -> Result<(), Box<dyn std::error::Error>> {
        let int: u16 = u16::MAX;
        assert_eq!(int, read_u16_slice(&int.to_le_bytes(), 0)?);
        Ok(())
    }

    #[test]
    fn test_reader() -> Result<(), Box<dyn std::error::Error>> {
        let expected_first: u8 = 101;
        let expected_second: u16 = 2134;
        let expected_third: &[u8] = &[1; 10];

        let bytes: &mut [u8] = &mut [0; 13];
        bytes[0] = expected_first;
        bytes[1..3].copy_from_slice(&expected_second.to_le_bytes());
        bytes[3..13].copy_from_slice(expected_third);

        let mut reader: Reader = Reader::new(bytes);
        let first: u8 = reader.read_u8()?;        
        println!("peek into second value: {}", reader.peek_u16()?);
        let second: u16 = reader.read_u16()?;
        let third: [u8; 10] = reader.read_bytes()?;

        assert_eq!(expected_first, first);
        assert_eq!(expected_second, second);
        assert_eq!(expected_third, third);

        assert_eq!(reader.remaining(), 0);

        Ok(())
    }

    #[test]
    fn test_read_pod() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(C)]
        struct Foo {
            arr: [u32; 4],
            val: u8,
            _pad: [u8; 3]
        }

        unsafe impl bytemuck::Zeroable for Foo {}
        unsafe impl bytemuck::Pod for Foo {}

        let expected_foo = Foo {
            arr: [101; 4],
            val: 101,
            _pad: [0; 3]
        };
        
        let ptr: *const u8 = &expected_foo as *const Foo as *const u8;
        let foo_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(ptr, std::mem::size_of::<Foo>())
        };

        let foo = read_pod_slice::<Foo>(foo_bytes, 0)?;

        assert_eq!(expected_foo, foo);        

        Ok(())
    }
}