use clap::Parser;
use clap::Subcommand;
use image_manipulation::commands::decode_command;
use image_manipulation::commands::encode_command;
use image_manipulation::gui::start_gui;

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
    /// Open GUI
    Gui {},
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Commands::Encode { inpath, data, key }) => {
            encode_command(inpath, data, key);
        }
        Some(Commands::Decode { inpath, key }) => {
            let _ = decode_command(inpath, key);
        }
        Some(Commands::Gui {}) => {
            start_gui("image-manipulation-gui");
        }
        None => {}
    }
}
