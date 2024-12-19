pub fn string_to_keys(input: &str, output_buffer: &mut [(u8, bool)]) -> usize {
    let mut index = 0;

    for c in input.chars() {
        if index >= output_buffer.len() {
            break; // Prevent buffer overflow
        }
        let (usage, shift) = match c {
            'a'..='z' => (0x04 + (c as u8 - b'a'), false),
            'A'..='Z' => (0x04 + (c as u8 - b'A'), true),
            '1' => (0x1E, false),
            '!' => (0x1E, true),
            '2' => (0x1F, false),
            '@' => (0x1F, true),
            '3' => (0x20, false),
            '#' => (0x20, true),
            '4' => (0x21, false),
            '$' => (0x21, true),
            '5' => (0x22, false),
            '%' => (0x22, true),
            '6' => (0x23, false),
            '^' => (0x23, true),
            '7' => (0x24, false),
            '&' => (0x24, true),
            '8' => (0x25, false),
            '*' => (0x25, true),
            '9' => (0x26, false),
            '(' => (0x26, true),
            '0' => (0x27, false),
            ')' => (0x27, true),
            '\n' => (0x28, false),
            '\x1B' => (0x29, false), // Escape
            '\x08' => (0x2A, false), // Backspace
            '\t' => (0x2B, false),   // Tab
            ' ' => (0x2C, false),
            '-' => (0x2D, false),
            '_' => (0x2D, true),
            '=' => (0x2E, false),
            '+' => (0x2E, true),
            '[' => (0x2F, false),
            '{' => (0x2F, true),
            ']' => (0x30, false),
            '}' => (0x30, true),
            '\\' => (0x31, false),
            '|' => (0x31, true),
            ';' => (0x33, false),
            ':' => (0x33, true),
            '\'' => (0x34, false),
            '"' => (0x34, true),
            '`' => (0x35, false),
            '~' => (0x35, true),
            ',' => (0x36, false),
            '<' => (0x36, true),
            '.' => (0x37, false),
            '>' => (0x37, true),
            '/' => (0x38, false),
            '?' => (0x38, true),
            _ => continue, // Ignore unsupported characters
        };

        output_buffer[index] = (usage, shift);
        index += 1;
    }

    return index;
}
