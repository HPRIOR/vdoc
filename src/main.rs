use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, Write},
};

use clap::Parser;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Root path containing vdoc directory
    #[arg(short, long, default_value_t = String::from("/home/harry/Documents/vdoc"))]
    target_path: String,

    /// Name of created document
    #[arg(short, long, default_value_t = String::from(""))]
    file_name: String,

    #[arg(short, long, default_value_t = true)]
    md_scrap: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let target_path = args.target_path;
    let file_name = args.file_name;
    let file_path = create_file(&file_name, &target_path, args.md_scrap)?;
    io::stdout().write_all(file_path.as_bytes())?;

    Ok(())
}

fn read_stdin() -> String {
    let stdin = io::stdin();
    let mut buffer = String::new();
    for line in stdin.lock().lines().flatten() {
        buffer.push_str(line.as_str());
    }

    buffer
}

fn create_file(
    file_name: &String,
    target_path: &String,
    md_scrap: bool,
) -> Result<String, Box<dyn Error>> {
    let full_path = match file_name.as_str() {
        "" => format!(
            "{}/.scratch/scratch-{}{}",
            target_path,
            Uuid::new_v4(),
            if md_scrap { ".md" } else { "" }
        ),
        file_name => format!("{}/{}", target_path, file_name),
    };
    let mut file = File::create(full_path.as_str())?;
    if !atty::is(atty::Stream::Stdin) {
        let stdin = read_stdin();
        if !stdin.is_empty() {
            file.write_all(stdin.as_bytes())?;
        };
    }
    Ok(full_path)
}
