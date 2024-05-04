use std::fs::File;
use std::io::Write;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AppImage file path to install
    #[arg(short, long)]
    file_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Trying to install AppImage: {}", args.file_path);

    // write a file to /usr/share/applications/test.txt using rust
    let mut file = File::create("/usr/share/applications/test.txt")?;
    file.write_all(b"Hello, world!")?;

    Ok(())
}
