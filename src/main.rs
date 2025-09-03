use std::io;
use std::io::Error;

use aes_gcm::AeadCore;
use aes_gcm::Aes256Gcm;
use aes_gcm::Key;
use aes_gcm::KeyInit;
use aes_gcm::aead::Aead;
use aes_gcm::aead::OsRng;
use clap::Parser;
use clap::Subcommand;

use image::ImageReader;
use image::RgbaImage;
use image_manipulation::stegano::steganography::{Decode, DefaultSteganoGrapher, Encode};

const FILE_READING_ERROR: &str = "Error: A problem occured while reading the input file.";
const FILE_SAVING_ERROR: &str = "Error: A problem occured while saving the output file.";

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

        /// Optional AES-GCM-256 key that encrypts the data before encoding
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

        /// Optional AES-GCM-256 key that decrypts the data after encoding
        ///
        /// length: 32 byte (256 bit)
        #[arg(short, long)]
        key: Option<String>,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Encode { inpath, data, key }) => {
            encode_command(inpath, data, key);
        }
        Some(Commands::Decode { inpath, key }) => {
            if let Some(data) = decode_command(inpath, key) {
                println!("Decoded Data:\n{data}")
            }
        }
        None => {}
    }
}

fn encode_command(inpath: &str, data: &str, key: &Option<String>) {
    let mut img = match open_image_as_rgba(inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return;
        }
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

fn decode_command(inpath: &str, key: &Option<String>) -> Option<String> {
    let img = match open_image_as_rgba(inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return None;
        }
    };
    let data = DefaultSteganoGrapher::decode(img);
    Some(data)
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
