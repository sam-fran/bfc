use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    input_path: String,

    #[arg(short)]
    output_path: Option<String>,
}


fn main() {
    let args = Args::parse();

    let input_string = match fs::read_to_string(&args.input_path) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to read '{}': {}", args.input_path, err);
            std::process::exit(1);
        }
    };

    let output = bfc::parse(input_string);

	let output_path = args.output_path.unwrap_or_else(|| {
    	format!("{}.c", args.input_path)
	});

    if let Err(err) = fs::write(&output_path, output) {
        eprintln!("Failed to write '{}': {}", output_path, err);
        std::process::exit(1);
    }
}
