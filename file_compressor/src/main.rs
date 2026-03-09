extern crate flate2;

use flate2::write::GzEncoder; // mports the Gzip implementation for write streams. It wraps a "writer" (like a file) and compresses everything sent to it.
use flate2::Compression; 
use std::env::args; // A function that returns an Iterator of the command-line arguments passed to the program (e.g., program_name source_file target_file).
use std::fs::File;// The standard struct for file system handles, allowing you to read or write bytes to disk.
use std::io::BufReader;// A "wrapper" that adds an internal memory buffer to a reader. Instead of asking the hard drive for 1 byte at a time, it grabs a large chunk (usually 8KB) and keeps it in RAM for faster access.
use std::time::Instant;
// Object: Instant
//Role: A monotonically increasing timer used to measure elapsed time (like a stopwatch).

fn main() {
    if args().len() != 3{
        // Checks if the user provided exactly two arguments (plus the program name). If not, it prints a usage error to stderr and exits.
        eprintln("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap()); 
    // First declares a mut var input because it changes as we read. opens the source file in read only mode , fetches the first argument from user input ad thenn umwrap helps in panicxking the function if the file doesn't exist or somwthimg crashed. 
    
    let output = File::create(args().nth(2).unwrap()).unwrap(); // File::create(...): Opens the target file in write-only mode. If it already exists, it is truncated (emptied).
    let mut encoder = GzEncoder::new(output, Compression::default()); 
    // t takes the output file and wraps it. Any data you "write" to encoder gets squeezed by the Gzip algorithm before being handed off to the output file.
    let start = Instant::now(); // "Starts the stopwatch" right before the heavy lifting begins.
    copy(&mut input, &mut encoder).unwrap();
   // this is the loop engine. It reads bytes from input (the source) and writes them to encoder (the Gzip stream). It continues until the end of the source file.
    let output = encoder.finish().unwrap(); 
    //Method: .finish()
    //Role: This is vital. It tells the encoder "I'm done." The encoder then writes the Gzip footer (checksums and sizes) and returns the ownership of the inner output file so you can inspect it.
    println!(
        "Source len: {:?}",
        input.get_ref().metadata.unwrap.len();
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!( "Elapsed: {:?}", start.elapsed());
}
    
