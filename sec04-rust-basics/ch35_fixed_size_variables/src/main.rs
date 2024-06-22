const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {

    println!("Welcome to our course \"{}\"", OUR_COURSE);
    
    // stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y = 4;
    println!("y is {}", y);
     
    // for loop with a range
    for i in 0..=y {
        print!("{} \n", i);
    }
    
    // mutation
    let mut z = 56;
    println!("z was {}", z);
    z = 66;
    println!("but now it is {}", z);

    // float
    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);

    // boolean & remainder
    let is_zero_remainder = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    // char
    let my_char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    // array stored on the stack
    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);
    
    // iterrating over array
    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

}
