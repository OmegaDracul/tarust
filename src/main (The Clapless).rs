// External flate2 and tar crates for the compression
extern crate flate2;
extern crate tar;

// Receiving user input
use std::env;

// Import for filesystem reading
use std::fs::File;

// Imports for Compression
use flate2::write::GzEncoder;
use flate2::Compression;

// Imports for Decompression
use flate2::read::GzDecoder;
use tar::Archive;

// TODO: Remove all unused imports
//use std::process;
//use std::io;
//use std::path::Path;
//use std::path::PathBuf;


fn main() {
    // Indexes the arguments provided when tarust is called in the terminal
    let args: Vec<String> = env::args().collect();

    // Variable containing args in the Config struct
    let config = Config::new(&args);

    /* .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }) */
    //compress(args.clone());


    // TODO: Create proper terminal documentation instructions on using tarust

    // Compresses or Decompresses the folder depending on the arguments provided and panics
    // if a wrong compression type is provided
    if config.compression_type == "compress" {
        println!("Compressing the {} directory...", config.filename);
        compress(args.clone()).unwrap();
        println!("Done compressing.");
    } else if config.compression_type == "decompress" {
        println!("Decompressing {}.tar.gz...", config.filename);
        decompress(args.clone()).unwrap();
        println!("Done decompressing.");
    } else {
        panic!("Wrong argument provided... Choose compress or decompress");
    }
}

// Struct containing the arguments that can be provided
struct Config {
    compression_type: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {

        // Panics when the number of arguments is not proper
        if args.len() < 3 {
            panic!("Not enough arguments");
        }

        if args.len() > 3 {
            panic!("Too many argumnets");
        }

        // Assigns the command argument index to variables for the compression_type and filename
        let compression_type = args[1].clone().parse().unwrap();
        let filename = args[2].clone().parse().unwrap();

        // Returns the Config struct
        Config {
            compression_type,
            filename,
        }
    }
}


fn compress(args: Vec<String>) -> Result<(), std::io::Error> {
    //let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let tar_gz = File::create(format!("{}.tar.gz", config.filename)).unwrap(); //Creates a .tar.gz with the filename argument as the archive's name

    // TODO: Experiment with other compression types
    let encoded_file = GzEncoder::new(tar_gz, Compression::best()); // Compresses using the optimal size for the data
    let mut tar = tar::Builder::new(encoded_file); // Creates the archive structure for the encoded_file

    // Recursively adds a directory and all its contents to the archive, giving the new archive
    // the name of the top-most directory on the tree
    tar.append_dir_all(format!("{}", config.filename), format!("{}", config.filename))?;

    // Returns a result
    Ok(())
}

fn decompress(args: Vec<String>) -> Result<(), std::io::Error> {
    // TODO: Show the user all the files extracted from the tarball


    //let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let path = format!("{}.tar.gz", config.filename); // Name of the .tar.gz file

    let tar_gz = File::open(path)?; // Opens the files in the path variable as read-only
    let tar = GzDecoder::new(tar_gz); // Begins decoding the .tar.gz file
    let mut archive = Archive::new(tar); // Creates a directory with the files in the tarball
    archive.unpack(".")?; // Unpacks all the files into the destination: The given directory created by the archive variable

    // Returns a result
    Ok(())



    // let file = File::open(config.filename)?;
    // let mut archive = Archive::new(GzDecoder::new(file));
    // let prefix = "bundle/logs";

    // println!("Extracted the following files:");
    // archive
    //     .entries()?
    //     .filter_map(|e| e.ok())
    //     .map(|mut entry| -> Result<PathBuf, std::io::Error> {
    //         let path = entry
    //             .path()
    //             .unwrap()
    //             .strip_prefix(prefix)
    //             .unwrap()
    //             .to_owned();
    //         entry.unpack(&path)?;
    //         Ok(path)
    //     })
    //     .filter_map(|e| e.ok())
    //     .for_each(|x| println!("> {}", x.display()));

    // Ok(())
}
