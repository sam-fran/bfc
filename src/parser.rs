pub fn parse(input: String) -> String {
	let mut output = String::new();

	output.push_str("#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;");

	
	for c in input.chars() {
		output.push_str(match c {
			'<' => "ptr--;",
			'>' => "ptr++;",
			'+' => "mem[ptr]++;",
			'-' => "mem[ptr]--;",
			',' => "mem[ptr] = getchar();",
			'.' => "putchar(mem[ptr]);",
			'[' => "while(mem[ptr]){",
			']' => "};",
			_ => "",
		});
	}

	output.push_str("}");

	output
}