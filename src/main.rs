use std::env;
use std::fs;

fn print_hex_data(content: &Vec<u8>) {
    // Print only 16 bytes in a row
    let mut line_number = 0x00;
    for chunk in content.chunks(16) {
        // print line number
        let mut line_buffer = format!("{:08x}  ", line_number);
        // print hex view
        for byte in chunk {
            line_buffer.push_str(&format!("{:02x} ", byte));
        }
        line_buffer.push_str(&format!("    "));
        // print string view
        if chunk.len() < 16 {
            // handling last line case
            line_buffer.push_str(&format!("{}", " ".repeat(48 - chunk.len() * 3)));
        }
        for byte in chunk {
            match byte {
                0x00 ... 0x1f => {
                    line_buffer.push_str(&format!("."));
                },
                0x20 ... 0x7e => {
                    line_buffer.push_str(&format!("{}", *byte as char));
                },
                0x7f ... 0xff => {
                    line_buffer.push_str(&format!("."));
                },
            }
        }
        println!("{}", line_buffer);
        line_number += 16;
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let file_to_load = &args[1];
    let content = fs::read(file_to_load).unwrap();
    print_hex_data(&content);
}