use std::fs;

fn main()
{
    let content = fs::read("Cargo.toml");
    println!("{:?}", content);
}