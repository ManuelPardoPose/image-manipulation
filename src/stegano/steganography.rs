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
        if data_bit_len > image_buffer.len() - HEADER_FIELD_SIZE {
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

#[cfg(test)]
mod tests {
    use image::ImageReader;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_simple_data() {
        let input_image =
            ImageReader::open("sample_images/concrete-shapes-free-image-2048x1365.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8();
        let data = "This is a simple Text";
        let encoded_res = DefaultSteganoGrapher::encode(&data, input_image);
        assert!(encoded_res.is_ok());
        let encoded = encoded_res.unwrap();
        let decoded = DefaultSteganoGrapher::decode(encoded);
        assert_eq!(data, &decoded, "{} {} should be equal", data, decoded);
    }

    #[test]
    fn test_more_complex_data() {
        let input_image =
            ImageReader::open("sample_images/concrete-shapes-free-image-2048x1365.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8();
        let data = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis at vero eros et accumsan et iusto odio dignissim qui blandit praesent luptatum zzril delenit augue duis dolore te feugait nulla facilisi. Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in vulputate velit es 😎";
        let encoded_res = DefaultSteganoGrapher::encode(&data, input_image);
        assert!(encoded_res.is_ok());
        let encoded = encoded_res.unwrap();
        let decoded = DefaultSteganoGrapher::decode(encoded);
        assert_eq!(data, &decoded, "{} {} should be equal", data, decoded);
    }

    #[test]
    fn test_max_data() {
        let input_image =
            ImageReader::open("sample_images/concrete-shapes-free-image-2048x1365.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8();
        let max_bytes = ((2048 * 1365) / 2) - (HEADER_FIELD_SIZE / 8);
        let mut valid_data = String::new();
        for _ in 0..max_bytes {
            valid_data.push_str("A");
        }
        let mut invalid_data = valid_data.clone();
        invalid_data.push_str("A");
        assert_eq!(valid_data.as_bytes().len(), max_bytes);
        assert!(invalid_data.as_bytes().len() > max_bytes);
        let encoded_res_valid = DefaultSteganoGrapher::encode(&valid_data, input_image.clone());
        let encoded_res_invalid = DefaultSteganoGrapher::encode(&invalid_data, input_image);
        assert!(
            encoded_res_valid.is_ok(),
            "{:?} should be ok",
            encoded_res_valid
        );
        assert!(
            encoded_res_invalid.is_err(),
            "{:?} should be err",
            encoded_res_invalid
        );
        let encoded_valid = encoded_res_valid.unwrap();
        let decoded = DefaultSteganoGrapher::decode(encoded_valid);
        assert_eq!(
            &valid_data, &decoded,
            "{} {} should be equal",
            valid_data, decoded
        );
    }
}
