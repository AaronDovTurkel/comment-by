use std::path::PathBuf;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;
use encoding_rs_io::DecodeReaderBytesBuilder;

/// A CLI tool that comments out parts of a file.
#[derive(StructOpt, Debug)]
#[structopt(name = "comment-by")]
struct Opt {

    /// Appended comment character
    #[structopt(short = "cc", long = "comment-character", default_value  = "--")]
    comment_char: String,

    /// String used to split the file
    #[structopt(short, long, default_value = "/****** Object:")]
    split_by: String,
    
    /// String used to split the file
    #[structopt(short = "i", long, default_value = "test")]
    comment_if: String,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
struct Chunk {
    comment_out: bool,
    lines: Vec<String>
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            comment_out: false,
            lines: Vec::new()
        }
    }
    fn process_line(&self, mut line: String, comment_char: &str) -> String {
        if self.comment_out {
            line = format!("{} {}", comment_char, line);
        };
        format!("{}\n", line)
    }
    fn write_chunk_to_file(&mut self, comment_char: &str, mut open_file: &File) -> Result<(), std::io::Error> {
        for chunk_line in &self.lines {
            open_file.write_all(self.process_line(String::from(chunk_line), comment_char).as_bytes())?
        };
        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();
    let file_string = opt.input.to_str().expect("could not read input file");
    let output_file = opt.output.to_str().expect("could not read output file");
    
    comment_chunks(
        &file_string,
        &output_file,
        &opt.split_by,
        &opt.comment_if,
        &opt.comment_char
    ).expect("Could not comment file chunks");
    
    println!(
        "Created {} file with any chunk that contains \"{}\" commented out.",
        output_file,
        opt.comment_if
    );
}

fn comment_chunks(
    input_file: &str,
    output_file: &str,
    split_by: &str,
    comment_if: &str,
    comment_char: &str
) -> Result<(), std::io::Error> {
    let file_in = File::open(input_file)?;
    let file_out = File::create(output_file)?;
    let reader = BufReader::new(DecodeReaderBytesBuilder::new().build(file_in));
    let mut chunk = Chunk::new();
    
    for (i, line) in reader.lines().enumerate() {
        let new_line = line?;
        if new_line.contains(&split_by) && i > 0 {
            chunk.write_chunk_to_file(comment_char, &file_out)?;
            chunk = Chunk::new();
        }
        if !chunk.comment_out && new_line.to_uppercase().contains(&comment_if.to_uppercase()) {
            chunk.comment_out = true;
        };
        chunk.lines.push(new_line)
    };
    chunk.write_chunk_to_file(comment_char, &file_out)?;

    Ok(())
}

