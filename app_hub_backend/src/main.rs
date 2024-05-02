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

    let command = format!("pkexec --user root sh -c \"echo '{}' > {}\"", "Hello, world!", "/usr/share/applications/test.txt");
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();

    let command = format!("pkexec --user root sh -c \"echo '{}' > {}\"", "Hello, world!", "/usr/share/applications/test.txt");
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("File written successfully");
                Ok(())
            } else {
                println!("Failed to write file: {}", String::from_utf8_lossy(&output.stderr));
                Err("Failed to write file".into())
            }
        },
        Err(e) => {
            println!("Failed to execute command '{}': {}", command, e);
            Err("Failed to execute command".into())
        }
    }
}
