use image::{ImageBuffer, Rgba};

pub trait Encode {
    fn encode(
        message: &str,
        image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>>;
}

pub trait Decode {
    fn decode(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String;
}

pub struct DefaultSteganoGrapher {}

impl Encode for DefaultSteganoGrapher {
    fn encode(
        message: &str,
        mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let message_bytes = message.as_bytes();
        let message_bytes_len = message_bytes.len();
        let message_bit_len = message_bytes_len * 8;
        let mut bit_pointer: usize = 0;

        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                if x == 0 && y == 0 {
                    let len_encoded_to_rgb: [u8; 4] = [
                        (message_bytes_len / (255 * 255 * 255) % 255) as u8,
                        (message_bytes_len / (255 * 255) % 255) as u8,
                        (message_bytes_len / 255 % 255) as u8,
                        (message_bytes_len % 255) as u8,
                    ];
                    image_buffer.put_pixel(x, y, Rgba(len_encoded_to_rgb));
                    continue;
                }

                if bit_pointer >= message_bit_len {
                    return image_buffer;
                }

                let pixel = image_buffer.get_pixel(x, y).0;
                let (mut r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                r -= r % 2;
                let byte = message_bytes[bit_pointer / 8];
                let bit = (byte >> (bit_pointer % 8)) % 2;
                image_buffer.put_pixel(x, y, Rgba([r + bit, g, b, a]));

                bit_pointer += 1;
            }
        }
        image_buffer
    }
}

impl Decode for DefaultSteganoGrapher {
    fn decode(image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> String {
        let mut message_bytes: Vec<u8> = Vec::new();
        let mut message_bytes_len: usize;
        let mut message_bit_len: usize = 0;
        let mut bit_pointer: usize = 0;
        let mut curr_byte_buffer: u8 = 0;

        for x in 0..image_buffer.width() {
            for y in 0..image_buffer.height() {
                let pixel = image_buffer.get_pixel(x, y).0;
                let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
                if x == 0 && y == 0 {
                    message_bytes_len = (r as u32 * 255 * 255 * 255
                        + g as u32 * 255 * 255
                        + b as u32 * 255
                        + a as u32) as usize;
                    message_bit_len = message_bytes_len * 8;
                    message_bytes = Vec::with_capacity(message_bytes_len);
                    continue;
                }

                if bit_pointer >= message_bit_len {
                    return String::from_utf8(message_bytes).unwrap_or(String::from("Not UTF8"));
                }

                let bit = (r % 2) << (bit_pointer % 8);
                curr_byte_buffer += bit;
                bit_pointer += 1;
                if bit_pointer % 8 == 0 {
                    message_bytes.push(curr_byte_buffer);
                    curr_byte_buffer = 0;
                }
            }
        }
        String::from_utf8(message_bytes).unwrap_or(String::from("Not UTF8"))
    }
}
