use std::io;
use std::io::Error;

use aes_siv::Aes128SivAead;
use aes_siv::Key;
use aes_siv::KeyInit;
use aes_siv::Nonce;
use aes_siv::aead::AeadMut;
use clap::Parser;
use clap::Subcommand;

use image::EncodableLayout;
use image::ImageReader;
use image::RgbaImage;
use image_manipulation::stegano::steganography::{Decode, DefaultSteganoGrapher, Encode};

const FILE_READING_ERROR: &str = "Error: A problem occured while reading the input file.";
const FILE_SAVING_ERROR: &str = "Error: A problem occured while saving the output file.";
const INVALID_KEY_ERROR: &str = "Error: The key should be 32 bytes.";

/// image-manipulation
/// Can Encode/Decode Data into Images
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes data into an image
    Encode {
        /// The path of the input file
        #[arg()]
        inpath: String,

        /// The data to be encoded. Alternative ways of passing in data:
        ///
        /// - pipe data into process (not yet implemented)
        #[arg()]
        data: String,

        /// Optional AES-SIV key that encrypts the data before encoding
        ///
        /// length: 32 byte (256 bit)
        #[arg(short, long)]
        key: Option<String>,
    },
    /// Decodes data from an image
    Decode {
        /// The path of the input file
        #[arg()]
        inpath: String,

        /// Optional AES-SIV key that decrypts the data after encoding
        ///
        /// length: 32 byte (256 bit)
        #[arg(short, long)]
        key: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Commands::Encode { inpath, data, key }) => {
            encode_command(inpath, data, key);
        }
        Some(Commands::Decode { inpath, key }) => {
            decode_command(inpath, key);
        }
        None => {}
    }
}

fn encode_command(inpath: String, data: String, key: Option<String>) {
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
        cipher.encrypt(nonce, data.as_bytes()).unwrap()
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

fn decode_command(inpath: String, key: Option<String>) {
    let img = match open_image_as_rgba(&inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return;
        }
    };
    let data = DefaultSteganoGrapher::decode(img);
    let data = if let Some(key) = key {
        let key_bytes = key.as_bytes();
        if key_bytes.len() != 32 {
            println!("{INVALID_KEY_ERROR}");
            return;
        }

        let key = Key::<Aes128SivAead>::from_slice(key_bytes);
        let mut cipher = Aes128SivAead::new(key);
        let nonce = Nonce::from_slice(b"any unique nonce");
        cipher.decrypt(nonce, data.as_bytes()).unwrap()
    } else {
        data
    };
    let data = String::from_utf8(data).unwrap_or(String::from("Not UTF8"));
    println!("Decoded Data:\n{data}");
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
        Err(e) => Err(io::Error::other(e)),
    }
}
