// External flate2 and tar crates for the compression
extern crate flate2;
extern crate tar;
extern crate indicatif;
extern crate progress;
extern crate pbr;
#[macro_use]
extern crate clap;

// Import for filesystem reading
use std::fs::{File};

// Imports for Compression
use flate2::write::GzEncoder;
use flate2::Compression;

// Imports for Decompression
use flate2::read::GzDecoder;
use tar::Archive;

// Clap Imports
use clap::App;

// Progress Bar and Spinner
/* use indicatif::{ProgressBar, ProgressStyle};
use pbr::{ProgressBar as PBar, Units}; */


//// TODO: Remove all unused imports

// For file sizes
//use tar::Header;
//#[macro_use]
//use clap::{Arg, App, SubCommand};
//use std::{thread, time};
//use std::process;
// Receiving user input
//use std::env;
//use std::io;
//use std::path::Path;
//use std::path::PathBuf;

///////////////////////////////////////////

fn main() {
    // Indexes the arguments provided when tarust is called in the terminal
    //let args: Vec<String> = env::args().collect();


    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml).get_matches();


    // Variable containing args in the Config struct
    //let config = Config::new(matches);


    // Creating the process and file variables for each argument for the tool
    let process = matches.value_of("process").unwrap();
    let file = matches.value_of("file").unwrap();

    /* let file_metadata = File::open(&file).unwrap().metadata().unwrap();
    let file_size = file_metadata.len(); */


    // Designing the progress bar
    /* let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-")); */

    /* .unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }) */
    //compress(args.clone());

    /* let mut progress_bar = PBar::new(file_size);
    progress_bar.set_units(Units::Bytes); */

    // TODO: Create proper terminal documentation instructions on using tarust

    // Compresses or Decompresses the folder depending on the arguments provided and panics
    // if a wrong compression type is provided
    if process == "compress" || process == "c" {
        println!("Compressing the {} directory...", file);
        compress(matches.clone()).unwrap();
        println!("Done compressing.");
    } else if process == "decompress" || process == "d" {
        println!("Decompressing {}.tar.gz...", file);
        decompress(matches.clone()).unwrap();
        println!("Done decompressing.");
    } else {
        panic!("Wrong argument provided... Choose compress or decompress");
    }
}


//
// Struct containing the arguments that can be provided
//
// struct Config {
//     compression_type: &'static str,
//     filename: &'static str,
// }
//
// impl Config {
//     fn new(args: clap::ArgMatches<'static>) -> Config {
//
//         // Panics when the number of arguments is not proper
//         /* if args.len() < 3 {
//             panic!("Not enough arguments");
//         }
//
//         if args.len() > 3 {
//             panic!("Too many argumnets");
//         } */
//
//         // Assigns the command argument index to variables for the compression_type and filename
//         let compression_type = &args.value_of("process").unwrap();
//         let filename = args.value_of("file").unwrap();
//
//         // Returns the Config struct
//         Config {
//             compression_type,
//             filename,
//         }
//     }
// }
//


fn compress(_args: clap::ArgMatches) -> Result<(), std::io::Error> {
    //let args: Vec<String> = env::args().collect();


    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml).get_matches();


    //let config = Config::new(matches);
    //let _process = matches.value_of("process").unwrap();


    let file = matches.value_of("file").unwrap();


    let tar_gz = File::create(format!("{}.tar.gz", file)).unwrap(); //Creates a .tar.gz with the filename argument as the archive's name

    // TODO: Experiment with other compression types
    let encoded_file = GzEncoder::new(tar_gz, Compression::best()); // Compresses using the optimal size for the data
    let mut tar = tar::Builder::new(encoded_file); // Creates the archive structure for the encoded_file


    // Recursively adds a directory and all its contents to the archive, giving the new archive
    // the name of the top-most directory on the tree
    tar.append_dir_all(format!("{}", file), format!("{}", file)).unwrap();

    // Returns a result
    Ok(())
}

fn decompress(_matches: clap::ArgMatches) -> Result<(), std::io::Error> {
    // TODO: Show the user all the files extracted from the tarball

    //let args: Vec<String> = env::args().collect();


    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(&yaml).get_matches();


    //let config = Config::new(matches);
    //let _process = matches.value_of("process").unwrap();


    let file = matches.value_of("file").unwrap();

    let path = format!("{}.tar.gz", file); // Name of the .tar.gz file

    let tar_gz = File::open(path)?; // Opens the files in the path variable as read-only
    let tar = GzDecoder::new(tar_gz); // Begins decoding the .tar.gz file
    let mut archive = Archive::new(tar); // Creates a directory with the files in the tarball
    archive.unpack(".")?; // Unpacks all the files into the destination: The given directory created by the archive variable

    // Returns a result
    Ok(())



    // let file = File::open(file)?;
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

// fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
//     let bar = match quiet_mode {
//         true => ProgressBar::hidden(),
//         false => {
//             match length {
//                 Some(len) => ProgressBar::new(len),
//                 None => ProgressBar::new_spinner(),
//             }
//         }
//     };

//     bar.set_message(msg);
//     match length.is_some() {
//         true => bar
//             .set_style(ProgressStyle::default_bar()
//                 .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
//                 .progress_chars("=> ")),
//         false => bar.set_style(ProgressStyle::default_spinner()),
//     };

//     bar
// }
