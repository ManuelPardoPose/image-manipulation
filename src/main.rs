use std::io;
use std::io::Error;

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
    },
    /// Decodes data from an image
    Decode {
        /// The path of the input file
        #[arg()]
        inpath: String,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Encode { inpath, data }) => {
            encode(inpath, data);
        }
        Some(Commands::Decode { inpath }) => {
            if let Some(data) = decode(inpath) {
                println!("Decoded Data:\n{data}")
            }
        }
        None => {}
    }
}

fn encode(inpath: &str, data: &str) {
    let mut img = match open_image_as_rgba(inpath) {
        Ok(img) => img,
        Err(e) => {
            println!("{FILE_READING_ERROR}\n{e}");
            return;
        }
    };
    img = DefaultSteganoGrapher::encode(data, img);
    let outpath = inpath.replace(".jpg", ".png").replace(".png", "-e.png");
    if let Err(e) = img.save(outpath) {
        println!("{FILE_SAVING_ERROR}\n{e}");
    };
}

fn decode(inpath: &str) -> Option<String> {
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
