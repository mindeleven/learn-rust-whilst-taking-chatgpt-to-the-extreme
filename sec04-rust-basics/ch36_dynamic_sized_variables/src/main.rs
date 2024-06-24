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
    println!("Chars after remove: {:?}", chars);

    // iterator
    chars.iter().for_each(|c| print!("{}", c));
    print!(" ain't a bad place to be\n");

    // vec! macro
    let chars_again: Vec<char> = vec![
        'a', 'l', 'l', ' ', 't', 'h', 'a', 't', 
        ' ', 'h', 'e', 'a', 'v', 'e', 'n', ' ', 
        'a', 'l', 'l', 'o', 'w', 's'
    ];
    dbg!(&chars_again);
    chars_again.iter().for_each(|c| print!("{}", c));
    println!("");

    // collecting into a string with the help of an iterator
    let collected: String = chars_again.iter().collect();
    dbg!(collected);
    
    // traditional for loop
    for c in chars_again {
        print!("{}", c);
        if c == 'l' {
            print!("=0x");
        }
    }
    println!("");

    // closures
    let num = 5;
    let add_num = |x: i32| x + num; // variable that can be called as a function
    let num_num = add_num(6);
    dbg!(num_num);

}
