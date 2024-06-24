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

    // working with binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a in binary is {:08b}", a); // print out as binary
    println!("b in binary is {:08b}", b);

    // logic gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);

    // bitwise operations
    println!("a << 1 {:08b}", a << 1); // shifted left by 1 bit
    println!("a's value is {}", a << 1);
    println!("a >> 1 {:08b}", a >> 1); // shifted right by 1 bit
    println!("a's value is {}", a >> 1);

    // little endian or big endian
    // https://en.wikipedia.org/wiki/Endianness
    // In computing, endianness is the order in which bytes within a word of digital data 
    // are transmitted over a data communication medium or addressed (by rising addresses) 
    // in computer memory, counting only byte significance compared to earliness.
    let n: u16 = 0x1234;
    println!("n is {:}", n);

    let big_endian = n.to_be_bytes();
    let little_endian = n.to_le_bytes();

    println!("n in big endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little endian: {:02X}{:02X}", little_endian[0], little_endian[1]);
    
} 
