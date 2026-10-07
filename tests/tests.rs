fn assert_parse(input: &str, expected: &str) {
    assert_eq!(bfc::parse(input.to_string()), expected.to_string());
}

#[test]
fn parse_empty_string() {
    assert_parse(
        "",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;}",
    );
}

#[test]
fn parse_greater_than_symbol() {
    assert_parse(
        ">+<",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;ptr++;mem[ptr]++;ptr--;}",
    );
}

#[test]
fn parse_less_than_symbol() {
    assert_parse(
        "<+>",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;ptr--;mem[ptr]++;ptr++;}",
    );
}

#[test]
fn parse_plus_symbol() {
    assert_parse(
        ">+<",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;ptr++;mem[ptr]++;ptr--;}",
    );
}

#[test]
fn parse_minus_symbol() {
    assert_parse(
        "-<+",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;mem[ptr]--;ptr--;mem[ptr]++;}",
    );
}

#[test]
fn parse_comma_symbol() {
    assert_parse(
        ",+>",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;mem[ptr] = getchar();mem[ptr]++;ptr++;}",
    );
}

#[test]
fn parse_period_symbol() {
    assert_parse(
        ".+>",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;putchar(mem[ptr]);mem[ptr]++;ptr++;}",
    );
}

#[test]
fn parse_open_bracket_symbol() {
    assert_parse(
        "[+]-",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;while(mem[ptr]){mem[ptr]++;};mem[ptr]--;}",
    );
}

#[test]
fn parse_close_bracket_symbol() {
    assert_parse(
        "]+",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;};mem[ptr]++;}",
    );
}

#[test]
fn parse_ignores_unknown_characters() {
    assert_parse(
        "abc<>123+-XYZ",
        "#include <stdio.h>\nint main(){unsigned char mem[3000] = {0};unsigned int ptr = 0;ptr--;ptr++;mem[ptr]++;mem[ptr]--;}",
    );
}
