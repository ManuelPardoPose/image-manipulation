use std::fmt;

use image::{ImageBuffer, Rgba};

#[derive(Debug, Clone)]
pub struct DataLengthError;

type Result<T> = std::result::Result<T, DataLengthError>;

impl fmt::Display for DataLengthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The data is to long to be encoded into the given image")
    }
}

pub trait Encode {
    fn encode(
        data: &str,
        image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>>;
}

pub trait Decode {
    fn decode(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String;
}

pub struct DefaultSteganoGrapher {}

const HEADER_FIELD_SIZE: usize = 32;

impl Encode for DefaultSteganoGrapher {
    fn encode(
        data: &str,
        mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let data_bytes = data.as_bytes();
        let data_bytes_len = data_bytes.len();
        let data_bit_len = data_bytes_len * 8;
        // TODO: check for data length and compare with image pixel array length
        // TODO: add Encode Error or something like that
        if data_bit_len >= image_buffer.len() {
            return Err(DataLengthError);
        }

        for (idx, val) in image_buffer.iter_mut().enumerate() {
            if idx >= HEADER_FIELD_SIZE + data_bit_len {
                break;
            }
            *val &= !1; // make even

            let bit: u8 = if idx < HEADER_FIELD_SIZE {
                (data_bytes_len >> idx) as u8 & 1
            } else {
                let data_idx = idx - HEADER_FIELD_SIZE;
                let byte = data_bytes[data_idx / 8];
                (byte >> (data_idx % 8)) & 1
            };
            *val += bit;
        }
        Ok(image_buffer)
    }
}

impl Decode for DefaultSteganoGrapher {
    fn decode(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
        let mut data_bytes: Vec<u8> = Vec::new();
        let mut data_bytes_len: usize = 0;
        let mut data_bit_len: usize = 0;
        let mut curr_byte_buffer: u8 = 0;

        for (idx, val) in image_buffer.iter().enumerate() {
            if idx >= HEADER_FIELD_SIZE + data_bit_len {
                break;
            }

            if idx < HEADER_FIELD_SIZE {
                let bit = (*val as usize & 1) << (idx);
                data_bytes_len += bit;
                if idx + 1 == HEADER_FIELD_SIZE {
                    data_bit_len = data_bytes_len * 8;
                    data_bytes = Vec::with_capacity(data_bytes_len);
                }
            } else {
                let data_idx = idx - HEADER_FIELD_SIZE;
                let bit = (*val & 1) << (data_idx % 8);
                curr_byte_buffer += bit;
                if (data_idx + 1) % 8 == 0 {
                    data_bytes.push(curr_byte_buffer);
                    curr_byte_buffer = 0;
                }
            }
        }
        String::from_utf8(data_bytes).unwrap_or(String::from("Not UTF8"))
    }
}
