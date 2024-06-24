fn main() {
    // number literals (taken from the Rust book)
    println!("Big number is {}", 98_220_000); // underscores for large numbers
    println!("Hex is {}", 0xff); // hexadecimal number
    println!("Octal is {}", 0o77); // octal number
    println!("Binary is {}", 0b1111_0000); // binary
    println!("Bites 'A' is {}", b'A'); // binary
    
    // raw strimg literals
    let raw = r#"{"message": "something raw"}"#;
    println!("{}", &raw);
    dbg!(raw);
    
}
