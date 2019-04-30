use std::env;
use std::fs;

fn print_hex_data(content: &Vec<u8>) {
    // Print only 16 bytes in a row
    let mut line_number = 0x00;
    for chunk in content.chunks(16) {
        // print line number
        print!("{:08x}  ", line_number);
        // print hex view
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        print!("    ");
        // print string view
        if chunk.len() < 16 {
            // handling last line case
            print!("{}  ", " ".repeat(16 - chunk.len()));
        }
        for byte in chunk {
            match byte {
                0x00 ... 0x1f => {
                    print!(".");
                },
                0x20 ... 0x7e => {
                    print!("{}", *byte as char);
                },
                0x7f ... 0xff => {
                    print!(".");
                },
            }
        }

        println!("");
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