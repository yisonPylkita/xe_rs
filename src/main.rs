use std::fs;

fn print_hex_data(content: &Vec<u8>) {
    // Print only 16 bytes in a row
    for chunk in content.chunks(16) {
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!("");
    }
}

fn main()
{
    let content = fs::read("Cargo.toml").unwrap();
    print_hex_data(&content);
}