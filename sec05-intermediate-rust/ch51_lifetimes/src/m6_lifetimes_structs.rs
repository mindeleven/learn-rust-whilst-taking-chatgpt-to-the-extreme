#![allow(dead_code, unused_imports, unused_variables)]

/// example: struct with references
/// struct needs to know how long references are going to live
/// struct needs lifetime specifier 
struct Person<'p> { 
    name: &'p str,
    points: &'p i32, // w/o lifetime: ´missing lifetime specifier´ error
}

// function example with generics
fn example_4_with_generics() {

    let highest_age: &i32;

    let alice_age = 20;
    let bob_age = 21; 

    highest_age = largest(&alice_age, &bob_age);

    println!("highest age is {}", highest_age);
    
    // specifying type T generic <'a, T>
    // without further specification we get an error: 
    // -> binary operation `>` cannot be applied to type `&T`
    // consider restricting type parameter `T`: `: std::cmp::PartialOrd`
    // type T needs to be a type the implements PartialOrd
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1 
        } else {
            compare_2
        }
    }

}