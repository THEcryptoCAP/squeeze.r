extern crate flate2;

use flate2::write::GzEncoder; // mports the Gzip implementation for write streams. It wraps a "writer" (like a file) and compresses everything sent to it.
use flate2::Compression; 
use std::env::args; // A function that returns an Iterator of the command-line arguments passed to the program (e.g., program_name source_file target_file).
use std::fs::File;// The standard struct for file system handles, allowing you to read or write bytes to disk.
use std::io::BufReader;// A "wrapper" that adds an internal memory buffer to a reader. Instead of asking the hard drive for 1 byte at a time, it grabs a large chunk (usually 8KB) and keeps it in RAM for faster access.
use std::time::Instant;

fn main() {
    if args().len() != 3{
        eprintln("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata.unwrap.len();
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!( "Elapsed: {:?}", start.elapsed());
}
    
