use crate::encoder::compression::{Compression, CompressionAlgorithm, Compressor};
use crate::tags::CompressionMethod;
use std::io::{Error, ErrorKind, Seek, Write};
use std::ops::DerefMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Jpeg {
    pub quality: u8,
    pub width: u16,
    pub height: u16,
}


impl CompressionAlgorithm for Jpeg {
    fn write_to<W: Write + Seek>(&mut self, mut writer: &mut W, bytes: &[u8]) -> Result<u64, Error> {
        let start = writer.stream_position()?;
        // println!("bl {}", bytes.len());
        {
            let enc = jpeg_encoder::Encoder::new(writer.deref_mut(), self.quality);
            enc.encode(
                bytes,
                self.height,
                self.width,
                jpeg_encoder::ColorType::Luma,
            ).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        }

        let end = writer.stream_position()?;

        Ok(end - start)
    }
}

impl Compression for Jpeg {
    const COMPRESSION_METHOD: CompressionMethod = CompressionMethod::ModernJPEG;

    fn get_algorithm(&self) -> Compressor {
        Compressor::Jpeg(*self)
    }
}
