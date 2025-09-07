use aes_siv::Aes128SivAead;
use aes_siv::Key;
use aes_siv::KeyInit;
use aes_siv::Nonce;
use aes_siv::aead::AeadMut;
use image::EncodableLayout;
use image::ImageReader;
use image::RgbaImage;
use std::io::Error;

use crate::steganography::Decode;
use crate::steganography::DefaultSteganoGrapher;
use crate::steganography::Encode;

const FILE_READING_ERROR: &str = "Error: A problem occured while reading the input file.";
const FILE_SAVING_ERROR: &str = "Error: A problem occured while saving the output file.";
const INVALID_KEY_ERROR: &str = "Error: The key should be 32 bytes.";
const DECRYPTION_ERROR: &str =
    "Error: Decryption did not work. Either invalid key or non decryptable file.";

pub fn encode_command(inpath: String, data: String, key: Option<String>) {
    let mut img = match open_image_as_rgba(&inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return;
        }
    };
    let data = if let Some(key) = key {
        let key_bytes = key.as_bytes();
        if key_bytes.len() != 32 {
            println!("{INVALID_KEY_ERROR}");
            return;
        }

        let key = Key::<Aes128SivAead>::from_slice(key_bytes);
        let mut cipher = Aes128SivAead::new(key);
        let nonce = Nonce::from_slice(b"any unique nonce");
        if let Ok(data) = cipher.encrypt(nonce, data.as_bytes()) {
            data
        } else {
            // theoreticall this is not possible
            return;
        }
    } else {
        data.as_bytes().to_vec()
    };
    img = match DefaultSteganoGrapher::encode(data, img) {
        Ok(img) => img,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let outpath = inpath.replace(".jpg", ".png").replace(".png", "-e.png");
    if let Err(e) = img.save(outpath) {
        println!("{FILE_SAVING_ERROR}\n{e}");
    };
}

pub fn decode_command(inpath: String, key: Option<String>) -> String {
    let img = match open_image_as_rgba(&inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return FILE_READING_ERROR.to_string();
        }
    };
    let data = DefaultSteganoGrapher::decode(img);
    let data = if let Some(key) = key {
        let key_bytes = key.as_bytes();
        if key_bytes.len() != 32 {
            println!("{INVALID_KEY_ERROR}");
            return INVALID_KEY_ERROR.to_string();
        }

        let key = Key::<Aes128SivAead>::from_slice(key_bytes);
        let mut cipher = Aes128SivAead::new(key);
        let nonce = Nonce::from_slice(b"any unique nonce");
        if let Ok(data) = cipher.decrypt(nonce, data.as_bytes()) {
            data
        } else {
            println!("{DECRYPTION_ERROR}");
            return DECRYPTION_ERROR.to_string();
        }
    } else {
        data
    };
    let data = String::from_utf8(data).unwrap_or(String::from("Not UTF8"));
    println!("Decoded Data:\n{data}");
    data
}

fn open_image_as_rgba(path: &str) -> Result<RgbaImage, Error> {
    let img_reader = match ImageReader::open(path) {
        Ok(img_reader) => img_reader,
        Err(e) => {
            return Err(e);
        }
    };
    match img_reader.decode() {
        Ok(dyn_img) => Ok(dyn_img.into_rgba8()),
        Err(e) => Err(Error::other(e)),
    }
}
