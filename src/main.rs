use std::fs;

fn print_hex_data(content: &Vec<u8>) {
    // Print only 16 bytes in a row
    for chunk in content.chunks(16) {
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
    }
}

fn main()
{
    let content = fs::read("Cargo.toml").unwrap();
    print_hex_data(&content);
}