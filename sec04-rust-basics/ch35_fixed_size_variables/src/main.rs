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
}
