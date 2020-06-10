

use crate::ilint::{encode, decode};
use super::{Reader, Writer, Result, ErrorKind};

macro_rules! data_reader_read_be_bytes {
    ($self: ident, $type: ident) => ({
        let mut tmp : [u8; std::mem::size_of::<$type>()] = 
                [0; std::mem::size_of::<$type>()];
        $self.read_all(&mut tmp)?;
        Ok($type::from_be_bytes(tmp))
    })
}

/// The DataReader is a Reader that implements some
/// data functions.
pub trait DataReader: Reader {

    fn as_reader(&mut self) -> &mut dyn Reader;

    fn read_u16(&mut self) -> Result<u16> {
        data_reader_read_be_bytes!(self, u16)
    }

    fn read_u32(&mut self) -> Result<u32> {
        data_reader_read_be_bytes!(self, u32)
    }

    fn read_u64(&mut self) -> Result<u64> {
        data_reader_read_be_bytes!(self, u64)
    }

    fn read_ilint(&mut self) -> Result<u64> {
        match decode(self.as_reader()) {
            Ok(value) => Ok(value),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::CorruptedData)
        }
    }

    fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_u32()?))
    }

    fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_u64()?))
    }

    fn read_i16(&mut self) -> Result<i16> {
        Ok(self.read_u16()? as i16)
    }

    fn read_i32(&mut self) -> Result<i32> {
        Ok(self.read_u32()? as i32)
    }

    fn read_i64(&mut self) -> Result<i64> {
        Ok(self.read_u64()? as i64)
    }

    fn read_string(&mut self, size: usize) -> Result<String> {
        let mut tmp: Vec<u8> = Vec::with_capacity(size);
        tmp.resize(size, 0);
        self.read_all(tmp.as_mut_slice())?;
        match String::from_utf8(tmp) {
            Ok(s) => Ok(s),
            _ => Err(ErrorKind::CorruptedData)
        }
    }
}

macro_rules! data_writer_write_be_bytes {
    ($self: ident, $value: expr) => {
        $self.write_all(&$value.to_be_bytes())
    };
}

pub trait DataWriter: Writer {
    
    fn as_writer(&mut self) -> &mut dyn Writer;

    fn write_u16(&mut self, value: u16) -> Result<()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_u32(&mut self, value: u32) -> Result<()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_u64(&mut self, value: u64) -> Result<()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_ilint(&mut self, value: u64) -> Result<()> {
        match encode(value, self.as_writer()) {
            Ok(()) => Ok(()),
            Err(crate::ilint::ErrorKind::IOError(e)) => Err(e),
            _ => Err(ErrorKind::UnableToWriteData),
        }
    }

    fn write_f32(&mut self, value: f32) -> Result<()> {
        data_writer_write_be_bytes!(self, value)
    }
    
    fn write_f64(&mut self, value: f64) -> Result<()> {
        data_writer_write_be_bytes!(self, value)
    }

    fn write_i16(&mut self, value: i16) -> Result<()> {
        self.write_u16(value as u16)
    }

    fn write_i32(&mut self, value: i32) -> Result<()> {
        self.write_u32(value as u32)
    }

    fn write_i64(&mut self, value: i64) -> Result<()> {
        self.write_u64(value as u64)
    }

    fn write_string(&mut self, value: &String) -> Result<()> {
        self.write_all(value.as_bytes())
    }    
}
