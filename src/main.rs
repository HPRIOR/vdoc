use std::{
    env,
    error::Error,
    fs::File,
    io::{self, BufRead, Write},
};

use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let file_name = args.get(1);
    let file_path = create_file(file_name)?;

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

fn create_file(file_name: Option<&String>) -> Result<String, Box<dyn Error>> {
    let full_path = match file_name {
        Some(file_name) => format!("/home/harry/Documents/vdoc/{}", file_name),
        None => format!(
            "/home/harry/Documents/vdoc/.scratch/scratch-{}",
            Uuid::new_v4()
        ),
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
