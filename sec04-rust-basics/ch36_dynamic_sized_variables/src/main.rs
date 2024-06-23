fn main() {
    // string literal == reference to a string
    let name: &str = "Fernand";
    println!("The name is {}", name);

    // dynamic string on the heap
    let dynamic_name = String::from("Fernand Braudel");
    // printing out with a pointer
    println!("The dynamic name ({}) in memory is {:p}", dynamic_name, &dynamic_name); // 0x16d08eb18
    
    let make_name_dynamic = name.to_string();
    println!("make_name_dynamic is {}", make_name_dynamic);
    
    // string slice of dynamic_string
    let str_slice = &dynamic_name[0..5];
    println!("str_slice of dynamic_name is {}", str_slice);
    
    // mutable vector
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h'); // inserting a value with index
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l'); // adding an element at the end
    chars.push('o');
    println!("{:?}", chars); // printing out chars
    dbg!(&chars); // printing out in debug mode
    
    let removed_char = chars.pop().unwrap();
    println!("Removed char: {}", removed_char);

}
