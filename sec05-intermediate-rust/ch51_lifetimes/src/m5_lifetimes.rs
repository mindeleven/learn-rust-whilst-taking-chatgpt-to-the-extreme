#![allow(dead_code, unused_imports, unused_variables)]

fn example_1() {
    // allocate some space in memory
    let highest_age: i32;

    let alice_age = 20; // lifetime 'a
    let bob_age = 21; // 'b: 'a (carries the same lifetime as 'a)

    highest_age = largest(&alice_age, &bob_age);

    println!("highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1 // compare_1 is a reference so we dereference
        } else {
            *compare_2
        }
    }
}

fn example_2() {
    // allocate some space in memory
    let highest_age: &i32;

    let alice_age = 20; // lifetime 'a
    let bob_age = 21; // 'b: 'a (carries the same lifetime as 'a)

    highest_age = largest(&alice_age, &bob_age);

   println!("highest age is {}", highest_age);

    /* 
        following code/function error:
        missing lifetime specifier
        this function's return type contains a borrowed value, but the signature does 
        not say whether it is borrowed from `compare_1` or `compare_2`
    */
    /* fn largest(compare_1: &i32, compare_2: &i32) -> &i32 {
        if compare_1 > compare_2 {
            compare_1 // compare_1 is a reference so we dereference
        } else {
            compare_2
        }
    } */
    // once more with lifetime and lifetime generics
    // the problem with lifetimes arises once we return a reference to something
    fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1 // compare_1 is a reference so we dereference
        } else {
            compare_2
        }
    }

}

// now same as above with an internal code block
fn example_3() {
    // allocate some space in memory
    let highest_age: &i32;
    let age_dereferenced: i32; // holding an i32 that's not a reference

    let alice_age = 20; // lifetime 'a
    {
        let bob_age = 21; // 'b: 'a (carries the same lifetime as 'a)

        highest_age = largest(&alice_age, &bob_age);

        // bob_age does not life long enough so highest age cannot be used outside this scope
        // therefore passing highest_age while dereferencing it
        age_dereferenced = *highest_age; 
    }

    println!("highest age is {}", age_dereferenced); // using derefenerende variable

    fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1 // compare_1 is a reference so we dereference
        } else {
            compare_2
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;
    
    // cargo test tests_lifetimes -- --nocapture
    #[test] 
    fn tests_lifetimes() {

        dbg!("tests_lifetimes");
    
    }

}
