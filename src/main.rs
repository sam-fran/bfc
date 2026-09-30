use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,
    output_path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
	let mut output = String::new()

    let input_string = match fs::read_to_string(&args.input_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read '{}': {}", args.input_path, err);
            std::process::exit(1);
        }
    };

    for c in input_string.chars() {
		output.push_str(match c {
			_ => "",
		})
    }

    Ok(())
}
