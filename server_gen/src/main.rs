use paw::Args;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "server-gen")]
struct Opt {
    #[structopt(name = "SRC_DIR", parse(from_os_str))]
    src_dir: PathBuf,
}

// Iter recursively through SRC_DIR checking .rs files
// Read line-by-line to find a matching struct or function annotation

// If found, tokenize the file with syn, saving the src file name in the process.
// Generate server code.
// Dump server file based on name of the src file.

// If not found, continue searching files

#[paw::main]
fn main(args: Args) -> Result<(), std::io::Error> {
    println!("Hello, world!");
    Ok(())
}
